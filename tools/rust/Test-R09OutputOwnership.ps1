$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$escFfiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_esc.rs'
$escProbePath = Join-Path $repoRoot 'tools/rust/R09OutputEscAbiProbe.hpp'
$source = Get-Content -Raw -LiteralPath $sourcePath
$escFfi = Get-Content -Raw -LiteralPath $escFfiPath
$escProbe = Get-Content -Raw -LiteralPath $escProbePath

$signature = 'bool OutputStateMachineEngine::ActionExecute(const wchar_t wch)'
$start = $source.IndexOf($signature, [StringComparison]::Ordinal)
if ($start -lt 0)
{
    throw 'R09 Output ownership gate: ActionExecute signature not found.'
}

$openBrace = $source.IndexOf('{', $start)
if ($openBrace -lt 0)
{
    throw 'R09 Output ownership gate: ActionExecute opening brace not found.'
}

$depth = 0
$end = -1
for ($i = $openBrace; $i -lt $source.Length; $i++)
{
    switch ($source[$i])
    {
        '{' { $depth++ }
        '}' {
            $depth--
            if ($depth -eq 0)
            {
                $end = $i
                break
            }
        }
    }

    if ($end -ge 0)
    {
        break
    }
}

if ($end -lt 0)
{
    throw 'R09 Output ownership gate: ActionExecute closing brace not found.'
}

$body = $source.Substring($start, $end - $start + 1)

if (-not $body.Contains('terminal_parser_ffi_output_execute_plan'))
{
    throw 'R09 Output ownership gate: ActionExecute no longer delegates C0 classification to Rust.'
}

if ($body.Contains('case AsciiChars::'))
{
    throw 'R09 Output ownership gate: portable C0 classification returned to C++.'
}

if (-not $body.Contains('_ClearLastChar();'))
{
    throw 'R09 Output ownership gate: native last-character sequencing was removed.'
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

Write-Host 'R09 Output ownership gate passed: Rust owns C0 classification and the Output ESC bridge remains replay-certified.'
