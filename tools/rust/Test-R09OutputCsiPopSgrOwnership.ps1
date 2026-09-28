$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$ffiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_pop_sgr.rs'
$probePath = Join-Path $repoRoot 'tools/rust/R09OutputCsiPopSgrAbiProbe.hpp'

$source = Get-Content -Raw -LiteralPath $sourcePath
$ffi = Get-Content -Raw -LiteralPath $ffiPath
$probe = Get-Content -Raw -LiteralPath $probePath

function Get-FunctionBody([string] $Signature)
{
    $start = $source.IndexOf($Signature, [StringComparison]::Ordinal)
    if ($start -lt 0) { throw "R09 CSI pop SGR ownership gate: $Signature signature not found." }
    $openBrace = $source.IndexOf('{', $start)
    if ($openBrace -lt 0) { throw "R09 CSI pop SGR ownership gate: $Signature opening brace not found." }
    $depth = 0
    for ($i = $openBrace; $i -lt $source.Length; $i++)
    {
        switch ($source[$i])
        {
            '{' { $depth++ }
            '}' { $depth--; if ($depth -eq 0) { return $source.Substring($start, $i - $start + 1) } }
        }
    }
    throw "R09 CSI pop SGR ownership gate: $Signature closing brace not found."
}

$csiBody = Get-FunctionBody 'bool OutputStateMachineEngine::ActionCsiDispatch(const VTID id, const VTParameters parameters)'

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_pop_sgr.h"')) { throw 'R09 CSI pop SGR ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $csiBody.Contains('terminal_parser_ffi_output_csi_pop_sgr_plan')) { throw 'R09 CSI pop SGR ownership gate: ActionCsiDispatch no longer delegates pop SGR classification to Rust.' }
if (-not $csiBody.Contains('_dispatch->PopGraphicsRendition();')) { throw 'R09 CSI pop SGR ownership gate: native PopGraphicsRendition dispatch materialization is missing.' }
if (-not $csiBody.Contains('if (popSgrPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_NONE)')) { throw 'R09 CSI pop SGR ownership gate: Rust-owned plan no longer short-circuits before the residual CSI switch.' }
if ($csiBody.Contains('case CsiActionCodes::XT_PopSgr:')) { throw 'R09 CSI pop SGR ownership gate: duplicate C++ XT_PopSgr classification returned.' }
if ($csiBody.Contains('case CsiActionCodes::XT_PopSgrAlias:')) { throw 'R09 CSI pop SGR ownership gate: duplicate C++ XT_PopSgrAlias classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_pop_sgr_plan')) { throw 'R09 CSI pop SGR ownership gate: terminal-parser-ffi no longer exports the planning seam.' }
if (-not $ffi.Contains('engine.action_csi_dispatch')) { throw 'R09 CSI pop SGR ownership gate: FFI no longer delegates classification to the Rust output engine.' }
if (-not $ffi.Contains('OutputAction::PopGraphicsRendition')) { throw 'R09 CSI pop SGR ownership gate: FFI no longer materializes the Rust PopGraphicsRendition action.' }
if (-not $ffi.Contains('expect("#}", OutputCsiPopSgrKind::Pop);')) { throw 'R09 CSI pop SGR ownership gate: primary Rust contract witness is missing.' }
if (-not $ffi.Contains('expect("#q", OutputCsiPopSgrKind::Pop);')) { throw 'R09 CSI pop SGR ownership gate: alias Rust contract witness is missing.' }
if (-not $probe.Contains("for (const char suffix : { '}', 'q' })")) { throw 'R09 CSI pop SGR ownership gate: native primary/alias replay witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_NONE')) { throw 'R09 CSI pop SGR ownership gate: unrelated CSI rejection witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI pop SGR ownership gate: null-pointer validation witness is missing.' }

Write-Host 'R09 CSI pop SGR ownership gate passed: Rust owns CSI SGR-stack pop classification including the Microsoft alias; C++ retains only ABI invocation and native PopGraphicsRendition dispatch materialization.'
