$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'ff673c8013f662e4cadaa9a849aa2a1751eef65a'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI mode source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_device_status_report.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_mode.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_mode.h'))
{
    throw 'CSI mode include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (deviceStatusReportPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_STATUS_REPORT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (deviceStatusReportPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_STATUS_REPORT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    constexpr size_t modePlanCapacity = 32;
    int32_t modeParameters[modePlanCapacity]{};
    size_t modeParameterCount = 0;
    parameters.for_each([&](const auto mode) {
        THROW_HR_IF(E_UNEXPECTED, modeParameterCount >= modePlanCapacity);
        modeParameters[modeParameterCount++] = static_cast<int32_t>(mode);
    });

    terminal_parser_ffi_output_csi_mode_result modePlans[modePlanCapacity]{};
    size_t modePlanCount = 0;
    const auto modeStatus = terminal_parser_ffi_output_csi_mode_plans(
        static_cast<uint64_t>(id),
        modeParameters,
        modeParameterCount,
        modePlans,
        modePlanCapacity,
        &modePlanCount);
    THROW_HR_IF(E_UNEXPECTED, modeStatus != TERMINAL_PARSER_FFI_OK);

    for (size_t index = 0; index < modePlanCount; ++index)
    {
        const auto& modePlan = modePlans[index];
        THROW_HR_IF(E_UNEXPECTED, modePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_MODE_MODE);

        if (modePlan.private_mode != 0)
        {
            if (modePlan.enabled != 0)
            {
                _dispatch->SetMode(static_cast<DispatchTypes::DECPrivateMode>(modePlan.mode));
            }
            else
            {
                _dispatch->ResetMode(static_cast<DispatchTypes::DECPrivateMode>(modePlan.mode));
            }
        }
        else if (modePlan.enabled != 0)
        {
            _dispatch->SetMode(static_cast<DispatchTypes::ANSIStandardMode>(modePlan.mode));
        }
        else
        {
            _dispatch->ResetMode(static_cast<DispatchTypes::ANSIStandardMode>(modePlan.mode));
        }
    }

    if (modePlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI mode dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyModes = @"
    case CsiActionCodes::SM_SetMode:
        parameters.for_each([&](const auto mode) {
            _dispatch->SetMode(DispatchTypes::ANSIStandardMode(mode));
        });
        break;
    case CsiActionCodes::DECSET_PrivateModeSet:
        parameters.for_each([&](const auto mode) {
            _dispatch->SetMode(DispatchTypes::DECPrivateMode(mode));
        });
        break;
    case CsiActionCodes::RM_ResetMode:
        parameters.for_each([&](const auto mode) {
            _dispatch->ResetMode(DispatchTypes::ANSIStandardMode(mode));
        });
        break;
    case CsiActionCodes::DECRST_PrivateModeReset:
        parameters.for_each([&](const auto mode) {
            _dispatch->ResetMode(DispatchTypes::DECPrivateMode(mode));
        });
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyModes))
{
    throw 'Legacy CSI mode cases marker mismatch.'
}
$text = $text.Replace($legacyModes, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI mode source diff.'
}

$legacyCases = @(
    'case CsiActionCodes::SM_SetMode:',
    'case CsiActionCodes::DECSET_PrivateModeSet:',
    'case CsiActionCodes::RM_ResetMode:',
    'case CsiActionCodes::DECRST_PrivateModeReset:'
)
foreach ($legacyCase in $legacyCases)
{
    if ($text.Contains($legacyCase))
    {
        throw "Legacy CSI mode case remains after candidate rewrite: $legacyCase"
    }
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_mode_plans'))
{
    throw 'CSI mode Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI mode Rust ownership candidate.'
