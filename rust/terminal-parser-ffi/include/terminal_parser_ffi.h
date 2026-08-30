#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_parser_ffi_status
{
    TERMINAL_PARSER_FFI_OK = 0,
    TERMINAL_PARSER_FFI_INVALID_ARGUMENT = 1,
    TERMINAL_PARSER_FFI_INVALID_BASE64 = 2,
    TERMINAL_PARSER_FFI_INVALID_UTF8 = 3,
    TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL = 4,
    TERMINAL_PARSER_FFI_PANIC = 255,
} terminal_parser_ffi_status;

uint32_t terminal_parser_ffi_abi_version(void);
terminal_parser_ffi_status terminal_parser_ffi_status_probe(void);
terminal_parser_ffi_status terminal_parser_ffi_base64_decode_utf16(
    const uint16_t* input,
    size_t input_len,
    uint16_t* output,
    size_t output_capacity,
    size_t* out_len);

#ifdef __cplusplus
}
#endif
