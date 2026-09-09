use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiDecrqcraKind {
    None = 0,
    RequestChecksumRectangularArea = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OutputCsiDecrqcraPlan {
    pub kind: u32,
    pub request_id: i32,
    pub page: i32,
    pub bottom: i32,
    pub right: i32,
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiDecrqcraPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan = match action {
            OutputAction::AdvancedCsi { id, parameters } if id == VtId::from_ascii("*y") => {
                OutputCsiDecrqcraPlan {
                    kind: OutputCsiDecrqcraKind::RequestChecksumRectangularArea as u32,
                    request_id: parameters.at(0).unwrap_or(0),
                    page: parameters.at(1).unwrap_or(0),
                    bottom: parameters.at(4).unwrap_or(0),
                    right: parameters.at(5).unwrap_or(0),
                }
            }
            _ => OutputCsiDecrqcraPlan::default(),
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

/// Replays DECRQCRA classification plus the four scalar defaults that are
/// portable across the native dispatch seam. The top and left VTParameter
/// values intentionally remain C++-owned so their native representation is
/// preserved for ITermDispatch::RequestChecksumRectangularArea.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_decrqcra_plan(
    identifier: u64,
    request_id: i32,
    page: i32,
    bottom: i32,
    right: i32,
    out_plan: *mut OutputCsiDecrqcraPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let parameters = Parameters::from_values(vec![
            Some(request_id),
            Some(page),
            None,
            None,
            Some(bottom),
            Some(right),
        ]);
        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        let _ = engine.action_csi_dispatch(id, &parameters);
        let plan = engine.into_dispatch().plan;

        // SAFETY: `out_plan` was checked non-null above and the ABI requires
        // one writable plan value for the duration of this call.
        unsafe { ptr::write(out_plan, plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        OutputCsiDecrqcraKind, OutputCsiDecrqcraPlan,
        terminal_parser_ffi_output_csi_decrqcra_plan,
    };
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn plan(
        id: &str,
        request_id: i32,
        page: i32,
        bottom: i32,
        right: i32,
    ) -> OutputCsiDecrqcraPlan {
        let mut plan = OutputCsiDecrqcraPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decrqcra_plan(
                VtId::from_ascii(id).value(),
                request_id,
                page,
                bottom,
                right,
                &mut plan,
            ),
            FfiStatus::Ok
        );
        plan
    }

    #[test]
    fn decrqcra_ffi_replays_microsoft_contract_and_scalar_defaults() {
        assert_eq!(
            plan("*y", 0, 0, 0, 0),
            OutputCsiDecrqcraPlan {
                kind: OutputCsiDecrqcraKind::RequestChecksumRectangularArea as u32,
                request_id: 0,
                page: 0,
                bottom: 0,
                right: 0,
            }
        );
        assert_eq!(
            plan("*y", 7, 2, 17, 23),
            OutputCsiDecrqcraPlan {
                kind: OutputCsiDecrqcraKind::RequestChecksumRectangularArea as u32,
                request_id: 7,
                page: 2,
                bottom: 17,
                right: 23,
            }
        );
        assert_eq!(plan("*x", 7, 2, 17, 23), OutputCsiDecrqcraPlan::default());
    }

    #[test]
    fn decrqcra_ffi_rejects_invalid_identifier_and_null_output() {
        let mut plan = OutputCsiDecrqcraPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decrqcra_plan(
                0xff00_0000_0000_0000,
                0,
                0,
                0,
                0,
                &mut plan,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_output_csi_decrqcra_plan(
                VtId::from_ascii("*y").value(),
                0,
                0,
                0,
                0,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );
    }
}
