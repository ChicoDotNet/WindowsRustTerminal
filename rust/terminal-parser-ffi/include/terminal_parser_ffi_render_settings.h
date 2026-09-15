#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_render_mode
{
    TERMINAL_PARSER_FFI_RENDER_MODE_INDEXED_DISTINGUISHABLE_COLORS = 1,
    TERMINAL_PARSER_FFI_RENDER_MODE_ALWAYS_DISTINGUISHABLE_COLORS = 2,
    TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BOLD = 3,
    TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BRIGHT = 4,
    TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED = 5,
    TERMINAL_PARSER_FFI_RENDER_MODE_SYNCHRONIZED_OUTPUT = 6,
} terminal_parser_ffi_render_mode;

typedef struct terminal_parser_ffi_render_settings_state
{
    uint32_t modes;
    uint32_t blink_should_be_faint;
} terminal_parser_ffi_render_settings_state;

terminal_parser_ffi_status terminal_parser_ffi_render_settings_default(
    terminal_parser_ffi_render_settings_state* out_state);
terminal_parser_ffi_status terminal_parser_ffi_render_settings_set_mode(
    terminal_parser_ffi_render_settings_state* state,
    uint32_t mode,
    uint32_t enabled);
terminal_parser_ffi_status terminal_parser_ffi_render_settings_get_mode(
    const terminal_parser_ffi_render_settings_state* state,
    uint32_t mode,
    uint32_t* out_enabled);
terminal_parser_ffi_status terminal_parser_ffi_render_settings_should_adjust_contrast(
    const terminal_parser_ffi_render_settings_state* state,
    uint32_t candidate,
    uint32_t background,
    uint32_t candidate_is_default_or_legacy,
    uint32_t background_is_default_or_legacy,
    uint32_t* out_should_adjust);
terminal_parser_ffi_status terminal_parser_ffi_render_settings_restore_programmable_defaults(
    terminal_parser_ffi_render_settings_state* state);
terminal_parser_ffi_status terminal_parser_ffi_render_settings_toggle_blink(
    terminal_parser_ffi_render_settings_state* state);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_render_settings_state) == 8);
static_assert(offsetof(terminal_parser_ffi_render_settings_state, modes) == 0);
static_assert(offsetof(terminal_parser_ffi_render_settings_state, blink_should_be_faint) == 4);
#endif
