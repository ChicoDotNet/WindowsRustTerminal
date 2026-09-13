$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_rect_erase.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiRectEraseAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_rect_erase.h"')) { throw 'R09 CSI rectangular erase ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_rect_erase_values')) { throw 'R09 CSI rectangular erase ownership gate: product no longer delegates DECERA/DECSERA classification and payload identity to Rust.' }
if (-not $source.Contains('if (rectEraseKind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE_NONE)')) { throw 'R09 CSI rectangular erase ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if (-not $source.Contains('_dispatch->EraseRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0))')) { throw 'R09 CSI rectangular erase ownership gate: native DECERA dispatch materialization is missing.' }
if (-not $source.Contains('_dispatch->SelectiveEraseRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0))')) { throw 'R09 CSI rectangular erase ownership gate: native DECSERA dispatch materialization is missing.' }
if ($source.Contains('case CsiActionCodes::DECERA_EraseRectangularArea:')) { throw 'R09 CSI rectangular erase ownership gate: duplicate C++ DECERA classification returned.' }
if ($source.Contains('case CsiActionCodes::DECSERA_SelectiveEraseRectangularArea:')) { throw 'R09 CSI rectangular erase ownership gate: duplicate C++ DECSERA classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_rect_erase_values')) { throw 'R09 CSI rectangular erase ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("$z")')) { throw 'R09 CSI rectangular erase ownership gate: Rust DECERA identifier classification is missing.' }
if (-not $ffi.Contains('id == VtId::from_ascii("${")')) { throw 'R09 CSI rectangular erase ownership gate: Rust DECSERA identifier classification is missing.' }
if (-not $ffi.Contains('replay("$z", &[1, 2, 3, 4, 99])')) { throw 'R09 CSI rectangular erase ownership gate: Rust DECERA complete-payload witness is missing.' }
if (-not $ffi.Contains('replay("${", &[5, 6, 0, 0])')) { throw 'R09 CSI rectangular erase ownership gate: Rust DECSERA complete-payload witness is missing.' }
if (-not $ffi.Contains('replay("$x", &[1, 2, 3, 4])')) { throw 'R09 CSI rectangular erase ownership gate: unrelated CSI rejection witness is missing.' }
if (-not $ffi.Contains('FfiStatus::BufferTooSmall')) { throw 'R09 CSI rectangular erase ownership gate: Rust capacity witness is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI rectangular erase ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE')) { throw 'R09 CSI rectangular erase ownership gate: native DECERA replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_SELECTIVE_ERASE')) { throw 'R09 CSI rectangular erase ownership gate: native DECSERA replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL')) { throw 'R09 CSI rectangular erase ownership gate: native capacity witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI rectangular erase ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI rectangular erase ownership gate passed: Rust owns DECERA/DECSERA classification and flat payload identity; C++ retains only ABI adaptation and native rectangle dispatch materialization.'
