$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '00e9b3e40a953d8d82dcd9101bc5239cd4cd1acd'

$actualBlob = (git rev-parse "HEAD:$source").Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI Kitty keyboard set source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source) -replace "`r`n", "`n"

$includeOld = '#include "terminal_parser_ffi_output_csi_kitty_keyboard_pop.h"' + "`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_kitty_keyboard_set.h"' + "`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_set.h'))
{
    throw 'CSI Kitty keyboard set include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (kittyKeyboardPopPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

$dispatchReplacement = @"
    if (kittyKeyboardPopPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_POP_NONE)
    {
        _ClearLastChar();
        return true;
    }

    const auto kittyKeyboardFlagsParameter = parameters.at(0);
    const auto kittyKeyboardModeParameter = parameters.at(1);
    terminal_parser_ffi_output_csi_kitty_keyboard_set_result kittyKeyboardSetPlan{};
    const auto kittyKeyboardSetStatus = terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
        static_cast<uint64_t>(id),
        kittyKeyboardFlagsParameter.has_value() ? 1u : 0u,
        static_cast<int32_t>(kittyKeyboardFlagsParameter.value_or(0)),
        kittyKeyboardModeParameter.has_value() ? 1u : 0u,
        static_cast<int32_t>(kittyKeyboardModeParameter.value_or(0)),
        &kittyKeyboardSetPlan);
    THROW_HR_IF(E_UNEXPECTED, kittyKeyboardSetStatus != TERMINAL_PARSER_FFI_OK);

    switch (kittyKeyboardSetPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_SET:
        _dispatch->SetKittyKeyboardProtocol(
            kittyKeyboardSetPlan.has_flags != 0 ? VTParameter{ kittyKeyboardSetPlan.flags } : VTParameter{},
            kittyKeyboardSetPlan.has_mode != 0 ? VTParameter{ kittyKeyboardSetPlan.mode } : VTParameter{});
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (kittyKeyboardSetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI Kitty keyboard set dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCase = @"
    case CsiActionCodes::KKP_KittyKeyboardSet:
        _dispatch->SetKittyKeyboardProtocol(parameters.at(0), parameters.at(1));
        break;
"@ -replace "`r`n", "`n"

if (-not $text.Contains($legacyCase))
{
    throw 'Legacy CSI Kitty keyboard set case marker mismatch.'
}
$text = $text.Replace($legacyCase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI Kitty keyboard set source diff.'
}

if ($text.Contains('case CsiActionCodes::KKP_KittyKeyboardSet:'))
{
    throw 'Legacy CSI Kitty keyboard set case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_kitty_keyboard_set_plan'))
{
    throw 'CSI Kitty keyboard set Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CSI Kitty keyboard set Rust ownership candidate.'
