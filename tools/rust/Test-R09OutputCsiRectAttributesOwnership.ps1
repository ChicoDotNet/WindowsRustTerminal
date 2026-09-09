$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_rect_attributes.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiRectAttributesAbiProbe.hpp')

if (-not $source.Contains('#include "rust_ffi/output_csi_rect_attributes.c.h"')) { throw 'R09 CSI rectangular attributes ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_rect_attributes_plan')) { throw 'R09 CSI rectangular attributes ownership gate: product no longer delegates DECCARA/DECRARA classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_CHANGE:')) { throw 'R09 CSI rectangular attributes ownership gate: Rust DECCARA plan no longer materializes native dispatch.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_REVERSE:')) { throw 'R09 CSI rectangular attributes ownership gate: Rust DECRARA plan no longer materializes native dispatch.' }
if (-not $source.Contains('rectAttributesPlan.bottom')) { throw 'R09 CSI rectangular attributes ownership gate: native dispatch no longer consumes Rust bottom normalization.' }
if (-not $source.Contains('rectAttributesPlan.right')) { throw 'R09 CSI rectangular attributes ownership gate: native dispatch no longer consumes Rust right normalization.' }
if (-not $source.Contains('parameters.at(0)')) { throw 'R09 CSI rectangular attributes ownership gate: native top VTParameter seam is missing.' }
if (-not $source.Contains('parameters.at(1)')) { throw 'R09 CSI rectangular attributes ownership gate: native left VTParameter seam is missing.' }
if (-not $source.Contains('parameters.subspan(4)')) { throw 'R09 CSI rectangular attributes ownership gate: native trailing VTParameters seam is missing.' }
if (-not $source.Contains('if (rectAttributesPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_NONE)')) { throw 'R09 CSI rectangular attributes ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::DECCARA_ChangeAttributesInRectangularArea:')) { throw 'R09 CSI rectangular attributes ownership gate: duplicate C++ DECCARA classification returned.' }
if ($source.Contains('case CsiActionCodes::DECRARA_ReverseAttributesInRectangularArea:')) { throw 'R09 CSI rectangular attributes ownership gate: duplicate C++ DECRARA classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_rect_attributes_plan')) { throw 'R09 CSI rectangular attributes ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("$r")')) { throw 'R09 CSI rectangular attributes ownership gate: Rust DECCARA identifier classification is missing.' }
if (-not $ffi.Contains('id == VtId::from_ascii("$t")')) { throw 'R09 CSI rectangular attributes ownership gate: Rust DECRARA identifier classification is missing.' }
if (-not $ffi.Contains('bottom: parameters.at(2).unwrap_or(0)')) { throw 'R09 CSI rectangular attributes ownership gate: Rust effective bottom contract changed.' }
if (-not $ffi.Contains('right: parameters.at(3).unwrap_or(0)')) { throw 'R09 CSI rectangular attributes ownership gate: Rust effective right contract changed.' }
if (-not $ffi.Contains('leading VTParameter values')) { throw 'R09 CSI rectangular attributes ownership gate: native VTParameter ownership boundary is no longer documented.' }
if (-not $ffi.Contains('trailing SGR parameter span intentionally remain C++-owned')) { throw 'R09 CSI rectangular attributes ownership gate: native VTParameters ownership boundary is no longer documented.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI rectangular attributes ownership gate: Rust invalid-argument contract is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_rect_attributes_plan')) { throw 'R09 CSI rectangular attributes ownership gate: native replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_CHANGE')) { throw 'R09 CSI rectangular attributes ownership gate: native DECCARA positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_REVERSE')) { throw 'R09 CSI rectangular attributes ownership gate: native DECRARA positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_NONE')) { throw 'R09 CSI rectangular attributes ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI rectangular attributes ownership gate: native invalid-identifier witness is missing.' }
if (-not $probe.Contains('nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI rectangular attributes ownership gate: native null-output validation witness is missing.' }

Write-Host 'R09 CSI rectangular attributes ownership gate passed: Rust owns DECCARA/DECRARA classification and bottom/right normalization; C++ retains native VTParameter/VTParameters representation and dispatch materialization.'
