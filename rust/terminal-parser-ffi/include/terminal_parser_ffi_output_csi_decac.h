#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_csi_decac_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_ASSIGN_COLOR = 1,
} terminal_parser_ffi_output_csi_decac_kind;

typedef struct terminal_parser_ffi_output_csi_decac_result
{
    uint32_t kind;
    int32_t item;
    int32_t foreground_index;
    int32_t background_index;
} terminal_parser_ffi_output_csi_decac_result;

terminal_parser_ffi_status terminal_parser_ffi_output_csi_decac_plan(
    uint64_t identifier,
    int32_t parameter0,
    int32_t parameter1,
    int32_t parameter2,
    terminal_parser_ffi_output_csi_decac_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_csi_decac_result) == 16);
static_assert(offsetof(terminal_parser_ffi_output_csi_decac_result, kind) == 0);
static_assert(offsetof(terminal_parser_ffi_output_csi_decac_result, item) == 4);
static_assert(offsetof(terminal_parser_ffi_output_csi_decac_result, foreground_index) == 8);
static_assert(offsetof(terminal_parser_ffi_output_csi_decac_result, background_index) == 12);
#endif
