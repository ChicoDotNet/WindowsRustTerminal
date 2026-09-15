$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_user_preference_charset.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiUserPreferenceCharsetAbiProbe.hpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_user_preference_charset.h"')) { throw 'R09 CSI user preference charset ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_user_preference_charset_plan')) { throw 'R09 CSI user preference charset ownership gate: product no longer delegates DECRQUPSS classification to Rust.' }
if (-not $source.Contains('case TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_REQUEST:')) { throw 'R09 CSI user preference charset ownership gate: Rust request plan no longer materializes native dispatch.' }
if (-not $source.Contains('_dispatch->RequestUserPreferenceCharset();')) { throw 'R09 CSI user preference charset ownership gate: native charset dispatch materialization is missing.' }
if (-not $source.Contains('if (userPreferenceCharsetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE)')) { throw 'R09 CSI user preference charset ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::DECRQUPSS_RequestUserPreferenceSupplementalSet:')) { throw 'R09 CSI user preference charset ownership gate: duplicate C++ DECRQUPSS classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_user_preference_charset_plan')) { throw 'R09 CSI user preference charset ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('id == VtId::from_ascii("&u")')) { throw 'R09 CSI user preference charset ownership gate: Rust DECRQUPSS identifier classification is missing.' }
if (-not $ffi.Contains('expect("&u", OutputCsiUserPreferenceCharsetKind::Request)')) { throw 'R09 CSI user preference charset ownership gate: Rust positive replay witness is missing.' }
if (-not $ffi.Contains('expect("$u", OutputCsiUserPreferenceCharsetKind::None)')) { throw 'R09 CSI user preference charset ownership gate: Rust unrelated-CSI rejection witness is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI user preference charset ownership gate: Rust invalid-argument witness is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_user_preference_charset_plan')) { throw 'R09 CSI user preference charset ownership gate: native DECRQUPSS replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_REQUEST')) { throw 'R09 CSI user preference charset ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE')) { throw 'R09 CSI user preference charset ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI user preference charset ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI user preference charset ownership gate passed: Rust owns DECRQUPSS classification; C++ retains only ABI adaptation and native charset dispatch materialization.'
