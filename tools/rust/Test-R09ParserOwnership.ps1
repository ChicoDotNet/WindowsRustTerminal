# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$parserRoot = Join-Path $repoRoot 'src\terminal\parser'
$base64Cpp = Join-Path $parserRoot 'base64.cpp'
$base64Header = Join-Path $parserRoot 'base64.hpp'
$parserBuild = Join-Path $parserRoot 'parser-common.vcxitems'
$inputEngine = Join-Path $parserRoot 'InputStateMachineEngine.cpp'
$ffiHeader = Join-Path $repoRoot 'rust\terminal-parser-ffi\include\terminal_parser_ffi.h'
$ffiKeymap = Join-Path $repoRoot 'rust\terminal-parser-ffi\src\input_keymap.rs'
$rustKeymap = Join-Path $repoRoot 'rust\terminal-parser\src\input_keymap.rs'
$rustInputEngine = Join-Path $repoRoot 'rust\terminal-parser\src\input_engine.rs'

if (Test-Path $base64Cpp)
{
    throw 'R09 parser ownership regression: base64.cpp must remain deleted after Rust ownership promotion.'
}

$staleBuildReferences = Get-ChildItem -Path $parserRoot -Recurse -File |
    Select-String -SimpleMatch 'base64.cpp' -ErrorAction SilentlyContinue
if ($staleBuildReferences)
{
    $locations = ($staleBuildReferences | ForEach-Object { "{0}:{1}" -f $_.Path, $_.LineNumber }) -join ', '
    throw "R09 parser ownership regression: stale base64.cpp reference(s) found at $locations"
}

$headerText = Get-Content -LiteralPath $base64Header -Raw
if ($headerText -notmatch 'terminal_parser_ffi_base64_decode_utf16')
{
    throw 'R09 parser ownership regression: base64.hpp no longer routes Base64 decoding through terminal-parser-ffi.'
}

$buildText = Get-Content -LiteralPath $parserBuild -Raw
if ($buildText -notmatch 'terminal_parser_ffi\.lib' -or $buildText -notmatch 'cargo build --locked -p terminal-parser-ffi')
{
    throw 'R09 parser ownership regression: parser-common.vcxitems no longer builds and links terminal-parser-ffi.'
}

$expectedKeymapFunctions = @(
    'terminal_parser_ffi_input_cursor_vkey',
    'terminal_parser_ffi_input_generic_vkey',
    'terminal_parser_ffi_input_ss3_vkey'
)
$expectedModifierFunctions = @(
    'terminal_parser_ffi_input_vt_modifier_state',
    'terminal_parser_ffi_input_sgr_mouse_modifier_state'
)

$ffiHeaderText = Get-Content -LiteralPath $ffiHeader -Raw
$ffiKeymapText = Get-Content -LiteralPath $ffiKeymap -Raw
$rustKeymapText = Get-Content -LiteralPath $rustKeymap -Raw
$rustInputEngineText = Get-Content -LiteralPath $rustInputEngine -Raw
$inputEngineText = Get-Content -LiteralPath $inputEngine -Raw
foreach ($function in $expectedKeymapFunctions)
{
    if ($ffiHeaderText -notmatch [regex]::Escape($function))
    {
        throw "R09 parser ownership regression: terminal_parser_ffi.h no longer declares $function."
    }
    if ($ffiKeymapText -notmatch [regex]::Escape($function))
    {
        throw "R09 parser ownership regression: terminal-parser-ffi no longer exports $function."
    }
    if ($inputEngineText -notmatch [regex]::Escape($function))
    {
        throw "R09 parser ownership regression: InputStateMachineEngine no longer routes key mapping through $function."
    }
}

foreach ($function in $expectedModifierFunctions)
{
    if ($ffiHeaderText -notmatch [regex]::Escape($function))
    {
        throw "R09 parser ownership regression: terminal_parser_ffi.h no longer declares modifier seam $function."
    }
    if ($ffiKeymapText -notmatch [regex]::Escape($function))
    {
        throw "R09 parser ownership regression: terminal-parser-ffi no longer exports modifier seam $function."
    }
    if ($inputEngineText -notmatch [regex]::Escape($function))
    {
        throw "R09 parser ownership regression: InputStateMachineEngine no longer routes modifier translation through $function."
    }
}

foreach ($ownerFunction in @(
    'cursor_virtual_key',
    'generic_virtual_key',
    'ss3_virtual_key',
    'vt_modifier_state',
    'sgr_mouse_modifier_state'
))
{
    if ($rustKeymapText -notmatch [regex]::Escape($ownerFunction))
    {
        throw "R09 parser ownership regression: terminal-parser no longer owns $ownerFunction."
    }
}

foreach ($rustModifierAdapter in @(
    'vt_modifier_state_from_parameter',
    'sgr_mouse_modifier_state_from_encoding'
))
{
    if ($rustInputEngineText -notmatch [regex]::Escape($rustModifierAdapter))
    {
        throw "R09 parser ownership regression: terminal-parser input engine no longer delegates modifier normalization through $rustModifierAdapter."
    }
}

foreach ($legacyRustModifierImplementation in @(
    'const VT_SHIFT:',
    'const VT_ALT:',
    'const VT_CTRL:',
    'const SGR_SHIFT:',
    'const SGR_META:',
    'const SGR_CTRL:',
    'fn vt_modifiers(',
    'fn sgr_mouse_modifiers('
))
{
    if ($rustInputEngineText -match [regex]::Escape($legacyRustModifierImplementation))
    {
        throw "R09 parser ownership regression: duplicate Rust modifier implementation returned to input_engine.rs: $legacyRustModifierImplementation"
    }
}

foreach ($legacySymbol in @(
    'CsiToVkey',
    'GenericToVkey',
    'Ss3ToVkey',
    's_csiMap',
    's_genericMap',
    's_ss3Map'
))
{
    if ($inputEngineText -match [regex]::Escape($legacySymbol))
    {
        throw "R09 parser ownership regression: legacy C++ key-map symbol returned after Rust promotion: $legacySymbol"
    }
}

foreach ($legacyModifierImplementation in @(
    'const auto vtParam = modifierParam - 1;',
    'WI_SetFlagIf(modifiers, SHIFT_PRESSED, WI_IsFlagSet(modifierParam, CsiMouseModifierCodes::Shift));'
))
{
    if ($inputEngineText -match [regex]::Escape($legacyModifierImplementation))
    {
        throw "R09 parser ownership regression: portable modifier translation returned to C++: $legacyModifierImplementation"
    }
}

Write-Host 'R09 parser ownership gate passed: Base64, input key maps, and modifier translation are Rust-owned; C++ routes promoted parser behavior through terminal-parser-ffi and duplicate portable implementations are absent.'
