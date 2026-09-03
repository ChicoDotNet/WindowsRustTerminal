$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'fbc3f93c35604475971ba9a8526abc5defacbb7f'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI scrolling source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_erase_characters.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_scroll.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_scroll.h'))
{
    throw 'CSI scrolling include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (eraseCharactersPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_CHARACTERS_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (eraseCharactersPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_CHARACTERS_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_scroll_result scrollPlan{};
    const auto scrollStatus = terminal_parser_ffi_output_csi_scroll_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &scrollPlan);
    THROW_HR_IF(E_UNEXPECTED, scrollStatus != TERMINAL_PARSER_FFI_OK);

    switch (scrollPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_SCROLL_UP:
        _dispatch->ScrollUp(scrollPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_SCROLL_DOWN:
        _dispatch->ScrollDown(scrollPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_SCROLL_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (scrollPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_SCROLL_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI scrolling dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyScroll = @"
    case CsiActionCodes::SU_ScrollUp:
        _dispatch->ScrollUp(parameters.at(0));
        break;
    case CsiActionCodes::SD_ScrollDown:
        _dispatch->ScrollDown(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyScroll))
{
    throw 'Legacy CSI scrolling case marker mismatch.'
}
$text = $text.Replace($legacyScroll, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI scrolling source diff.'
}

if ($text.Contains('case CsiActionCodes::SU_ScrollUp:') -or $text.Contains('case CsiActionCodes::SD_ScrollDown:'))
{
    throw 'Legacy CSI scrolling cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_scroll_plan'))
{
    throw 'CSI scrolling Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI scrolling Rust ownership candidate.'
