use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiDecinvmKind {
    None = 0,
    InvokeMacro = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputCsiDecinvmPlan {
    pub kind: u32,
    pub macro_id: i32,
    pub reserved0: u32,
    pub reserved1: u32,
}

impl Default for OutputCsiDecinvmPlan {
    fn default() -> Self {
        Self {
            kind: OutputCsiDecinvmKind::None as u32,
            macro_id: 0,
            reserved0: 0,
            reserved1: 0,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiDecinvmPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan = match action {
            OutputAction::AdvancedCsi { id, parameters } if id == VtId::from_ascii("*z") => {
                OutputCsiDecinvmPlan {
                    kind: OutputCsiDecinvmKind::InvokeMacro as u32,
                    macro_id: parameters.at(0).unwrap_or(0),
                    reserved0: 0,
                    reserved1: 0,
                }
            }
            _ => OutputCsiDecinvmPlan::default(),
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

/// Replays DECINVM classification and the effective macro identifier through
/// the Rust output engine. C++ retains only native dispatch materialization.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_decinvm_plan(
    identifier: u64,
    parameter0: i32,
    out_plan: *mut OutputCsiDecinvmPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let parameters = Parameters::from_values(vec![Some(parameter0)]);
        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        let _ = engine.action_csi_dispatch(id, &parameters);
        let dispatch = engine.into_dispatch();

        // SAFETY: `out_plan` was checked non-null above and the ABI requires
        // one writable plan for this call.
        unsafe { ptr::write(out_plan, dispatch.plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        OutputCsiDecinvmKind, OutputCsiDecinvmPlan,
        terminal_parser_ffi_output_csi_decinvm_plan,
    };
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn expect(id: &str, parameter0: i32, kind: OutputCsiDecinvmKind, macro_id: i32) {
        let mut result = OutputCsiDecinvmPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decinvm_plan(
                VtId::from_ascii(id).value(),
                parameter0,
                &mut result,
            ),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, kind as u32, "id={id:?}");
        assert_eq!(result.macro_id, macro_id, "id={id:?}");
        assert_eq!(result.reserved0, 0);
        assert_eq!(result.reserved1, 0);
    }

    #[test]
    fn csi_decinvm_ffi_replays_microsoft_contract() {
        expect("*z", 0, OutputCsiDecinvmKind::InvokeMacro, 0);
        expect("*z", 1, OutputCsiDecinvmKind::InvokeMacro, 1);
        expect("*z", 63, OutputCsiDecinvmKind::InvokeMacro, 63);
        expect("*y", 1, OutputCsiDecinvmKind::None, 0);
    }

    #[test]
    fn csi_decinvm_ffi_validates_pointer_and_identifier() {
        assert_eq!(
            terminal_parser_ffi_output_csi_decinvm_plan(
                VtId::from_ascii("*z").value(),
                0,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );

        let mut result = OutputCsiDecinvmPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decinvm_plan(
                0xff00_0000_0000_0000,
                0,
                &mut result,
            ),
            FfiStatus::InvalidArgument
        );
    }
}
