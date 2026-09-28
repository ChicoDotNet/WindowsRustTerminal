#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_csi_rep_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_REPEAT = 1,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_HANDLED_NOOP = 2,
} terminal_parser_ffi_output_csi_rep_kind;

typedef struct terminal_parser_ffi_output_csi_rep_result
{
    uint32_t kind;
    int32_t count;
} terminal_parser_ffi_output_csi_rep_result;

terminal_parser_ffi_status terminal_parser_ffi_output_csi_rep_plan(
    uint64_t identifier,
    int32_t parameter0,
    uint16_t last_printed_char,
    terminal_parser_ffi_output_csi_rep_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_csi_rep_result) == 8);
static_assert(offsetof(terminal_parser_ffi_output_csi_rep_result, kind) == 0);
static_assert(offsetof(terminal_parser_ffi_output_csi_rep_result, count) == 4);
#endif
