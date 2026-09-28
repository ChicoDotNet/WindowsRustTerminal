use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiRectAttributesKind {
    None = 0,
    Change = 1,
    Reverse = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OutputCsiRectAttributesPlan {
    pub kind: u32,
    pub bottom: i32,
    pub right: i32,
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiRectAttributesPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        if let OutputAction::AdvancedCsi { id, parameters } = action {
            let kind = if id == VtId::from_ascii("$r") {
                OutputCsiRectAttributesKind::Change
            } else if id == VtId::from_ascii("$t") {
                OutputCsiRectAttributesKind::Reverse
            } else {
                OutputCsiRectAttributesKind::None
            };

            if kind != OutputCsiRectAttributesKind::None {
                self.plan = OutputCsiRectAttributesPlan {
                    kind: kind as u32,
                    bottom: parameters.at(2).unwrap_or(0),
                    right: parameters.at(3).unwrap_or(0),
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

/// Replays DECCARA/DECRARA classification and the two scalar defaults that are
/// portable across the native dispatch seam. The leading VTParameter values
/// and the trailing SGR parameter span intentionally remain C++-owned because
/// preserving their native representation is part of the Windows parser seam.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_rect_attributes_plan(
    identifier: u64,
    bottom: i32,
    right: i32,
    out_plan: *mut OutputCsiRectAttributesPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let parameters = Parameters::from_values(vec![None, None, Some(bottom), Some(right)]);
        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        let _ = engine.action_csi_dispatch(id, &parameters);
        let plan = engine.into_dispatch().plan;

        // SAFETY: `out_plan` was checked non-null above and the ABI requires
        // one writable plan value for the duration of this call.
        unsafe { std::ptr::write(out_plan, plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        OutputCsiRectAttributesKind, OutputCsiRectAttributesPlan,
        terminal_parser_ffi_output_csi_rect_attributes_plan,
    };
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn plan(id: &str, bottom: i32, right: i32) -> OutputCsiRectAttributesPlan {
        let mut plan = OutputCsiRectAttributesPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_rect_attributes_plan(
                VtId::from_ascii(id).value(),
                bottom,
                right,
                &mut plan,
            ),
            FfiStatus::Ok
        );
        plan
    }

    #[test]
    fn rect_attribute_ffi_replays_both_actions_and_scalar_defaults() {
        assert_eq!(
            plan("$r", 0, 0),
            OutputCsiRectAttributesPlan {
                kind: OutputCsiRectAttributesKind::Change as u32,
                bottom: 0,
                right: 0,
            }
        );
        assert_eq!(
            plan("$t", 17, 23),
            OutputCsiRectAttributesPlan {
                kind: OutputCsiRectAttributesKind::Reverse as u32,
                bottom: 17,
                right: 23,
            }
        );
        assert_eq!(plan("$u", 17, 23), OutputCsiRectAttributesPlan::default());
    }

    #[test]
    fn rect_attribute_ffi_rejects_invalid_identifier_and_null_output() {
        let mut plan = OutputCsiRectAttributesPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_rect_attributes_plan(
                0xff00_0000_0000_0000,
                0,
                0,
                &mut plan,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_output_csi_rect_attributes_plan(
                VtId::from_ascii("$r").value(),
                0,
                0,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );
    }
}
