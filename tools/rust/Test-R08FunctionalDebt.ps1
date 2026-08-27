#Requires -Version 7
param(
    [switch]$RequireZero
)

$ErrorActionPreference = 'Stop'

$globalScript = Join-Path $PSScriptRoot 'Get-MicrosoftGlobalTestInventory.ps1'
$ledger = Get-Content -Raw (Join-Path $PSScriptRoot 'microsoft-rust-equivalence.json') | ConvertFrom-Json -AsHashtable
$manifest = Get-Content -Raw (Join-Path $PSScriptRoot 'microsoft-rust-partial-debt.json') | ConvertFrom-Json -AsHashtable
$raw = (& $globalScript | Out-String).Trim()
$inventory = @($raw | ConvertFrom-Json)

if ([int]$manifest.schemaVersion -ne 1) {
    throw 'Unsupported R08 Partial-debt manifest schema.'
}
if (-not $manifest.ContainsKey('expectedPartialTotal') -or
    -not $manifest.ContainsKey('defaultClass') -or
    -not $manifest.ContainsKey('allowedClasses') -or
    -not $manifest.ContainsKey('exceptions')) {
    throw 'R08 Partial-debt manifest requires expectedPartialTotal, defaultClass, allowedClasses and exceptions.'
}

$allowedClasses = @($manifest.allowedClasses)
if ([string]$manifest.defaultClass -notin $allowedClasses) {
    throw "Unknown default Partial-debt class '$($manifest.defaultClass)'."
}

$entryKeys = @{}
foreach ($entry in @($ledger.entries)) {
    $key = "$($entry.suite)|$($entry.source)|$($entry.method)"
    if ($entryKeys.ContainsKey($key)) {
        throw "Duplicate equivalence ledger entry: $key"
    }
    $entryKeys[$key] = $entry
}

$sourceRules = @{}
$overlayFiles = @(Get-ChildItem -Path $PSScriptRoot -Filter 'microsoft-rust-equivalence-*.json' -File | Sort-Object Name)
foreach ($overlayFile in $overlayFiles) {
    $overlay = Get-Content -Raw $overlayFile.FullName | ConvertFrom-Json -AsHashtable
    if ($overlay.ContainsKey('entries')) {
        foreach ($entry in @($overlay.entries)) {
            $key = "$($entry.suite)|$($entry.source)|$($entry.method)"
            if ($entryKeys.ContainsKey($key)) {
                throw "Duplicate equivalence ledger entry across overlays: $key"
            }
            $entryKeys[$key] = $entry
        }
    }
    if ($overlay.ContainsKey('sourceRules')) {
        foreach ($rule in @($overlay.sourceRules)) {
            $key = "$($rule.suite)|$($rule.source)"
            if ($sourceRules.ContainsKey($key)) {
                throw "Duplicate source equivalence rule across overlays: $key"
            }
            $sourceRules[$key] = $rule
        }
    }
}

$methodExceptions = @{}
$sourceExceptions = @{}
foreach ($exception in @($manifest.exceptions)) {
    if ([string]::IsNullOrWhiteSpace([string]$exception.suite) -or
        [string]::IsNullOrWhiteSpace([string]$exception.source) -or
        [string]::IsNullOrWhiteSpace([string]$exception.class) -or
        [string]::IsNullOrWhiteSpace([string]$exception.reason)) {
        throw 'Every Partial-debt exception requires suite, source, class and reason.'
    }
    if ([string]$exception.class -notin $allowedClasses -or [string]$exception.class -eq 'functional') {
        throw "Partial-debt exception must use a non-functional allowed class: $($exception.class)"
    }

    $sourceKey = "$($exception.suite)|$($exception.source)"
    if ($exception.ContainsKey('method') -and -not [string]::IsNullOrWhiteSpace([string]$exception.method)) {
        $key = "$sourceKey|$($exception.method)"
        if ($methodExceptions.ContainsKey($key)) {
            throw "Duplicate method Partial-debt exception: $key"
        }
        $methodExceptions[$key] = $exception
    }
    else {
        if ($sourceExceptions.ContainsKey($sourceKey)) {
            throw "Duplicate source Partial-debt exception: $sourceKey"
        }
        $sourceExceptions[$sourceKey] = $exception
    }
}

$counts = @{}
foreach ($class in $allowedClasses) { $counts[$class] = 0 }
$partialKeys = @{}
$missingCount = 0

foreach ($item in $inventory) {
    $key = "$($item.suite)|$($item.source)|$($item.method)"
    $sourceKey = "$($item.suite)|$($item.source)"

    if ($entryKeys.ContainsKey($key)) {
        $coverage = [string]$entryKeys[$key].coverage
    }
    elseif ($sourceRules.ContainsKey($sourceKey)) {
        $coverage = [string]$sourceRules[$sourceKey].coverage
    }
    else {
        $coverage = [string]$ledger.suites[$item.suite].defaultCoverage
    }

    if ($coverage -eq 'Missing') {
        $missingCount++
        continue
    }
    if ($coverage -ne 'Partial') {
        continue
    }

    $partialKeys[$key] = $true
    if ($methodExceptions.ContainsKey($key)) {
        $class = [string]$methodExceptions[$key].class
    }
    elseif ($sourceExceptions.ContainsKey($sourceKey)) {
        $class = [string]$sourceExceptions[$sourceKey].class
    }
    else {
        $class = [string]$manifest.defaultClass
    }
    $counts[$class]++
}

foreach ($key in $methodExceptions.Keys) {
    if (-not $partialKeys.ContainsKey($key)) {
        throw "Method Partial-debt exception no longer references an effective Partial contract: $key"
    }
}
foreach ($sourceKey in $sourceExceptions.Keys) {
    $prefix = "$sourceKey|"
    if (@($partialKeys.Keys | Where-Object { $_.StartsWith($prefix, [System.StringComparison]::Ordinal) }).Count -eq 0) {
        throw "Source Partial-debt exception no longer references any effective Partial contract: $sourceKey"
    }
}

$partialTotal = 0
foreach ($class in $allowedClasses) { $partialTotal += [int]$counts[$class] }
if ($partialTotal -ne [int]$manifest.expectedPartialTotal) {
    throw "R08 Partial-debt total changed: expected $($manifest.expectedPartialTotal), got $partialTotal. Re-audit the classification manifest before accepting the new census."
}

$summary = @($allowedClasses | ForEach-Object { "$_=$($counts[$_])" }) -join ', '
Write-Host "R08 Partial debt: total=$partialTotal; $summary; Missing=$missingCount"

if ($RequireZero) {
    if ($missingCount -ne 0) {
        throw "R08 exit gate failed: Missing=$missingCount; expected 0."
    }
    if ([int]$counts['functional'] -ne 0) {
        throw "R08 exit gate failed: Partial(functional)=$($counts['functional']); expected 0."
    }
    Write-Host 'R08 functional-debt exit gate passed (Missing=0; Partial(functional)=0).'
}
else {
    Write-Host "R08 functional-debt classification gate passed (Partial(functional)=$($counts['functional']))."
}
