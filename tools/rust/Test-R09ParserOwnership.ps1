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

# The next InputStateMachineEngine ownership slice is deliberately staged behind the
# same parser FFI boundary. These checks make the Rust side mechanically complete
# before the large native translation unit is edited: the product-facing header must
# expose all three lookups, the FFI module must export them, and the safe parser crate
# must remain the semantic owner of the lookup functions.
$expectedKeymapFunctions = @(
    'terminal_parser_ffi_input_cursor_vkey',
    'terminal_parser_ffi_input_generic_vkey',
    'terminal_parser_ffi_input_ss3_vkey'
)

$ffiHeaderText = Get-Content -LiteralPath $ffiHeader -Raw
$ffiKeymapText = Get-Content -LiteralPath $ffiKeymap -Raw
$rustKeymapText = Get-Content -LiteralPath $rustKeymap -Raw
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
}

foreach ($ownerFunction in @('cursor_virtual_key', 'generic_virtual_key', 'ss3_virtual_key'))
{
    if ($rustKeymapText -notmatch [regex]::Escape($ownerFunction))
    {
        throw "R09 parser ownership regression: terminal-parser no longer owns $ownerFunction."
    }
}

# Until the native call sites are atomically switched, keep the remaining duplicate
# tables visible as explicit migration debt instead of silently losing track of them.
$inputEngineText = Get-Content -LiteralPath $inputEngine -Raw
$stagedTables = @('s_csiMap', 's_genericMap', 's_ss3Map')
$presentTables = @($stagedTables | Where-Object { $inputEngineText -match [regex]::Escape($_) })
if ($presentTables.Count -ne $stagedTables.Count)
{
    $missing = @($stagedTables | Where-Object { $_ -notin $presentTables }) -join ', '
    throw "R09 parser ownership state changed unexpectedly: staged C++ key-map table(s) missing without the ownership gate being promoted: $missing"
}

Write-Host 'R09 parser ownership gate passed: Base64 is Rust-owned; InputStateMachineEngine key-map ABI is complete and staged for the next ownership swap.'
