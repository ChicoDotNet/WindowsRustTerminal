#Requires -Version 7
$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '../..')
$mapPath = Join-Path $PSScriptRoot 'r07-renderer-source-map.json'
$map = Get-Content -Raw $mapPath | ConvertFrom-Json -AsHashtable

if ([int]$map.schemaVersion -ne 1 -or [string]$map.stage -ne 'R07') {
    throw "Unsupported R07 renderer source-map schema/stage."
}

$allowedOwnership = @('split', 'native')
$seenSources = @{}
$mappedWitnesses = @{}
$splitCount = 0
$nativeCount = 0
$rustOwnerCount = 0
$witnessCount = 0

foreach ($entry in @($map.entries)) {
    $sourcePath = [string]$entry.sourcePath
    if ([string]::IsNullOrWhiteSpace($sourcePath)) {
        throw 'R07 renderer source-map entry is missing sourcePath.'
    }
    if ($seenSources.ContainsKey($sourcePath)) {
        throw "Duplicate R07 renderer source-map entry: $sourcePath"
    }
    $seenSources[$sourcePath] = $true

    $sourceFullPath = Join-Path $repoRoot $sourcePath
    if (-not (Test-Path -LiteralPath $sourceFullPath -PathType Leaf)) {
        throw "R07 renderer source no longer exists: $sourcePath"
    }
    $sourceContent = Get-Content -Raw $sourceFullPath
    $sourcePatterns = if ($entry.ContainsKey('sourcePatterns')) { @($entry.sourcePatterns) } else { @() }
    foreach ($pattern in $sourcePatterns) {
        if (-not $sourceContent.Contains([string]$pattern)) {
            throw "R07 renderer source pattern '$pattern' is missing from $sourcePath"
        }
    }

    $ownership = [string]$entry.ownership
    if ($ownership -notin $allowedOwnership) {
        throw "Unknown R07 renderer ownership '$ownership' for $sourcePath"
    }
    if ([string]::IsNullOrWhiteSpace([string]$entry.nativeBoundary)) {
        throw "R07 renderer entry must document its native boundary: $sourcePath"
    }

    $owners = if ($entry.ContainsKey('rustOwners')) { @($entry.rustOwners) } else { @() }
    if ($ownership -eq 'split') {
        $splitCount++
        if ($owners.Count -eq 0) {
            throw "Split R07 renderer entry requires at least one Rust owner: $sourcePath"
        }

        foreach ($owner in $owners) {
            $rustOwnerCount++
            $rustPath = [string]$owner.rustPath
            if ([string]::IsNullOrWhiteSpace($rustPath)) {
                throw "R07 renderer Rust owner is missing rustPath for $sourcePath"
            }
            $rustFullPath = Join-Path $repoRoot $rustPath
            if (-not (Test-Path -LiteralPath $rustFullPath -PathType Leaf)) {
                throw "R07 Rust renderer owner no longer exists: $rustPath"
            }

            $rustContent = Get-Content -Raw $rustFullPath
            $witnesses = if ($owner.ContainsKey('rustWitnesses')) { @($owner.rustWitnesses) } else { @() }
            if ($witnesses.Count -eq 0) {
                throw "R07 renderer Rust owner requires witnesses: $rustPath"
            }
            foreach ($witness in $witnesses) {
                $witness = [string]$witness
                $key = "$rustPath|$witness"
                if ($mappedWitnesses.ContainsKey($key)) {
                    throw "Duplicate R07 renderer Rust witness: $key"
                }
                if (-not $rustContent.Contains("fn $witness")) {
                    throw "R07 renderer Rust witness '$witness' is missing from $rustPath"
                }
                $mappedWitnesses[$key] = $true
                $witnessCount++
            }
        }
    }
    else {
        $nativeCount++
        if ($owners.Count -ne 0) {
            throw "Native R07 renderer entry must not claim Rust owners: $sourcePath"
        }
    }
}

$rendererRoot = Join-Path $repoRoot 'rust/terminal-renderer/src'
$actualTests = @{}
foreach ($rustFile in Get-ChildItem -LiteralPath $rendererRoot -Filter '*.rs' -File) {
    $relativePath = [System.IO.Path]::GetRelativePath($repoRoot, $rustFile.FullName).Replace('\\', '/')
    $content = Get-Content -Raw $rustFile.FullName
    foreach ($match in [regex]::Matches($content, '(?ms)#\[test\]\s*fn\s+([A-Za-z0-9_]+)')) {
        $testName = $match.Groups[1].Value
        $key = "$relativePath|$testName"
        if ($actualTests.ContainsKey($key)) {
            throw "Duplicate R07 renderer test identity: $key"
        }
        $actualTests[$key] = $true
    }
}

foreach ($key in $mappedWitnesses.Keys) {
    if (-not $actualTests.ContainsKey($key)) {
        throw "R07 renderer source map references a non-test witness: $key"
    }
}
foreach ($key in $actualTests.Keys) {
    if (-not $mappedWitnesses.ContainsKey($key)) {
        throw "R07 renderer test has no deliberate C++ ownership mapping: $key"
    }
}

$libPath = Join-Path $rendererRoot 'lib.rs'
if (-not (Get-Content -Raw $libPath).Contains('#![forbid(unsafe_code)]')) {
    throw 'R07 terminal-renderer must retain #![forbid(unsafe_code)].'
}

$expected = $map.expected
if ($splitCount -ne [int]$expected.split -or
    $nativeCount -ne [int]$expected.native -or
    $rustOwnerCount -ne [int]$expected.rustOwners -or
    $witnessCount -ne [int]$expected.rustWitnesses -or
    $actualTests.Count -ne [int]$expected.rustWitnesses) {
    throw "R07 renderer source-map summary changed unexpectedly: split=$splitCount native=$nativeCount owners=$rustOwnerCount witnesses=$witnessCount actualTests=$($actualTests.Count)"
}

Write-Host "R07 renderer seam gate passed (split=$splitCount, native=$nativeCount, Rust owners=$rustOwnerCount, Rust witnesses=$witnessCount)."
