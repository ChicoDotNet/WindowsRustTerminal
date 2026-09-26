$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_column.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiColumnAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_column.h"')) { throw 'R09 CSI column ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_column_plan')) { throw 'R09 CSI column ownership gate: product no longer delegates DECIC/DECDC classification to Rust.' }
if (-not $source.Contains('_dispatch->InsertColumn(columnPlan.count)')) { throw 'R09 CSI column ownership gate: native InsertColumn dispatch materialization is missing.' }
if (-not $source.Contains('_dispatch->DeleteColumn(columnPlan.count)')) { throw 'R09 CSI column ownership gate: native DeleteColumn dispatch materialization is missing.' }
if (-not $source.Contains('if (columnPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_NONE)')) { throw 'R09 CSI column ownership gate: Rust plan no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::DECIC_InsertColumn:')) { throw 'R09 CSI column ownership gate: duplicate C++ DECIC classification returned.' }
if ($source.Contains('case CsiActionCodes::DECDC_DeleteColumn:')) { throw 'R09 CSI column ownership gate: duplicate C++ DECDC classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_column_plan')) { throw 'R09 CSI column ownership gate: terminal-parser-ffi no longer exports the plan seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("''}")')) { throw 'R09 CSI column ownership gate: Rust DECIC identifier classification is missing.' }
if (-not $ffi.Contains('id == VtId::from_ascii("''~")')) { throw 'R09 CSI column ownership gate: Rust DECDC identifier classification is missing.' }
if (-not $ffi.Contains('expect("''}", 4, OutputCsiColumnKind::InsertColumn, 4)')) { throw 'R09 CSI column ownership gate: Rust DECIC replay witness is missing.' }
if (-not $ffi.Contains('expect("''~", 7, OutputCsiColumnKind::DeleteColumn, 7)')) { throw 'R09 CSI column ownership gate: Rust DECDC replay witness is missing.' }
if (-not $ffi.Contains('expect("$x", 3, OutputCsiColumnKind::None, 0)')) { throw 'R09 CSI column ownership gate: unrelated CSI rejection witness is missing.' }

if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_INSERT')) { throw 'R09 CSI column ownership gate: native DECIC replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_DELETE')) { throw 'R09 CSI column ownership gate: native DECDC replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI column ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI column ownership gate passed: Rust owns DECIC/DECDC classification and count; C++ retains only ABI adaptation and native column dispatch materialization.'
