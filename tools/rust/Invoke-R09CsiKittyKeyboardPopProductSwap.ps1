$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'ffd7aacb66df99ebb496d3cd506348dc6f0e5253'

$actualBlob = (git rev-parse "HEAD:$source").Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI Kitty keyboard pop source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source) -replace "`r`n", "`n"

$includeOld = '#include "terminal_parser_ffi_output_csi_kitty_keyboard_push.h"' + "`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_kitty_keyboard_pop.h"' + "`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_pop.h'))
{
    throw 'CSI Kitty keyboard pop include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (kittyKeyboardPushPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

$dispatchReplacement = @"
    if (kittyKeyboardPushPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_kitty_keyboard_pop_result kittyKeyboardPopPlan{};
    const auto kittyKeyboardPopStatus = terminal_parser_ffi_output_csi_kitty_keyboard_pop_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &kittyKeyboardPopPlan);
    THROW_HR_IF(E_UNEXPECTED, kittyKeyboardPopStatus != TERMINAL_PARSER_FFI_OK);

    switch (kittyKeyboardPopPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_POP:
        _dispatch->PopKittyKeyboardProtocol(kittyKeyboardPopPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (kittyKeyboardPopPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI Kitty keyboard pop dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCase = @"
    case CsiActionCodes::KKP_KittyKeyboardPop:
        _dispatch->PopKittyKeyboardProtocol(parameters.at(0));
        break;
"@ -replace "`r`n", "`n"

if (-not $text.Contains($legacyCase))
{
    throw 'Legacy CSI Kitty keyboard pop case marker mismatch.'
}
$text = $text.Replace($legacyCase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI Kitty keyboard pop source diff.'
}

if ($text.Contains('case CsiActionCodes::KKP_KittyKeyboardPop:'))
{
    throw 'Legacy CSI Kitty keyboard pop case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_pop_plan'))
{
    throw 'CSI Kitty keyboard pop Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CSI Kitty keyboard pop Rust ownership candidate.'
