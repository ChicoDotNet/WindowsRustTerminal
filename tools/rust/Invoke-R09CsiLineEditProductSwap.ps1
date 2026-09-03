$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'c51b3f7cbdd5d95bd0a2b970d0a90d7756286e40'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI line edit source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_edit.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_line_edit.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_line_edit.h'))
{
    throw 'CSI line edit include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (editPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_EDIT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (editPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_EDIT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_line_edit_result lineEditPlan{};
    const auto lineEditStatus = terminal_parser_ffi_output_csi_line_edit_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &lineEditPlan);
    THROW_HR_IF(E_UNEXPECTED, lineEditStatus != TERMINAL_PARSER_FFI_OK);

    switch (lineEditPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_LINE_EDIT_INSERT_LINE:
        _dispatch->InsertLine(lineEditPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_LINE_EDIT_DELETE_LINE:
        _dispatch->DeleteLine(lineEditPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_LINE_EDIT_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (lineEditPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_LINE_EDIT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI line edit dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyInsert = @"
    case CsiActionCodes::IL_InsertLine:
        _dispatch->InsertLine(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"
$legacyDelete = @"
    case CsiActionCodes::DL_DeleteLine:
        _dispatch->DeleteLine(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyInsert) -or -not $text.Contains($legacyDelete))
{
    throw 'Legacy CSI line edit case marker mismatch.'
}
$text = $text.Replace($legacyInsert, '')
$text = $text.Replace($legacyDelete, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI line edit source diff.'
}

$diff = git diff -- $source
$addedLegacy = $diff -split "`n" | Where-Object { $_ -match '^\+' -and $_ -match 'CsiActionCodes::(IL_InsertLine|DL_DeleteLine)' }
if ($addedLegacy)
{
    throw 'Legacy CSI line edit cases were reintroduced in the candidate.'
}

if ($text.Contains('case CsiActionCodes::IL_InsertLine:') -or $text.Contains('case CsiActionCodes::DL_DeleteLine:'))
{
    throw 'Legacy CSI line edit cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_line_edit_plan'))
{
    throw 'CSI line edit Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI line editing Rust ownership candidate.'
