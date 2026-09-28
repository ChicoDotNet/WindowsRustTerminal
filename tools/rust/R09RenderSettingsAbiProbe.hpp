#pragma once

#include "terminal_parser_ffi_render_settings.h"
#include "R09RenderAttributeColorsAbiProbe.hpp"

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

    inline bool expect_contrast_adjustment(
        const terminal_parser_ffi_render_settings_state& state,
        const uint32_t candidate,
        const uint32_t background,
        const bool candidateIsDefaultOrLegacy,
        const bool backgroundIsDefaultOrLegacy,
        const bool expected)
    {
        uint32_t shouldAdjust = UINT32_MAX;
        const auto status = terminal_parser_ffi_render_settings_should_adjust_contrast(
            &state,
            candidate,
            background,
            static_cast<uint32_t>(candidateIsDefaultOrLegacy),
            static_cast<uint32_t>(backgroundIsDefaultOrLegacy),
            &shouldAdjust);
        return status == TERMINAL_PARSER_FFI_OK && shouldAdjust == static_cast<uint32_t>(expected);
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
            !expect_render_mode(state, TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BRIGHT, false) ||
            !expect_contrast_adjustment(state, 0x00112233, 0x00445566, false, false, true) ||
            !expect_contrast_adjustment(state, 0x00445566, 0x00445566, false, false, false))
        {
            return false;
        }

        terminal_parser_ffi_render_settings_state indexedState{};
        if (terminal_parser_ffi_render_settings_default(&indexedState) != TERMINAL_PARSER_FFI_OK ||
            terminal_parser_ffi_render_settings_set_mode(
                &indexedState,
                TERMINAL_PARSER_FFI_RENDER_MODE_INDEXED_DISTINGUISHABLE_COLORS,
                1) != TERMINAL_PARSER_FFI_OK ||
            !expect_contrast_adjustment(indexedState, 0x00112233, 0x00445566, true, true, true) ||
            !expect_contrast_adjustment(indexedState, 0x00112233, 0x00445566, false, true, false))
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
        uint32_t shouldAdjust = 0;

        return
            terminal_parser_ffi_render_settings_set_mode(&state, 0, 1) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_set_mode(&state, 7, 1) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_set_mode(
                &state,
                TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED,
                2) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_should_adjust_contrast(
                &state, 0, 1, 2, 0, &shouldAdjust) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_should_adjust_contrast(
                &state, 0, 1, 0, 0, nullptr) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_should_adjust_contrast(
                nullptr, 0, 1, 0, 0, &shouldAdjust) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_toggle_blink(&invalidState) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_settings_default(nullptr) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            render_attribute_colors_replay();
    }
}
