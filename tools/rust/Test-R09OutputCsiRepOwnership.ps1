$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$source = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp')
$ffi = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_rep.rs')
$probe = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09OutputCsiRepAbiProbe.hpp')
$runner = Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'tools/rust/R09ControlCharacterAbiProbe.cpp')

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_rep.h"')) { throw 'R09 CSI REP ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $source.Contains('terminal_parser_ffi_output_csi_rep_plan')) { throw 'R09 CSI REP ownership gate: product no longer delegates REP classification to Rust.' }
if (-not $source.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_REPEAT')) { throw 'R09 CSI REP ownership gate: Rust REP repeat plan no longer materializes native PrintString.' }
if (-not $source.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_HANDLED_NOOP')) { throw 'R09 CSI REP ownership gate: handled no-op state is no longer preserved.' }
if (-not $source.Contains('repPlan.count')) { throw 'R09 CSI REP ownership gate: native materialization no longer consumes Rust count normalization.' }
if (-not $source.Contains('_lastPrintedChar')) { throw 'R09 CSI REP ownership gate: native last-printed-character seam is missing.' }
if (-not $source.Contains('_dispatch->PrintString(repeated)')) { throw 'R09 CSI REP ownership gate: native PrintString materialization seam is missing.' }
if (-not $source.Contains('if (repPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE)')) { throw 'R09 CSI REP ownership gate: Rust result no longer short-circuits before the residual CSI switch.' }
if ($source.Contains('case CsiActionCodes::REP_RepeatCharacter:')) { throw 'R09 CSI REP ownership gate: duplicate C++ REP classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_rep_plan')) { throw 'R09 CSI REP ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('engine.action_csi_dispatch(id, &parameters)')) { throw 'R09 CSI REP ownership gate: REP classification no longer replays through the real Rust output engine.' }
if (-not $ffi.Contains('OutputAction::PrintString(text)')) { throw 'R09 CSI REP ownership gate: Rust replay no longer observes the owner PrintString action.' }
if (-not $ffi.Contains('plan("b", 0')) { throw 'R09 CSI REP ownership gate: Rust REP identifier/default witness is missing.' }
if (-not $ffi.Contains('OutputCsiRepKind::Repeat')) { throw 'R09 CSI REP ownership gate: Rust repeat classification is missing.' }
if (-not $ffi.Contains('OutputCsiRepKind::HandledNoop')) { throw 'R09 CSI REP ownership gate: Rust handled-noop classification is missing.' }
if (-not $ffi.Contains('FfiStatus::InvalidArgument')) { throw 'R09 CSI REP ownership gate: Rust invalid-argument contract is missing.' }

if (-not $probe.Contains('terminal_parser_ffi_output_csi_rep_plan')) { throw 'R09 CSI REP ownership gate: native replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_REPEAT')) { throw 'R09 CSI REP ownership gate: native positive replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_HANDLED_NOOP')) { throw 'R09 CSI REP ownership gate: native handled-noop replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE')) { throw 'R09 CSI REP ownership gate: native negative replay witness is missing.' }
if (-not $probe.Contains('invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI REP ownership gate: native invalid-identifier witness is missing.' }
if (-not $probe.Contains('nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI REP ownership gate: native null-output validation witness is missing.' }
if ($probe.Contains('R09OutputCsiDecpsAbiProbe.hpp') -or $probe.Contains('output_csi_decps_replay')) { throw 'R09 CSI REP ownership gate: REP replay is coupled to the DECPS witness.' }
if (-not $runner.Contains('#include "R09OutputCsiRepAbiProbe.hpp"')) { throw 'R09 CSI REP ownership gate: aggregate native replay no longer includes REP directly.' }
if (-not $runner.Contains('const bool outputCsiRepOk = r09::output_csi_rep_replay();')) { throw 'R09 CSI REP ownership gate: aggregate native replay no longer executes REP directly.' }
if (-not $runner.Contains('!outputCsiRepOk')) { throw 'R09 CSI REP ownership gate: aggregate native replay no longer fails closed on REP.' }

Write-Host 'R09 CSI REP ownership gate passed: Rust owns CSI b recognition, count normalization, and handled-noop semantics; C++ retains last-character storage and PrintString materialization.'
