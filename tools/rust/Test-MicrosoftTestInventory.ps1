#Requires -Version 7
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$inventoryScript = Join-Path $PSScriptRoot 'Get-MicrosoftTestInventory.ps1'
$parserTests = Join-Path $root 'src\terminal\parser\ut_parser'

$inventory = @(& $inventoryScript -Path $parserTests -Suite terminal | ConvertFrom-Json)
if ($inventory.Count -eq 0) {
    throw 'Microsoft source test inventory is empty.'
}

$base64 = @($inventory | Where-Object source -eq 'Base64Test.cpp')
$expected = @('DecodeFuzz', 'DecodeUTF8')
$actual = @($base64.method | Sort-Object)
if (($actual -join ',') -ne (($expected | Sort-Object) -join ',')) {
    throw "Base64 source inventory changed: expected $($expected -join ', '), got $($actual -join ', ')."
}

$duplicates = @($inventory | Group-Object suite, source, method | Where-Object Count -gt 1)
if ($duplicates.Count -ne 0) {
    throw 'Microsoft source test inventory contains duplicate method identities.'
}

Write-Host "Microsoft source inventory self-test passed ($($inventory.Count) terminal source methods)."
