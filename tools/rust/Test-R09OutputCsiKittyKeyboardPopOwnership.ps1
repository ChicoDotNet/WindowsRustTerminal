$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_kitty_keyboard_pop.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiKittyKeyboardPopAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_kitty_keyboard_pop.h"')) { throw 'R09 CSI Kitty keyboard pop ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_pop_plan')) { throw 'R09 CSI Kitty keyboard pop ownership gate: product no longer delegates KKP pop classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_POP:')) { throw 'R09 CSI Kitty keyboard pop ownership gate: Rust pop plan no longer materializes native dispatch.' }
if (-not $source.Contains('_dispatch->PopKittyKeyboardProtocol(kittyKeyboardPopPlan.count);')) { throw 'R09 CSI Kitty keyboard pop ownership gate: native Kitty pop dispatch materialization is missing or bypasses Rust count.' }
if (-not $source.Contains('if (kittyKeyboardPopPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_NONE)')) { throw 'R09 CSI Kitty keyboard pop ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::KKP_KittyKeyboardPop:')) { throw 'R09 CSI Kitty keyboard pop ownership gate: duplicate C++ KKP pop classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_pop_plan')) { throw 'R09 CSI Kitty keyboard pop ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("<u")')) { throw 'R09 CSI Kitty keyboard pop ownership gate: Rust KKP pop identifier classification is missing.' }
if (-not $ffi.Contains('parameters.at(0).unwrap_or(0)')) { throw 'R09 CSI Kitty keyboard pop ownership gate: Rust effective-count contract changed.' }
if (-not $ffi.Contains('expect("<u", 7, OutputCsiKittyKeyboardPopKind::Pop, 7)')) { throw 'R09 CSI Kitty keyboard pop ownership gate: Rust positive replay witness is missing.' }
if (-not $ffi.Contains('expect(">u", 7, OutputCsiKittyKeyboardPopKind::None, 0)')) { throw 'R09 CSI Kitty keyboard pop ownership gate: Rust unrelated-CSI rejection witness is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI Kitty keyboard pop ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_pop_plan')) { throw 'R09 CSI Kitty keyboard pop ownership gate: native KKP pop replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_POP')) { throw 'R09 CSI Kitty keyboard pop ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_NONE')) { throw 'R09 CSI Kitty keyboard pop ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI Kitty keyboard pop ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI Kitty keyboard pop ownership gate passed: Rust owns KKP pop classification and effective count; C++ retains only ABI adaptation and native Kitty pop dispatch materialization.'
