use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiPopSgrKind {
    None = 0,
    Pop = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputCsiPopSgrPlan {
    pub kind: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

impl Default for OutputCsiPopSgrPlan {
    fn default() -> Self {
        Self {
            kind: OutputCsiPopSgrKind::None as u32,
            reserved0: 0,
            reserved1: 0,
            reserved2: 0,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiPopSgrPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan = match action {
            OutputAction::PopGraphicsRendition => OutputCsiPopSgrPlan {
                kind: OutputCsiPopSgrKind::Pop as u32,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            },
            _ => OutputCsiPopSgrPlan::default(),
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

/// Replays CSI SGR-stack pop classification through the existing Rust output
/// engine. C++ remains the native dispatch owner until this plan is replayed
/// through the product ABI and independently promoted.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_pop_sgr_plan(
    identifier: u64,
    out_plan: *mut OutputCsiPopSgrPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let parameters = Parameters::default();
        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        let _ = engine.action_csi_dispatch(id, &parameters);
        let dispatch = engine.into_dispatch();

        // SAFETY: `out_plan` was checked non-null above and the ABI requires
        // one writable `OutputCsiPopSgrPlan` for this call.
        unsafe { ptr::write(out_plan, dispatch.plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        OutputCsiPopSgrKind, OutputCsiPopSgrPlan, terminal_parser_ffi_output_csi_pop_sgr_plan,
    };
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn expect(id: &str, kind: OutputCsiPopSgrKind) {
        let mut result = OutputCsiPopSgrPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_pop_sgr_plan(VtId::from_ascii(id).value(), &mut result),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, kind as u32, "id={id:?}");
    }

    #[test]
    fn csi_pop_sgr_ffi_replays_microsoft_contract_and_alias() {
        expect("#}", OutputCsiPopSgrKind::Pop);
        expect("#q", OutputCsiPopSgrKind::Pop);
        expect("m", OutputCsiPopSgrKind::None);
    }

    #[test]
    fn csi_pop_sgr_ffi_validates_pointer_and_identifier() {
        assert_eq!(
            terminal_parser_ffi_output_csi_pop_sgr_plan(0, std::ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
        let mut result = OutputCsiPopSgrPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_pop_sgr_plan(
                0xff00_0000_0000_0000,
                &mut result,
            ),
            FfiStatus::InvalidArgument
        );
    }
}
