$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_kitty_keyboard_set.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiKittyKeyboardSetAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_kitty_keyboard_set.h"')) { throw 'R09 CSI Kitty keyboard set ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_set_plan')) { throw 'R09 CSI Kitty keyboard set ownership gate: product no longer delegates KKP set classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_SET:')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust set plan no longer materializes native dispatch.' }
if (-not $source.Contains('kittyKeyboardSetPlan.has_flags != 0 ? VTParameter{ kittyKeyboardSetPlan.flags } : VTParameter{}')) { throw 'R09 CSI Kitty keyboard set ownership gate: native Kitty set dispatch no longer reconstructs optional flags from the Rust plan.' }
if (-not $source.Contains('kittyKeyboardSetPlan.has_mode != 0 ? VTParameter{ kittyKeyboardSetPlan.mode } : VTParameter{}')) { throw 'R09 CSI Kitty keyboard set ownership gate: native Kitty set dispatch no longer reconstructs optional mode from the Rust plan.' }
if (-not $source.Contains('if (kittyKeyboardSetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_NONE)')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::KKP_KittyKeyboardSet:')) { throw 'R09 CSI Kitty keyboard set ownership gate: duplicate C++ KKP set classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_set_plan')) { throw 'R09 CSI Kitty keyboard set ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("=u")')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust KKP set identifier classification is missing.' }
if (-not $ffi.Contains('has_flags: u32::from(flags.is_some())')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust optional-flags contract changed.' }
if (-not $ffi.Contains('has_mode: u32::from(mode.is_some())')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust optional-mode contract changed.' }
if (-not $ffi.Contains('OutputCsiKittyKeyboardSetKind::Set')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust positive replay witness is missing.' }
if (-not $ffi.Contains('OutputCsiKittyKeyboardSetKind::None')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust unrelated-CSI rejection witness is missing.' }
if (-not $ffi.Contains('has_flags > 1 || has_mode > 1')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust presence-bit validation is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI Kitty keyboard set ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_set_plan')) { throw 'R09 CSI Kitty keyboard set ownership gate: native KKP set replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_SET')) { throw 'R09 CSI Kitty keyboard set ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_NONE')) { throw 'R09 CSI Kitty keyboard set ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('invalidFlagsPresenceStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI Kitty keyboard set ownership gate: native flags-presence validation witness is missing.' }
if (-not $probe.Contains('invalidModePresenceStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI Kitty keyboard set ownership gate: native mode-presence validation witness is missing.' }
if (-not $probe.Contains('nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI Kitty keyboard set ownership gate: native null-output validation witness is missing.' }

Write-Host 'R09 CSI Kitty keyboard set ownership gate passed: Rust owns KKP set classification and optional flags/mode; C++ retains only ABI adaptation and native Kitty set dispatch materialization.'
