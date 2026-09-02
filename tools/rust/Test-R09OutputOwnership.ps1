$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$escFfiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_esc.rs'
$escProbePath = Join-Path $repoRoot 'tools/rust/R09OutputEscAbiProbe.hpp'
$vt52FfiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_vt52.rs'
$vt52ProbePath = Join-Path $repoRoot 'tools/rust/R09OutputVt52AbiProbe.hpp'
$source = Get-Content -Raw -LiteralPath $sourcePath
$escFfi = Get-Content -Raw -LiteralPath $escFfiPath
$escProbe = Get-Content -Raw -LiteralPath $escProbePath
$vt52Ffi = Get-Content -Raw -LiteralPath $vt52FfiPath
$vt52Probe = Get-Content -Raw -LiteralPath $vt52ProbePath

function Get-FunctionBody([string] $Signature)
{
    $start = $source.IndexOf($Signature, [StringComparison]::Ordinal)
    if ($start -lt 0)
    {
        throw "R09 Output ownership gate: $Signature signature not found."
    }

    $openBrace = $source.IndexOf('{', $start)
    if ($openBrace -lt 0)
    {
        throw "R09 Output ownership gate: $Signature opening brace not found."
    }

    $depth = 0
    for ($i = $openBrace; $i -lt $source.Length; $i++)
    {
        switch ($source[$i])
        {
            '{' { $depth++ }
            '}' {
                $depth--
                if ($depth -eq 0)
                {
                    return $source.Substring($start, $i - $start + 1)
                }
            }
        }
    }

    throw "R09 Output ownership gate: $Signature closing brace not found."
}

$executeBody = Get-FunctionBody 'bool OutputStateMachineEngine::ActionExecute(const wchar_t wch)'

if (-not $executeBody.Contains('terminal_parser_ffi_output_execute_plan'))
{
    throw 'R09 Output ownership gate: ActionExecute no longer delegates C0 classification to Rust.'
}

if ($executeBody.Contains('case AsciiChars::'))
{
    throw 'R09 Output ownership gate: portable C0 classification returned to C++.'
}

if (-not $executeBody.Contains('_ClearLastChar();'))
{
    throw 'R09 Output ownership gate: native C0 last-character sequencing was removed.'
}

$escBody = Get-FunctionBody 'bool OutputStateMachineEngine::ActionEscDispatch(const VTID id)'

if (-not $escBody.Contains('terminal_parser_ffi_output_esc_plan'))
{
    throw 'R09 Output ownership gate: ActionEscDispatch no longer delegates ESC classification to Rust.'
}

if ($escBody.Contains('switch (id)'))
{
    throw 'R09 Output ownership gate: portable ESC classification returned to C++.'
}

if (-not $escBody.Contains('switch (plan.kind)'))
{
    throw 'R09 Output ownership gate: native ESC dispatch materialization no longer consumes the Rust plan.'
}

if (-not $escBody.Contains('_ClearLastChar();'))
{
    throw 'R09 Output ownership gate: native ESC last-character sequencing was removed.'
}

if (-not $escFfi.Contains('terminal_parser_ffi_output_esc_plan'))
{
    throw 'R09 Output ownership gate: terminal-parser-ffi no longer exports the Output ESC planning seam.'
}

if (-not $escFfi.Contains('engine.action_esc_dispatch(id)'))
{
    throw 'R09 Output ownership gate: Output ESC FFI no longer delegates to the Rust output engine.'
}

if (-not $escProbe.Contains('terminal_parser_ffi_output_esc_plan'))
{
    throw 'R09 Output ownership gate: native replay no longer exercises the Output ESC planning seam.'
}

$vt52Body = Get-FunctionBody 'bool OutputStateMachineEngine::ActionVt52EscDispatch(const VTID id, const VTParameters parameters)'

if (-not $vt52Body.Contains('terminal_parser_ffi_output_vt52_plan'))
{
    throw 'R09 Output ownership gate: ActionVt52EscDispatch no longer delegates VT52 classification to Rust.'
}

if ($vt52Body.Contains('switch (id)') -or $vt52Body.Contains('case Vt52ActionCodes::'))
{
    throw 'R09 Output ownership gate: portable VT52 classification returned to C++.'
}

if (-not $vt52Body.Contains('switch (plan.kind)'))
{
    throw 'R09 Output ownership gate: native VT52 dispatch materialization no longer consumes the Rust plan.'
}

if (-not $vt52Body.Contains('_ClearLastChar();'))
{
    throw 'R09 Output ownership gate: native VT52 last-character sequencing was removed.'
}

if (-not $vt52Ffi.Contains('terminal_parser_ffi_output_vt52_plan'))
{
    throw 'R09 Output ownership gate: terminal-parser-ffi no longer exports the Output VT52 planning seam.'
}

if (-not $vt52Ffi.Contains('engine.action_vt52_esc_dispatch(id, &parameters)'))
{
    throw 'R09 Output ownership gate: Output VT52 FFI no longer delegates to the Rust output engine.'
}

if (-not $vt52Probe.Contains('terminal_parser_ffi_output_vt52_plan'))
{
    throw 'R09 Output ownership gate: native replay no longer exercises the Output VT52 planning seam.'
}

Write-Host 'R09 Output ownership gate passed: Rust owns C0, ESC, and VT52 classification; native dispatch sequencing remains at the Windows seam.'
