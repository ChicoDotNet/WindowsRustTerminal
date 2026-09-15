use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiDecacKind {
    None = 0,
    AssignColor = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputCsiDecacPlan {
    pub kind: u32,
    pub item: i32,
    pub foreground_index: i32,
    pub background_index: i32,
}

impl Default for OutputCsiDecacPlan {
    fn default() -> Self {
        Self {
            kind: OutputCsiDecacKind::None as u32,
            item: 0,
            foreground_index: 0,
            background_index: 0,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiDecacPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan = match action {
            OutputAction::AdvancedCsi { id, parameters } if id == VtId::from_ascii(",|") => {
                OutputCsiDecacPlan {
                    kind: OutputCsiDecacKind::AssignColor as u32,
                    item: parameters.at(0).unwrap_or(0),
                    foreground_index: parameters.at(1).unwrap_or(0),
                    background_index: parameters.at(2).unwrap_or(0),
                }
            }
            _ => OutputCsiDecacPlan::default(),
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

/// Replays DECAC classification and its effective color assignment through
/// the Rust output engine. C++ retains only native enum/dispatch materialization.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_decac_plan(
    identifier: u64,
    parameter0: i32,
    parameter1: i32,
    parameter2: i32,
    out_plan: *mut OutputCsiDecacPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let parameters = Parameters::from_values(vec![
            Some(parameter0),
            Some(parameter1),
            Some(parameter2),
        ]);
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
    use super::{OutputCsiDecacKind, OutputCsiDecacPlan, terminal_parser_ffi_output_csi_decac_plan};
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn expect(
        id: &str,
        parameters: [i32; 3],
        kind: OutputCsiDecacKind,
        expected: [i32; 3],
    ) {
        let mut result = OutputCsiDecacPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decac_plan(
                VtId::from_ascii(id).value(),
                parameters[0],
                parameters[1],
                parameters[2],
                &mut result,
            ),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, kind as u32, "id={id:?}");
        assert_eq!(result.item, expected[0], "id={id:?}");
        assert_eq!(result.foreground_index, expected[1], "id={id:?}");
        assert_eq!(result.background_index, expected[2], "id={id:?}");
    }

    #[test]
    fn csi_decac_ffi_replays_microsoft_contract() {
        expect(",|", [0, 0, 0], OutputCsiDecacKind::AssignColor, [0, 0, 0]);
        expect(",|", [1, 7, 4], OutputCsiDecacKind::AssignColor, [1, 7, 4]);
        expect(",|", [2, 15, 0], OutputCsiDecacKind::AssignColor, [2, 15, 0]);
        expect(",~", [1, 7, 4], OutputCsiDecacKind::None, [0, 0, 0]);
    }

    #[test]
    fn csi_decac_ffi_validates_pointer_and_identifier() {
        assert_eq!(
            terminal_parser_ffi_output_csi_decac_plan(
                VtId::from_ascii(",|").value(),
                0,
                0,
                0,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );

        let mut result = OutputCsiDecacPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decac_plan(
                0xff00_0000_0000_0000,
                0,
                0,
                0,
                &mut result,
            ),
            FfiStatus::InvalidArgument
        );
    }
}
