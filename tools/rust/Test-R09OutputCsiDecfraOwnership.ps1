$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$ffiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_decfra.rs'
$probePath = Join-Path $repoRoot 'tools/rust/R09OutputCsiDecfraAbiProbe.hpp'

$source = Get-Content -Raw -LiteralPath $sourcePath
$ffi = Get-Content -Raw -LiteralPath $ffiPath
$probe = Get-Content -Raw -LiteralPath $probePath

function Get-FunctionBody([string] $Signature)
{
    $start = $source.IndexOf($Signature, [StringComparison]::Ordinal)
    if ($start -lt 0) { throw "R09 CSI DECFRA ownership gate: $Signature signature not found." }
    $openBrace = $source.IndexOf('{', $start)
    if ($openBrace -lt 0) { throw "R09 CSI DECFRA ownership gate: $Signature opening brace not found." }
    $depth = 0
    for ($i = $openBrace; $i -lt $source.Length; $i++)
    {
        switch ($source[$i])
        {
            '{' { $depth++ }
            '}' { $depth--; if ($depth -eq 0) { return $source.Substring($start, $i - $start + 1) } }
        }
    }
    throw "R09 CSI DECFRA ownership gate: $Signature closing brace not found."
}

$csiBody = Get-FunctionBody 'bool OutputStateMachineEngine::ActionCsiDispatch(const VTID id, const VTParameters parameters)'

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_decfra.h"')) { throw 'R09 CSI DECFRA ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $csiBody.Contains('terminal_parser_ffi_output_csi_decfra_values')) { throw 'R09 CSI DECFRA ownership gate: ActionCsiDispatch no longer delegates DECFRA classification and payload replay to Rust.' }
if (-not $csiBody.Contains('_dispatch->FillRectangularArea(')) { throw 'R09 CSI DECFRA ownership gate: native FillRectangularArea dispatch materialization is missing.' }
if (-not $csiBody.Contains('if (decfraMatched != 0)')) { throw 'R09 CSI DECFRA ownership gate: Rust match state no longer short-circuits before the residual CSI switch.' }
if (-not $csiBody.Contains('decfraOutputCount != decfraInputCount')) { throw 'R09 CSI DECFRA ownership gate: payload cardinality verification is missing.' }
if (-not $csiBody.Contains('decfraOutput[index] != decfraInput[index]')) { throw 'R09 CSI DECFRA ownership gate: payload identity verification is missing.' }
if ($csiBody.Contains('case CsiActionCodes::DECFRA_FillRectangularArea:')) { throw 'R09 CSI DECFRA ownership gate: duplicate C++ DECFRA classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_decfra_values')) { throw 'R09 CSI DECFRA ownership gate: terminal-parser-ffi no longer exports the replay seam.' }
if (-not $ffi.Contains('engine.action_csi_dispatch')) { throw 'R09 CSI DECFRA ownership gate: FFI no longer delegates classification to the Rust output engine.' }
if (-not $ffi.Contains('OutputAction::AdvancedCsi')) { throw 'R09 CSI DECFRA ownership gate: FFI no longer materializes the Rust advanced CSI action.' }
if (-not $ffi.Contains('id == VtId::from_ascii("$x")')) { throw 'R09 CSI DECFRA ownership gate: Rust DECFRA identifier classification is missing.' }
if (-not $ffi.Contains('replay("$x", &[65, 1, 2, 3, 4])')) { throw 'R09 CSI DECFRA ownership gate: complete flat payload Rust witness is missing.' }
if (-not $ffi.Contains('replay("$r", &[1, 2, 3, 4, 5])')) { throw 'R09 CSI DECFRA ownership gate: unrelated advanced CSI rejection witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL')) { throw 'R09 CSI DECFRA ownership gate: native sizing/capacity witness is missing.' }
if (-not $probe.Contains('output[0] != 65 || output[1] != 1 || output[2] != 2 || output[3] != 3 || output[4] != 4')) { throw 'R09 CSI DECFRA ownership gate: native complete payload witness is missing.' }
if (-not $probe.Contains('required != 0 || matched != 0')) { throw 'R09 CSI DECFRA ownership gate: native unrelated-CSI match-state witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI DECFRA ownership gate: native invalid-argument witness is missing.' }

Write-Host 'R09 CSI DECFRA ownership gate passed: Rust owns DECFRA classification and flat parameter replay; C++ retains only ABI adaptation, payload identity verification, and native FillRectangularArea dispatch materialization.'
