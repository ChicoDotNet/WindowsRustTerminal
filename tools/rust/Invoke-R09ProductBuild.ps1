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

function Set-R09MsbuildDevEnvironment
{
    [CmdletBinding()]
    param()

    try {
        Set-MsbuildDevEnvironment
        return
    }
    catch {
        if ($_.Exception.Message -notmatch 'VSSetup|Find-Package|Find-Module') {
            throw
        }

        Write-Warning 'VSSetup could not be resolved; falling back to the installed Visual Studio DevShell.'
    }

    $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
    if (-not (Test-Path $vswhere)) {
        throw "VSSetup is unavailable and vswhere was not found at '$vswhere'."
    }

    $installationPath = (& $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath | Select-Object -First 1)
    if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($installationPath)) {
        throw 'Unable to locate a Visual Studio installation with the VC++ x64 toolset.'
    }

    switch ($env:PROCESSOR_ARCHITECTURE.ToLowerInvariant()) {
        'amd64' { $arch = 'x64' }
        'x86' { $arch = 'x86' }
        'arm64' { $arch = 'arm64' }
        default { throw "Unknown architecture: $($env:PROCESSOR_ARCHITECTURE)" }
    }

    $devShellModule = Join-Path $installationPath 'Common7\Tools\Microsoft.VisualStudio.DevShell.dll'
    if (-not (Test-Path $devShellModule)) {
        throw "Visual Studio DevShell module was not found at '$devShellModule'."
    }

    Import-Module -Global -Name $devShellModule
    Enter-VsDevShell -VsInstallPath $installationPath -SkipAutomaticLocation -DevCmdArguments "-arch=$arch" | Out-Null
    Set-Item -Force -Path 'Env:\Platform' -Value $arch

    Write-Host "Dev environment variables set from $installationPath" -ForegroundColor Green
}

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
    Set-R09MsbuildDevEnvironment

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
