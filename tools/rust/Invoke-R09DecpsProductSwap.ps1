# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$enginePath = Join-Path $repoRoot 'src\terminal\parser\OutputStateMachineEngine.cpp'
$expectedEngineBlob = '6dde264a0f2d54debf4f720d805663e355653eee'

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

$includeAnchor = '#include "terminal_parser_ffi_output_csi_rep.h"' + $nl
$engine = Replace-Once $engine $includeAnchor ($includeAnchor + '#include "terminal_parser_ffi_output_csi_decps.h"' + $nl) 'REP include'

$routeAnchor = @"
    if (repPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE)
    {
        _ClearLastChar();
        return true;
    }
    switch (id)
"@ -replace "`n", $nl
$route = @"
    if (repPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_decps_result decpsPlan{};
    const auto decpsStatus = terminal_parser_ffi_output_csi_decps_plan(
        static_cast<uint64_t>(id),
        &decpsPlan);
    THROW_HR_IF(E_UNEXPECTED, decpsStatus != TERMINAL_PARSER_FFI_OK);

    switch (decpsPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_PLAY_SOUNDS:
        _dispatch->PlaySounds(parameters);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (decpsPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_NONE)
    {
        _ClearLastChar();
        return true;
    }
    switch (id)
"@ -replace "`n", $nl
$engine = Replace-Once $engine $routeAnchor $route 'post-REP route'

$legacy = @"
    case CsiActionCodes::DECPS_PlaySound:
        _dispatch->PlaySounds(parameters);
        break;

"@ -replace "`n", $nl
$engine = Replace-Once $engine $legacy '' 'legacy DECPS case'

$switchStartAnchor = "    switch (id)$nl    {$nl"
$switchStart = $engine.IndexOf($switchStartAnchor, [System.StringComparison]::Ordinal)
if ($switchStart -lt 0 -or $engine.IndexOf($switchStartAnchor, $switchStart + $switchStartAnchor.Length, [System.StringComparison]::Ordinal) -ge 0) {
    throw 'Expected exactly one residual CSI switch after DECPS removal.'
}
$defaultBlock = "    default:$nl        _dispatch->UnknownSequence();$nl        break;$nl    }$nl"
$defaultStart = $engine.IndexOf($defaultBlock, $switchStart + $switchStartAnchor.Length, [System.StringComparison]::Ordinal)
if ($defaultStart -lt 0) { throw 'Residual CSI switch default block missing.' }
$residualBody = $engine.Substring($switchStart + $switchStartAnchor.Length, $defaultStart - ($switchStart + $switchStartAnchor.Length))
if (-not [string]::IsNullOrWhiteSpace($residualBody)) { throw 'Residual CSI switch still contains product-owned cases.' }
$emptySwitch = $engine.Substring($switchStart, ($defaultStart + $defaultBlock.Length) - $switchStart)
$engine = Replace-Once $engine $emptySwitch ("    _dispatch->UnknownSequence();$nl") 'default-only residual CSI switch'

[System.IO.File]::WriteAllText($enginePath, $engine, [System.Text.UTF8Encoding]::new($false))

& (Join-Path $repoRoot 'tools\rust\Test-R09OutputCsiDecpsOwnership.ps1')
if ($LASTEXITCODE -ne 0) { throw 'DECPS ownership gate failed after swap.' }
Write-Host 'Prepared fail-closed DECPS Rust ownership candidate.'
