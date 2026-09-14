$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_dcs.rs')
$header = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/include/terminal_parser_ffi_output_dcs.h')

if (-not $source.Contains('#include "terminal_parser_ffi_output_dcs.h"')) { throw 'R09 DCS ownership gate: native product no longer includes the Rust DCS ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_dcs_plan')) { throw 'R09 DCS ownership gate: product no longer delegates DCS classification to Rust.' }
if (-not $source.Contains('switch (plan.kind)')) { throw 'R09 DCS ownership gate: native product no longer materializes the Rust DCS plan.' }

$nativeSeams = @(
    '_dispatch->DefineSixelImage(',
    '_dispatch->DownloadDRCS(',
    '_dispatch->AssignUserPreferenceCharset(',
    '_dispatch->DefineMacro(',
    '_dispatch->RestoreTerminalState(',
    '_dispatch->RequestSetting()',
    '_dispatch->RestorePresentationState('
)
foreach ($nativeSeam in $nativeSeams) {
    if (-not $source.Contains($nativeSeam)) { throw "R09 DCS ownership gate: native DCS materialization seam is missing: $nativeSeam" }
}

$legacyCases = @(
    'case DcsActionCodes::SIXEL_DefineImage:',
    'case DcsActionCodes::DECDLD_DownloadDRCS:',
    'case DcsActionCodes::DECAUPSS_AssignUserPreferenceSupplementalSet:',
    'case DcsActionCodes::DECDMAC_DefineMacro:',
    'case DcsActionCodes::DECRSTS_RestoreTerminalState:',
    'case DcsActionCodes::DECRQSS_RequestSetting:',
    'case DcsActionCodes::DECRSPS_RestorePresentationState:'
)
foreach ($legacyCase in $legacyCases) {
    if ($source.Contains($legacyCase)) { throw "R09 DCS ownership gate: duplicate C++ DCS classification returned: $legacyCase" }
}

$rustWitnesses = @(
    'expect("q", OutputDcsKind::DefineSixelImage)',
    'expect("{", OutputDcsKind::DownloadDrcs)',
    'expect("!u", OutputDcsKind::AssignUserPreferenceCharset)',
    'expect("!z", OutputDcsKind::DefineMacro)',
    'expect("$p", OutputDcsKind::RestoreTerminalState)',
    'expect("$q", OutputDcsKind::RequestSetting)',
    'expect("$t", OutputDcsKind::RestorePresentationState)',
    'expect("x", OutputDcsKind::None)'
)
foreach ($rustWitness in $rustWitnesses) {
    if (-not $ffi.Contains($rustWitness)) { throw "R09 DCS ownership gate: Rust contract replay witness is missing: $rustWitness" }
}
if (-not $ffi.Contains('terminal_parser_ffi_output_dcs_plan')) { throw 'R09 DCS ownership gate: terminal-parser-ffi no longer exports the DCS plan seam.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 DCS ownership gate: Rust invalid-argument witness is missing.' }

$abiKinds = @(
    'TERMINAL_PARSER_FFI_OUTPUT_DCS_DEFINE_SIXEL_IMAGE',
    'TERMINAL_PARSER_FFI_OUTPUT_DCS_DOWNLOAD_DRCS',
    'TERMINAL_PARSER_FFI_OUTPUT_DCS_ASSIGN_USER_PREFERENCE_CHARSET',
    'TERMINAL_PARSER_FFI_OUTPUT_DCS_DEFINE_MACRO',
    'TERMINAL_PARSER_FFI_OUTPUT_DCS_RESTORE_TERMINAL_STATE',
    'TERMINAL_PARSER_FFI_OUTPUT_DCS_REQUEST_SETTING',
    'TERMINAL_PARSER_FFI_OUTPUT_DCS_RESTORE_PRESENTATION_STATE'
)
foreach ($abiKind in $abiKinds) {
    if (-not $header.Contains($abiKind)) { throw "R09 DCS ownership gate: ABI kind is missing: $abiKind" }
}
if (-not $header.Contains('static_assert(sizeof(terminal_parser_ffi_output_dcs_plan) == 4)')) { throw 'R09 DCS ownership gate: DCS plan ABI size assertion is missing.' }

Write-Host 'R09 DCS ownership gate passed: Rust owns DCS identifier classification; C++ retains parameters, StringHandler, native dispatch materialization, UnknownSequence, and last-character lifecycle.'