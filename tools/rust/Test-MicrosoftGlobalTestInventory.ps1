#Requires -Version 7
$ErrorActionPreference = 'Stop'

$globalScript = Join-Path $PSScriptRoot 'Get-MicrosoftGlobalTestInventory.ps1'
$census = Get-Content -Raw (Join-Path $PSScriptRoot 'microsoft-test-source-census.json') | ConvertFrom-Json -AsHashtable
$ledger = Get-Content -Raw (Join-Path $PSScriptRoot 'microsoft-rust-equivalence.json') | ConvertFrom-Json -AsHashtable
$baseline = Get-Content -Raw (Join-Path $PSScriptRoot 'contract-baseline.json') | ConvertFrom-Json -AsHashtable
$raw = (& $globalScript | Out-String).Trim()
$inventory = @($raw | ConvertFrom-Json)

$expectedSuites = @($baseline.suites.Keys | Sort-Object)
if (($expectedSuites -join ',') -ne (@($census.suites.Keys | Sort-Object) -join ',')) {
    throw 'Microsoft source census suites do not match contract-baseline.json.'
}
if (($expectedSuites -join ',') -ne (@($ledger.suites.Keys | Sort-Object) -join ',')) {
    throw 'Microsoft equivalence ledger suites do not match contract-baseline.json.'
}

$allowedCoverage = @($ledger.coverageClasses)
$entryKeys = @{}
foreach ($entry in @($ledger.entries)) {
    if ($entry.coverage -notin $allowedCoverage) {
        throw "Unknown coverage '$($entry.coverage)' in equivalence ledger."
    }
    $key = "$($entry.suite)|$($entry.source)|$($entry.method)"
    if ($entryKeys.ContainsKey($key)) {
        throw "Duplicate equivalence ledger entry: $key"
    }
    $entryKeys[$key] = $entry
}

$currentKeys = @{}
$bootstrapRequired = $false
foreach ($suite in $expectedSuites) {
    $items = @($inventory | Where-Object suite -eq $suite)
    if ($items.Count -eq 0) {
        throw "$suite source inventory is empty."
    }
    foreach ($item in $items) {
        $currentKeys["$($item.suite)|$($item.source)|$($item.method)"] = $true
    }

    $identities = @($items | ForEach-Object { "$($_.suite)|$($_.source)|$($_.method)" } | Sort-Object -Unique)
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($identities -join "`n")
    $hash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
    $frozen = $census.suites[$suite]
    $runtimeTotal = [int]$baseline.suites[$suite].total

    if ([int]$frozen.runtimeBaseline -ne $runtimeTotal) {
        throw "$suite runtime baseline differs from contract-baseline.json."
    }
    if ($ledger.suites[$suite].defaultCoverage -notin $allowedCoverage) {
        throw "$suite has an invalid default coverage."
    }

    if ($null -eq $frozen.sourceMethodCount -or [string]::IsNullOrWhiteSpace([string]$frozen.identitySha256)) {
        $bootstrapRequired = $true
        Write-Host "CENSUS_BOOTSTRAP|$suite|$($items.Count)|$hash"
    }
    elseif ([int]$frozen.sourceMethodCount -ne $items.Count -or [string]$frozen.identitySha256 -ne $hash) {
        throw "$suite Microsoft source contract changed: expected $($frozen.sourceMethodCount) methods / $($frozen.identitySha256), got $($items.Count) / $hash. Reconcile the ledger before updating the census."
    }

    $coverageCounts = @{}
    foreach ($item in $items) {
        $key = "$($item.suite)|$($item.source)|$($item.method)"
        $coverage = if ($entryKeys.ContainsKey($key)) { $entryKeys[$key].coverage } else { $ledger.suites[$suite].defaultCoverage }
        if (-not $coverageCounts.ContainsKey($coverage)) { $coverageCounts[$coverage] = 0 }
        $coverageCounts[$coverage]++
    }
    $summary = @($coverageCounts.Keys | Sort-Object | ForEach-Object { "$_=$($coverageCounts[$_])" }) -join ', '
    Write-Host "Microsoft source census: $suite=$($items.Count); runtime=$runtimeTotal; $summary"
}

foreach ($key in $entryKeys.Keys) {
    if (-not $currentKeys.ContainsKey($key)) {
        throw "Equivalence ledger references a removed Microsoft contract: $key"
    }
}
if ($bootstrapRequired) {
    throw 'Global Microsoft source census requires bootstrap fingerprints. Copy all CENSUS_BOOTSTRAP values into microsoft-test-source-census.json.'
}

Write-Host "Microsoft global source inventory gate passed ($($inventory.Count) source methods across $($expectedSuites.Count) suites)."
