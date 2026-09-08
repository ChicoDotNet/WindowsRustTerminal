# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$enginePath = Join-Path $repoRoot 'src\terminal\parser\OutputStateMachineEngine.cpp'
$expectedBlob = '1f24678eacce1ae50eb923a4362407e2b3e987a0'

$actualBlob = (& git -C $repoRoot hash-object -- 'src/terminal/parser/OutputStateMachineEngine.cpp').Trim()
if ($LASTEXITCODE -ne 0 -or $actualBlob -ne $expectedBlob)
{
    throw "OutputStateMachineEngine.cpp drifted before the R09 report/color ownership swap. Expected $expectedBlob, found $actualBlob."
}

$text = [System.IO.File]::ReadAllText($enginePath)

function Replace-ExactlyOnce
{
    param(
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Replacement,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $matches = [regex]::Matches($script:text, $Pattern, [System.Text.RegularExpressions.RegexOptions]::Multiline)
    if ($matches.Count -ne 1)
    {
        throw "Expected exactly one $Description match, found $($matches.Count)."
    }

    $script:text = [regex]::Replace(
        $script:text,
        $Pattern,
        $Replacement,
        [System.Text.RegularExpressions.RegexOptions]::Multiline)
}

$includeReplacement = @'
#include "terminal_parser_ffi_output_csi_decinvm.h"
#include "terminal_parser_ffi_output_csi_decrqtsr.h"
#include "terminal_parser_ffi_output_csi_decac.h"
'@ -replace "`n", "`r`n"
$includeReplacement += "`r`n"

Replace-ExactlyOnce `
    -Description 'DECINVM include anchor' `
    -Pattern '#include "terminal_parser_ffi_output_csi_decinvm\.h"\r?\n' `
    -Replacement $includeReplacement

$plans = @'
    const auto decrqtsrReportFormatParameter = parameters.at(1);
    terminal_parser_ffi_output_csi_decrqtsr_result decrqtsrPlan{};
    const auto decrqtsrStatus = terminal_parser_ffi_output_csi_decrqtsr_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        decrqtsrReportFormatParameter.has_value() ? static_cast<int32_t>(decrqtsrReportFormatParameter.value_or(0)) : TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REPORT_FORMAT_OMITTED,
        &decrqtsrPlan);
    THROW_HR_IF(E_UNEXPECTED, decrqtsrStatus != TERMINAL_PARSER_FFI_OK);

    switch (decrqtsrPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_KIND_REQUEST_TERMINAL_STATE_REPORT:
        _dispatch->RequestTerminalStateReport(
            decrqtsrPlan.format,
            decrqtsrPlan.report_format == TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REPORT_FORMAT_OMITTED ?
                std::optional<VTInt>{} :
                std::optional<VTInt>{ decrqtsrPlan.report_format });
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_KIND_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (decrqtsrPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_KIND_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_decac_result decacPlan{};
    const auto decacStatus = terminal_parser_ffi_output_csi_decac_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        static_cast<int32_t>(parameters.at(1).value_or(0)),
        static_cast<int32_t>(parameters.at(2).value_or(0)),
        &decacPlan);
    THROW_HR_IF(E_UNEXPECTED, decacStatus != TERMINAL_PARSER_FFI_OK);

    switch (decacPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_KIND_ASSIGN_COLOR:
        _dispatch->AssignColor(
            static_cast<DispatchTypes::ColorItem>(decacPlan.item),
            decacPlan.foreground_index,
            decacPlan.background_index);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_KIND_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (decacPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_KIND_NONE)
    {
        _ClearLastChar();
        return true;
    }

'@

$legacySwitchAnchor = @'
    if (decinvmPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECINVM_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
    {
'@
$legacySwitchAnchorPattern = [regex]::Escape($legacySwitchAnchor) -replace '\\r\\n', '\r?\n' -replace '\\n', '\r?\n'
$legacySwitchReplacement = ($legacySwitchAnchor.Substring(0, $legacySwitchAnchor.LastIndexOf('    switch (id)'))) + $plans + "    switch (id)`r`n    {`r`n"

Replace-ExactlyOnce `
    -Description 'post-DECINVM legacy CSI switch anchor' `
    -Pattern $legacySwitchAnchorPattern `
    -Replacement $legacySwitchReplacement

Replace-ExactlyOnce `
    -Description 'legacy DECRQTSR case' `
    -Pattern '    case CsiActionCodes::DECRQTSR_RequestTerminalStateReport:\r?\n        _dispatch->RequestTerminalStateReport\(parameters\.at\(0\), parameters\.at\(1\)\);\r?\n        break;\r?\n' `
    -Replacement ''

Replace-ExactlyOnce `
    -Description 'legacy DECAC case' `
    -Pattern '    case CsiActionCodes::DECAC_AssignColor:\r?\n        _dispatch->AssignColor\(parameters\.at\(0\), parameters\.at\(1\)\.value_or\(0\), parameters\.at\(2\)\.value_or\(0\)\);\r?\n        break;\r?\n' `
    -Replacement ''

[System.IO.File]::WriteAllText($enginePath, $text, [System.Text.UTF8Encoding]::new($false))

$updated = [System.IO.File]::ReadAllText($enginePath)
foreach ($required in @(
    'terminal_parser_ffi_output_csi_decrqtsr.h',
    'terminal_parser_ffi_output_csi_decrqtsr_plan(',
    'TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REPORT_FORMAT_OMITTED',
    'terminal_parser_ffi_output_csi_decac.h',
    'terminal_parser_ffi_output_csi_decac_plan(',
    'static_cast<DispatchTypes::ColorItem>(decacPlan.item)'
))
{
    if (-not $updated.Contains($required))
    {
        throw "R09 report/color candidate is missing required marker: $required"
    }
}

foreach ($legacy in @(
    'case CsiActionCodes::DECRQTSR_RequestTerminalStateReport:',
    'case CsiActionCodes::DECAC_AssignColor:'
))
{
    if ($updated.Contains($legacy))
    {
        throw "R09 report/color candidate still contains legacy ownership: $legacy"
    }
}

Write-Host 'Prepared fail-closed DECRQTSR + DECAC Rust ownership candidate.'
