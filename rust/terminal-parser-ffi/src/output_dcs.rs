use std::ptr;

use terminal_parser::output_engine::{DcsAction, OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputDcsKind {
    None = 0,
    DefineSixelImage = 1,
    DownloadDrcs = 2,
    AssignUserPreferenceCharset = 3,
    DefineMacro = 4,
    RestoreTerminalState = 5,
    RequestSetting = 6,
    RestorePresentationState = 7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputDcsPlan {
    pub kind: u32,
}

impl Default for OutputDcsPlan {
    fn default() -> Self {
        Self {
            kind: OutputDcsKind::None as u32,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputDcsPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        if matches!(action, OutputAction::UnknownSequence) {
            self.plan = OutputDcsPlan::default();
        }
    }

    fn begin_dcs(&mut self, action: DcsAction) -> bool {
        self.plan.kind = match action {
            DcsAction::DefineSixelImage(_) => OutputDcsKind::DefineSixelImage,
            DcsAction::DownloadDrcs(_) => OutputDcsKind::DownloadDrcs,
            DcsAction::AssignUserPreferenceCharset(_) => OutputDcsKind::AssignUserPreferenceCharset,
            DcsAction::DefineMacro(_) => OutputDcsKind::DefineMacro,
            DcsAction::RestoreTerminalState(_) => OutputDcsKind::RestoreTerminalState,
            DcsAction::RequestSetting => OutputDcsKind::RequestSetting,
            DcsAction::RestorePresentationState(_) => OutputDcsKind::RestorePresentationState,
        } as u32;
        true
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

/// Classifies one Windows Terminal DCS identifier through the existing safe
/// Rust output engine. Parameters remain caller-owned because DCS handler
/// materialization is a native dispatch seam and does not affect classification.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_dcs_plan(
    identifier: u64,
    out_plan: *mut OutputDcsPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        let _ = engine.action_dcs_dispatch(id, &Parameters::default());
        let dispatch = engine.into_dispatch();

        // SAFETY: `out_plan` was checked non-null above and the ABI requires it
        // to reference one writable `OutputDcsPlan` for this call.
        unsafe { ptr::write(out_plan, dispatch.plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{OutputDcsKind, OutputDcsPlan, terminal_parser_ffi_output_dcs_plan};
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn expect(id: &str, kind: OutputDcsKind) {
        let mut result = OutputDcsPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_dcs_plan(VtId::from_ascii(id).value(), &mut result),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, kind as u32, "id={id:?}");
    }

    #[test]
    fn output_dcs_ffi_replays_all_cpp_classification_cases() {
        expect("q", OutputDcsKind::DefineSixelImage);
        expect("{", OutputDcsKind::DownloadDrcs);
        expect("!u", OutputDcsKind::AssignUserPreferenceCharset);
        expect("!z", OutputDcsKind::DefineMacro);
        expect("$p", OutputDcsKind::RestoreTerminalState);
        expect("$q", OutputDcsKind::RequestSetting);
        expect("$t", OutputDcsKind::RestorePresentationState);
        expect("x", OutputDcsKind::None);
    }

    #[test]
    fn output_dcs_ffi_validates_pointer_and_identifier() {
        assert_eq!(
            terminal_parser_ffi_output_dcs_plan(
                VtId::from_ascii("q").value(),
                std::ptr::null_mut()
            ),
            FfiStatus::InvalidArgument
        );
        let mut result = OutputDcsPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_dcs_plan(0x0100_0000_0000_0000, &mut result),
            FfiStatus::InvalidArgument
        );
    }

    #[test]
    fn output_dcs_plan_layout_is_stable() {
        assert_eq!(std::mem::size_of::<OutputDcsPlan>(), 4);
        assert_eq!(std::mem::align_of::<OutputDcsPlan>(), 4);
    }
}