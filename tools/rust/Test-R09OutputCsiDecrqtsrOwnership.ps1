$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_decrqtsr.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiDecrqtsrAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_decrqtsr.h"')) { throw 'R09 CSI DECRQTSR ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_decrqtsr_plan')) { throw 'R09 CSI DECRQTSR ownership gate: product no longer delegates DECRQTSR classification to Rust.' }
if (-not $source.Contains('decrqtsrReportFormatParameter.has_value()')) { throw 'R09 CSI DECRQTSR ownership gate: native seam no longer preserves omitted report-format parameter state into Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REQUEST_TERMINAL_STATE:')) { throw 'R09 CSI DECRQTSR ownership gate: Rust report plan no longer materializes native dispatch.' }
if (-not $source.Contains('static_cast<DispatchTypes::ReportFormat>(decrqtsrPlan.format)')) { throw 'R09 CSI DECRQTSR ownership gate: native enum adaptation no longer consumes the Rust report format.' }
if (-not $source.Contains('decrqtsrReportFormatParameter);')) { throw 'R09 CSI DECRQTSR ownership gate: native VTParameter is no longer preserved at the dispatch seam.' }
if (-not $source.Contains('if (decrqtsrPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_NONE)')) { throw 'R09 CSI DECRQTSR ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::DECRQTSR_RequestTerminalStateReport:')) { throw 'R09 CSI DECRQTSR ownership gate: duplicate C++ DECRQTSR classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_decrqtsr_plan')) { throw 'R09 CSI DECRQTSR ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("$u")')) { throw 'R09 CSI DECRQTSR ownership gate: Rust DECRQTSR identifier classification is missing.' }
if (-not $ffi.Contains('format: parameters.at(0).unwrap_or(0)')) { throw 'R09 CSI DECRQTSR ownership gate: Rust effective report format contract changed.' }
if (-not $ffi.Contains('format_option: parameters.at(1).unwrap_or(-1)')) { throw 'R09 CSI DECRQTSR ownership gate: Rust omission-preserving report option contract changed.' }
if (-not $ffi.Contains('(value >= 0).then_some(value)')) { throw 'R09 CSI DECRQTSR ownership gate: Rust raw-parameter omission decoding changed.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI DECRQTSR ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_decrqtsr_plan')) { throw 'R09 CSI DECRQTSR ownership gate: native replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REQUEST_TERMINAL_STATE')) { throw 'R09 CSI DECRQTSR ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_NONE')) { throw 'R09 CSI DECRQTSR ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECRQTSR ownership gate: native invalid-identifier witness is missing.' }
if (-not $probe.Contains('nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECRQTSR ownership gate: native null-output validation witness is missing.' }

Write-Host 'R09 CSI DECRQTSR ownership gate passed: Rust owns DECRQTSR classification/report format while C++ preserves VTParameter and native dispatch adaptation.'
