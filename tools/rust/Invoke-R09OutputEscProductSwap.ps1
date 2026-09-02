# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$sourcePath = Join-Path $repoRoot 'src\terminal\parser\OutputStateMachineEngine.cpp'
$source = [IO.File]::ReadAllText($sourcePath)
$newline = if ($source.Contains("`r`n")) { "`r`n" } else { "`n" }

$escHeader = '#include "terminal_parser_ffi_output_esc.h"'
if (-not $source.Contains($escHeader))
{
    $ffiHeader = '#include "terminal_parser_ffi.h"'
    $needle = $ffiHeader + $newline
    if (-not $source.Contains($needle))
    {
        throw 'R09 Output ESC swap: terminal_parser_ffi.h include anchor not found.'
    }
    $source = $source.Replace($needle, $needle + $escHeader + $newline)
}

$signature = 'bool OutputStateMachineEngine::ActionEscDispatch(const VTID id)'
$start = $source.IndexOf($signature, [StringComparison]::Ordinal)
if ($start -lt 0)
{
    throw 'R09 Output ESC swap: ActionEscDispatch signature not found.'
}

$openBrace = $source.IndexOf('{', $start)
if ($openBrace -lt 0)
{
    throw 'R09 Output ESC swap: ActionEscDispatch opening brace not found.'
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
    if ($end -ge 0) { break }
}
if ($end -lt 0)
{
    throw 'R09 Output ESC swap: ActionEscDispatch closing brace not found.'
}

$oldAction = $source.Substring($start, $end - $start + 1)
if (-not $oldAction.Contains('case EscActionCodes::') -or -not $oldAction.Contains('const auto commandChar = id[0];'))
{
    throw 'R09 Output ESC swap: expected legacy ESC classifier was not found; refusing a non-baseline transform.'
}

$newActionLines = @(
    'bool OutputStateMachineEngine::ActionEscDispatch(const VTID id)',
    '{',
    '    terminal_parser_ffi_output_esc_result plan{};',
    '    const auto status = terminal_parser_ffi_output_esc_plan(static_cast<uint64_t>(id), &plan);',
    '    THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_PARSER_FFI_OK);',
    '',
    '    switch (plan.kind)',
    '    {',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_BACK_INDEX:',
    '        _dispatch->BackIndex();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_CURSOR_SAVE_STATE:',
    '        _dispatch->CursorSaveState();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_CURSOR_RESTORE_STATE:',
    '        _dispatch->CursorRestoreState();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_FORWARD_INDEX:',
    '        _dispatch->ForwardIndex();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_SET_KEYPAD_MODE:',
    '        _dispatch->SetKeypadMode(plan.argument != 0);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_LINE_FEED_WITH_RETURN:',
    '        _dispatch->LineFeed(DispatchTypes::LineFeedType::WithReturn);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_LINE_FEED_WITHOUT_RETURN:',
    '        _dispatch->LineFeed(DispatchTypes::LineFeedType::WithoutReturn);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_REVERSE_LINE_FEED:',
    '        _dispatch->ReverseLineFeed();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_HORIZONTAL_TAB_SET:',
    '        _dispatch->HorizontalTabSet();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_DEVICE_ATTRIBUTES_PRIMARY:',
    '        _dispatch->DeviceAttributes();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_HARD_RESET:',
    '        _dispatch->HardReset(true);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_SINGLE_SHIFT:',
    '        _dispatch->SingleShift(plan.argument);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_LOCKING_SHIFT:',
    '        _dispatch->LockingShift(plan.argument);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_LOCKING_SHIFT_RIGHT:',
    '        _dispatch->LockingShiftRight(plan.argument);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_ACCEPT_C1_CONTROLS:',
    '        _dispatch->AcceptC1Controls(plan.argument != 0);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_SEND_C1_CONTROLS:',
    '        _dispatch->SendC1Controls(plan.argument != 0);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_ANNOUNCE_CODE_STRUCTURE:',
    '        _dispatch->AnnounceCodeStructure(plan.argument);',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_SET_LINE_RENDITION:',
    '        switch (plan.argument)',
    '        {',
    '        case TERMINAL_PARSER_FFI_OUTPUT_ESC_SINGLE_WIDTH:',
    '            _dispatch->SetLineRendition(LineRendition::SingleWidth);',
    '            break;',
    '        case TERMINAL_PARSER_FFI_OUTPUT_ESC_DOUBLE_WIDTH:',
    '            _dispatch->SetLineRendition(LineRendition::DoubleWidth);',
    '            break;',
    '        case TERMINAL_PARSER_FFI_OUTPUT_ESC_DOUBLE_HEIGHT_TOP:',
    '            _dispatch->SetLineRendition(LineRendition::DoubleHeightTop);',
    '            break;',
    '        case TERMINAL_PARSER_FFI_OUTPUT_ESC_DOUBLE_HEIGHT_BOTTOM:',
    '            _dispatch->SetLineRendition(LineRendition::DoubleHeightBottom);',
    '            break;',
    '        default:',
    '            THROW_HR(E_UNEXPECTED);',
    '        }',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_SCREEN_ALIGNMENT_PATTERN:',
    '        _dispatch->ScreenAlignmentPattern();',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_DESIGNATE_CODING_SYSTEM:',
    '        _dispatch->DesignateCodingSystem(VTID{ plan.payload });',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_DESIGNATE_94_CHARSET:',
    '        _dispatch->Designate94Charset(plan.argument, VTID{ plan.payload });',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_DESIGNATE_96_CHARSET:',
    '        _dispatch->Designate96Charset(plan.argument, VTID{ plan.payload });',
    '        break;',
    '    case TERMINAL_PARSER_FFI_OUTPUT_ESC_NONE:',
    '    default:',
    '        break;',
    '    }',
    '',
    '    _ClearLastChar();',
    '',
    '    return true;',
    '}'
)
$newAction = $newActionLines -join $newline
$source = $source.Remove($start, $oldAction.Length).Insert($start, $newAction)

