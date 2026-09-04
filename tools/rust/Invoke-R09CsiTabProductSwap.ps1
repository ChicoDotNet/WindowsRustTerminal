$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'caf4470dfc4f8b92b05b65f975997bab3f60941a'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI tab movement source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_page_position.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_tab.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_tab.h'))
{
    throw 'CSI tab movement include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (pagePositionPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_POSITION_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (pagePositionPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_PAGE_POSITION_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_tab_result tabPlan{};
    const auto tabStatus = terminal_parser_ffi_output_csi_tab_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &tabPlan);
    THROW_HR_IF(E_UNEXPECTED, tabStatus != TERMINAL_PARSER_FFI_OK);

    switch (tabPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_TAB_FORWARD:
        _dispatch->ForwardTab(tabPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_TAB_BACKWARD:
        _dispatch->BackwardsTab(tabPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_TAB_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (tabPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_TAB_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI tab movement dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyTabMovement = @"
    case CsiActionCodes::CHT_CursorForwardTab:
        _dispatch->ForwardTab(parameters.at(0));
        break;
    case CsiActionCodes::CBT_CursorBackTab:
        _dispatch->BackwardsTab(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyTabMovement))
{
    throw 'Legacy CSI tab movement case marker mismatch.'
}
$text = $text.Replace($legacyTabMovement, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI tab movement source diff.'
}

if ($text.Contains('case CsiActionCodes::CHT_CursorForwardTab:') -or
    $text.Contains('case CsiActionCodes::CBT_CursorBackTab:'))
{
    throw 'Legacy CSI tab movement cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_tab_plan'))
{
    throw 'CSI tab movement Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI tab movement Rust ownership candidate.'
