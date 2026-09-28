use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{ffi_guard, FfiStatus};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiKittyKeyboardSetKind {
    None = 0,
    Set = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputCsiKittyKeyboardSetPlan {
    pub kind: u32,
    pub has_flags: u32,
    pub flags: i32,
    pub has_mode: u32,
    pub mode: i32,
    pub reserved0: u32,
}

impl Default for OutputCsiKittyKeyboardSetPlan {
    fn default() -> Self {
        Self {
            kind: OutputCsiKittyKeyboardSetKind::None as u32,
            has_flags: 0,
            flags: 0,
            has_mode: 0,
            mode: 0,
            reserved0: 0,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputCsiKittyKeyboardSetPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan = match action {
            OutputAction::AdvancedCsi { id, parameters } if id == VtId::from_ascii("=u") => {
                let flags = parameters.at(0);
                let mode = parameters.at(1);
                OutputCsiKittyKeyboardSetPlan {
                    kind: OutputCsiKittyKeyboardSetKind::Set as u32,
                    has_flags: u32::from(flags.is_some()),
                    flags: flags.unwrap_or(0),
                    has_mode: u32::from(mode.is_some()),
                    mode: mode.unwrap_or(0),
                    reserved0: 0,
                }
            }
            _ => OutputCsiKittyKeyboardSetPlan::default(),
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

/// Replays Kitty keyboard set classification and preserves optional flags/mode
/// through the Rust output engine. Native C++ retains terminal-state materialization.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
    identifier: u64,
    has_flags: u32,
    flags: i32,
    has_mode: u32,
    mode: i32,
    out_plan: *mut OutputCsiKittyKeyboardSetPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() || has_flags > 1 || has_mode > 1 {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let parameters = Parameters::from_values(vec![
            (has_flags != 0).then_some(flags),
            (has_mode != 0).then_some(mode),
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
        OutputCsiKittyKeyboardSetKind, OutputCsiKittyKeyboardSetPlan,
        terminal_parser_ffi_output_csi_kitty_keyboard_set_plan,
    };
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn expect(
        id: &str,
        flags: Option<i32>,
        mode: Option<i32>,
        kind: OutputCsiKittyKeyboardSetKind,
    ) -> OutputCsiKittyKeyboardSetPlan {
        let mut result = OutputCsiKittyKeyboardSetPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
                VtId::from_ascii(id).value(),
                u32::from(flags.is_some()),
                flags.unwrap_or(0),
                u32::from(mode.is_some()),
                mode.unwrap_or(0),
                &mut result,
            ),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, kind as u32, "id={id:?}");
        result
    }

    #[test]
    fn csi_kitty_keyboard_set_ffi_replays_microsoft_contract() {
        let plan = expect(
            "=u",
            Some(3),
            Some(2),
            OutputCsiKittyKeyboardSetKind::Set,
        );
        assert_eq!((plan.has_flags, plan.flags), (1, 3));
        assert_eq!((plan.has_mode, plan.mode), (1, 2));

        let plan = expect(
            "=u",
            Some(5),
            None,
            OutputCsiKittyKeyboardSetKind::Set,
        );
        assert_eq!((plan.has_flags, plan.flags), (1, 5));
        assert_eq!((plan.has_mode, plan.mode), (0, 0));

        let plan = expect("=u", None, None, OutputCsiKittyKeyboardSetKind::Set);
        assert_eq!((plan.has_flags, plan.flags), (0, 0));
        assert_eq!((plan.has_mode, plan.mode), (0, 0));

        let plan = expect(
            ">u",
            Some(3),
            Some(2),
            OutputCsiKittyKeyboardSetKind::None,
        );
        assert_eq!(plan, OutputCsiKittyKeyboardSetPlan::default());
    }

    #[test]
    fn csi_kitty_keyboard_set_ffi_validates_pointer_identifier_and_presence_bits() {
        let id = VtId::from_ascii("=u").value();
        assert_eq!(
            terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
                id,
                1,
                3,
                1,
                2,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );

        let mut result = OutputCsiKittyKeyboardSetPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
                0xff00_0000_0000_0000,
                1,
                3,
                1,
                2,
                &mut result,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_output_csi_kitty_keyboard_set_plan(
                id,
                2,
                3,
                1,
                2,
                &mut result,
            ),
            FfiStatus::InvalidArgument
        );
    }
}