$encoding = [Text.UTF8Encoding]::new($false)
[IO.File]::WriteAllText($sourcePath, $source, $encoding)

$updated = [IO.File]::ReadAllText($sourcePath)
$updatedStart = $updated.IndexOf($signature, [StringComparison]::Ordinal)
$updatedOpenBrace = $updated.IndexOf('{', $updatedStart)
$depth = 0
$updatedEnd = -1
for ($i = $updatedOpenBrace; $i -lt $updated.Length; $i++)
{
    switch ($updated[$i])
    {
        '{' { $depth++ }
        '}' {
            $depth--
            if ($depth -eq 0)
            {
                $updatedEnd = $i
                break
            }
        }
    }
    if ($updatedEnd -ge 0) { break }
}
$updatedAction = $updated.Substring($updatedStart, $updatedEnd - $updatedStart + 1)

if (-not $updatedAction.Contains('terminal_parser_ffi_output_esc_plan'))
{
    throw 'R09 Output ESC swap: product route to Rust was not installed.'
}
if ($updatedAction.Contains('case EscActionCodes::') -or $updatedAction.Contains('const auto commandChar = id[0];'))
{
    throw 'R09 Output ESC swap: portable legacy classifier survived the transform.'
}
if (-not $updatedAction.Contains('_ClearLastChar();'))
{
    throw 'R09 Output ESC swap: native last-character sequencing was lost.'
}

$normalizedUpdated = $updated.Replace("`r`n", "`n")
if ($normalizedUpdated -match '[ \t]+(?=\n)')
{
    throw 'R09 Output ESC swap: transformed source contains trailing horizontal whitespace.'
}

Push-Location $repoRoot
try
{
    $numstat = git diff --numstat -- 'src/terminal/parser/OutputStateMachineEngine.cpp'
    if ($LASTEXITCODE -ne 0 -or -not $numstat) { throw 'R09 Output ESC swap: expected source diff was not produced.' }
    $parts = $numstat -split '\s+'
    $added = [int]$parts[0]
    $deleted = [int]$parts[1]
    if ($added -gt 130 -or $deleted -gt 130)
    {
        throw "R09 Output ESC swap: diff is unexpectedly broad (+$added/-$deleted); refusing churn."
    }
    Write-Host "R09 Output ESC product swap candidate prepared (+$added/-$deleted) with source line endings preserved."
}
finally
{
    Pop-Location
}
