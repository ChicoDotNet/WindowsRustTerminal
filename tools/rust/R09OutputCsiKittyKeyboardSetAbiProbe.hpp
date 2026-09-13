#pragma once

#include "terminal_parser_ffi_output_csi_kitty_keyboard_set.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_kitty_keyboard_set_id(const char intermediate, const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(intermediate)) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_kitty_keyboard_set_plan(
        const char intermediate,
        const char finalCharacter,
        const uint32_t hasFlags,
        const int32_t flags,
        const uint32_t hasMode,
        const int32_t mode,
        const uint32_t expectedKind,
        const uint32_t expectedHasFlags,
        const int32_t expectedFlags,
        const uint32_t expectedHasMode,
        const int32_t expectedMode)
    {
        terminal_parser_ffi_output_csi_kitty_keyboard_set_result plan{};
        const auto status = terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
            packed_csi_kitty_keyboard_set_id(intermediate, finalCharacter),
            hasFlags,
            flags,
            hasMode,
            mode,
            &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI Kitty keyboard set status %u for CSI %c%c\n",
                static_cast<unsigned>(status),
                intermediate,
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind ||
            plan.has_flags != expectedHasFlags ||
            plan.flags != expectedFlags ||
            plan.has_mode != expectedHasMode ||
            plan.mode != expectedMode)
        {
            std::fprintf(
                stderr,
                "output CSI Kitty keyboard set mismatch for CSI %c%c: kind=%u has_flags=%u flags=%d has_mode=%u mode=%d\n",
                intermediate,
                finalCharacter,
                plan.kind,
                plan.has_flags,
                plan.flags,
                plan.has_mode,
                plan.mode);
            return false;
        }

        return true;
    }

    inline bool output_csi_kitty_keyboard_set_replay()
    {
        terminal_parser_ffi_output_csi_kitty_keyboard_set_result invalid{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
            UINT64_C(0xff00000000000000), 1, 3, 1, 2, &invalid);
        const auto invalidFlagsPresenceStatus = terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
            packed_csi_kitty_keyboard_set_id('=', 'u'), 2, 3, 1, 2, &invalid);
        const auto invalidModePresenceStatus = terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
            packed_csi_kitty_keyboard_set_id('=', 'u'), 1, 3, 2, 2, &invalid);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
            packed_csi_kitty_keyboard_set_id('=', 'u'), 1, 3, 1, 2, nullptr);

        return
            expect_output_csi_kitty_keyboard_set_plan(
                '=', 'u', 1, 3, 1, 2,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_SET, 1, 3, 1, 2) &&
            expect_output_csi_kitty_keyboard_set_plan(
                '=', 'u', 1, 5, 0, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_SET, 1, 5, 0, 0) &&
            expect_output_csi_kitty_keyboard_set_plan(
                '=', 'u', 0, 0, 1, 4,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_SET, 0, 0, 1, 4) &&
            expect_output_csi_kitty_keyboard_set_plan(
                '=', 'u', 0, 0, 0, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_SET, 0, 0, 0, 0) &&
            expect_output_csi_kitty_keyboard_set_plan(
                '>', 'u', 1, 3, 1, 2,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_NONE, 0, 0, 0, 0) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            invalidFlagsPresenceStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            invalidModePresenceStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
