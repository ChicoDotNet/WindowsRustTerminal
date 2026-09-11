#pragma once

#include "terminal_parser_ffi_render_settings.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline bool expect_render_mode(
        const terminal_parser_ffi_render_settings_state& state,
        const uint32_t mode,
        const bool expected)
    {
        uint32_t enabled = UINT32_MAX;
        const auto status = terminal_parser_ffi_render_settings_get_mode(&state, mode, &enabled);
        if (status != TERMINAL_PARSER_FFI_OK || enabled != static_cast<uint32_t>(expected))
        {
            std::fprintf(
                stderr,
                "render mode mismatch: mode=%u status=%u enabled=%u expected=%u\n",
                mode,
                static_cast<unsigned>(status),
                enabled,
                static_cast<unsigned>(expected));
            return false;
        }
        return true;
    }

    inline bool render_settings_policy_replay()
    {
        terminal_parser_ffi_render_settings_state state{};
        if (terminal_parser_ffi_render_settings_default(&state) != TERMINAL_PARSER_FFI_OK)
        {
            return false;
        }

        if (!expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BRIGHT, true) ||
            !expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED, false) ||
            !expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_SYNCHRONIZED_OUTPUT, false) ||
            state.blink_should_be_faint != 0)
        {
            return false;
        }

        if (terminal_parser_ffi_render_settings_set_mode(
                &state,
                TERMINAL_PARSER_FFI_RENDER_MODE_ALWAYS_DISTINGUISHABLE_COLORS,
                1) != TERMINAL_PARSER_FFI_OK ||
            terminal_parser_ffi_render_settings_set_mode(
                &state,
                TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BRIGHT,
                0) != TERMINAL_PARSER_FFI_OK ||
            !expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_ALWAYS_DISTINGUISHABLE_COLORS, true) ||
            !expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BRIGHT, false))
        {
            return false;
        }

        if (terminal_parser_ffi_render_settings_set_mode(
                &state,
                TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED,
                1) != TERMINAL_PARSER_FFI_OK ||
            terminal_parser_ffi_render_settings_set_mode(
                &state,
                TERMINAL_PARSER_FFI_RENDER_MODE_SYNCHRONIZED_OUTPUT,
                1) != TERMINAL_PARSER_FFI_OK ||
            terminal_parser_ffi_render_settings_toggle_blink(&state) != TERMINAL_PARSER_FFI_OK ||
            terminal_parser_ffi_render_settings_restore_programmable_defaults(&state) != TERMINAL_PARSER_FFI_OK)
        {
            return false;
        }

        if (!expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_ALWAYS_DISTINGUISHABLE_COLORS, true) ||
            !expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED, false) ||
            !expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_SYNCHRONIZED_OUTPUT, false) ||
            state.blink_should_be_faint != 1)
        {
            return false;
        }

        auto invalidState = state;
        invalidState.modes = 1u << 31;

        return
            terminal_parser_ffi_render_settings_set_mode(&state, 0, 1) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_set_mode(&state, 7, 1) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_set_mode(
                &state,
                TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED,
                2) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_toggle_blink(&invalidState) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_default(nullptr) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
