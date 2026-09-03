$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '75224e6f39abaad7271e141d60249e7d9a32de99'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI page navigation source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_scroll.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_page.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_page.h'))
{
    throw 'CSI page navigation include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (scrollPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_SCROLL_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (scrollPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_SCROLL_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_page_result pagePlan{};
    const auto pageStatus = terminal_parser_ffi_output_csi_page_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &pagePlan);
    THROW_HR_IF(E_UNEXPECTED, pageStatus != TERMINAL_PARSER_FFI_OK);

    switch (pagePlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_NEXT:
        _dispatch->NextPage(pagePlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_PRECEDING:
        _dispatch->PrecedingPage(pagePlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (pagePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI page navigation dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyPage = @"
    case CsiActionCodes::NP_NextPage:
        _dispatch->NextPage(parameters.at(0));
        break;
    case CsiActionCodes::PP_PrecedingPage:
        _dispatch->PrecedingPage(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyPage))
{
    throw 'Legacy CSI page navigation case marker mismatch.'
}
$text = $text.Replace($legacyPage, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI page navigation source diff.'
}

if ($text.Contains('case CsiActionCodes::NP_NextPage:') -or $text.Contains('case CsiActionCodes::PP_PrecedingPage:'))
{
    throw 'Legacy CSI page navigation cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_page_plan'))
{
    throw 'CSI page navigation Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI page navigation Rust ownership candidate.'
