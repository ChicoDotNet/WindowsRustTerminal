// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "terminal_parser_ffi.h"

#include <cstdint>
#include <cstdio>

namespace
{
    bool expect_control_plan(
        const uint16_t codeUnit,
        const uint32_t writeAlt,
        const uint32_t expectedKind,
        const uint16_t expectedCharacter,
        const uint16_t expectedVirtualKey,
        const uint32_t expectedWriteCtrl,
        const uint32_t expectedClearLayoutModifiers)
    {
        terminal_parser_ffi_control_character_plan plan{};
        const auto status = terminal_parser_ffi_input_control_character_plan(codeUnit, writeAlt, &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(stderr, "control plan status %u for U+%04X\n", static_cast<unsigned>(status), codeUnit);
            return false;
        }

        if (plan.kind != expectedKind ||
            plan.character != expectedCharacter ||
            plan.forced_virtual_key != expectedVirtualKey ||
            plan.write_ctrl != expectedWriteCtrl ||
            plan.clear_layout_modifiers != expectedClearLayoutModifiers)
        {
            std::fprintf(
                stderr,
                "control plan mismatch for U+%04X: kind=%u char=%04X vk=%04X ctrl=%u clear=%u\n",
                codeUnit,
                plan.kind,
                plan.character,
                plan.forced_virtual_key,
                plan.write_ctrl,
                plan.clear_layout_modifiers);
            return false;
        }

        return true;
    }

    bool expect_mouse_plan(
        const uint32_t previousButtonState,
        const uint32_t encoding,
        const uint32_t buttonDown,
        const uint32_t expectedButtonId,
        const uint32_t expectedButtonState,
        const uint32_t expectedPersistentButtonState,
        const uint32_t expectedEventFlags,
        const uint32_t expectedTrackClick)
    {
        terminal_parser_ffi_sgr_mouse_plan plan{};
        const auto status = terminal_parser_ffi_input_sgr_mouse_plan(previousButtonState, encoding, buttonDown, &plan);
        if (status != TERMINAL_PARSER_FFI_OK || plan.valid == 0)
        {
            std::fprintf(stderr, "mouse plan status %u valid=%u for encoding %u\n", static_cast<unsigned>(status), plan.valid, encoding);
            return false;
        }

        if (plan.button_id != expectedButtonId ||
            plan.button_state != expectedButtonState ||
            plan.persistent_button_state != expectedPersistentButtonState ||
            plan.event_flags != expectedEventFlags ||
            plan.track_click != expectedTrackClick)
        {
            std::fprintf(
                stderr,
                "mouse plan mismatch for encoding %u: id=%u state=%08X persistent=%08X flags=%08X track=%u\n",
                encoding,
                plan.button_id,
                plan.button_state,
                plan.persistent_button_state,
                plan.event_flags,
                plan.track_click);
            return false;
        }

        return true;
    }
}

int main()
{
    const bool controlOk =
        expect_control_plan(0x03, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_CTRL_C, 0x03, 0x43, 1, 1) &&
        expect_control_plan(0x03, 1, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x03, 0x00, 1, 0) &&
        expect_control_plan(0x08, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x7f, 0x00, 1, 1) &&
        expect_control_plan(0x09, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x09, 0x00, 0, 0) &&
        expect_control_plan(0x0d, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x0d, 0x00, 0, 1) &&
        expect_control_plan(0x1b, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x1b, 0x1b, 0, 1) &&
        expect_control_plan(0x7f, 1, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_DELETE_AS_BACKSPACE, 0x08, 0x08, 0, 1) &&
        expect_control_plan(0x41, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_PRINT, 0x41, 0x00, 0, 0);

    constexpr uint32_t leftButton = 0x0001;
    constexpr uint32_t rightButton = 0x0002;
    constexpr uint32_t middleButton = 0x0004;
    constexpr uint32_t mouseMoved = 0x0001;
    constexpr uint32_t mouseWheeled = 0x0004;
    constexpr uint32_t mouseHorizontalWheeled = 0x0008;
    constexpr uint32_t scrollBackward = 0xff800000;
    constexpr uint32_t scrollForward = 0x00800000;
    constexpr uint32_t allPrimaryButtons = leftButton | rightButton | middleButton;

    const bool mouseOk =
        expect_mouse_plan(0, 0, 1, 0, leftButton, leftButton, 0, 1) &&
        expect_mouse_plan(leftButton, 32, 1, 0, leftButton, leftButton, mouseMoved, 1) &&
        expect_mouse_plan(leftButton, 0, 0, 0, 0, 0, 0, 0) &&
        expect_mouse_plan(leftButton, 64, 1, 4, leftButton | scrollForward, leftButton, mouseWheeled, 0) &&
        expect_mouse_plan(leftButton, 65, 1, 5, leftButton | scrollBackward, leftButton, mouseWheeled, 0) &&
        expect_mouse_plan(leftButton, 66, 1, 6, leftButton | scrollBackward, leftButton, mouseHorizontalWheeled, 0) &&
        expect_mouse_plan(leftButton, 67, 1, 7, leftButton | scrollForward, leftButton, mouseHorizontalWheeled, 0) &&
        expect_mouse_plan(allPrimaryButtons, 1, 0, 1, leftButton | rightButton, leftButton | rightButton, 0, 0) &&
        expect_mouse_plan(leftButton | rightButton, 2, 0, 2, leftButton, leftButton, 0, 0) &&
        expect_mouse_plan(leftButton, 0, 0, 0, 0, 0, 0, 0);

    if (!controlOk || !mouseOk)
    {
        return 1;
    }

    std::puts("R09 parser C ABI replay passed.");
    return 0;
}
