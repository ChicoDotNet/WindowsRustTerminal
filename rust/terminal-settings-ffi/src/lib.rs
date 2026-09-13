//! Narrow C ABI for portable Terminal Settings behavior owned by Rust.
//!
//! The Windows/C++ layer retains WinRT storage, locking, persistence, and UI
//! ownership. This crate exposes only deterministic settings decisions that
//! already have parity evidence in `terminal-settings`.

#![deny(unsafe_op_in_unsafe_fn)]

use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    ptr,
};

use terminal_settings::{
    application_state::{WorkspaceRenamePlan, workspace_rename_plan},
    settings_fixup::{CommandlineFixupPlan, commandline_fixup_plan},
};

/// Stable status values returned across the C ABI.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiStatus {
    Ok = 0,
    InvalidArgument = 1,
    Panic = 255,
}

/// Stable workspace rename decisions returned to the native settings seam.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceRenamePlanFfi {
    Noop = 0,
    Remove = 1,
    Rename = 2,
}

/// Stable commandline-fixup decisions returned to the native settings seam.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandlineFixupPlanFfi {
    None = 0,
    ClearOverride = 1,
    RestoreCmdFullPath = 2,
    RestorePowershellFullPath = 3,
}

pub const ABI_VERSION: u32 = 1;

fn ffi_guard(operation: impl FnOnce() -> FfiStatus) -> FfiStatus {
    catch_unwind(AssertUnwindSafe(operation)).unwrap_or(FfiStatus::Panic)
}

fn ffi_bool(value: u8) -> Result<bool, FfiStatus> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(FfiStatus::InvalidArgument),
    }
}

fn ffi_utf16(value: *const u16, len: usize) -> Result<String, FfiStatus> {
    if len == 0 {
        return Ok(String::new());
    }
    if value.is_null() {
        return Err(FfiStatus::InvalidArgument);
    }

    // SAFETY: the ABI requires `value` to reference `len` readable UTF-16 code
    // units for the duration of this call. The pointer was checked non-null.
    let units = unsafe { std::slice::from_raw_parts(value, len) };
    String::from_utf16(units).map_err(|_| FfiStatus::InvalidArgument)
}

impl From<WorkspaceRenamePlan> for WorkspaceRenamePlanFfi {
    fn from(value: WorkspaceRenamePlan) -> Self {
        match value {
            WorkspaceRenamePlan::Noop => Self::Noop,
            WorkspaceRenamePlan::Remove => Self::Remove,
            WorkspaceRenamePlan::Rename => Self::Rename,
        }
    }
}

impl From<CommandlineFixupPlan> for CommandlineFixupPlanFfi {
    fn from(value: CommandlineFixupPlan) -> Self {
        match value {
            CommandlineFixupPlan::None => Self::None,
            CommandlineFixupPlan::ClearOverride => Self::ClearOverride,
            CommandlineFixupPlan::RestoreCmdFullPath => Self::RestoreCmdFullPath,
            CommandlineFixupPlan::RestorePowershellFullPath => Self::RestorePowershellFullPath,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_settings_ffi_abi_version() -> u32 {
    ABI_VERSION
}

/// Decides the mutation required to rename one persisted workspace.
///
/// String comparison and map lookup remain in native code so no WinRT or
/// UTF-16 representation crosses this ABI. The caller supplies only the four
/// facts required by the portable policy and owns the output storage.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_settings_ffi_workspace_rename_plan(
    old_name_empty: u8,
    names_equal: u8,
    old_exists: u8,
    new_name_empty: u8,
    out_plan: *mut u32,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }

        let old_name_empty = match ffi_bool(old_name_empty) {
            Ok(value) => value,
            Err(status) => return status,
        };
        let names_equal = match ffi_bool(names_equal) {
            Ok(value) => value,
            Err(status) => return status,
        };
        let old_exists = match ffi_bool(old_exists) {
            Ok(value) => value,
            Err(status) => return status,
        };
        let new_name_empty = match ffi_bool(new_name_empty) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let plan = WorkspaceRenamePlanFfi::from(workspace_rename_plan(
            old_name_empty,
            names_equal,
            old_exists,
            new_name_empty,
        ));

        // SAFETY: `out_plan` was checked non-null and the ABI requires it to
        // reference one writable u32 for the duration of this call.
        unsafe { ptr::write(out_plan, plan as u32) };
        FfiStatus::Ok
    })
}

