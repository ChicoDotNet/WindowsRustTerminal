# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$enginePath = Join-Path $repoRoot 'src\terminal\parser\OutputStateMachineEngine.cpp'
$expectedEngineBlob = '67fee22e76bc11d4cab68f9b0783d2f955a81aa7'

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
$includeAnchor = '#include "terminal_parser_ffi_output_csi_decrqcra.h"' + $nl
$engine = Replace-Once $engine $includeAnchor ($includeAnchor + '#include "terminal_parser_ffi_output_csi_rep.h"' + $nl) 'DECRQCRA include'

$routeAnchor = @"
    if (decrqcraPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_NONE)
    {
        _ClearLastChar();
        return true;
    }
    switch (id)
"@ -replace "`n", $nl
$route = @"
    if (decrqcraPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_rep_result repPlan{};
    const auto repStatus = terminal_parser_ffi_output_csi_rep_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        static_cast<uint16_t>(_lastPrintedChar),
        &repPlan);
    THROW_HR_IF(E_UNEXPECTED, repStatus != TERMINAL_PARSER_FFI_OK);

    switch (repPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_REPEAT:
    {
        const std::wstring repeated(static_cast<size_t>(repPlan.count), _lastPrintedChar);
        _dispatch->PrintString(repeated);
        break;
    }
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_HANDLED_NOOP:
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (repPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE)
    {
        _ClearLastChar();
        return true;
    }
    switch (id)
"@ -replace "`n", $nl
$engine = Replace-Once $engine $routeAnchor $route 'post-DECRQCRA route'

$legacy = @"
    case CsiActionCodes::REP_RepeatCharacter:
        // Handled w/o the dispatch. This function is unique in that way
        // If this were in the ITerminalDispatch, then each
        // implementation would effectively be the same, calling only
        // functions that are already part of the interface.
        // Print the last graphical character a number of times.
        if (_lastPrintedChar != AsciiChars::NUL)
        {
            const size_t repeatCount = parameters.at(0);
            std::wstring wstr(repeatCount, _lastPrintedChar);
            _dispatch->PrintString(wstr);
        }
        break;

"@ -replace "`n", $nl
$engine = Replace-Once $engine $legacy '' 'legacy REP case'
[System.IO.File]::WriteAllText($enginePath, $engine, [System.Text.UTF8Encoding]::new($false))

& (Join-Path $repoRoot 'tools\rust\Test-R09OutputCsiRepOwnership.ps1')
if ($LASTEXITCODE -ne 0) { throw 'REP ownership gate failed after swap.' }
Write-Host 'Prepared fail-closed REP Rust ownership candidate.'