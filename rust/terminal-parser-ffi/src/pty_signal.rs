use std::ptr;

use terminal_host::pty_signal::{PtySignal, plan_signal};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtySignalKind {
    ShowHideWindow = 1,
    ClearBuffer = 2,
    SetParent = 3,
    ResizeWindow = 8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PtySignalReadPlan {
    pub kind: u32,
    pub payload_len: u32,
}

/// Returns the deterministic read plan for one native ConPTY signal ID.
///
/// Pipe ownership and `ReadFile` remain native. Rust owns the wire
/// discriminator and exact payload length so the product seam need not carry a
/// duplicate signal-size table.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_pty_signal_read_plan(
    signal_id: u16,
    out_plan: *mut PtySignalReadPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }

        let Ok(plan) = plan_signal(signal_id.to_le_bytes()) else {
            return FfiStatus::InvalidArgument;
        };
        let kind = match plan.signal {
            PtySignal::ShowHideWindow => PtySignalKind::ShowHideWindow,
            PtySignal::ClearBuffer => PtySignalKind::ClearBuffer,
            PtySignal::SetParent => PtySignalKind::SetParent,
            PtySignal::ResizeWindow => PtySignalKind::ResizeWindow,
        };
        let plan = PtySignalReadPlan {
            kind: kind as u32,
            payload_len: plan.payload_len as u32,
        };

        // SAFETY: `out_plan` was checked non-null above and the ABI requires
        // one writable plan value for the duration of this call.
        unsafe { ptr::write(out_plan, plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{PtySignalKind, PtySignalReadPlan, terminal_parser_ffi_pty_signal_read_plan};
    use crate::FfiStatus;

    fn plan(signal_id: u16) -> PtySignalReadPlan {
        let mut plan = PtySignalReadPlan::default();
        assert_eq!(
            terminal_parser_ffi_pty_signal_read_plan(signal_id, &mut plan),
            FfiStatus::Ok
        );
        plan
    }

    #[test]
    fn ffi_replays_rust_signal_read_owner() {
        assert_eq!(plan(1), PtySignalReadPlan { kind: PtySignalKind::ShowHideWindow as u32, payload_len: 2 });
        assert_eq!(plan(2), PtySignalReadPlan { kind: PtySignalKind::ClearBuffer as u32, payload_len: 2 });
        assert_eq!(plan(3), PtySignalReadPlan { kind: PtySignalKind::SetParent as u32, payload_len: 8 });
        assert_eq!(plan(8), PtySignalReadPlan { kind: PtySignalKind::ResizeWindow as u32, payload_len: 4 });
    }

    #[test]
    fn ffi_rejects_unknown_signal_and_null_output() {
        let mut plan = PtySignalReadPlan::default();
        assert_eq!(terminal_parser_ffi_pty_signal_read_plan(4, &mut plan), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_pty_signal_read_plan(1, std::ptr::null_mut()), FfiStatus::InvalidArgument);
    }
}
