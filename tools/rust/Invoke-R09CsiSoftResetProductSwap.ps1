$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '85e66a2ceef608d703039e8ea8b2cc59da812b03'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI soft reset source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_cursor_restore.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_soft_reset.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_soft_reset.h'))
{
    throw 'CSI soft reset include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (cursorRestorePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_RESTORE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (cursorRestorePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_RESTORE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_soft_reset_result softResetPlan{};
    const auto softResetStatus = terminal_parser_ffi_output_csi_soft_reset_plan(
        static_cast<uint64_t>(id),
        &softResetPlan);
    THROW_HR_IF(E_UNEXPECTED, softResetStatus != TERMINAL_PARSER_FFI_OK);

    switch (softResetPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_SOFT_RESET_SOFT_RESET:
        _dispatch->SoftReset();
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_SOFT_RESET_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (softResetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_SOFT_RESET_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI soft reset dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacySoftReset = @"
    case CsiActionCodes::DECSTR_SoftReset:
        _dispatch->SoftReset();
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacySoftReset))
{
    throw 'Legacy CSI soft reset case marker mismatch.'
}
$text = $text.Replace($legacySoftReset, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI soft reset source diff.'
}

if ($text.Contains('case CsiActionCodes::DECSTR_SoftReset:'))
{
    throw 'Legacy CSI soft reset case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_soft_reset_plan'))
{
    throw 'CSI soft reset Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI soft reset Rust ownership candidate.'
