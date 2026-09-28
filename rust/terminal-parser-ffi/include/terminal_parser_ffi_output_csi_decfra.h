#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

terminal_parser_ffi_status terminal_parser_ffi_output_csi_decfra_values(
    uint64_t identifier,
    const int32_t* values,
    size_t value_count,
    int32_t* out_values,
    size_t output_capacity,
    size_t* out_count,
    uint32_t* out_matched);

#ifdef __cplusplus
}
#endif
