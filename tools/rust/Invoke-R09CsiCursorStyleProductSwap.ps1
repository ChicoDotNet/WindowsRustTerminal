$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '57c3c10b9ab2d853f19e5ff6c733df550f7d1082'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI cursor style source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_displayed_extent.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_cursor_style.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_cursor_style.h'))
{
    throw 'CSI cursor style include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (displayedExtentPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DISPLAYED_EXTENT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (displayedExtentPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DISPLAYED_EXTENT_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_cursor_style_result cursorStylePlan{};
    const auto cursorStyleStatus = terminal_parser_ffi_output_csi_cursor_style_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &cursorStylePlan);
    THROW_HR_IF(E_UNEXPECTED, cursorStyleStatus != TERMINAL_PARSER_FFI_OK);

    switch (cursorStylePlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_STYLE_SET_CURSOR_STYLE:
        _dispatch->SetCursorStyle(static_cast<DispatchTypes::CursorStyle>(cursorStylePlan.style));
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_STYLE_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (cursorStylePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_STYLE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI cursor style dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCursorStyle = @"
    case CsiActionCodes::DECSCUSR_SetCursorStyle:
        _dispatch->SetCursorStyle(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyCursorStyle))
{
    throw 'Legacy CSI cursor style case marker mismatch.'
}
$text = $text.Replace($legacyCursorStyle, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI cursor style source diff.'
}

if ($text.Contains('case CsiActionCodes::DECSCUSR_SetCursorStyle:'))
{
    throw 'Legacy CSI cursor style case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_cursor_style_plan'))
{
    throw 'CSI cursor style Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI cursor style Rust ownership candidate.'
