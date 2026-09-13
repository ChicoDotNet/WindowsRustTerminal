#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_pty_signal_kind
{
    TERMINAL_PARSER_FFI_PTY_SIGNAL_SHOW_HIDE_WINDOW = 1,
    TERMINAL_PARSER_FFI_PTY_SIGNAL_CLEAR_BUFFER = 2,
    TERMINAL_PARSER_FFI_PTY_SIGNAL_SET_PARENT = 3,
    TERMINAL_PARSER_FFI_PTY_SIGNAL_RESIZE_WINDOW = 8,
} terminal_parser_ffi_pty_signal_kind;

typedef struct terminal_parser_ffi_pty_signal_read_plan_result
{
    uint32_t kind;
    uint32_t payload_len;
} terminal_parser_ffi_pty_signal_read_plan_result;

terminal_parser_ffi_status terminal_parser_ffi_pty_signal_read_plan(
    uint16_t signal_id,
    terminal_parser_ffi_pty_signal_read_plan_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_pty_signal_read_plan_result) == 8);
static_assert(offsetof(terminal_parser_ffi_pty_signal_read_plan_result, kind) == 0);
static_assert(offsetof(terminal_parser_ffi_pty_signal_read_plan_result, payload_len) == 4);
#endif
