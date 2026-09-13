# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$enginePath = Join-Path $repoRoot 'src\terminal\parser\OutputStateMachineEngine.cpp'
$expectedEngineBlob = '4d57a64f3d62fe9e22f7ef2686d91cdaeb694c48'

function Assert-Blob([string]$Path, [string]$Expected, [string]$Description) {
    $actual = (& git -C $repoRoot hash-object -- $Path).Trim()
    if ($LASTEXITCODE -ne 0 -or $actual -ne $Expected) { throw "$Description drifted. Expected $Expected, found $actual." }
}
Assert-Blob 'src/terminal/parser/OutputStateMachineEngine.cpp' $expectedEngineBlob 'OutputStateMachineEngine.cpp'

function Replace-Once([string]$Text, [string]$Old, [string]$New, [string]$Description) {
    $first = $Text.IndexOf($Old, [System.StringComparison]::Ordinal)
    if ($first -lt 0 -or $Text.IndexOf($Old, $first + $Old.Length, [System.StringComparison]::Ordinal) -ge 0) { throw "Expected exactly one $Description anchor." }
    return $Text.Substring(0, $first) + $New + $Text.Substring($first + $Old.Length)
}

$engine = [System.IO.File]::ReadAllText($enginePath)
$nl = if ($engine.Contains("`r`n")) { "`r`n" } else { "`n" }
$includeAnchor = '#include "terminal_parser_ffi_output_csi_rect_attributes.h"' + $nl
$engine = Replace-Once $engine $includeAnchor ($includeAnchor + '#include "terminal_parser_ffi_output_csi_decrqcra.h"' + $nl) 'rectangular-attributes include'

$routeAnchor = @"
    if (rectAttributesPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_NONE)
    {
        _ClearLastChar();
        return true;
    }
    switch (id)
"@ -replace "`n", $nl
$route = @"
    if (rectAttributesPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_decrqcra_result decrqcraPlan{};
    const auto decrqcraStatus = terminal_parser_ffi_output_csi_decrqcra_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        static_cast<int32_t>(parameters.at(1).value_or(0)),
        static_cast<int32_t>(parameters.at(4).value_or(0)),
        static_cast<int32_t>(parameters.at(5).value_or(0)),
        &decrqcraPlan);
    THROW_HR_IF(E_UNEXPECTED, decrqcraStatus != TERMINAL_PARSER_FFI_OK);

    switch (decrqcraPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_REQUEST_CHECKSUM_RECTANGULAR_AREA:
        _dispatch->RequestChecksumRectangularArea(
            decrqcraPlan.request_id,
            decrqcraPlan.page,
            parameters.at(2),
            parameters.at(3),
            decrqcraPlan.bottom,
            decrqcraPlan.right);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (decrqcraPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_NONE)
    {
        _ClearLastChar();
        return true;
    }
    switch (id)
"@ -replace "`n", $nl
$engine = Replace-Once $engine $routeAnchor $route 'post-rectangular-attributes route'
$legacy = @"
    case CsiActionCodes::DECRQCRA_RequestChecksumRectangularArea:
        _dispatch->RequestChecksumRectangularArea(parameters.at(0).value_or(0), parameters.at(1).value_or(0), parameters.at(2), parameters.at(3), parameters.at(4).value_or(0), parameters.at(5).value_or(0));
        break;

"@ -replace "`n", $nl
$engine = Replace-Once $engine $legacy '' 'legacy DECRQCRA case'
[System.IO.File]::WriteAllText($enginePath, $engine, [System.Text.UTF8Encoding]::new($false))

& (Join-Path $repoRoot 'tools\rust\Test-R09OutputCsiDecrqcraOwnership.ps1')
if ($LASTEXITCODE -ne 0) { throw 'DECRQCRA ownership gate failed after swap.' }
Write-Host 'Prepared fail-closed DECRQCRA Rust ownership candidate.'
