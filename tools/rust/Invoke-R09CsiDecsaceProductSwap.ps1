$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '09763485c85efb242f1e991b40e180f781a519f3'

$actualBlob = (git rev-parse "HEAD:$source").Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI DECSACE source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source) -replace "`r`n", "`n"

$includeOld = '#include "terminal_parser_ffi_output_csi_request_presentation_state.h"' + "`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_decsace.h"' + "`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_decsace.h'))
{
    throw 'CSI DECSACE include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (presentationStatePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

$dispatchReplacement = @"
    if (presentationStatePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_decsace_result decsacePlan{};
    const auto decsaceStatus = terminal_parser_ffi_output_csi_decsace_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &decsacePlan);
    THROW_HR_IF(E_UNEXPECTED, decsaceStatus != TERMINAL_PARSER_FFI_OK);

    switch (decsacePlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_SELECT_ATTRIBUTE_CHANGE_EXTENT:
        _dispatch->SelectAttributeChangeExtent(
            static_cast<DispatchTypes::ChangeExtent>(decsacePlan.change_extent));
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (decsacePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI DECSACE dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCase = @"
    case CsiActionCodes::DECSACE_SelectAttributeChangeExtent:
        _dispatch->SelectAttributeChangeExtent(parameters.at(0));
        break;
"@ -replace "`r`n", "`n"

if (-not $text.Contains($legacyCase))
{
    throw 'Legacy CSI DECSACE case marker mismatch.'
}
$text = $text.Replace($legacyCase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI DECSACE source diff.'
}

if ($text.Contains('case CsiActionCodes::DECSACE_SelectAttributeChangeExtent:'))
{
    throw 'Legacy CSI DECSACE case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_decsace_plan'))
{
    throw 'CSI DECSACE Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CSI DECSACE Rust ownership candidate.'
