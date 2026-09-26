#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_csi_decsace_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_SELECT_ATTRIBUTE_CHANGE_EXTENT = 1,
} terminal_parser_ffi_output_csi_decsace_kind;

typedef struct terminal_parser_ffi_output_csi_decsace_result
{
    uint32_t kind;
    int32_t change_extent;
    uint32_t reserved0;
    uint32_t reserved1;
} terminal_parser_ffi_output_csi_decsace_result;

terminal_parser_ffi_status terminal_parser_ffi_output_csi_decsace_plan(
    uint64_t identifier,
    int32_t parameter0,
    terminal_parser_ffi_output_csi_decsace_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_csi_decsace_result) == 16);
static_assert(offsetof(terminal_parser_ffi_output_csi_decsace_result, kind) == 0);
static_assert(offsetof(terminal_parser_ffi_output_csi_decsace_result, change_extent) == 4);
static_assert(offsetof(terminal_parser_ffi_output_csi_decsace_result, reserved0) == 8);
static_assert(offsetof(terminal_parser_ffi_output_csi_decsace_result, reserved1) == 12);
#endif
