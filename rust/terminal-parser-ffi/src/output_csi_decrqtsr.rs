use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiDecrqtsrKind {
    None = 0,
    RequestTerminalState = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputCsiDecrqtsrPlan {
    pub kind: u32,
    pub format: i32,
    /// Raw VT parameter value. `-1` preserves an omitted option across the ABI.
    pub format_option: i32,
    pub reserved0: u32,
}

impl Default for OutputCsiDecrqtsrPlan {
    fn default() -> Self {
        Self {
            kind: OutputCsiDecrqtsrKind::None as u32,
            format: 0,
            format_option: -1,
            reserved0: 0,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiDecrqtsrPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan = match action {
            OutputAction::AdvancedCsi { id, parameters } if id == VtId::from_ascii("$u") => {
                OutputCsiDecrqtsrPlan {
                    kind: OutputCsiDecrqtsrKind::RequestTerminalState as u32,
                    format: parameters.at(0).unwrap_or(0),
                    format_option: parameters.at(1).unwrap_or(-1),
                    reserved0: 0,
                }
            }
            _ => OutputCsiDecrqtsrPlan::default(),
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

fn raw_parameter(value: i32) -> Option<i32> {
    (value >= 0).then_some(value)
}

/// Replays DECRQTSR classification while preserving the distinction between
/// an omitted format option and an explicit zero. C++ retains native enum and
/// VTParameter materialization only.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_decrqtsr_plan(
    identifier: u64,
    parameter0_raw: i32,
    parameter1_raw: i32,
    out_plan: *mut OutputCsiDecrqtsrPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let parameters = Parameters::from_values(vec![
            raw_parameter(parameter0_raw),
            raw_parameter(parameter1_raw),
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
    use super::{
        OutputCsiDecrqtsrKind, OutputCsiDecrqtsrPlan,
        terminal_parser_ffi_output_csi_decrqtsr_plan,
    };
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn expect(
        id: &str,
        raw: [i32; 2],
        kind: OutputCsiDecrqtsrKind,
        format: i32,
        format_option: i32,
    ) {
        let mut result = OutputCsiDecrqtsrPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decrqtsr_plan(
                VtId::from_ascii(id).value(),
                raw[0],
                raw[1],
                &mut result,
            ),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, kind as u32, "id={id:?}");
        assert_eq!(result.format, format, "id={id:?}");
        assert_eq!(result.format_option, format_option, "id={id:?}");
        assert_eq!(result.reserved0, 0);
    }

    #[test]
    fn csi_decrqtsr_ffi_replays_microsoft_contract_without_laundering_omission() {
        expect(
            "$u",
            [-1, -1],
            OutputCsiDecrqtsrKind::RequestTerminalState,
            0,
            -1,
        );
        expect(
            "$u",
            [1, -1],
            OutputCsiDecrqtsrKind::RequestTerminalState,
            1,
            -1,
        );
        expect(
            "$u",
            [2, 0],
            OutputCsiDecrqtsrKind::RequestTerminalState,
            2,
            0,
        );
        expect(
            "$u",
            [2, 7],
            OutputCsiDecrqtsrKind::RequestTerminalState,
            2,
            7,
        );
        expect(
            "$w",
            [1, -1],
            OutputCsiDecrqtsrKind::None,
            0,
            -1,
        );
    }

    #[test]
    fn csi_decrqtsr_ffi_validates_pointer_and_identifier() {
        assert_eq!(
            terminal_parser_ffi_output_csi_decrqtsr_plan(
                VtId::from_ascii("$u").value(),
                -1,
                -1,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );

        let mut result = OutputCsiDecrqtsrPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decrqtsr_plan(
                0xff00_0000_0000_0000,
                -1,
                -1,
                &mut result,
            ),
            FfiStatus::InvalidArgument
        );
    }
}
