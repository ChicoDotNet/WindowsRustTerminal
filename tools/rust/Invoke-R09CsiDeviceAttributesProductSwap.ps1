$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '389d366803d128c42d38dcbf39592c4c5f151e60'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI device attributes source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_terminal_parameters.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_device_attributes.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_device_attributes.h'))
{
    throw 'CSI device attributes include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (terminalParametersPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_TERMINAL_PARAMETERS_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (terminalParametersPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_TERMINAL_PARAMETERS_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_device_attributes_result deviceAttributesPlan{};
    const auto deviceAttributesStatus = terminal_parser_ffi_output_csi_device_attributes_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &deviceAttributesPlan);
    THROW_HR_IF(E_UNEXPECTED, deviceAttributesStatus != TERMINAL_PARSER_FFI_OK);

    switch (deviceAttributesPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_ATTRIBUTES_PRIMARY:
        _dispatch->DeviceAttributes();
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_ATTRIBUTES_SECONDARY:
        _dispatch->SecondaryDeviceAttributes();
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_ATTRIBUTES_TERTIARY:
        _dispatch->TertiaryDeviceAttributes();
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_ATTRIBUTES_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (deviceAttributesPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_ATTRIBUTES_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI device attributes dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyDeviceAttributes = @"
    case CsiActionCodes::DA_DeviceAttributes:
        if (parameters.at(0).value_or(0) == 0)
        {
            _dispatch->DeviceAttributes();
        }
        break;
    case CsiActionCodes::DA2_SecondaryDeviceAttributes:
        if (parameters.at(0).value_or(0) == 0)
        {
            _dispatch->SecondaryDeviceAttributes();
        }
        break;
    case CsiActionCodes::DA3_TertiaryDeviceAttributes:
        if (parameters.at(0).value_or(0) == 0)
        {
            _dispatch->TertiaryDeviceAttributes();
        }
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyDeviceAttributes))
{
    throw 'Legacy CSI device attributes case marker mismatch.'
}
$text = $text.Replace($legacyDeviceAttributes, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI device attributes source diff.'
}

if ($text.Contains('case CsiActionCodes::DA_DeviceAttributes:') -or
    $text.Contains('case CsiActionCodes::DA2_SecondaryDeviceAttributes:') -or
    $text.Contains('case CsiActionCodes::DA3_TertiaryDeviceAttributes:'))
{
    throw 'Legacy CSI device attributes cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_device_attributes_plan'))
{
    throw 'CSI device attributes Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI device attributes Rust ownership candidate.'
