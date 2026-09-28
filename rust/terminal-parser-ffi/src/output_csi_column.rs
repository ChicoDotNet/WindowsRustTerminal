use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiColumnKind {
    None = 0,
    InsertColumn = 1,
    DeleteColumn = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputCsiColumnPlan {
    pub kind: u32,
    pub count: i32,
    pub reserved0: u32,
    pub reserved1: u32,
}

impl Default for OutputCsiColumnPlan {
    fn default() -> Self {
        Self {
            kind: OutputCsiColumnKind::None as u32,
            count: 0,
            reserved0: 0,
            reserved1: 0,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiColumnPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan = match action {
            OutputAction::AdvancedCsi { id, parameters } if id == VtId::from_ascii("'}") => {
                plan(OutputCsiColumnKind::InsertColumn, parameters.at(0).unwrap_or(0))
            }
            OutputAction::AdvancedCsi { id, parameters } if id == VtId::from_ascii("'~") => {
                plan(OutputCsiColumnKind::DeleteColumn, parameters.at(0).unwrap_or(0))
            }
            _ => OutputCsiColumnPlan::default(),
        };
    }
}

const fn plan(kind: OutputCsiColumnKind, count: i32) -> OutputCsiColumnPlan {
    OutputCsiColumnPlan {
        kind: kind as u32,
        count,
        reserved0: 0,
        reserved1: 0,
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

/// Replays DECIC/DECDC classification and their single flat parameter through
/// the Rust output engine. The returned count is intentionally raw: native
/// dispatch remains responsible for any terminal-side default semantics.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_column_plan(
    identifier: u64,
    parameter0: i32,
    out_plan: *mut OutputCsiColumnPlan,
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
        // one writable `OutputCsiColumnPlan` for this call.
        unsafe { ptr::write(out_plan, dispatch.plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        OutputCsiColumnKind, OutputCsiColumnPlan, terminal_parser_ffi_output_csi_column_plan,
    };
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn expect(id: &str, parameter0: i32, kind: OutputCsiColumnKind, count: i32) {
        let mut result = OutputCsiColumnPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_column_plan(
                VtId::from_ascii(id).value(),
                parameter0,
                &mut result,
            ),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, kind as u32, "id={id:?}");
        assert_eq!(result.count, count, "id={id:?}");
    }

    #[test]
    fn csi_column_ffi_replays_insert_and_delete_contracts() {
        expect("'}", 0, OutputCsiColumnKind::InsertColumn, 0);
        expect("'}", 4, OutputCsiColumnKind::InsertColumn, 4);
        expect("'~", 0, OutputCsiColumnKind::DeleteColumn, 0);
        expect("'~", 7, OutputCsiColumnKind::DeleteColumn, 7);
        expect("$x", 3, OutputCsiColumnKind::None, 0);
    }

    #[test]
    fn csi_column_ffi_validates_pointer_and_identifier() {
        assert_eq!(
            terminal_parser_ffi_output_csi_column_plan(0, 0, std::ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
        let mut result = OutputCsiColumnPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_column_plan(
                0xff00_0000_0000_0000,
                0,
                &mut result,
            ),
            FfiStatus::InvalidArgument
        );
    }
}
