$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '82ba08539c01ffd1f521ce86439443e7168efd95'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI Kitty keyboard push source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
$hasCrLf = $text.Contains("`r`n")
$hasBareLf = [regex]::IsMatch($text, "(?<!`r)`n")
if ($hasCrLf -and $hasBareLf)
{
    throw 'OutputStateMachineEngine.cpp has mixed line endings; refusing mechanical rewrite.'
}

# Normalize the exact, hash-pinned source to LF for deterministic markers and repository output.
$text = $text -replace "`r`n", "`n"

$includeOld = '#include "terminal_parser_ffi_output_csi_kitty_keyboard_query.h"' + "`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_kitty_keyboard_push.h"' + "`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_push.h'))
{
    throw 'CSI Kitty keyboard push include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (kittyKeyboardQueryPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@

$dispatchReplacement = @"
    if (kittyKeyboardQueryPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_kitty_keyboard_push_result kittyKeyboardPushPlan{};
    const auto kittyKeyboardPushStatus = terminal_parser_ffi_output_csi_kitty_keyboard_push_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &kittyKeyboardPushPlan);
    THROW_HR_IF(E_UNEXPECTED, kittyKeyboardPushStatus != TERMINAL_PARSER_FFI_OK);

    switch (kittyKeyboardPushPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_PUSH:
        _dispatch->PushKittyKeyboardProtocol(kittyKeyboardPushPlan.flags);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (kittyKeyboardPushPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI Kitty keyboard push dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCase = @"
    case CsiActionCodes::KKP_KittyKeyboardPush:
        _dispatch->PushKittyKeyboardProtocol(parameters.at(0));
        break;
"@

if (-not $text.Contains($legacyCase))
{
    throw 'Legacy CSI Kitty keyboard push case marker mismatch.'
}
$text = $text.Replace($legacyCase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI Kitty keyboard push source diff.'
}

if ($text.Contains('case CsiActionCodes::KKP_KittyKeyboardPush:'))
{
    throw 'Legacy CSI Kitty keyboard push case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_push_plan'))
{
    throw 'CSI Kitty keyboard push Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CSI Kitty keyboard push Rust ownership candidate.'
