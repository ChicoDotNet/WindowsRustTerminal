$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$ffiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_decsca.rs'
$probePath = Join-Path $repoRoot 'tools/rust/R09OutputCsiDecscaAbiProbe.hpp'

$source = Get-Content -Raw -LiteralPath $sourcePath
$ffi = Get-Content -Raw -LiteralPath $ffiPath
$probe = Get-Content -Raw -LiteralPath $probePath

function Get-FunctionBody([string] $Signature)
{
    $start = $source.IndexOf($Signature, [StringComparison]::Ordinal)
    if ($start -lt 0) { throw "R09 CSI DECSCA ownership gate: $Signature signature not found." }
    $openBrace = $source.IndexOf('{', $start)
    if ($openBrace -lt 0) { throw "R09 CSI DECSCA ownership gate: $Signature opening brace not found." }
    $depth = 0
    for ($i = $openBrace; $i -lt $source.Length; $i++)
    {
        switch ($source[$i])
        {
            '{' { $depth++ }
            '}' { $depth--; if ($depth -eq 0) { return $source.Substring($start, $i - $start + 1) } }
        }
    }
    throw "R09 CSI DECSCA ownership gate: $Signature closing brace not found."
}

$csiBody = Get-FunctionBody 'bool OutputStateMachineEngine::ActionCsiDispatch(const VTID id, const VTParameters parameters)'

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_decsca.h"')) { throw 'R09 CSI DECSCA ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $csiBody.Contains('terminal_parser_ffi_output_csi_decsca_values')) { throw 'R09 CSI DECSCA ownership gate: ActionCsiDispatch no longer delegates DECSCA classification and payload replay to Rust.' }
if (-not $csiBody.Contains('_dispatch->SetCharacterProtectionAttribute(parameters);')) { throw 'R09 CSI DECSCA ownership gate: native SetCharacterProtectionAttribute dispatch materialization is missing.' }
if (-not $csiBody.Contains('if (decscaOutputCount != 0)')) { throw 'R09 CSI DECSCA ownership gate: Rust-owned result no longer short-circuits before the residual CSI switch.' }
if (-not $csiBody.Contains('decscaOutputCount != decscaInputCount')) { throw 'R09 CSI DECSCA ownership gate: payload cardinality verification is missing.' }
if (-not $csiBody.Contains('decscaOutput[index] != decscaInput[index]')) { throw 'R09 CSI DECSCA ownership gate: payload identity verification is missing.' }
if ($csiBody.Contains('case CsiActionCodes::DECSCA_SetCharacterProtectionAttribute:')) { throw 'R09 CSI DECSCA ownership gate: duplicate C++ DECSCA classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_decsca_values')) { throw 'R09 CSI DECSCA ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('engine.action_csi_dispatch')) { throw 'R09 CSI DECSCA ownership gate: FFI no longer delegates classification to the Rust output engine.' }
if (-not $ffi.Contains('OutputAction::SetCharacterProtectionAttribute')) { throw 'R09 CSI DECSCA ownership gate: FFI no longer materializes the Rust DECSCA action.' }
if (-not $ffi.Contains('assert_eq!(replay("\"q", &[0]), [0]);')) { throw 'R09 CSI DECSCA ownership gate: single-value Rust contract witness is missing.' }
if (-not $ffi.Contains('assert_eq!(replay("\"q", &[1, 2]), [1, 2]);')) { throw 'R09 CSI DECSCA ownership gate: multi-value Rust contract witness is missing.' }
if (-not $ffi.Contains('assert!(replay("m", &[1]).is_empty());')) { throw 'R09 CSI DECSCA ownership gate: unrelated CSI rejection witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL')) { throw 'R09 CSI DECSCA ownership gate: native sizing/capacity witness is missing.' }
if (-not $probe.Contains('multipleOutput[0] != 1 || multipleOutput[1] != 2')) { throw 'R09 CSI DECSCA ownership gate: native multi-value payload witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECSCA ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI DECSCA ownership gate passed: Rust owns DECSCA classification and flat parameter replay; C++ retains only ABI adaptation, payload identity verification, and native SetCharacterProtectionAttribute dispatch materialization.'
