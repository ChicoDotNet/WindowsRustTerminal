#Requires -Version 7
[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [ValidateSet('host', 'textBuffer', 'terminalCore', 'terminalApp', 'localTerminalApp', 'unitSettingsModel', 'unitControl', 'interactivityWin32', 'terminal', 'adapter', 'types', 'til')]
    [string] $Suite,

    [ValidateSet('x64', 'x86')]
    [string] $Platform = 'x64',

    [ValidateSet('Debug', 'Release')]
    [string] $Configuration = 'Debug',

    [string] $BaselinePath = (Join-Path $PSScriptRoot 'contract-baseline.json'),

    [string] $OutputDirectory = (Join-Path (Resolve-Path (Join-Path $PSScriptRoot '..\..')) 'artifacts\contract'),

    [switch] $MeasureOnly
)

$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$openConsoleModule = Join-Path $root 'tools\OpenConsole.psm1'
$contractModule = Join-Path $PSScriptRoot 'TaefContract.psm1'

Import-Module $openConsoleModule -Force
Import-Module $contractModule -Force

New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$logPath = Join-Path $OutputDirectory "$Suite.log"
$jsonPath = Join-Path $OutputDirectory "$Suite.json"

$stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
try {
    & {
        Invoke-OpenConsoleTests -Test $Suite -Platform $Platform -Configuration $Configuration
    } *>&1 | Tee-Object -FilePath $logPath | Out-Host
}
finally {
    $stopwatch.Stop()
}

$text = Get-Content -Raw -Path $logPath
$summary = Get-TaefSummary -Text $text

$result = [ordered]@{
    suite         = $Suite
    platform      = $Platform
    configuration = $Configuration
    durationMs    = $stopwatch.ElapsedMilliseconds
    total         = $summary.Total
    passed        = $summary.Passed
    failed        = $summary.Failed
    blocked       = $summary.Blocked
    notRun        = $summary.NotRun
    skipped       = $summary.Skipped
    baselinePass  = $null
    violations    = @()
}

if (-not $MeasureOnly) {
    $baselineDocument = Get-Content -Raw -Path $BaselinePath | ConvertFrom-Json
    $baseline = $baselineDocument.suites.$Suite
    if ($null -eq $baseline) {
        throw "No contract baseline exists for suite '$Suite'."
    }

    $comparison = Test-TaefSummaryAgainstBaseline -Summary $summary -Baseline $baseline
    $result.baselinePass = $comparison.Passed
    $result.violations = @($comparison.Violations)
}

$result | ConvertTo-Json -Depth 5 | Set-Content -Path $jsonPath -Encoding utf8

Write-Host ''
Write-Host "Contract suite: $Suite"
Write-Host ("Duration:       {0}" -f $stopwatch.Elapsed)
Write-Host ("TAEF:           Total={0}, Passed={1}, Failed={2}, Blocked={3}, NotRun={4}, Skipped={5}" -f `
    $summary.Total, $summary.Passed, $summary.Failed, $summary.Blocked, $summary.NotRun, $summary.Skipped)

if (-not $MeasureOnly -and -not $result.baselinePass) {
    throw "Contract regression in '$Suite': $($result.violations -join ' ')"
}

[pscustomobject]$result
