$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_kitty_keyboard_push.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiKittyKeyboardPushAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_kitty_keyboard_push.h"')) { throw 'R09 CSI Kitty keyboard push ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_push_plan')) { throw 'R09 CSI Kitty keyboard push ownership gate: product no longer delegates KKP push classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_PUSH:')) { throw 'R09 CSI Kitty keyboard push ownership gate: Rust push plan no longer materializes native dispatch.' }
if (-not $source.Contains('_dispatch->PushKittyKeyboardProtocol(kittyKeyboardPushPlan.flags);')) { throw 'R09 CSI Kitty keyboard push ownership gate: native Kitty push dispatch materialization is missing or bypasses Rust flags.' }
if (-not $source.Contains('if (kittyKeyboardPushPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_NONE)')) { throw 'R09 CSI Kitty keyboard push ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::KKP_KittyKeyboardPush:')) { throw 'R09 CSI Kitty keyboard push ownership gate: duplicate C++ KKP push classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_push_plan')) { throw 'R09 CSI Kitty keyboard push ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii(">u")')) { throw 'R09 CSI Kitty keyboard push ownership gate: Rust KKP push identifier classification is missing.' }
if (-not $ffi.Contains('parameters.at(0).unwrap_or(0)')) { throw 'R09 CSI Kitty keyboard push ownership gate: Rust effective-flags contract changed.' }
if (-not $ffi.Contains('expect(">u", 42, OutputCsiKittyKeyboardPushKind::Push, 42)')) { throw 'R09 CSI Kitty keyboard push ownership gate: Rust positive replay witness is missing.' }
if (-not $ffi.Contains('expect("?u", 42, OutputCsiKittyKeyboardPushKind::None, 0)')) { throw 'R09 CSI Kitty keyboard push ownership gate: Rust unrelated-CSI rejection witness is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI Kitty keyboard push ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_push_plan')) { throw 'R09 CSI Kitty keyboard push ownership gate: native KKP push replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_PUSH')) { throw 'R09 CSI Kitty keyboard push ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_NONE')) { throw 'R09 CSI Kitty keyboard push ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI Kitty keyboard push ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI Kitty keyboard push ownership gate passed: Rust owns KKP push classification and effective flags; C++ retains only ABI adaptation and native Kitty push dispatch materialization.'
