$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_decrqcra.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiDecrqcraAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_decrqcra.h"')) { throw 'R09 CSI DECRQCRA ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_decrqcra_plan')) { throw 'R09 CSI DECRQCRA ownership gate: product no longer delegates DECRQCRA classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_REQUEST_CHECKSUM_RECTANGULAR_AREA:')) { throw 'R09 CSI DECRQCRA ownership gate: Rust DECRQCRA plan no longer materializes native dispatch.' }
if (-not $source.Contains('decrqcraPlan.request_id')) { throw 'R09 CSI DECRQCRA ownership gate: native dispatch no longer consumes Rust request-id normalization.' }
if (-not $source.Contains('decrqcraPlan.page')) { throw 'R09 CSI DECRQCRA ownership gate: native dispatch no longer consumes Rust page normalization.' }
if (-not $source.Contains('decrqcraPlan.bottom')) { throw 'R09 CSI DECRQCRA ownership gate: native dispatch no longer consumes Rust bottom normalization.' }
if (-not $source.Contains('decrqcraPlan.right')) { throw 'R09 CSI DECRQCRA ownership gate: native dispatch no longer consumes Rust right normalization.' }
if (-not $source.Contains('parameters.at(2)')) { throw 'R09 CSI DECRQCRA ownership gate: native top VTParameter seam is missing.' }
if (-not $source.Contains('parameters.at(3)')) { throw 'R09 CSI DECRQCRA ownership gate: native left VTParameter seam is missing.' }
if (-not $source.Contains('if (decrqcraPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_NONE)')) { throw 'R09 CSI DECRQCRA ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::DECRQCRA_RequestChecksumRectangularArea:')) { throw 'R09 CSI DECRQCRA ownership gate: duplicate C++ DECRQCRA classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_decrqcra_plan')) { throw 'R09 CSI DECRQCRA ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("*y")')) { throw 'R09 CSI DECRQCRA ownership gate: Rust DECRQCRA identifier classification is missing.' }
if (-not $ffi.Contains('request_id: parameters.at(0).unwrap_or(0)')) { throw 'R09 CSI DECRQCRA ownership gate: Rust request-id contract changed.' }
if (-not $ffi.Contains('page: parameters.at(1).unwrap_or(0)')) { throw 'R09 CSI DECRQCRA ownership gate: Rust page contract changed.' }
if (-not $ffi.Contains('bottom: parameters.at(4).unwrap_or(0)')) { throw 'R09 CSI DECRQCRA ownership gate: Rust bottom contract changed.' }
if (-not $ffi.Contains('right: parameters.at(5).unwrap_or(0)')) { throw 'R09 CSI DECRQCRA ownership gate: Rust right contract changed.' }
if (-not $ffi.Contains('top and left VTParameter')) { throw 'R09 CSI DECRQCRA ownership gate: native VTParameter ownership boundary is no longer documented.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI DECRQCRA ownership gate: Rust invalid-argument contract is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_decrqcra_plan')) { throw 'R09 CSI DECRQCRA ownership gate: native replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_REQUEST_CHECKSUM_RECTANGULAR_AREA')) { throw 'R09 CSI DECRQCRA ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_NONE')) { throw 'R09 CSI DECRQCRA ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECRQCRA ownership gate: native invalid-identifier witness is missing.' }
if (-not $probe.Contains('nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECRQCRA ownership gate: native null-output validation witness is missing.' }

Write-Host 'R09 CSI DECRQCRA ownership gate passed: Rust owns *y classification and request-id/page/bottom/right normalization; C++ retains top/left VTParameter representation and dispatch materialization.'
