$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$ffiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_push_sgr.rs'
$probePath = Join-Path $repoRoot 'tools/rust/R09OutputCsiPushSgrAbiProbe.hpp'

$source = Get-Content -Raw -LiteralPath $sourcePath
$ffi = Get-Content -Raw -LiteralPath $ffiPath
$probe = Get-Content -Raw -LiteralPath $probePath

function Get-FunctionBody([string] $Signature)
{
    $start = $source.IndexOf($Signature, [StringComparison]::Ordinal)
    if ($start -lt 0) { throw "R09 CSI push SGR ownership gate: $Signature signature not found." }
    $openBrace = $source.IndexOf('{', $start)
    if ($openBrace -lt 0) { throw "R09 CSI push SGR ownership gate: $Signature opening brace not found." }
    $depth = 0
    for ($i = $openBrace; $i -lt $source.Length; $i++)
    {
        switch ($source[$i])
        {
            '{' { $depth++ }
            '}' { $depth--; if ($depth -eq 0) { return $source.Substring($start, $i - $start + 1) } }
        }
    }
    throw "R09 CSI push SGR ownership gate: $Signature closing brace not found."
}

$csiBody = Get-FunctionBody 'bool OutputStateMachineEngine::ActionCsiDispatch(const VTID id, const VTParameters parameters)'

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_push_sgr.h"')) { throw 'R09 CSI push SGR ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $csiBody.Contains('terminal_parser_ffi_output_csi_push_sgr_values')) { throw 'R09 CSI push SGR ownership gate: ActionCsiDispatch no longer delegates push-SGR classification and payload replay to Rust.' }
if (-not $csiBody.Contains('_dispatch->PushGraphicsRendition(parameters);')) { throw 'R09 CSI push SGR ownership gate: native PushGraphicsRendition dispatch materialization is missing.' }
if (-not $csiBody.Contains('if (pushSgrMatched != 0)')) { throw 'R09 CSI push SGR ownership gate: Rust match state no longer short-circuits before the residual CSI switch.' }
if (-not $csiBody.Contains('pushSgrOutputCount != pushSgrInputCount')) { throw 'R09 CSI push SGR ownership gate: payload cardinality verification is missing.' }
if (-not $csiBody.Contains('pushSgrOutput[index] != pushSgrInput[index]')) { throw 'R09 CSI push SGR ownership gate: payload identity verification is missing.' }
if ($csiBody.Contains('case CsiActionCodes::XT_PushSgr:')) { throw 'R09 CSI push SGR ownership gate: duplicate C++ primary push-SGR classification returned.' }
if ($csiBody.Contains('case CsiActionCodes::XT_PushSgrAlias:')) { throw 'R09 CSI push SGR ownership gate: duplicate C++ push-SGR alias classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_push_sgr_values')) { throw 'R09 CSI push SGR ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('engine.action_csi_dispatch')) { throw 'R09 CSI push SGR ownership gate: FFI no longer delegates classification to the Rust output engine.' }
if (-not $ffi.Contains('OutputAction::PushGraphicsRendition')) { throw 'R09 CSI push SGR ownership gate: FFI no longer materializes the Rust push-SGR action.' }
if (-not $ffi.Contains('assert_eq!(replay("#{", &[1]), (true, vec![1]));')) { throw 'R09 CSI push SGR ownership gate: primary Rust contract witness is missing.' }
if (-not $ffi.Contains('assert_eq!(replay("#p", &[1, 2]), (true, vec![1, 2]));')) { throw 'R09 CSI push SGR ownership gate: alias Rust contract witness is missing.' }
if (-not $ffi.Contains('assert_eq!(replay("#{", &[]), (true, vec![0]));')) { throw 'R09 CSI push SGR ownership gate: implicit default-parameter Rust witness is missing.' }
if (-not $ffi.Contains('assert_eq!(replay("m", &[1]), (false, Vec::new()));')) { throw 'R09 CSI push SGR ownership gate: unrelated CSI rejection witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL')) { throw 'R09 CSI push SGR ownership gate: native sizing/capacity witness is missing.' }
if (-not $probe.Contains('multipleOutput[0] != 1 || multipleOutput[1] != 2')) { throw 'R09 CSI push SGR ownership gate: native alias multi-value payload witness is missing.' }
if (-not $probe.Contains('defaultOutput != 0')) { throw 'R09 CSI push SGR ownership gate: native implicit default-parameter witness is missing.' }
if (-not $probe.Contains('required != 0 || matched != 0')) { throw 'R09 CSI push SGR ownership gate: native unrelated-CSI match-state witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI push SGR ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI push SGR ownership gate passed: Rust owns primary/alias push-SGR classification and flat parameter replay; C++ retains only ABI adaptation, payload identity verification, and native PushGraphicsRendition dispatch materialization.'
