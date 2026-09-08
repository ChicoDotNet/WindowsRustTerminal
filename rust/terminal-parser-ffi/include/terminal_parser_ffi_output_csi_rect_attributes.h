#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_output_csi_rect_attributes_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_CHANGE = 1,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_REVERSE = 2,
} terminal_parser_ffi_output_csi_rect_attributes_kind;

typedef struct terminal_parser_ffi_output_csi_rect_attributes_plan
{
    uint32_t kind;
    int32_t bottom;
    int32_t right;
} terminal_parser_ffi_output_csi_rect_attributes_plan;

terminal_parser_ffi_status terminal_parser_ffi_output_csi_rect_attributes_plan(
    uint64_t identifier,
    int32_t bottom,
    int32_t right,
    terminal_parser_ffi_output_csi_rect_attributes_plan* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_output_csi_rect_attributes_plan) == 12);
static_assert(offsetof(terminal_parser_ffi_output_csi_rect_attributes_plan, kind) == 0);
static_assert(offsetof(terminal_parser_ffi_output_csi_rect_attributes_plan, bottom) == 4);
static_assert(offsetof(terminal_parser_ffi_output_csi_rect_attributes_plan, right) == 8);
#endif
