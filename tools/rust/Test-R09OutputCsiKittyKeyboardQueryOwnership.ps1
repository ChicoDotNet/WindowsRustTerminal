$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_kitty_keyboard_query.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiKittyKeyboardQueryAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_kitty_keyboard_query.h"')) { throw 'R09 CSI Kitty keyboard query ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_query_plan')) { throw 'R09 CSI Kitty keyboard query ownership gate: product no longer delegates KKP query classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_QUERY:')) { throw 'R09 CSI Kitty keyboard query ownership gate: Rust query plan no longer materializes native dispatch.' }
if (-not $source.Contains('_dispatch->QueryKittyKeyboardProtocol();')) { throw 'R09 CSI Kitty keyboard query ownership gate: native Kitty query dispatch materialization is missing.' }
if (-not $source.Contains('if (kittyKeyboardQueryPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_NONE)')) { throw 'R09 CSI Kitty keyboard query ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::KKP_KittyKeyboardQuery:')) { throw 'R09 CSI Kitty keyboard query ownership gate: duplicate C++ KKP query classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_query_plan')) { throw 'R09 CSI Kitty keyboard query ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("?u")')) { throw 'R09 CSI Kitty keyboard query ownership gate: Rust KKP query identifier classification is missing.' }
if (-not $ffi.Contains('expect("?u", OutputCsiKittyKeyboardQueryKind::Query)')) { throw 'R09 CSI Kitty keyboard query ownership gate: Rust positive replay witness is missing.' }
if (-not $ffi.Contains('expect("&u", OutputCsiKittyKeyboardQueryKind::None)')) { throw 'R09 CSI Kitty keyboard query ownership gate: Rust unrelated-CSI rejection witness is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI Kitty keyboard query ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_query_plan')) { throw 'R09 CSI Kitty keyboard query ownership gate: native KKP query replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_QUERY')) { throw 'R09 CSI Kitty keyboard query ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_NONE')) { throw 'R09 CSI Kitty keyboard query ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI Kitty keyboard query ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI Kitty keyboard query ownership gate passed: Rust owns KKP query classification; C++ retains only ABI adaptation and native Kitty query dispatch materialization.'
