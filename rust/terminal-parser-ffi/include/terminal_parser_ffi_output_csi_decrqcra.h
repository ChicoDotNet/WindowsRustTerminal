#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_csi_decrqcra_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_REQUEST_CHECKSUM_RECTANGULAR_AREA = 1,
} terminal_parser_ffi_output_csi_decrqcra_kind;

typedef struct terminal_parser_ffi_output_csi_decrqcra_result
{
    uint32_t kind;
    int32_t request_id;
    int32_t page;
    int32_t bottom;
    int32_t right;
} terminal_parser_ffi_output_csi_decrqcra_result;

terminal_parser_ffi_status terminal_parser_ffi_output_csi_decrqcra_plan(
    uint64_t identifier,
    int32_t request_id,
    int32_t page,
    int32_t bottom,
    int32_t right,
    terminal_parser_ffi_output_csi_decrqcra_result* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_csi_decrqcra_result) == 20);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqcra_result, kind) == 0);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqcra_result, request_id) == 4);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqcra_result, page) == 8);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqcra_result, bottom) == 12);
static_assert(offsetof(terminal_parser_ffi_output_csi_decrqcra_result, right) == 16);
#endif
