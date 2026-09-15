$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_decac.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiDecacAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_decac.h"')) { throw 'R09 CSI DECAC ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_decac_plan')) { throw 'R09 CSI DECAC ownership gate: product no longer delegates DECAC classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_ASSIGN_COLOR:')) { throw 'R09 CSI DECAC ownership gate: Rust color plan no longer materializes native dispatch.' }
if (-not $source.Contains('static_cast<DispatchTypes::ColorItem>(decacPlan.item)')) { throw 'R09 CSI DECAC ownership gate: native enum adaptation no longer consumes the Rust item.' }
if (-not $source.Contains('decacPlan.foreground_index')) { throw 'R09 CSI DECAC ownership gate: native dispatch no longer consumes the Rust foreground index.' }
if (-not $source.Contains('decacPlan.background_index')) { throw 'R09 CSI DECAC ownership gate: native dispatch no longer consumes the Rust background index.' }
if (-not $source.Contains('if (decacPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_NONE)')) { throw 'R09 CSI DECAC ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::DECAC_AssignColor:')) { throw 'R09 CSI DECAC ownership gate: duplicate C++ DECAC classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_decac_plan')) { throw 'R09 CSI DECAC ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii(",|")')) { throw 'R09 CSI DECAC ownership gate: Rust DECAC identifier classification is missing.' }
if (-not $ffi.Contains('item: parameters.at(0).unwrap_or(0)')) { throw 'R09 CSI DECAC ownership gate: Rust effective item contract changed.' }
if (-not $ffi.Contains('foreground_index: parameters.at(1).unwrap_or(0)')) { throw 'R09 CSI DECAC ownership gate: Rust effective foreground contract changed.' }
if (-not $ffi.Contains('background_index: parameters.at(2).unwrap_or(0)')) { throw 'R09 CSI DECAC ownership gate: Rust effective background contract changed.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI DECAC ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_decac_plan')) { throw 'R09 CSI DECAC ownership gate: native replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_ASSIGN_COLOR')) { throw 'R09 CSI DECAC ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_NONE')) { throw 'R09 CSI DECAC ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECAC ownership gate: native invalid-identifier witness is missing.' }
if (-not $probe.Contains('nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECAC ownership gate: native null-output validation witness is missing.' }

Write-Host 'R09 CSI DECAC ownership gate passed: Rust owns DECAC classification and effective color indices; C++ retains only ABI/native enum adaptation and dispatch materialization.'
