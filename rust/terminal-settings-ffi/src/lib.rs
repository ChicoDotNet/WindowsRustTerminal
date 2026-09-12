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

use terminal_settings::{WorkspaceRenamePlan, workspace_rename_plan};

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

impl From<WorkspaceRenamePlan> for WorkspaceRenamePlanFfi {
    fn from(value: WorkspaceRenamePlan) -> Self {
        match value {
            WorkspaceRenamePlan::Noop => Self::Noop,
            WorkspaceRenamePlan::Remove => Self::Remove,
            WorkspaceRenamePlan::Rename => Self::Rename,
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

#[cfg(test)]
mod tests {
    use super::{
        FfiStatus, WorkspaceRenamePlanFfi, terminal_settings_ffi_abi_version,
        terminal_settings_ffi_workspace_rename_plan,
    };

    fn plan(old_empty: u8, equal: u8, exists: u8, new_empty: u8) -> (FfiStatus, u32) {
        let mut out = u32::MAX;
        let status = terminal_settings_ffi_workspace_rename_plan(
            old_empty, equal, exists, new_empty, &mut out,
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
}
