# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$cppPath = Join-Path $repoRoot 'src/host/ConsoleArguments.cpp'
$hppPath = Join-Path $repoRoot 'src/host/ConsoleArguments.hpp'

$cpp = Get-Content -Raw -LiteralPath $cppPath
$hpp = Get-Content -Raw -LiteralPath $hppPath

$requiredCpp = @(
    '#include "console_arguments_ffi.h"',
    'CommandLineToArgvW',
    'terminal_parser_ffi_console_arguments_plan'
)

$legacySymbols = @(
    'EscapeArgument',
    '_GetClientCommandline',
    's_ConsumeArg',
    's_GetArgumentValue',
    's_HandleFeatureValue',
    's_ParseHandleArg'
)

$failures = [System.Collections.Generic.List[string]]::new()
foreach ($needle in $requiredCpp)
{
    if (-not $cpp.Contains($needle))
    {
        $failures.Add("ConsoleArguments.cpp must retain the Rust product route marker: $needle")
    }
}

foreach ($symbol in $legacySymbols)
{
    if ($cpp.Contains($symbol) -or $hpp.Contains($symbol))
    {
        $failures.Add("Legacy deterministic parser symbol must not return to ConsoleArguments C++: $symbol")
    }
}

if ($failures.Count -ne 0)
{
    $failures | ForEach-Object { Write-Error $_ }
    throw "R09 ConsoleArguments ownership gate failed with $($failures.Count) violation(s)."
}

Write-Host 'R09 ConsoleArguments ownership gate passed: Windows tokenization remains native and deterministic interpretation remains Rust-owned.'
