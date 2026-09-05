use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiModeKind {
    None = 0,
    Mode = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputCsiModePlan {
    pub kind: u32,
    pub private_mode: u32,
    pub enabled: u32,
    pub mode: i32,
}

impl Default for OutputCsiModePlan {
    fn default() -> Self {
        Self {
            kind: OutputCsiModeKind::None as u32,
            private_mode: 0,
            enabled: 0,
            mode: 0,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiModePlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan = match action {
            OutputAction::SetMode {
                private,
                enabled,
                mode,
            } => OutputCsiModePlan {
                kind: OutputCsiModeKind::Mode as u32,
                private_mode: u32::from(private),
                enabled: u32::from(enabled),
                mode,
            },
            _ => OutputCsiModePlan::default(),
        };
    }
}

fn vt_id_from_value(identifier: u64) -> Option<VtId> {
    if identifier & 0xff00_0000_0000_0000 != 0 {
        return None;
    }

    let bytes = identifier.to_le_bytes();
    let length = bytes[..7]
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(7);
    if bytes[length..7].iter().any(|byte| *byte != 0) || !bytes[..length].is_ascii() {
        return None;
    }
    let text = std::str::from_utf8(&bytes[..length]).ok()?;
    Some(VtId::from_ascii(text))
}

/// Replays one ANSI/DEC CSI set/reset-mode parameter through the Rust output
/// engine. This is bridge evidence only: the product still owns iteration over
/// multiple parameters until that contract is independently verified.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_mode_plan(
    identifier: u64,
    mode: i32,
    out_plan: *mut OutputCsiModePlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let parameters = Parameters::from_values(vec![Some(mode)]);
        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        let _ = engine.action_csi_dispatch(id, &parameters);
        let dispatch = engine.into_dispatch();

        // SAFETY: `out_plan` was checked non-null above and the ABI requires one
        // writable `OutputCsiModePlan` for this call.
        unsafe { ptr::write(out_plan, dispatch.plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{OutputCsiModeKind, OutputCsiModePlan, terminal_parser_ffi_output_csi_mode_plan};
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn expect(id: &str, mode: i32, private_mode: u32, enabled: u32) {
        let mut result = OutputCsiModePlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_mode_plan(VtId::from_ascii(id).value(), mode, &mut result),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, OutputCsiModeKind::Mode as u32, "id={id:?}");
        assert_eq!(result.private_mode, private_mode, "id={id:?}");
        assert_eq!(result.enabled, enabled, "id={id:?}");
        assert_eq!(result.mode, mode, "id={id:?}");
    }

    #[test]
    fn csi_mode_ffi_replays_set_and_reset_contracts() {
        expect("h", 4, 0, 1);
        expect("?h", 25, 1, 1);
        expect("l", 4, 0, 0);
        expect("?l", 25, 1, 0);

        let mut unrelated = OutputCsiModePlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_mode_plan(
                VtId::from_ascii("m").value(),
                3,
                &mut unrelated,
            ),
            FfiStatus::Ok
        );
        assert_eq!(unrelated.kind, OutputCsiModeKind::None as u32);
    }

    #[test]
    fn csi_mode_ffi_validates_pointer_and_identifier() {
        assert_eq!(
            terminal_parser_ffi_output_csi_mode_plan(0, 0, std::ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
        let mut result = OutputCsiModePlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_mode_plan(
                0xff00_0000_0000_0000,
                0,
                &mut result,
            ),
            FfiStatus::InvalidArgument
        );
    }
}
