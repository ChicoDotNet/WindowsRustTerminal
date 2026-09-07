$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '6fdcae4d82dae63f1d802bc7f334bd5204769c49'

$actualBlob = (git rev-parse "HEAD:$source").Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI presentation state request source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source) -replace "`r`n", "`n"

$includeOld = '#include "terminal_parser_ffi_output_csi_request_mode.h"' + "`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_request_presentation_state.h"' + "`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_request_presentation_state.h'))
{
    throw 'CSI presentation state request include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (kittyKeyboardSetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

$dispatchReplacement = @"
    if (kittyKeyboardSetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_request_presentation_state_result presentationStatePlan{};
    const auto presentationStateStatus = terminal_parser_ffi_output_csi_request_presentation_state_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &presentationStatePlan);
    THROW_HR_IF(E_UNEXPECTED, presentationStateStatus != TERMINAL_PARSER_FFI_OK);

    switch (presentationStatePlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_REQUEST:
        _dispatch->RequestPresentationStateReport(
            static_cast<DispatchTypes::PresentationReportFormat>(presentationStatePlan.format));
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (presentationStatePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI presentation state request dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCase = @"
    case CsiActionCodes::DECRQPSR_RequestPresentationStateReport:
        _dispatch->RequestPresentationStateReport(parameters.at(0));
        break;
"@ -replace "`r`n", "`n"

if (-not $text.Contains($legacyCase))
{
    throw 'Legacy CSI presentation state request case marker mismatch.'
}
$text = $text.Replace($legacyCase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI presentation state request source diff.'
}

if ($text.Contains('case CsiActionCodes::DECRQPSR_RequestPresentationStateReport:'))
{
    throw 'Legacy CSI presentation state request case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_request_presentation_state_plan'))
{
    throw 'CSI presentation state request Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CSI presentation state request Rust ownership candidate.'
