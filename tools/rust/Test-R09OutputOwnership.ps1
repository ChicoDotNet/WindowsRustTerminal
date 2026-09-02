$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$source = Get-Content -Raw -LiteralPath $sourcePath

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

Write-Host 'R09 Output ownership gate passed: Rust owns C0 classification; C++ retains native dispatch execution.'
