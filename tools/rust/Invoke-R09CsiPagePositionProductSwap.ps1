$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'ee3a694d32de9559b0e69cbfa7e051c0cae2b64a'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI page positioning source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_page.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_page_position.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_page_position.h'))
{
    throw 'CSI page positioning include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (pagePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (pagePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_page_position_result pagePositionPlan{};
    const auto pagePositionStatus = terminal_parser_ffi_output_csi_page_position_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &pagePositionPlan);
    THROW_HR_IF(E_UNEXPECTED, pagePositionStatus != TERMINAL_PARSER_FFI_OK);

    switch (pagePositionPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_POSITION_ABSOLUTE:
        _dispatch->PagePositionAbsolute(pagePositionPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_POSITION_RELATIVE:
        _dispatch->PagePositionRelative(pagePositionPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_POSITION_BACK:
        _dispatch->PagePositionBack(pagePositionPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_POSITION_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (pagePositionPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_POSITION_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI page positioning dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyPagePosition = @"
    case CsiActionCodes::PPA_PagePositionAbsolute:
        _dispatch->PagePositionAbsolute(parameters.at(0));
        break;
    case CsiActionCodes::PPR_PagePositionRelative:
        _dispatch->PagePositionRelative(parameters.at(0));
        break;
    case CsiActionCodes::PPB_PagePositionBack:
        _dispatch->PagePositionBack(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyPagePosition))
{
    throw 'Legacy CSI page positioning case marker mismatch.'
}
$text = $text.Replace($legacyPagePosition, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI page positioning source diff.'
}

if ($text.Contains('case CsiActionCodes::PPA_PagePositionAbsolute:') -or
    $text.Contains('case CsiActionCodes::PPR_PagePositionRelative:') -or
    $text.Contains('case CsiActionCodes::PPB_PagePositionBack:'))
{
    throw 'Legacy CSI page positioning cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_page_position_plan'))
{
    throw 'CSI page positioning Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI page positioning Rust ownership candidate.'
