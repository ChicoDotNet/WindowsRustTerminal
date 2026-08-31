#Requires -Version 7

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$root = (git rev-parse --show-toplevel 2>$null)
if (-not $root) {
    throw 'Run this script from inside the WindowsRusTerminal/terminal checkout.'
}

$fzfHeaderPath = Join-Path $root 'src\cascadia\fzf\fzf.h'
$fzfSourcePath = Join-Path $root 'src\cascadia\fzf\fzf.cpp'
$ffiHeaderPath = Join-Path $root 'rust\terminal-app-ffi\include\terminal_app_ffi.h'
$rustFzfPath = Join-Path $root 'rust\terminal-app\src\fzf.rs'

$fzfHeader = Get-Content $fzfHeaderPath -Raw
$fzfSource = Get-Content $fzfSourcePath -Raw
$ffiHeader = Get-Content $ffiHeaderPath -Raw
$rustFzf = Get-Content $rustFzfPath -Raw

function Assert-Contains {
    param(
        [string]$Content,
        [string]$Needle,
        [string]$Message
    )

    if (-not $Content.Contains($Needle, [System.StringComparison]::Ordinal)) {
        throw $Message
    }
}

function Assert-NotContains {
    param(
        [string]$Content,
        [string]$Needle,
        [string]$Message
    )

    if ($Content.Contains($Needle, [System.StringComparison]::Ordinal)) {
        throw $Message
    }
}

# The C++ surface is now a compatibility seam. Portable pattern representation,
# folding/tokenization, matching, scoring, and highlight generation belong to Rust.
Assert-Contains $fzfHeader 'std::shared_ptr<terminal_app_ffi_fzf_pattern> RustPattern;' 'FZF Pattern must remain an opaque Rust-owned handle.'
Assert-NotContains $fzfHeader 'terms' 'C++ must not regain ownership of parsed FZF terms.'

Assert-Contains $fzfSource 'terminal_app_ffi_fzf_pattern_create_utf16' 'ParsePattern must delegate to terminal-app-ffi.'
Assert-Contains $fzfSource 'terminal_app_ffi_fzf_match_utf16' 'Match must delegate to terminal-app-ffi.'
Assert-Contains $fzfSource 'terminal_app_ffi_fzf_pattern_is_empty' 'Pattern emptiness must be queried from the Rust owner.'
Assert-Contains $fzfSource 'terminal_app_ffi_fzf_pattern_destroy' 'Rust-owned FZF patterns must be released through terminal-app-ffi.'
Assert-NotContains $fzfSource 'return Match({}, pattern).has_value();' 'C++ must not infer Rust pattern emptiness by executing a match.'

foreach ($legacySymbol in @(
    'utf16ToUtf32',
    'foldStringUtf32',
    'serializePattern',
    'Pattern::terms',
    'scoreMatrix',
    'consecutiveMatrix'
)) {
    Assert-NotContains $fzfSource $legacySymbol "Legacy portable FZF implementation '$legacySymbol' must not return to C++."
}

Assert-Contains $ffiHeader 'terminal_app_ffi_fzf_pattern_create_utf16' 'The C ABI must expose Rust-owned FZF pattern creation.'
Assert-Contains $ffiHeader 'terminal_app_ffi_fzf_pattern_is_empty' 'The C ABI must expose Rust-owned FZF pattern emptiness.'
Assert-Contains $ffiHeader 'terminal_app_ffi_fzf_match_utf16' 'The C ABI must expose Rust-owned FZF matching.'
Assert-Contains $ffiHeader 'terminal_app_ffi_fzf_pattern_destroy' 'The C ABI must expose Rust-owned FZF pattern destruction.'

Assert-Contains $rustFzf 'pub fn parse_pattern' 'terminal-app must remain the semantic FZF pattern owner.'
Assert-Contains $rustFzf 'pub fn match_text' 'terminal-app must remain the semantic FZF matcher owner.'

Write-Host 'R09 FZF ownership promotion validated: C++ is a narrow compatibility seam and portable FZF behavior remains Rust-owned.'
