$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$ffiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_sgr.rs'
$probePath = Join-Path $repoRoot 'tools/rust/R09OutputCsiSgrAbiProbe.hpp'

$source = Get-Content -Raw -LiteralPath $sourcePath
$ffi = Get-Content -Raw -LiteralPath $ffiPath
$probe = Get-Content -Raw -LiteralPath $probePath

function Get-FunctionBody([string] $Signature)
{
    $start = $source.IndexOf($Signature, [StringComparison]::Ordinal)
    if ($start -lt 0) { throw "R09 CSI SGR ownership gate: $Signature signature not found." }
    $openBrace = $source.IndexOf('{', $start)
    if ($openBrace -lt 0) { throw "R09 CSI SGR ownership gate: $Signature opening brace not found." }
    $depth = 0
    for ($i = $openBrace; $i -lt $source.Length; $i++)
    {
        switch ($source[$i])
        {
            '{' { $depth++ }
            '}' { $depth--; if ($depth -eq 0) { return $source.Substring($start, $i - $start + 1) } }
        }
    }
    throw "R09 CSI SGR ownership gate: $Signature closing brace not found."
}

$csiBody = Get-FunctionBody 'bool OutputStateMachineEngine::ActionCsiDispatch(const VTID id, const VTParameters parameters)'

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_sgr.h"')) { throw 'R09 CSI SGR ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $csiBody.Contains('terminal_parser_ffi_output_csi_sgr_values')) { throw 'R09 CSI SGR ownership gate: ActionCsiDispatch no longer delegates SGR classification and payload replay to Rust.' }
if (-not $csiBody.Contains('_dispatch->SetGraphicsRendition(parameters);')) { throw 'R09 CSI SGR ownership gate: native SetGraphicsRendition dispatch materialization is missing.' }
if (-not $csiBody.Contains('if (sgrMatched != 0)')) { throw 'R09 CSI SGR ownership gate: Rust match state no longer short-circuits before the residual CSI switch.' }
if (-not $csiBody.Contains('sgrOutputCount != sgrInputCount')) { throw 'R09 CSI SGR ownership gate: payload cardinality verification is missing.' }
if (-not $csiBody.Contains('sgrOutput[index] != sgrInput[index]')) { throw 'R09 CSI SGR ownership gate: payload identity verification is missing.' }
if ($csiBody.Contains('case CsiActionCodes::SGR_SetGraphicsRendition:')) { throw 'R09 CSI SGR ownership gate: duplicate C++ SGR classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_sgr_values')) { throw 'R09 CSI SGR ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('engine.action_csi_dispatch')) { throw 'R09 CSI SGR ownership gate: FFI no longer delegates classification to the Rust output engine.' }
if (-not $ffi.Contains('OutputAction::SetGraphicsRendition')) { throw 'R09 CSI SGR ownership gate: FFI no longer materializes the Rust SGR action.' }
if (-not $ffi.Contains('assert_eq!(replay("m", &[]), (true, vec![0]));')) { throw 'R09 CSI SGR ownership gate: implicit default-parameter Rust witness is missing.' }
if (-not $ffi.Contains('assert_eq!(replay("m", &[1]), (true, vec![1]));')) { throw 'R09 CSI SGR ownership gate: single-parameter Rust witness is missing.' }
if (-not $ffi.Contains('assert_eq!(replay("m", &[1, 31, 44]), (true, vec![1, 31, 44]));')) { throw 'R09 CSI SGR ownership gate: multiple-parameter Rust witness is missing.' }
if (-not $ffi.Contains('assert_eq!(replay("#{", &[1]), (false, Vec::new()));')) { throw 'R09 CSI SGR ownership gate: unrelated CSI rejection witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL')) { throw 'R09 CSI SGR ownership gate: native sizing/capacity witness is missing.' }
if (-not $probe.Contains('defaultOutput != 0')) { throw 'R09 CSI SGR ownership gate: native implicit default-parameter witness is missing.' }
if (-not $probe.Contains('singleOutput != 1')) { throw 'R09 CSI SGR ownership gate: native single-parameter witness is missing.' }
if (-not $probe.Contains('multipleOutput[0] != 1 || multipleOutput[1] != 31 || multipleOutput[2] != 44')) { throw 'R09 CSI SGR ownership gate: native multiple-parameter payload witness is missing.' }
if (-not $probe.Contains('required != 0 || matched != 0')) { throw 'R09 CSI SGR ownership gate: native unrelated-CSI match-state witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI SGR ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI SGR ownership gate passed: Rust owns SGR classification and flat parameter replay; C++ retains only ABI adaptation, payload identity verification, and native SetGraphicsRendition dispatch materialization.'
