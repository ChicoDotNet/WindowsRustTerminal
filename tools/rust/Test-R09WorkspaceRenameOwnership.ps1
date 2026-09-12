# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

$ErrorActionPreference = 'Stop'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$applicationStatePath = Join-Path $repoRoot 'src\cascadia\TerminalSettingsModel\ApplicationState.cpp'
$targetsPath = Join-Path $repoRoot 'Directory.Build.targets'
$ffiHeaderPath = Join-Path $repoRoot 'rust\terminal-settings-ffi\include\terminal_settings_ffi.h'

$applicationState = Get-Content -Raw -LiteralPath $applicationStatePath
$targets = Get-Content -Raw -LiteralPath $targetsPath
$ffiHeader = Get-Content -Raw -LiteralPath $ffiHeaderPath

$signature = 'bool ApplicationState::RenameWorkspace(const hstring& oldName, const hstring& newName)'
$start = $applicationState.IndexOf($signature, [System.StringComparison]::Ordinal)
if ($start -lt 0)
{
    throw 'R09 workspace rename ownership: RenameWorkspace implementation was not found.'
}

$nextMethodMarker = '    // Method Description:'
$end = $applicationState.IndexOf($nextMethodMarker, $start + $signature.Length, [System.StringComparison]::Ordinal)
if ($end -lt 0)
{
    throw 'R09 workspace rename ownership: could not isolate RenameWorkspace implementation.'
}
$renameBody = $applicationState.Substring($start, $end - $start)

function Assert-Contains([string]$Text, [string]$Needle, [string]$Message)
{
    if (-not $Text.Contains($Needle, [System.StringComparison]::Ordinal))
    {
        throw $Message
    }
}

function Assert-NotContains([string]$Text, [string]$Needle, [string]$Message)
{
    if ($Text.Contains($Needle, [System.StringComparison]::Ordinal))
    {
        throw $Message
    }
}

Assert-Contains $applicationState '#include "terminal_settings_ffi.h"' 'R09 workspace rename ownership: ApplicationState.cpp must include the settings FFI contract.'
Assert-Contains $renameBody 'terminal_settings_ffi_workspace_rename_plan(' 'R09 workspace rename ownership: product RenameWorkspace must route policy through Rust.'
Assert-Contains $renameBody 'status != TERMINAL_SETTINGS_FFI_OK' 'R09 workspace rename ownership: ABI failure must be checked fail-closed.'
Assert-Contains $renameBody 'TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_RENAME' 'R09 workspace rename ownership: Rename plan must be handled by the native mutation seam.'
Assert-Contains $renameBody 'TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_REMOVE' 'R09 workspace rename ownership: Remove plan must be handled by the native mutation seam.'
Assert-Contains $renameBody 'TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_NOOP' 'R09 workspace rename ownership: Noop plan must be handled explicitly.'

Assert-NotContains $renameBody 'if (oldName == newName || oldName.empty())' 'R09 workspace rename ownership: duplicated C++ rename-policy early return has reappeared.'
Assert-NotContains $renameBody 'if (map.HasKey(oldName))' 'R09 workspace rename ownership: duplicated C++ old-workspace policy branch has reappeared.'
Assert-NotContains $renameBody 'if (!newName.empty())' 'R09 workspace rename ownership: duplicated C++ destination-name policy branch has reappeared.'

Assert-Contains $targets 'cargo build --locked -p terminal-settings-ffi' 'R09 workspace rename ownership: terminal-settings-ffi must remain in the canonical MSBuild graph.'
Assert-Contains $targets 'terminal_settings_ffi.lib' 'R09 workspace rename ownership: Settings.Model must remain linked to the Rust static library.'
Assert-Contains $ffiHeader 'terminal_settings_ffi_workspace_rename_plan(' 'R09 workspace rename ownership: settings FFI header no longer exposes the rename owner.'

Write-Host 'R09 workspace rename ownership guard passed.'