/// Reports whether a profile is eligible for Microsoft's built-in legacy-shell
/// commandline fixup without mutating the native inheritance graph.
///
/// Rust owns the GUID/short-commandline policy. Native code must call this
/// before `ClearCommandline()` so non-candidates are never touched merely to
/// discover inherited state.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_settings_ffi_commandline_fixup_candidate(
    guid: *const u16,
    guid_len: usize,
    explicit_commandline: *const u16,
    explicit_commandline_len: usize,
    out_candidate: *mut u8,
) -> FfiStatus {
    ffi_guard(|| {
        if out_candidate.is_null() {
            return FfiStatus::InvalidArgument;
        }

        let guid = match ffi_utf16(guid, guid_len) {
            Ok(value) => value,
            Err(status) => return status,
        };
        let explicit_commandline = match ffi_utf16(explicit_commandline, explicit_commandline_len) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let candidate = !matches!(
            commandline_fixup_plan(&guid, Some(&explicit_commandline), None),
            CommandlineFixupPlan::None
        );

        // SAFETY: `out_candidate` was checked non-null and the ABI requires it
        // to reference one writable byte for the duration of this call.
        unsafe { ptr::write(out_candidate, u8::from(candidate)) };
        FfiStatus::Ok
    })
}

/// Decides the final mutation after native code has cleared a confirmed
/// candidate's explicit commandline and observed the resulting inheritance.
///
/// All strings are borrowed UTF-16 slices. No WinRT representation or owned
/// allocation crosses this ABI.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_settings_ffi_commandline_fixup_plan(
    guid: *const u16,
    guid_len: usize,
    explicit_commandline: *const u16,
    explicit_commandline_len: usize,
    inherited_after_clear: *const u16,
    inherited_after_clear_len: usize,
    out_plan: *mut u32,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }

        let guid = match ffi_utf16(guid, guid_len) {
            Ok(value) => value,
            Err(status) => return status,
        };
        let explicit_commandline = match ffi_utf16(explicit_commandline, explicit_commandline_len) {
            Ok(value) => value,
            Err(status) => return status,
        };
        let inherited_after_clear = match ffi_utf16(inherited_after_clear, inherited_after_clear_len) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let plan = CommandlineFixupPlanFfi::from(commandline_fixup_plan(
            &guid,
            Some(&explicit_commandline),
            Some(&inherited_after_clear),
        ));

        // SAFETY: `out_plan` was checked non-null and the ABI requires it to
        // reference one writable u32 for the duration of this call.
        unsafe { ptr::write(out_plan, plan as u32) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        CommandlineFixupPlanFfi, FfiStatus, WorkspaceRenamePlanFfi,
        terminal_settings_ffi_abi_version, terminal_settings_ffi_commandline_fixup_candidate,
        terminal_settings_ffi_commandline_fixup_plan, terminal_settings_ffi_workspace_rename_plan,
    };

    fn plan(old_empty: u8, equal: u8, exists: u8, new_empty: u8) -> (FfiStatus, u32) {
        let mut out = u32::MAX;
        let status = terminal_settings_ffi_workspace_rename_plan(
            old_empty, equal, exists, new_empty, &mut out,
        );
        (status, out)
    }

    fn utf16(value: &str) -> Vec<u16> {
        value.encode_utf16().collect()
    }

    fn commandline_candidate(guid: &str, explicit: &str) -> (FfiStatus, u8) {
        let guid = utf16(guid);
        let explicit = utf16(explicit);
        let mut out = u8::MAX;
        let status = terminal_settings_ffi_commandline_fixup_candidate(
            guid.as_ptr(),
            guid.len(),
            explicit.as_ptr(),
            explicit.len(),
            &mut out,
        );
        (status, out)
    }

    fn commandline_plan(guid: &str, explicit: &str, inherited: &str) -> (FfiStatus, u32) {
        let guid = utf16(guid);
        let explicit = utf16(explicit);
        let inherited = utf16(inherited);
        let mut out = u32::MAX;
        let status = terminal_settings_ffi_commandline_fixup_plan(
            guid.as_ptr(),
            guid.len(),
            explicit.as_ptr(),
            explicit.len(),
            inherited.as_ptr(),
            inherited.len(),
            &mut out,
        );
        (status, out)
    }

    #[test]
    fn abi_version_is_stable() {
        assert_eq!(terminal_settings_ffi_abi_version(), 1);
    }

    #[test]
    fn workspace_rename_contract_replays_native_decisions() {
        assert_eq!(
            plan(1, 0, 1, 0),
            (FfiStatus::Ok, WorkspaceRenamePlanFfi::Noop as u32)
        );
        assert_eq!(
            plan(0, 1, 1, 0),
            (FfiStatus::Ok, WorkspaceRenamePlanFfi::Noop as u32)
        );
        assert_eq!(
            plan(0, 0, 0, 0),
            (FfiStatus::Ok, WorkspaceRenamePlanFfi::Noop as u32)
        );
        assert_eq!(
            plan(0, 0, 1, 1),
            (FfiStatus::Ok, WorkspaceRenamePlanFfi::Remove as u32)
        );
        assert_eq!(
            plan(0, 0, 1, 0),
            (FfiStatus::Ok, WorkspaceRenamePlanFfi::Rename as u32)
        );
    }

    #[test]
    fn workspace_rename_rejects_invalid_abi_inputs_fail_closed() {
        let invalid_inputs = [
            (2, 0, 1, 0),
            (0, 2, 1, 0),
            (0, 0, 2, 0),
            (0, 0, 1, 2),
        ];

        for (old_empty, equal, exists, new_empty) in invalid_inputs {
            let mut out = u32::MAX;
            assert_eq!(
                terminal_settings_ffi_workspace_rename_plan(
                    old_empty, equal, exists, new_empty, &mut out,
                ),
                FfiStatus::InvalidArgument
            );
            assert_eq!(out, u32::MAX);
        }

        assert_eq!(
            terminal_settings_ffi_workspace_rename_plan(0, 0, 1, 0, std::ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
    }

    #[test]
    fn commandline_fixup_candidate_replays_native_entry_policy() {
        const CMD_GUID: &str = "{0CAA0DAD-35BE-5F56-A8FF-AFCEEEAA6101}";
        const POWERSHELL_GUID: &str = "{61c54bbd-c2c6-5271-96e7-009a87ff44bf}";

        assert_eq!(commandline_candidate(CMD_GUID, "CMD.EXE"), (FfiStatus::Ok, 1));
        assert_eq!(
            commandline_candidate(POWERSHELL_GUID, "PowerShell.exe"),
            (FfiStatus::Ok, 1)
        );
        assert_eq!(
            commandline_candidate(CMD_GUID, "%SystemRoot%\\System32\\cmd.exe"),
            (FfiStatus::Ok, 0)
        );
        assert_eq!(
            commandline_candidate(
                "{00000000-0000-0000-0000-000000000000}",
                "cmd.exe",
            ),
            (FfiStatus::Ok, 0)
        );
    }

    #[test]
    fn commandline_fixup_contract_replays_post_clear_decisions() {
        const CMD_GUID: &str = "{0CAA0DAD-35BE-5F56-A8FF-AFCEEEAA6101}";
        const POWERSHELL_GUID: &str = "{61c54bbd-c2c6-5271-96e7-009a87ff44bf}";
        const CMD_FULL_PATH: &str = "%SystemRoot%\\System32\\cmd.exe";
        const POWERSHELL_FULL_PATH: &str =
            "%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe";

        assert_eq!(
            commandline_plan(CMD_GUID, "CMD.EXE", CMD_FULL_PATH),
            (FfiStatus::Ok, CommandlineFixupPlanFfi::ClearOverride as u32)
        );
        assert_eq!(
            commandline_plan(CMD_GUID, "cmd.exe", "%SYSTEMROOT%\\System32\\cmd.exe"),
            (FfiStatus::Ok, CommandlineFixupPlanFfi::RestoreCmdFullPath as u32)
        );
        assert_eq!(
            commandline_plan(POWERSHELL_GUID, "PowerShell.exe", POWERSHELL_FULL_PATH),
            (FfiStatus::Ok, CommandlineFixupPlanFfi::ClearOverride as u32)
        );
        assert_eq!(
            commandline_plan(POWERSHELL_GUID, "powershell.exe", "pwsh.exe"),
            (
                FfiStatus::Ok,
                CommandlineFixupPlanFfi::RestorePowershellFullPath as u32,
            )
        );
    }

    #[test]
    fn commandline_fixup_rejects_invalid_abi_inputs_fail_closed() {
        let mut candidate = u8::MAX;
        assert_eq!(
            terminal_settings_ffi_commandline_fixup_candidate(
                std::ptr::null(),
                1,
                std::ptr::null(),
                0,
                &mut candidate,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(candidate, u8::MAX);

        let invalid_utf16 = [0xd800u16];
        let mut out = u32::MAX;
        assert_eq!(
            terminal_settings_ffi_commandline_fixup_plan(
                invalid_utf16.as_ptr(),
                invalid_utf16.len(),
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
                &mut out,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(out, u32::MAX);

        assert_eq!(
            terminal_settings_ffi_commandline_fixup_plan(
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );
    }
}
