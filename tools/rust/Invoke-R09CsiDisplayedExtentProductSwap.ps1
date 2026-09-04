$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'a8b66155749beb373c13b15df7c26514de36fc99'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI displayed extent source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_soft_reset.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_displayed_extent.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_displayed_extent.h'))
{
    throw 'CSI displayed extent include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (softResetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_SOFT_RESET_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (softResetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_SOFT_RESET_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_displayed_extent_result displayedExtentPlan{};
    const auto displayedExtentStatus = terminal_parser_ffi_output_csi_displayed_extent_plan(
        static_cast<uint64_t>(id),
        &displayedExtentPlan);
    THROW_HR_IF(E_UNEXPECTED, displayedExtentStatus != TERMINAL_PARSER_FFI_OK);

    switch (displayedExtentPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DISPLAYED_EXTENT_REQUEST:
        _dispatch->RequestDisplayedExtent();
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DISPLAYED_EXTENT_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (displayedExtentPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DISPLAYED_EXTENT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI displayed extent dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyDisplayedExtent = @"
    case CsiActionCodes::DECRQDE_RequestDisplayedExtent:
        _dispatch->RequestDisplayedExtent();
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyDisplayedExtent))
{
    throw 'Legacy CSI displayed extent case marker mismatch.'
}
$text = $text.Replace($legacyDisplayedExtent, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI displayed extent source diff.'
}

if ($text.Contains('case CsiActionCodes::DECRQDE_RequestDisplayedExtent:'))
{
    throw 'Legacy CSI displayed extent case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_displayed_extent_plan'))
{
    throw 'CSI displayed extent Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI displayed extent Rust ownership candidate.'
