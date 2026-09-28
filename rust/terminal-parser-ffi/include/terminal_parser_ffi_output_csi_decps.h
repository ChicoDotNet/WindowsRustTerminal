#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_csi_decps_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_PLAY_SOUNDS = 1,
} terminal_parser_ffi_output_csi_decps_kind;

typedef struct terminal_parser_ffi_output_csi_decps_result
{
    uint32_t kind;
} terminal_parser_ffi_output_csi_decps_result;

terminal_parser_ffi_status terminal_parser_ffi_output_csi_decps_plan(
    uint64_t identifier,
    terminal_parser_ffi_output_csi_decps_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_csi_decps_result) == 4);
static_assert(offsetof(terminal_parser_ffi_output_csi_decps_result, kind) == 0);
#endif
