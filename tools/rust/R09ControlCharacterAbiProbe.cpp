// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "terminal_parser_ffi.h"

#include <cstdint>
#include <cstdio>

namespace
{
    bool expect_plan(
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
}

int main()
{
    const bool ok =
        expect_plan(0x03, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_CTRL_C, 0x03, 0x43, 1, 1) &&
        expect_plan(0x03, 1, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x03, 0x00, 1, 0) &&
        expect_plan(0x08, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x7f, 0x00, 1, 1) &&
        expect_plan(0x09, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x09, 0x00, 0, 0) &&
        expect_plan(0x0d, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x0d, 0x00, 0, 1) &&
        expect_plan(0x1b, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_MAPPED_C0, 0x1b, 0x1b, 0, 1) &&
        expect_plan(0x7f, 1, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_DELETE_AS_BACKSPACE, 0x08, 0x08, 0, 1) &&
        expect_plan(0x41, 0, TERMINAL_PARSER_FFI_CONTROL_CHARACTER_PRINT, 0x41, 0x00, 0, 0);

    if (!ok)
    {
        return 1;
    }

    std::puts("R09 control-character C ABI replay passed.");
    return 0;
}
