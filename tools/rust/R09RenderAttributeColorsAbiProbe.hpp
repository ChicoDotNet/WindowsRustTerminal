#pragma once

#include "terminal_parser_ffi_render_attribute_colors.h"

#include <cstdint>

namespace r09
{
    inline bool render_attribute_colors_replay()
    {
        terminal_parser_ffi_render_attribute_colors colors{};
        if (terminal_parser_ffi_render_attribute_effects(
                0x00604020,
                0x00112233,
                1,
                1,
                0,
                1,
                &colors) != TERMINAL_PARSER_FFI_OK ||
            colors.foreground != 0x00302010 ||
            colors.background != 0x00302010)
        {
            return false;
        }

        colors = {};
        if (terminal_parser_ffi_render_attribute_effects_from_rendition(
                0x00604020,
                0x00112233,
                0,
                1,
                1,
                0,
                0,
                0,
                &colors) != TERMINAL_PARSER_FFI_OK ||
            colors.foreground != 0x00302010 ||
            colors.background != 0x00112233)
        {
            return false;
        }

        colors = {};
        if (terminal_parser_ffi_render_attribute_effects_from_rendition(
                0x00604020,
                0x00112233,
                0,
                1,
                0,
                0,
                0,
                0,
                &colors) != TERMINAL_PARSER_FFI_OK ||
            colors.foreground != 0x00604020 ||
            colors.background != 0x00112233)
        {
            return false;
        }

        colors = { 0x00112233, 0x00445566 };
        if (terminal_parser_ffi_render_attribute_effects(
                colors.foreground,
                colors.background,
                0,
                1,
                1,
                0,
                &colors) != TERMINAL_PARSER_FFI_OK ||
            colors.foreground != 0x00112233 ||
            colors.background != 0x00445566)
        {
            return false;
        }

        if (terminal_parser_ffi_render_attribute_alpha(colors, 1, 0, 0, 0, &colors) != TERMINAL_PARSER_FFI_OK ||
            colors.foreground != 0xFF112233 ||
            colors.background != 0x00445566)
        {
            return false;
        }

        return
            terminal_parser_ffi_render_attribute_effects(0, 0, 2, 0, 0, 0, &colors) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_attribute_effects(0, 0, 0, 2, 0, 0, &colors) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_attribute_effects(0, 0, 0, 0, 0, 0, nullptr) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_attribute_effects_from_rendition(0, 0, 0, 0, 2, 0, 0, 0, &colors) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_attribute_effects_from_rendition(0, 0, 0, 0, 0, 0, 0, 0, nullptr) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_attribute_alpha(colors, 2, 0, 0, 0, &colors) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            terminal_parser_ffi_render_attribute_alpha(colors, 0, 0, 0, 0, nullptr) == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
