#pragma once

#include <stddef.h>
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

typedef enum terminal_settings_ffi_commandline_fixup_plan_value
{
    TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_NONE = 0,
    TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_CLEAR_OVERRIDE = 1,
    TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_RESTORE_CMD_FULL_PATH = 2,
    TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_RESTORE_POWERSHELL_FULL_PATH = 3,
} terminal_settings_ffi_commandline_fixup_plan_value;

uint32_t terminal_settings_ffi_abi_version(void);

terminal_settings_ffi_status terminal_settings_ffi_workspace_rename_plan(
    uint8_t old_name_empty,
    uint8_t names_equal,
    uint8_t old_exists,
    uint8_t new_name_empty,
    uint32_t* out_plan);

terminal_settings_ffi_status terminal_settings_ffi_commandline_fixup_candidate(
    const uint16_t* guid,
    size_t guid_len,
    const uint16_t* explicit_commandline,
    size_t explicit_commandline_len,
    uint8_t* out_candidate);

terminal_settings_ffi_status terminal_settings_ffi_commandline_fixup_plan(
    const uint16_t* guid,
    size_t guid_len,
    const uint16_t* explicit_commandline,
    size_t explicit_commandline_len,
    const uint16_t* inherited_after_clear,
    size_t inherited_after_clear_len,
    uint32_t* out_plan);

#ifdef __cplusplus
}

static_assert(TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_NOOP == 0);
static_assert(TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_REMOVE == 1);
static_assert(TERMINAL_SETTINGS_FFI_WORKSPACE_RENAME_RENAME == 2);
static_assert(TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_NONE == 0);
static_assert(TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_CLEAR_OVERRIDE == 1);
static_assert(TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_RESTORE_CMD_FULL_PATH == 2);
static_assert(TERMINAL_SETTINGS_FFI_COMMANDLINE_FIXUP_RESTORE_POWERSHELL_FULL_PATH == 3);
#endif
