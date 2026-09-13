#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

enum terminal_parser_ffi_output_csi_rect_erase_kind
{
    TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE_NONE = 0,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE = 1,
    TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_SELECTIVE_ERASE = 2,
};

terminal_parser_ffi_status terminal_parser_ffi_output_csi_rect_erase_values(
    uint64_t identifier,
    const int32_t* values,
    size_t value_count,
    int32_t* out_values,
    size_t output_capacity,
    size_t* out_count,
    uint32_t* out_kind);

#ifdef __cplusplus
}
#endif
