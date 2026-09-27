#pragma once

#include "terminal_parser_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct terminal_parser_ffi_console_argument_token
{
    const uint16_t* data;
    size_t len;
} terminal_parser_ffi_console_argument_token;

typedef struct terminal_parser_ffi_console_argument_plan
{
    int16_t width;
    int16_t height;
    uint16_t flags;
    uint8_t has_server_handle;
    uint8_t has_signal_handle;
    uint32_t server_handle;
    uint32_t signal_handle;
    size_t client_commandline_len;
    size_t text_measurement_len;
} terminal_parser_ffi_console_argument_plan;

enum terminal_parser_ffi_console_argument_flags
{
    TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_AMBIGUOUS_IS_WIDE = 1u << 0,
    TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_FORCE_V1 = 1u << 1,
    TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_FORCE_NO_HANDOFF = 1u << 2,
    TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_HEADLESS = 1u << 3,
    TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_RUN_AS_COM_SERVER = 1u << 4,
    TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_CREATE_SERVER_HANDLE = 1u << 5,
    TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_INHERIT_CURSOR = 1u << 6,
};

terminal_parser_ffi_status terminal_parser_ffi_console_arguments_plan(
    const terminal_parser_ffi_console_argument_token* tokens,
    size_t token_count,
    uint16_t* client_commandline,
    size_t client_commandline_capacity,
    uint16_t* text_measurement,
    size_t text_measurement_capacity,
    terminal_parser_ffi_console_argument_plan* out_plan);

#ifdef __cplusplus
}

static_assert(sizeof(terminal_parser_ffi_console_argument_token) == sizeof(void*) + sizeof(size_t));
static_assert(offsetof(terminal_parser_ffi_console_argument_token, data) == 0);
static_assert(offsetof(terminal_parser_ffi_console_argument_token, len) == sizeof(void*));
static_assert(offsetof(terminal_parser_ffi_console_argument_plan, width) == 0);
static_assert(offsetof(terminal_parser_ffi_console_argument_plan, height) == 2);
static_assert(offsetof(terminal_parser_ffi_console_argument_plan, flags) == 4);
static_assert(offsetof(terminal_parser_ffi_console_argument_plan, has_server_handle) == 6);
static_assert(offsetof(terminal_parser_ffi_console_argument_plan, has_signal_handle) == 7);
static_assert(offsetof(terminal_parser_ffi_console_argument_plan, server_handle) == 8);
static_assert(offsetof(terminal_parser_ffi_console_argument_plan, signal_handle) == 12);
static_assert(offsetof(terminal_parser_ffi_console_argument_plan, client_commandline_len) == 16);
#endif
