$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '20f30fc8181dd665efebde8f2485198a1c1848bb'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI edit source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_margins.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_edit.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_edit.h'))
{
    throw 'CSI edit include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$old = @"
    switch (id)
    {
    case CsiActionCodes::ICH_InsertCharacter:
        _dispatch->InsertCharacter(parameters.at(0));
        break;
    case CsiActionCodes::DCH_DeleteCharacter:
        _dispatch->DeleteCharacter(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

$new = @"
    terminal_parser_ffi_output_csi_edit_result editPlan{};
    const auto editStatus = terminal_parser_ffi_output_csi_edit_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &editPlan);
    THROW_HR_IF(E_UNEXPECTED, editStatus != TERMINAL_PARSER_FFI_OK);

    switch (editPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_EDIT_INSERT_CHARACTER:
        _dispatch->InsertCharacter(editPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_EDIT_DELETE_CHARACTER:
        _dispatch->DeleteCharacter(editPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_EDIT_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (editPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_EDIT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
    {
"@ -replace "`n", "`r`n"

if (-not $text.Contains($old))
{
    throw 'CSI edit dispatch block marker mismatch.'
}
$text = $text.Replace($old, $new)

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI edit source diff.'
}

$diff = git diff -- $source
$addedLegacy = $diff -split "`n" | Where-Object { $_ -match '^\+' -and $_ -match 'CsiActionCodes::(ICH_InsertCharacter|DCH_DeleteCharacter)' }
if ($addedLegacy)
{
    throw 'Legacy CSI edit cases were reintroduced in the candidate.'
}

if ($text.Contains('case CsiActionCodes::ICH_InsertCharacter:') -or $text.Contains('case CsiActionCodes::DCH_DeleteCharacter:'))
{
    throw 'Legacy CSI edit cases remain after candidate rewrite.'
}

Write-Host 'Prepared CRLF-safe CSI character editing Rust ownership candidate.'
