$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'd14f15ccd4237444dbbd4ba214b6619d5abc9304'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI erase characters source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_line_edit.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_erase_characters.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_erase_characters.h'))
{
    throw 'CSI erase characters include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (lineEditPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_LINE_EDIT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (lineEditPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_LINE_EDIT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_erase_characters_result eraseCharactersPlan{};
    const auto eraseCharactersStatus = terminal_parser_ffi_output_csi_erase_characters_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &eraseCharactersPlan);
    THROW_HR_IF(E_UNEXPECTED, eraseCharactersStatus != TERMINAL_PARSER_FFI_OK);

    switch (eraseCharactersPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_CHARACTERS_ERASE_CHARACTERS:
        _dispatch->EraseCharacters(eraseCharactersPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_CHARACTERS_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (eraseCharactersPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_CHARACTERS_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI erase characters dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyErase = @"
    case CsiActionCodes::ECH_EraseCharacters:
        _dispatch->EraseCharacters(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyErase))
{
    throw 'Legacy CSI erase characters case marker mismatch.'
}
$text = $text.Replace($legacyErase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI erase characters source diff.'
}

if ($text.Contains('case CsiActionCodes::ECH_EraseCharacters:'))
{
    throw 'Legacy CSI erase characters case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_erase_characters_plan'))
{
    throw 'CSI erase characters Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI erase characters Rust ownership candidate.'
