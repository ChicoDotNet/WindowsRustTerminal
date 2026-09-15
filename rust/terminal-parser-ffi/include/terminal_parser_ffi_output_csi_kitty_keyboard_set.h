#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_csi_kitty_keyboard_set_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_SET_SET = 1,
} terminal_parser_ffi_output_csi_kitty_keyboard_set_kind;

typedef struct terminal_parser_ffi_output_csi_kitty_keyboard_set_result
{
    uint32_t kind;
    uint32_t has_flags;
    int32_t flags;
    uint32_t has_mode;
    int32_t mode;
    uint32_t reserved0;
} terminal_parser_ffi_output_csi_kitty_keyboard_set_result;

terminal_parser_ffi_status terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
    uint64_t identifier,
    uint32_t has_flags,
    int32_t flags,
    uint32_t has_mode,
    int32_t mode,
    terminal_parser_ffi_output_csi_kitty_keyboard_set_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_csi_kitty_keyboard_set_result) == 24);
static_assert(offsetof(terminal_parser_ffi_output_csi_kitty_keyboard_set_result, kind) == 0);
static_assert(offsetof(terminal_parser_ffi_output_csi_kitty_keyboard_set_result, has_flags) == 4);
static_assert(offsetof(terminal_parser_ffi_output_csi_kitty_keyboard_set_result, flags) == 8);
static_assert(offsetof(terminal_parser_ffi_output_csi_kitty_keyboard_set_result, has_mode) == 12);
static_assert(offsetof(terminal_parser_ffi_output_csi_kitty_keyboard_set_result, mode) == 16);
static_assert(offsetof(terminal_parser_ffi_output_csi_kitty_keyboard_set_result, reserved0) == 20);
#endif
