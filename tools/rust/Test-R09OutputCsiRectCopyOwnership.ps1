$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_rect_copy.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiRectCopyAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_rect_copy.h"')) { throw 'R09 CSI rectangular copy ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_rect_copy_values')) { throw 'R09 CSI rectangular copy ownership gate: product no longer delegates DECCRA classification and flat payload identity to Rust.' }
if (-not $source.Contains('if (rectCopyMatched != 0)')) { throw 'R09 CSI rectangular copy ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if (-not $source.Contains('_dispatch->CopyRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0), parameters.at(4), parameters.at(5), parameters.at(6), parameters.at(7))')) { throw 'R09 CSI rectangular copy ownership gate: native DECCRA dispatch materialization is missing.' }
if ($source.Contains('case CsiActionCodes::DECCRA_CopyRectangularArea:')) { throw 'R09 CSI rectangular copy ownership gate: duplicate C++ DECCRA classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_rect_copy_values')) { throw 'R09 CSI rectangular copy ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("$v")')) { throw 'R09 CSI rectangular copy ownership gate: Rust DECCRA identifier classification is missing.' }
if (-not $ffi.Contains('replay("$v", &[1, 2, 3, 4, 5, 6, 7, 8, 99])')) { throw 'R09 CSI rectangular copy ownership gate: Rust complete-payload witness is missing.' }
if (-not $ffi.Contains('replay("$x", &[1, 2, 3, 4])')) { throw 'R09 CSI rectangular copy ownership gate: unrelated CSI rejection witness is missing.' }
if (-not $ffi.Contains('FfiStatus::BufferTooSmall')) { throw 'R09 CSI rectangular copy ownership gate: Rust capacity witness is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI rectangular copy ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_rect_copy_values')) { throw 'R09 CSI rectangular copy ownership gate: native DECCRA replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL')) { throw 'R09 CSI rectangular copy ownership gate: native capacity witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI rectangular copy ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI rectangular copy ownership gate passed: Rust owns DECCRA classification and flat payload identity; C++ retains only ABI adaptation and native rectangle-copy dispatch materialization.'
