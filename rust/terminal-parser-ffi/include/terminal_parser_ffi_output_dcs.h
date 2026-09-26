#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_dcs_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_DCS_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_DCS_DEFINE_SIXEL_IMAGE = 1,
    TERMINAL_PARSER_FFI_OUTPUT_DCS_DOWNLOAD_DRCS = 2,
    TERMINAL_PARSER_FFI_OUTPUT_DCS_ASSIGN_USER_PREFERENCE_CHARSET = 3,
    TERMINAL_PARSER_FFI_OUTPUT_DCS_DEFINE_MACRO = 4,
    TERMINAL_PARSER_FFI_OUTPUT_DCS_RESTORE_TERMINAL_STATE = 5,
    TERMINAL_PARSER_FFI_OUTPUT_DCS_REQUEST_SETTING = 6,
    TERMINAL_PARSER_FFI_OUTPUT_DCS_RESTORE_PRESENTATION_STATE = 7,
} terminal_parser_ffi_output_dcs_kind;

typedef struct terminal_parser_ffi_output_dcs_plan_result
{
    uint32_t kind;
} terminal_parser_ffi_output_dcs_plan_result;

terminal_parser_ffi_status terminal_parser_ffi_output_dcs_plan(
    uint64_t identifier,
    terminal_parser_ffi_output_dcs_plan_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_dcs_plan_result) == 4);
static_assert(offsetof(terminal_parser_ffi_output_dcs_plan_result, kind) == 0);
#endif