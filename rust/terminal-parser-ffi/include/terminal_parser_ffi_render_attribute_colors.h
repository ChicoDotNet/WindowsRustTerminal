#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct terminal_parser_ffi_render_attribute_colors
{
    uint32_t foreground;
    uint32_t background;
} terminal_parser_ffi_render_attribute_colors;

terminal_parser_ffi_status terminal_parser_ffi_render_attribute_effects(
    uint32_t foreground,
    uint32_t background,
    uint32_t dim_foreground,
    uint32_t reverse_video,
    uint32_t screen_reversed,
    uint32_t invisible,
    terminal_parser_ffi_render_attribute_colors* out_colors);

terminal_parser_ffi_status terminal_parser_ffi_render_attribute_alpha(
    terminal_parser_ffi_render_attribute_colors colors,
    uint32_t background_default,
    uint32_t reverse_video,
    uint32_t screen_reversed,
    uint32_t invisible,
    terminal_parser_ffi_render_attribute_colors* out_colors);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_render_attribute_colors) == 8);
static_assert(offsetof(terminal_parser_ffi_render_attribute_colors, foreground) == 0);
static_assert(offsetof(terminal_parser_ffi_render_attribute_colors, background) == 4);
#endif
