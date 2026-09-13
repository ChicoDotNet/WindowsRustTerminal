#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum terminal_settings_ffi_status
{
    TERMINAL_SETTINGS_FFI_OK = 0,
    TERMINAL_SETTINGS_FFI_INVALID_ARGUMENT = 1,
    TERMINAL_SETTINGS_FFI_PANIC = 255,
} terminal_settings_ffi_status;

typedef enum terminal_settings_ffi_workspace_rename_plan_value
{
    TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_NOOP = 0,
    TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_REMOVE = 1,
    TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_RENAME = 2,
} terminal_settings_ffi_workspace_rename_plan_value;

uint32_t terminal_settings_ffi_abi_version(void);

terminal_settings_ffi_status terminal_settings_ffi_workspace_rename_plan(
    uint8_t old_name_empty,
    uint8_t names_equal,
    uint8_t old_exists,
    uint8_t new_name_empty,
    uint32_t* out_plan);

#ifdef __cplusplus
}

static_assert(TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_NOOP == 0);
static_assert(TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_REMOVE == 1);
static_assert(TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_RENAME == 2);
#endif
