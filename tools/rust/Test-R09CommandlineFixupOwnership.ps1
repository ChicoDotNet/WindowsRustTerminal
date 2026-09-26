# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

$ErrorActionPreference = 'Stop'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$serializationPath = Join-Path $repoRoot 'src\cascadia\TerminalSettingsModel\CascadiaSettingsSerialization.cpp'
$targetsPath = Join-Path $repoRoot 'Directory.Build.targets'
$ffiHeaderPath = Join-Path $repoRoot 'rust\terminal-settings-ffi\include\terminal_settings_ffi.h'
$ffiSourcePath = Join-Path $repoRoot 'rust\terminal-settings-ffi\src\lib.rs'

$serialization = Get-Content -Raw -LiteralPath $serializationPath
$targets = Get-Content -Raw -LiteralPath $targetsPath
$ffiHeader = Get-Content -Raw -LiteralPath $ffiHeaderPath
$ffiSource = Get-Content -Raw -LiteralPath $ffiSourcePath

$signature = 'bool SettingsLoader::FixupUserSettings()'
$start = $serialization.IndexOf($signature, [System.StringComparison]::Ordinal)
if ($start -lt 0)
{
    throw 'R09 commandline fixup ownership: FixupUserSettings implementation was not found.'
}

$nextMethodMarker = '// Give a string of length N'
$end = $serialization.IndexOf($nextMethodMarker, $start + $signature.Length, [System.StringComparison]::Ordinal)
if ($end -lt 0)
{
    throw 'R09 commandline fixup ownership: could not isolate FixupUserSettings implementation.'
}
$fixupBody = $serialization.Substring($start, $end - $start)

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

Assert-Contains $serialization '#include "terminal_settings_ffi.h"' 'R09 commandline fixup ownership: Settings serialization must include the settings FFI contract.'
Assert-Contains $fixupBody 'terminal_settings_ffi_commandline_fixup_candidate(' 'R09 commandline fixup ownership: candidate policy must route through Rust before native mutation.'
Assert-Contains $fixupBody 'terminal_settings_ffi_commandline_fixup_plan(' 'R09 commandline fixup ownership: post-clear policy must route through Rust.'
Assert-Contains $fixupBody 'candidateStatus == TERMINAL_SETTINGS_FFI_OK && candidate == 1' 'R09 commandline fixup ownership: candidate seam must fail closed before native mutation.'
Assert-Contains $fixupBody 'profile->ClearCommandline();' 'R09 commandline fixup ownership: native inheritance mutation seam must remain explicit.'
Assert-Contains $fixupBody 'if (planStatus != TERMINAL_SETTINGS_FFI_OK)' 'R09 commandline fixup ownership: post-clear ABI failure must be handled explicitly.'
Assert-Contains $fixupBody 'TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_CLEAR_OVERRIDE' 'R09 commandline fixup ownership: ClearOverride plan must be handled explicitly.'
Assert-Contains $fixupBody 'TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_RESTORE_CMD_FULL_PATH' 'R09 commandline fixup ownership: CMD restore plan must be handled explicitly.'
Assert-Contains $fixupBody 'TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_RESTORE_POWERSHELL_FULL_PATH' 'R09 commandline fixup ownership: PowerShell restore plan must be handled explicitly.'
Assert-Contains $fixupBody 'inboxSettings.profilesByGuid.find(guid)' 'R09 commandline fixup ownership: native restore must materialize the canonical inbox profile for the Rust-selected GUID.'
Assert-Contains $fixupBody 'profile->Commandline(inboxProfile->second->Commandline());' 'R09 commandline fixup ownership: native restore must copy the inbox commandline instead of duplicating Rust policy data.'
Assert-Contains $fixupBody 'profile->Commandline(explicitCommandline);' 'R09 commandline fixup ownership: ABI failure or invalid post-clear plan must restore the original explicit commandline.'

Assert-NotContains $fixupBody 'struct CommandlinePatch' 'R09 commandline fixup ownership: duplicated C++ commandline patch policy has reappeared.'
Assert-NotContains $fixupBody 'commandlinePatches' 'R09 commandline fixup ownership: duplicated C++ commandline patch table has reappeared.'
Assert-NotContains $fixupBody 'til::equals_insensitive_ascii(profile->Commandline()' 'R09 commandline fixup ownership: legacy commandline matching policy must remain Rust-owned.'
Assert-NotContains $fixupBody '%SystemRoot%\System32\cmd.exe' 'R09 commandline fixup ownership: CMD restore value must remain Rust-owned, not duplicated in C++.'
Assert-NotContains $fixupBody '%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe' 'R09 commandline fixup ownership: PowerShell restore value must remain Rust-owned, not duplicated in C++.'

$candidate = $fixupBody.IndexOf('terminal_settings_ffi_commandline_fixup_candidate(', [System.StringComparison]::Ordinal)
$clear = $fixupBody.IndexOf('profile->ClearCommandline();', [System.StringComparison]::Ordinal)
$plan = $fixupBody.IndexOf('terminal_settings_ffi_commandline_fixup_plan(', [System.StringComparison]::Ordinal)
if ($candidate -lt 0 -or $clear -lt 0 -or $plan -lt 0 -or -not ($candidate -lt $clear -and $clear -lt $plan))
{
    throw 'R09 commandline fixup ownership: two-phase route must decide candidate before ClearCommandline and decide final plan after inherited state is observable.'
}

Assert-Contains $targets 'cargo build --locked -p terminal-settings-ffi' 'R09 commandline fixup ownership: terminal-settings-ffi must remain in the canonical MSBuild graph.'
Assert-Contains $targets 'terminal_settings_ffi.lib' 'R09 commandline fixup ownership: Settings.Model must remain linked to the Rust static library.'
Assert-Contains $ffiHeader 'terminal_settings_ffi_commandline_fixup_candidate(' 'R09 commandline fixup ownership: FFI header no longer exposes the candidate owner.'
Assert-Contains $ffiHeader 'terminal_settings_ffi_commandline_fixup_plan(' 'R09 commandline fixup ownership: FFI header no longer exposes the post-clear owner.'
Assert-Contains $ffiSource 'commandline_fixup_plan(' 'R09 commandline fixup ownership: Rust FFI must continue delegating policy to terminal-settings.'

Write-Host 'R09 commandline fixup ownership guard passed.'