$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'f082cf9bd2d59f3dda7723c0161f099ca8d01aae'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI cursor source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_vt52.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_cursor.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_cursor.h'))
{
    throw 'CSI cursor include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$old = @"
    switch (id)
    {
    case CsiActionCodes::CUU_CursorUp:
        _dispatch->CursorUp(parameters.at(0));
        break;
    case CsiActionCodes::CUD_CursorDown:
        _dispatch->CursorDown(parameters.at(0));
        break;
    case CsiActionCodes::CUF_CursorForward:
        _dispatch->CursorForward(parameters.at(0));
        break;
    case CsiActionCodes::CUB_CursorBackward:
        _dispatch->CursorBackward(parameters.at(0));
        break;
    case CsiActionCodes::CNL_CursorNextLine:
        _dispatch->CursorNextLine(parameters.at(0));
        break;
    case CsiActionCodes::CPL_CursorPrevLine:
        _dispatch->CursorPrevLine(parameters.at(0));
        break;
    case CsiActionCodes::CHA_CursorHorizontalAbsolute:
    case CsiActionCodes::HPA_HorizontalPositionAbsolute:
        _dispatch->CursorHorizontalPositionAbsolute(parameters.at(0));
        break;
    case CsiActionCodes::VPA_VerticalLinePositionAbsolute:
        _dispatch->VerticalLinePositionAbsolute(parameters.at(0));
        break;
    case CsiActionCodes::HPR_HorizontalPositionRelative:
        _dispatch->HorizontalPositionRelative(parameters.at(0));
        break;
    case CsiActionCodes::VPR_VerticalPositionRelative:
        _dispatch->VerticalPositionRelative(parameters.at(0));
        break;
    case CsiActionCodes::CUP_CursorPosition:
    case CsiActionCodes::HVP_HorizontalVerticalPosition:
        _dispatch->CursorPosition(parameters.at(0), parameters.at(1));
        break;
"@ -replace "`n", "`r`n"

$new = @"
    terminal_parser_ffi_output_csi_cursor_result cursorPlan{};
    const auto cursorStatus = terminal_parser_ffi_output_csi_cursor_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        static_cast<int32_t>(parameters.at(1).value_or(0)),
        &cursorPlan);
    THROW_HR_IF(E_UNEXPECTED, cursorStatus != TERMINAL_PARSER_FFI_OK);

    switch (cursorPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_UP:
        _dispatch->CursorUp(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_DOWN:
        _dispatch->CursorDown(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_FORWARD:
        _dispatch->CursorForward(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_BACKWARD:
        _dispatch->CursorBackward(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_NEXT_LINE:
        _dispatch->CursorNextLine(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_PREVIOUS_LINE:
        _dispatch->CursorPrevLine(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_HORIZONTAL_ABSOLUTE:
        _dispatch->CursorHorizontalPositionAbsolute(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_VERTICAL_ABSOLUTE:
        _dispatch->VerticalLinePositionAbsolute(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_HORIZONTAL_RELATIVE:
        _dispatch->HorizontalPositionRelative(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_VERTICAL_RELATIVE:
        _dispatch->VerticalPositionRelative(cursorPlan.argument1);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_POSITION:
        _dispatch->CursorPosition(cursorPlan.argument1, cursorPlan.argument2);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (cursorPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
    {
"@ -replace "`n", "`r`n"

if (-not $text.Contains($old))
{
    throw 'CSI cursor dispatch block marker mismatch.'
}
$text = $text.Replace($old, $new)

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI cursor source diff.'
}

$diff = git diff -- $source
if ($diff -match 'CsiActionCodes::CUU_CursorUp|CsiActionCodes::CUD_CursorDown|CsiActionCodes::CUF_CursorForward|CsiActionCodes::CUB_CursorBackward|CsiActionCodes::CNL_CursorNextLine|CsiActionCodes::CPL_CursorPrevLine|CsiActionCodes::CHA_CursorHorizontalAbsolute|CsiActionCodes::HPA_HorizontalPositionAbsolute|CsiActionCodes::VPA_VerticalLinePositionAbsolute|CsiActionCodes::HPR_HorizontalPositionRelative|CsiActionCodes::VPR_VerticalPositionRelative|CsiActionCodes::CUP_CursorPosition|CsiActionCodes::HVP_HorizontalVerticalPosition')
{
    # These names should only appear as removed lines in the diff.
    $addedLegacy = $diff -split "`n" | Where-Object { $_ -match '^\+' -and $_ -match 'CsiActionCodes::(CUU|CUD|CUF|CUB|CNL|CPL|CHA|HPA|VPA|HPR|VPR|CUP|HVP)' }
    if ($addedLegacy)
    {
        throw 'Legacy CSI cursor cases were reintroduced in the candidate.'
    }
}

Write-Host 'Prepared CRLF-safe CSI cursor Rust ownership candidate.'
