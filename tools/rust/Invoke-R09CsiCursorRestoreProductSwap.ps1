$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '129319c9b1000540f1194dcc61a8839b18bc18e6'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI cursor restore source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_device_attributes.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_cursor_restore.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_cursor_restore.h'))
{
    throw 'CSI cursor restore include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (deviceAttributesPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_ATTRIBUTES_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (deviceAttributesPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DEVICE_ATTRIBUTES_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_cursor_restore_result cursorRestorePlan{};
    const auto cursorRestoreStatus = terminal_parser_ffi_output_csi_cursor_restore_plan(
        static_cast<uint64_t>(id),
        &cursorRestorePlan);
    THROW_HR_IF(E_UNEXPECTED, cursorRestoreStatus != TERMINAL_PARSER_FFI_OK);

    switch (cursorRestorePlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_RESTORE_RESTORE:
        _dispatch->CursorRestoreState();
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_RESTORE_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (cursorRestorePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_RESTORE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI cursor restore dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCursorRestore = @"
    case CsiActionCodes::ANSISYSRC_CursorRestore:
        _dispatch->CursorRestoreState();
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyCursorRestore))
{
    throw 'Legacy CSI cursor restore case marker mismatch.'
}
$text = $text.Replace($legacyCursorRestore, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI cursor restore source diff.'
}

if ($text.Contains('case CsiActionCodes::ANSISYSRC_CursorRestore:'))
{
    throw 'Legacy CSI cursor restore case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_cursor_restore_plan'))
{
    throw 'CSI cursor restore Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI cursor restore Rust ownership candidate.'
