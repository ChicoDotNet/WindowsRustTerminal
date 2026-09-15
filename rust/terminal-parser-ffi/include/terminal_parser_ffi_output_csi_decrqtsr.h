#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_csi_decrqtsr_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REQUEST_TERMINAL_STATE = 1,
} terminal_parser_ffi_output_csi_decrqtsr_kind;

typedef struct terminal_parser_ffi_output_csi_decrqtsr_result
{
    uint32_t kind;
    int32_t format;
    int32_t format_option;
    uint32_t reserved0;
} terminal_parser_ffi_output_csi_decrqtsr_result;

terminal_parser_ffi_status terminal_parser_ffi_output_csi_decrqtsr_plan(
    uint64_t identifier,
    int32_t parameter0_raw,
    int32_t parameter1_raw,
    terminal_parser_ffi_output_csi_decrqtsr_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_csi_decrqtsr_result) == 16);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqtsr_result, kind) == 0);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqtsr_result, format) == 4);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqtsr_result, format_option) == 8);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqtsr_result, reserved0) == 12);
#endif
