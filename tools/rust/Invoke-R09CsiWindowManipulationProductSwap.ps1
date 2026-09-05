$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '5de56f7abb2e4d7678036bed0d6e19ca370f9461'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI window manipulation source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_tab_control.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_window_manipulation.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_window_manipulation.h'))
{
    throw 'CSI window manipulation include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (tabControlPlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (tabControlPlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_window_manipulation_result windowManipulationPlan{};
    const auto windowManipulationStatus = terminal_parser_ffi_output_csi_window_manipulation_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        static_cast<int32_t>(parameters.at(1).value_or(0)),
        static_cast<int32_t>(parameters.at(2).value_or(0)),
        &windowManipulationPlan);
    THROW_HR_IF(E_UNEXPECTED, windowManipulationStatus != TERMINAL_PARSER_FFI_OK);

    switch (windowManipulationPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_WINDOW_MANIPULATION:
        _dispatch->WindowManipulation(static_cast<DispatchTypes::WindowManipulationType>(windowManipulationPlan.function), windowManipulationPlan.parameter1, windowManipulationPlan.parameter2);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_WINDOW_MANIPULATION_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (windowManipulationPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_WINDOW_MANIPULATION_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI window manipulation dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyWindowManipulation = @"
    case CsiActionCodes::DTTERM_WindowManipulation:
        _dispatch->WindowManipulation(parameters.at(0), parameters.at(1), parameters.at(2));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyWindowManipulation))
{
    throw 'Legacy CSI window manipulation case marker mismatch.'
}
$text = $text.Replace($legacyWindowManipulation, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI window manipulation source diff.'
}

if ($text.Contains('case CsiActionCodes::DTTERM_WindowManipulation:'))
{
    throw 'Legacy CSI window manipulation case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_window_manipulation_plan'))
{
    throw 'CSI window manipulation Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI window manipulation Rust ownership candidate.'