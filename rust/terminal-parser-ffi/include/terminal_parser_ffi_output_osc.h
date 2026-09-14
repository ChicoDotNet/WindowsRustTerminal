#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_osc_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_OSC_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_WINDOW_TITLE = 1,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_COLOR_TABLE = 2,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_DYNAMIC_COLOR = 3,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_CLIPBOARD = 4,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_COLOR_TABLE = 5,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_DYNAMIC_COLOR = 6,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_CURRENT_WORKING_DIRECTORY = 7,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_HYPERLINK = 8,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_CONEMU_ACTION = 9,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_FINALTERM_ACTION = 10,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_VSCODE_ACTION = 11,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_URXVT_ACTION = 12,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_ITERM2_ACTION = 13,
    TERMINAL_PARSER_FFI_OUTPUT_OSC_WT_ACTION = 14,
} terminal_parser_ffi_output_osc_kind;

typedef struct terminal_parser_ffi_output_osc_plan_result
{
    uint32_t kind;
} terminal_parser_ffi_output_osc_plan_result;

terminal_parser_ffi_status terminal_parser_ffi_output_osc_plan(
    uint64_t parameter,
    terminal_parser_ffi_output_osc_plan_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_osc_plan_result) == 4);
static_assert(offsetof(terminal_parser_ffi_output_osc_plan_result, kind) == 0);
#endif
