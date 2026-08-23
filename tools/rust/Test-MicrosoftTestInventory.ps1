#Requires -Version 7
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$inventoryScript = Join-Path $PSScriptRoot 'Get-MicrosoftTestInventory.ps1'
$parserTests = Join-Path $root 'src\terminal\parser\ut_parser'

$inventory = @(& $inventoryScript -Path $parserTests -Suite terminal | ConvertFrom-Json)
if ($inventory.Count -eq 0) {
    throw 'Microsoft source test inventory is empty.'
}

function Assert-SourceMethodSet {
    param(
        [Parameter(Mandatory)]
        [object[]] $Inventory,

        [Parameter(Mandatory)]
        [string] $Source,

        [Parameter(Mandatory)]
        [string[]] $Expected
    )

    $actual = @($Inventory | Where-Object source -eq $Source | Select-Object -ExpandProperty method | Sort-Object)
    $expectedSorted = @($Expected | Sort-Object)

    if (($actual -join ',') -ne ($expectedSorted -join ',')) {
        throw "$Source inventory changed: expected $($expectedSorted -join ', '), got $($actual -join ', ')."
    }
}

Assert-SourceMethodSet -Inventory $inventory -Source 'Base64Test.cpp' -Expected @(
    'DecodeFuzz',
    'DecodeUTF8'
)

Assert-SourceMethodSet -Inventory $inventory -Source 'StateMachineTest.cpp' -Expected @(
    'BulkTextPrint',
    'DcsDataStringsReceivedByHandler',
    'PassThroughUnhandled',
    'PassThroughUnhandledSplitAcrossWrites',
    'RunStorageBeforeEscape',
    'TwoStateMachinesDoNotInterfereWithEachOther',
    'VtParameterSubspanTest'
)

$duplicates = @($inventory | Group-Object suite, source, method | Where-Object Count -gt 1)
if ($duplicates.Count -ne 0) {
    throw 'Microsoft source test inventory contains duplicate method identities.'
}

Write-Host "Microsoft source inventory self-test passed ($($inventory.Count) terminal source methods)."
