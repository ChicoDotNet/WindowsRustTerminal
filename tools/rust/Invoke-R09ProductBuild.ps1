#Requires -Version 7

[CmdletBinding()]
param(
    [ValidateSet('Debug', 'Release', 'AuditMode')]
    [string]$Configuration = 'Debug',

    [ValidateSet('x64', 'Win32', 'ARM64')]
    [string]$Platform = 'x64',

    [switch]$SkipSubmodules
)

$ErrorActionPreference = 'Stop'

$root = (git rev-parse --show-toplevel 2>$null)
if (-not $root) {
    throw 'Run this script from inside the WindowsRusTerminal/terminal checkout.'
}

Push-Location $root
try {
    if (-not $SkipSubmodules) {
        git submodule update --init --recursive
        if ($LASTEXITCODE -ne 0) {
            throw "git submodule update failed with exit code $LASTEXITCODE"
        }
    }

    Import-Module (Join-Path $root 'tools\OpenConsole.psm1') -Force
    Set-MsbuildDevEnvironment

    $msbuildArgs = @(
        "/p:Configuration=$Configuration",
        "/p:Platform=$Platform",
        '/p:AppxSymbolPackageEnabled=false',
        '/t:Terminal\CascadiaPackage',
        '/m'
    )

    Write-Host "Building the canonical Terminal product path: Terminal\CascadiaPackage ($Platform $Configuration)"
    Invoke-OpenConsoleBuild @msbuildArgs

    if ($LASTEXITCODE -ne 0) {
        throw "Terminal product build failed with exit code $LASTEXITCODE"
    }

    Write-Host 'Canonical Terminal product build completed successfully.'
}
finally {
    Pop-Location
}
