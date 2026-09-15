use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiRepKind {
    None = 0,
    Repeat = 1,
    HandledNoop = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OutputCsiRepPlan {
    pub kind: u32,
    pub count: i32,
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiRepPlan,
    emitted_action: bool,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.emitted_action = true;
        if let OutputAction::PrintString(text) = action {
            if !text.is_empty() {
                self.plan = OutputCsiRepPlan {
                    kind: OutputCsiRepKind::Repeat as u32,
                    count: i32::try_from(text.len()).unwrap_or(i32::MAX),
                };
            }
        }
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

/// Replays REP through the Rust output engine while keeping the product's
/// native `_lastPrintedChar` storage on the C++ side of the seam. The supplied
/// character seeds the Rust engine only for contract replay; Rust owns REP
/// recognition and numeric defaulting, while C++ can continue materializing
/// the final native string until product routing is independently certified.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_rep_plan(
    identifier: u64,
    parameter0: i32,
    last_printed_char: u16,
    out_plan: *mut OutputCsiRepPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        if last_printed_char != 0 {
            let _ = engine.action_print(last_printed_char);
        }
        let parameters = Parameters::from_values(vec![Some(parameter0)]);
        let _ = engine.action_csi_dispatch(id, &parameters);
        let mut dispatch = engine.into_dispatch();
        if !dispatch.emitted_action {
            dispatch.plan = OutputCsiRepPlan {
                kind: OutputCsiRepKind::HandledNoop as u32,
                count: 0,
            };
        }

        // SAFETY: `out_plan` was checked non-null above and the ABI requires
        // one writable plan value for the duration of this call.
        unsafe { ptr::write(out_plan, dispatch.plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{OutputCsiRepKind, OutputCsiRepPlan, terminal_parser_ffi_output_csi_rep_plan};
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn plan(id: &str, parameter0: i32, last_printed_char: u16) -> OutputCsiRepPlan {
        let mut plan = OutputCsiRepPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_rep_plan(
                VtId::from_ascii(id).value(),
                parameter0,
                last_printed_char,
                &mut plan,
            ),
            FfiStatus::Ok
        );
        plan
    }

    #[test]
    fn rep_ffi_replays_microsoft_numeric_defaults_and_last_character_contract() {
        assert_eq!(
            plan("b", 0, u16::from(b'A')),
            OutputCsiRepPlan {
                kind: OutputCsiRepKind::Repeat as u32,
                count: 1,
            }
        );
        assert_eq!(
            plan("b", 4, u16::from(b'Z')),
            OutputCsiRepPlan {
                kind: OutputCsiRepKind::Repeat as u32,
                count: 4,
            }
        );
        assert_eq!(
            plan("b", 7, 0),
            OutputCsiRepPlan {
                kind: OutputCsiRepKind::HandledNoop as u32,
                count: 0,
            }
        );
        assert_eq!(plan("X", 4, u16::from(b'Z')), OutputCsiRepPlan::default());
    }

    #[test]
    fn rep_ffi_rejects_invalid_identifier_and_null_output() {
        let mut plan = OutputCsiRepPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_rep_plan(
                0xff00_0000_0000_0000,
                0,
                u16::from(b'A'),
                &mut plan,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_output_csi_rep_plan(
                VtId::from_ascii("b").value(),
                0,
                u16::from(b'A'),
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );
    }
}
