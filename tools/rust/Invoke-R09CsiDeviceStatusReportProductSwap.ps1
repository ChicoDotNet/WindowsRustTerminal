$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '5b620bfebc004496af21a4199913ddca0e78be76'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI device status report source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_request_mode.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_device_status_report.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_device_status_report.h'))
{
    throw 'CSI device status report include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (requestModePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_MODE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (requestModePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_MODE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_device_status_report_result deviceStatusReportPlan{};
    const auto deviceStatusReportStatus = terminal_parser_ffi_output_csi_device_status_report_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        parameters.at(1).has_value() ? 1u : 0u,
        static_cast<int32_t>(parameters.at(1).value_or(0)),
        &deviceStatusReportPlan);
    THROW_HR_IF(E_UNEXPECTED, deviceStatusReportStatus != TERMINAL_PARSER_FFI_OK);

    switch (deviceStatusReportPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_STATUS_REPORT_REPORT:
    {
        const auto reportId = deviceStatusReportPlan.has_id != 0 ? VTParameter{ deviceStatusReportPlan.id } : VTParameter{};
        if (deviceStatusReportPlan.private_mode != 0)
        {
            _dispatch->DeviceStatusReport(DispatchTypes::DECPrivateStatus(deviceStatusReportPlan.status), reportId);
        }
        else
        {
            _dispatch->DeviceStatusReport(DispatchTypes::ANSIStandardStatus(deviceStatusReportPlan.status), reportId);
        }
        break;
    }
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_STATUS_REPORT_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (deviceStatusReportPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_STATUS_REPORT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI device status report dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyDeviceStatusReport = @"
    case CsiActionCodes::DSR_DeviceStatusReport:
        _dispatch->DeviceStatusReport(DispatchTypes::ANSIStandardStatus(parameters.at(0)), parameters.at(1));
        break;
    case CsiActionCodes::DSR_PrivateDeviceStatusReport:
        _dispatch->DeviceStatusReport(DispatchTypes::DECPrivateStatus(parameters.at(0)), parameters.at(1));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyDeviceStatusReport))
{
    throw 'Legacy CSI device status report cases marker mismatch.'
}
$text = $text.Replace($legacyDeviceStatusReport, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI device status report source diff.'
}

if ($text.Contains('case CsiActionCodes::DSR_DeviceStatusReport:') -or $text.Contains('case CsiActionCodes::DSR_PrivateDeviceStatusReport:'))
{
    throw 'Legacy CSI device status report cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_device_status_report_plan'))
{
    throw 'CSI device status report Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI device status report Rust ownership candidate.'
