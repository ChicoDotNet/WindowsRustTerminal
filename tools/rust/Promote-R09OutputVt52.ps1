$ErrorActionPreference = 'Stop'

$path = Join-Path (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)) 'src/terminal/parser/OutputStateMachineEngine.cpp'
$bytes = [IO.File]::ReadAllBytes($path)
$text = [Text.Encoding]::UTF8.GetString($bytes)

if ($text.Contains('terminal_parser_ffi_output_vt52_plan'))
{
    Write-Host 'Output VT52 product ownership is already promoted.'
    return
}

if (-not $text.Contains("`r`n"))
{
    throw 'Expected CRLF source before VT52 promotion.'
}

$includeOld = "#include `"terminal_parser_ffi_output_esc.h`"`r`n"
$includeNew = $includeOld + "#include `"terminal_parser_ffi_output_vt52.h`"`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_vt52.h'))
{
    throw 'Unexpected VT52 include state.'
}
$text = $text.Replace($includeOld, $includeNew)

$signature = 'bool OutputStateMachineEngine::ActionVt52EscDispatch(const VTID id, const VTParameters parameters)'
$start = $text.IndexOf($signature, [StringComparison]::Ordinal)
if ($start -lt 0)
{
    throw 'VT52 consumer signature not found.'
}

$nextMarker = "// Routine Description:`r`n// - Triggers the CsiDispatch"
$next = $text.IndexOf($nextMarker, $start, [StringComparison]::Ordinal)
if ($next -lt 0)
{
    throw 'VT52 consumer end marker not found.'
}

$oldFunction = $text.Substring($start, $next - $start)
if (-not $oldFunction.Contains('switch (id)') -or -not $oldFunction.Contains('case Vt52ActionCodes::DirectCursorAddress:'))
{
    throw 'VT52 portable decision tree no longer matches the certified baseline.'
}

$newLines = @(
    'bool OutputStateMachineEngine::ActionVt52EscDispatch(const VTID id, const VTParameters parameters)',
    '{',
    '    terminal_parser_ffi_output_vt52_result plan{};',
    '    const auto status = terminal_parser_ffi_output_vt52_plan(',
    '        static_cast<uint64_t>(id),',
    '        static_cast<int32_t>(parameters.at(0).value_or(0)),',
    '        static_cast<int32_t>(parameters.at(1).value_or(0)),',
    '        &plan);',
    '    THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_PARSER_FFI_OK);',
    '',
    '    switch (plan.kind)',
    '    {',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_CURSOR_UP:',
    '        _dispatch->CursorUp(plan.argument1);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_CURSOR_DOWN:',
    '        _dispatch->CursorDown(plan.argument1);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_CURSOR_FORWARD:',
    '        _dispatch->CursorForward(plan.argument1);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_CURSOR_BACKWARD:',
    '        _dispatch->CursorBackward(plan.argument1);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_DESIGNATE_94_CHARSET:',
    '        _dispatch->Designate94Charset(plan.argument1, VTID{ plan.payload });',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_CURSOR_POSITION:',
    '        _dispatch->CursorPosition(plan.argument1, plan.argument2);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_REVERSE_LINE_FEED:',
    '        _dispatch->ReverseLineFeed();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_ERASE_IN_DISPLAY:',
    '        _dispatch->EraseInDisplay(DispatchTypes::EraseType::ToEnd);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_ERASE_IN_LINE:',
    '        _dispatch->EraseInLine(DispatchTypes::EraseType::ToEnd);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_DEVICE_ATTRIBUTES:',
    '        _dispatch->Vt52DeviceAttributes();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_SET_KEYPAD_MODE:',
    '        _dispatch->SetKeypadMode(plan.argument1 != 0);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_SET_ANSI_MODE:',
    '        _dispatch->SetMode(DispatchTypes::ModeParams::DECANM_AnsiMode);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_VT52_NONE:',
    '    default:',
    '        break;',
    '    }',
    '',
    '    _ClearLastChar();',
    '',
    '    return true;',
    '}',
    '',
    ''
)
foreach ($line in $newLines)
{
    if ($line -match '[ \t]+$')
    {
        throw "Generated VT52 line contains trailing whitespace: '$line'"
    }
}
$newFunction = $newLines -join "`r`n"
$text = $text.Substring(0, $start) + $newFunction + $text.Substring($next)
[IO.File]::WriteAllText($path, $text, [Text.UTF8Encoding]::new($false))

$patched = [IO.File]::ReadAllBytes($path)
for ($i = 0; $i -lt $patched.Length; $i++)
{
    if ($patched[$i] -eq 10 -and ($i -eq 0 -or $patched[$i - 1] -ne 13))
    {
        throw "Bare LF introduced at byte $i."
    }
}

$patchedText = [Text.Encoding]::UTF8.GetString($patched)
$patchedStart = $patchedText.IndexOf($signature, [StringComparison]::Ordinal)
$patchedNext = $patchedText.IndexOf($nextMarker, $patchedStart, [StringComparison]::Ordinal)
$patchedFunction = $patchedText.Substring($patchedStart, $patchedNext - $patchedStart)
if (-not $patchedText.Contains('terminal_parser_ffi_output_vt52.h') -or
    -not $patchedFunction.Contains('terminal_parser_ffi_output_vt52_plan') -or
    -not $patchedFunction.Contains('switch (plan.kind)') -or
    $patchedFunction.Contains('switch (id)') -or
    $patchedFunction.Contains('case Vt52ActionCodes::') -or
    -not $patchedFunction.Contains('_ClearLastChar();'))
{
    throw 'VT52 postconditions failed after mechanical promotion.'
}

$diffNames = @(git diff --name-only)
if ($diffNames.Count -ne 1 -or $diffNames[0] -ne 'src/terminal/parser/OutputStateMachineEngine.cpp')
{
    throw "Unexpected files changed: $($diffNames -join ', ')"
}

$numstat = git diff --numstat -- src/terminal/parser/OutputStateMachineEngine.cpp
if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($numstat))
{
    throw 'Unable to measure VT52 promotion diff.'
}
$parts = $numstat -split "`t"
if ($parts.Count -lt 2 -or $parts[0] -notmatch '^\d+$' -or $parts[1] -notmatch '^\d+$')
{
    throw "Unexpected VT52 numstat: $numstat"
}
$changedLines = [int]$parts[0] + [int]$parts[1]
if ($changedLines -gt 160)
{
    throw "VT52 promotion diff is unexpectedly large: $changedLines changed lines."
}

git diff --stat
