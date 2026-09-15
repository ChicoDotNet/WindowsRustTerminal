$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_request_presentation_state.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiRequestPresentationStateAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_request_presentation_state.h"')) { throw 'R09 CSI presentation state request ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_request_presentation_state_plan')) { throw 'R09 CSI presentation state request ownership gate: product no longer delegates DECRQPSR classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_REQUEST:')) { throw 'R09 CSI presentation state request ownership gate: Rust request plan no longer materializes native dispatch.' }
if (-not $source.Contains('static_cast<DispatchTypes::PresentationReportFormat>(presentationStatePlan.format)')) { throw 'R09 CSI presentation state request ownership gate: native report dispatch no longer adapts the Rust format.' }
if (-not $source.Contains('if (presentationStatePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_NONE)')) { throw 'R09 CSI presentation state request ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::DECRQPSR_RequestPresentationStateReport:')) { throw 'R09 CSI presentation state request ownership gate: duplicate C++ DECRQPSR classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_request_presentation_state_plan')) { throw 'R09 CSI presentation state request ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("$w")')) { throw 'R09 CSI presentation state request ownership gate: Rust DECRQPSR identifier classification is missing.' }
if (-not $ffi.Contains('format: parameters.at(0).unwrap_or(0)')) { throw 'R09 CSI presentation state request ownership gate: Rust effective presentation format contract changed.' }
if (-not $ffi.Contains('OutputCsiRequestPresentationStateKind::Request')) { throw 'R09 CSI presentation state request ownership gate: Rust positive replay witness is missing.' }
if (-not $ffi.Contains('OutputCsiRequestPresentationStateKind::None')) { throw 'R09 CSI presentation state request ownership gate: Rust unrelated-CSI rejection witness is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI presentation state request ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_request_presentation_state_plan')) { throw 'R09 CSI presentation state request ownership gate: native DECRQPSR replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_REQUEST')) { throw 'R09 CSI presentation state request ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_PRESENTATION_STATE_NONE')) { throw 'R09 CSI presentation state request ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI presentation state request ownership gate: native invalid-identifier witness is missing.' }
if (-not $probe.Contains('nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI presentation state request ownership gate: native null-output validation witness is missing.' }

Write-Host 'R09 CSI presentation state request ownership gate passed: Rust owns DECRQPSR classification and effective format; C++ retains only ABI adaptation and native report materialization.'
