# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$enginePath = Join-Path $repoRoot 'src\terminal\parser\OutputStateMachineEngine.cpp'
$expectedBlob = '1b535cee2c1487844f4d42e6fac0712993223f5f'

$actualBlob = (& git -C $repoRoot hash-object -- 'src/terminal/parser/OutputStateMachineEngine.cpp').Trim()
if ($LASTEXITCODE -ne 0 -or $actualBlob -ne $expectedBlob)
{
    throw "OutputStateMachineEngine.cpp drifted before the R09 rectangular attribute ownership swap. Expected $expectedBlob, found $actualBlob."
}

$text = [System.IO.File]::ReadAllText($enginePath)
$newline = if ($text.Contains("`r`n")) { "`r`n" } else { "`n" }

function Normalize-Newlines
{
    param([Parameter(Mandatory = $true)][string]$Value)
    return [regex]::Replace($Value, "\r\n|\r|\n", $script:newline)
}

function Replace-ExactlyOnce
{
    param(
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][AllowEmptyString()][string]$Replacement,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $matches = [regex]::Matches($script:text, $Pattern, [System.Text.RegularExpressions.RegexOptions]::Multiline)
    if ($matches.Count -ne 1) { throw "Expected exactly one $Description match, found $($matches.Count)." }
    $script:text = [regex]::Replace($script:text, $Pattern, $Replacement, [System.Text.RegularExpressions.RegexOptions]::Multiline)
}

$includeReplacement = Normalize-Newlines -Value @'
#include "terminal_parser_ffi_output_csi_decac.h"
#include "terminal_parser_ffi_output_csi_rect_attributes.h"
'@
$includeReplacement += $newline
Replace-ExactlyOnce -Description 'DECAC include anchor' -Pattern '#include "terminal_parser_ffi_output_csi_decac\.h"\r?\n' -Replacement $includeReplacement

$plans = Normalize-Newlines -Value @'
    terminal_parser_ffi_output_csi_rect_attributes_result rectAttributesPlan{};
    const auto rectAttributesStatus = terminal_parser_ffi_output_csi_rect_attributes_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(2).value_or(0)),
        static_cast<int32_t>(parameters.at(3).value_or(0)),
        &rectAttributesPlan);
    THROW_HR_IF(E_UNEXPECTED, rectAttributesStatus != TERMINAL_PARSER_FFI_OK);

    switch (rectAttributesPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_CHANGE:
        _dispatch->ChangeAttributesRectangularArea(
            parameters.at(0),
            parameters.at(1),
            rectAttributesPlan.bottom,
            rectAttributesPlan.right,
            parameters.subspan(4));
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_REVERSE:
        _dispatch->ReverseAttributesRectangularArea(
            parameters.at(0),
            parameters.at(1),
            rectAttributesPlan.bottom,
            rectAttributesPlan.right,
            parameters.subspan(4));
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (rectAttributesPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_NONE)
    {
        _ClearLastChar();
        return true;
    }

'@

$legacySwitchAnchor = Normalize-Newlines -Value @'
    if (decacPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_NONE)
    {
        _ClearLastChar();
        return true;
    }
    switch (id)
    {
'@
$legacySwitchAnchorPattern = [regex]::Escape($legacySwitchAnchor)
$legacySwitchReplacement = ($legacySwitchAnchor.Substring(0, $legacySwitchAnchor.LastIndexOf('    switch (id)'))) + $plans + "    switch (id)$newline    {$newline"
Replace-ExactlyOnce -Description 'post-DECAC legacy CSI switch anchor' -Pattern $legacySwitchAnchorPattern -Replacement $legacySwitchReplacement

Replace-ExactlyOnce -Description 'legacy DECCARA case' -Pattern '    case CsiActionCodes::DECCARA_ChangeAttributesRectangularArea:\r?\n        _dispatch->ChangeAttributesRectangularArea\(parameters\.at\(0\), parameters\.at\(1\), parameters\.at\(2\)\.value_or\(0\), parameters\.at\(3\)\.value_or\(0\), parameters\.subspan\(4\)\);\r?\n        break;\r?\n' -Replacement ''
Replace-ExactlyOnce -Description 'legacy DECRARA case' -Pattern '    case CsiActionCodes::DECRARA_ReverseAttributesRectangularArea:\r?\n        _dispatch->ReverseAttributesRectangularArea\(parameters\.at\(0\), parameters\.at\(1\), parameters\.at\(2\)\.value_or\(0\), parameters\.at\(3\)\.value_or\(0\), parameters\.subspan\(4\)\);\r?\n        break;\r?\n' -Replacement ''

[System.IO.File]::WriteAllText($enginePath, $text, [System.Text.UTF8Encoding]::new($false))
$updated = [System.IO.File]::ReadAllText($enginePath)
foreach ($required in @(
    'terminal_parser_ffi_output_csi_rect_attributes.h',
    'terminal_parser_ffi_output_csi_rect_attributes_plan(',
    'TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_CHANGE',
    'TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_REVERSE',
    'parameters.at(0),',
    'parameters.at(1),',
    'rectAttributesPlan.bottom,',
    'rectAttributesPlan.right,',
    'parameters.subspan(4));'
))
{
    if (-not $updated.Contains($required)) { throw "R09 rectangular attribute candidate is missing required marker: $required" }
}
foreach ($legacy in @('case CsiActionCodes::DECCARA_ChangeAttributesRectangularArea:', 'case CsiActionCodes::DECRARA_ReverseAttributesRectangularArea:'))
{
    if ($updated.Contains($legacy)) { throw "R09 rectangular attribute candidate still contains legacy ownership: $legacy" }
}
Write-Host 'Prepared fail-closed DECCARA + DECRARA Rust ownership candidate.'
