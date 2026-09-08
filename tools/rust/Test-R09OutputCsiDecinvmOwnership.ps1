$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_decinvm.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiDecinvmAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_decinvm.h"')) { throw 'R09 CSI DECINVM ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_decinvm_plan')) { throw 'R09 CSI DECINVM ownership gate: product no longer delegates DECINVM classification to Rust.' }
if (-not $source.Contains('parameters.at(0).value_or(0)')) { throw 'R09 CSI DECINVM ownership gate: product no longer supplies the effective default macro identifier to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECINVM_INVOKE_MACRO:')) { throw 'R09 CSI DECINVM ownership gate: Rust macro plan no longer materializes native dispatch.' }
if (-not $source.Contains('_dispatch->InvokeMacro(decinvmPlan.macro_id);')) { throw 'R09 CSI DECINVM ownership gate: native dispatch no longer consumes the Rust macro identifier.' }
if (-not $source.Contains('if (decinvmPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECINVM_NONE)')) { throw 'R09 CSI DECINVM ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::DECINVM_InvokeMacro:')) { throw 'R09 CSI DECINVM ownership gate: duplicate C++ DECINVM classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_decinvm_plan')) { throw 'R09 CSI DECINVM ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("*z")')) { throw 'R09 CSI DECINVM ownership gate: Rust DECINVM identifier classification is missing.' }
if (-not $ffi.Contains('macro_id: parameters.at(0).unwrap_or(0)')) { throw 'R09 CSI DECINVM ownership gate: Rust effective macro identifier contract changed.' }
if (-not $ffi.Contains('expect("*y", 1, OutputCsiDecinvmKind::None, 0)')) { throw 'R09 CSI DECINVM ownership gate: Rust unrelated-CSI rejection witness is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI DECINVM ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_decinvm_plan')) { throw 'R09 CSI DECINVM ownership gate: native replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECINVM_INVOKE_MACRO')) { throw 'R09 CSI DECINVM ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECINVM_NONE')) { throw 'R09 CSI DECINVM ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECINVM ownership gate: native invalid-identifier witness is missing.' }
if (-not $probe.Contains('nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECINVM ownership gate: native null-output validation witness is missing.' }

Write-Host 'R09 CSI DECINVM ownership gate passed: Rust owns DECINVM classification and effective macro identifier; C++ retains only ABI adaptation and native dispatch materialization.'
