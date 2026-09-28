#pragma once

#include "terminal_parser_ffi_output_csi_kitty_keyboard_push.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_kitty_keyboard_push_id(const char intermediate, const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(intermediate)) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_kitty_keyboard_push_plan(
        const char intermediate,
        const char finalCharacter,
        const int32_t parameter0,
        const uint32_t expectedKind,
        const int32_t expectedFlags)
    {
        terminal_parser_ffi_output_csi_kitty_keyboard_push_result plan{};
        const auto status = terminal_parser_ffi_output_csi_kitty_keyboard_push_plan(
            packed_csi_kitty_keyboard_push_id(intermediate, finalCharacter), parameter0, &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI Kitty keyboard push status %u for CSI %d%c%c\n",
                static_cast<unsigned>(status),
                parameter0,
                intermediate,
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind || plan.flags != expectedFlags)
        {
            std::fprintf(
                stderr,
                "output CSI Kitty keyboard push mismatch for CSI %d%c%c: kind=%u flags=%d\n",
                parameter0,
                intermediate,
                finalCharacter,
                plan.kind,
                plan.flags);
            return false;
        }

        return true;
    }

    inline bool output_csi_kitty_keyboard_push_replay()
    {
        terminal_parser_ffi_output_csi_kitty_keyboard_push_result invalidIdentifier{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_kitty_keyboard_push_plan(
            UINT64_C(0xff00000000000000), 0, &invalidIdentifier);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_kitty_keyboard_push_plan(
            packed_csi_kitty_keyboard_push_id('>', 'u'), 0, nullptr);

        return
            expect_output_csi_kitty_keyboard_push_plan(
                '>', 'u', 0, TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_PUSH, 0) &&
            expect_output_csi_kitty_keyboard_push_plan(
                '>', 'u', 1, TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_PUSH, 1) &&
            expect_output_csi_kitty_keyboard_push_plan(
                '>', 'u', 42, TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_PUSH, 42) &&
            expect_output_csi_kitty_keyboard_push_plan(
                '?', 'u', 42, TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_PUSH_NONE, 0) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
