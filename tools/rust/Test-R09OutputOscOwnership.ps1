$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_osc.rs')
$header = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/include/terminal_parser_ffi_output_osc.h')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputOscAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_osc.h"')) { throw 'R09 OSC ownership gate: native product no longer includes the Rust OSC ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_osc_plan')) { throw 'R09 OSC ownership gate: product no longer delegates OSC classification to Rust.' }
if (-not $source.Contains('switch (plan.kind)')) { throw 'R09 OSC ownership gate: native product no longer materializes the Rust OSC plan.' }

$nativeSeams = @(
    '_dispatch->SetWindowTitle(',
    '_GetOscSetColorTable(',
    '_GetOscSetColor(',
    '_GetOscSetClipboard(',
    '_dispatch->SetCurrentWorkingDirectory(',
    '_ParseHyperlink(',
    '_dispatch->DoConEmuAction(',
    '_dispatch->DoFinalTermAction(',
    '_dispatch->DoVsCodeAction(',
    '_dispatch->DoUrxvtAction(',
    '_dispatch->DoITerm2Action(',
    '_dispatch->DoWTAction('
)
foreach ($nativeSeam in $nativeSeams) {
    if (-not $source.Contains($nativeSeam)) { throw "R09 OSC ownership gate: native OSC materialization seam is missing: $nativeSeam" }
}

$legacyCases = @(
    'case OscActionCodes::SetIconAndWindowTitle:',
    'case OscActionCodes::SetWindowIcon:',
    'case OscActionCodes::SetWindowTitle:',
    'case OscActionCodes::DECSWT_SetWindowTitle:',
    'case OscActionCodes::SetColor:',
    'case OscActionCodes::SetForegroundColor:',
    'case OscActionCodes::SetBackgroundColor:',
    'case OscActionCodes::SetCursorColor:',
    'case OscActionCodes::SetHighlightColor:',
    'case OscActionCodes::SetClipboard:',
    'case OscActionCodes::ResetColor:',
    'case OscActionCodes::ResetForegroundColor:',
    'case OscActionCodes::ResetBackgroundColor:',
    'case OscActionCodes::ResetCursorColor:',
    'case OscActionCodes::ResetHighlightColor:',
    'case OscActionCodes::CurrentWorkingDirectory:',
    'case OscActionCodes::Hyperlink:',
    'case OscActionCodes::ConEmuAction:',
    'case OscActionCodes::FinalTermAction:',
    'case OscActionCodes::VsCodeAction:',
    'case OscActionCodes::UrxvtAction:',
    'case OscActionCodes::ITerm2Action:',
    'case OscActionCodes::WTAction:'
)
foreach ($legacyCase in $legacyCases) {
    if ($source.Contains($legacyCase)) { throw "R09 OSC ownership gate: duplicate C++ OSC classification returned: $legacyCase" }
}

$abiKinds = @(
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_WINDOW_TITLE',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_COLOR_TABLE',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_DYNAMIC_COLOR',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_CLIPBOARD',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_COLOR_TABLE',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_DYNAMIC_COLOR',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_CURRENT_WORKING_DIRECTORY',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_HYPERLINK',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_CONEMU_ACTION',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_FINALTERM_ACTION',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_VSCODE_ACTION',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_URXVT_ACTION',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_ITERM2_ACTION',
    'TERMINAL_PARSER_FFI_OUTPUT_OSC_WT_ACTION'
)
foreach ($abiKind in $abiKinds) {
    if (-not $header.Contains($abiKind)) { throw "R09 OSC ownership gate: ABI kind is missing: $abiKind" }
    if (-not $probe.Contains($abiKind)) { throw "R09 OSC ownership gate: native ABI replay witness is missing: $abiKind" }
}

if (-not $ffi.Contains('terminal_parser_ffi_output_osc_plan')) { throw 'R09 OSC ownership gate: terminal-parser-ffi no longer exports the OSC plan seam.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 OSC ownership gate: Rust invalid-argument witness is missing.' }
if (-not $header.Contains('terminal_parser_ffi_output_osc_plan_result')) { throw 'R09 OSC ownership gate: OSC result type is missing.' }
if (-not $header.Contains('static_assert(sizeof(terminal_parser_ffi_output_osc_plan_result) == 4)')) { throw 'R09 OSC ownership gate: OSC plan ABI size assertion is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 OSC ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 OSC ownership gate passed: Rust owns OSC parameter classification; C++ retains payload parsing, native dispatch materialization, UnknownSequence, and last-character lifecycle.'
