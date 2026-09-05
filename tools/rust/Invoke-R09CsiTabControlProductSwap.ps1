$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'f832f9fa9715f0712cb95b9bbb7171f9fe15bf4c'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI tab control source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_erase.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_tab_control.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_tab_control.h'))
{
    throw 'CSI tab control include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (erasePlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (erasePlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    constexpr size_t tabControlPlanCapacity = 32;
    int32_t tabControlParameters[tabControlPlanCapacity]{};
    size_t tabControlParameterCount = 0;
    parameters.for_each([&](const auto tabControlType) {
        THROW_HR_IF(E_UNEXPECTED, tabControlParameterCount >= tabControlPlanCapacity);
        tabControlParameters[tabControlParameterCount++] = static_cast<int32_t>(tabControlType);
    });

    terminal_parser_ffi_output_csi_tab_control_result tabControlPlans[tabControlPlanCapacity]{};
    size_t tabControlPlanCount = 0;
    const auto tabControlStatus = terminal_parser_ffi_output_csi_tab_control_plans(
        static_cast<uint64_t>(id),
        tabControlParameters,
        tabControlParameterCount,
        tabControlPlans,
        tabControlPlanCapacity,
        &tabControlPlanCount);
    THROW_HR_IF(E_UNEXPECTED, tabControlStatus != TERMINAL_PARSER_FFI_OK);

    for (size_t index = 0; index < tabControlPlanCount; ++index)
    {
        const auto& tabControlPlan = tabControlPlans[index];

        switch (tabControlPlan.kind)
        {
        case TERMINAL_PARSER_FFI_OUTPUT_CSI_TAB_CONTROL_CLEAR:
            _dispatch->TabClear(tabControlPlan.value);
            break;
        case TERMINAL_PARSER_FFI_OUTPUT_CSI_TAB_CONTROL_SET:
            _dispatch->TabSet(tabControlPlan.value);
            break;
        default:
            THROW_HR(E_UNEXPECTED);
        }
    }

    if (tabControlPlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI tab control dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyTabControl = @"
    case CsiActionCodes::TBC_TabClear:
        parameters.for_each([&](const auto clearType) {
            _dispatch->TabClear(clearType);
        });
        break;
    case CsiActionCodes::DECST8C_SetTabEvery8Columns:
        parameters.for_each([&](const auto setType) {
            _dispatch->TabSet(setType);
        });
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyTabControl))
{
    throw 'Legacy CSI tab control cases marker mismatch.'
}
$text = $text.Replace($legacyTabControl, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI tab control source diff.'
}

$legacyCases = @(
    'case CsiActionCodes::TBC_TabClear:',
    'case CsiActionCodes::DECST8C_SetTabEvery8Columns:'
)
foreach ($legacyCase in $legacyCases)
{
    if ($text.Contains($legacyCase))
    {
        throw "Legacy CSI tab control case remains after candidate rewrite: $legacyCase"
    }
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_tab_control_plans'))
{
    throw 'CSI tab control Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI tab control Rust ownership candidate.'
