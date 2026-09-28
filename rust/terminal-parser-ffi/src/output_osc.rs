use std::ptr;

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::StateMachineEngine;

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputOscKind {
    None = 0,
    SetWindowTitle = 1,
    SetColorTable = 2,
    SetDynamicColor = 3,
    SetClipboard = 4,
    ResetColorTable = 5,
    ResetDynamicColor = 6,
    SetCurrentWorkingDirectory = 7,
    Hyperlink = 8,
    ConEmuAction = 9,
    FinalTermAction = 10,
    VsCodeAction = 11,
    UrxvtAction = 12,
    ITerm2Action = 13,
    WtAction = 14,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputOscPlan {
    pub kind: u32,
}

impl Default for OutputOscPlan {
    fn default() -> Self {
        Self {
            kind: OutputOscKind::None as u32,
        }
    }
}

#[derive(Default)]
struct PlanDispatch {
    plan: OutputOscPlan,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.plan.kind = match action {
            OutputAction::UnknownSequence => OutputOscKind::None,
            OutputAction::SetWindowTitle(_) => OutputOscKind::SetWindowTitle,
            OutputAction::SetColorTableEntry { .. } | OutputAction::RequestColorTableEntry(_) => {
                OutputOscKind::SetColorTable
            }
            OutputAction::SetXtermColorResource { .. }
            | OutputAction::RequestXtermColorResource(_) => OutputOscKind::SetDynamicColor,
            OutputAction::SetClipboard(_) => OutputOscKind::SetClipboard,
            OutputAction::ResetColorTable | OutputAction::ResetColorTableEntry(_) => {
                OutputOscKind::ResetColorTable
            }
            OutputAction::ResetXtermColorResource(_) => OutputOscKind::ResetDynamicColor,
            OutputAction::SetCurrentWorkingDirectory(_) => OutputOscKind::SetCurrentWorkingDirectory,
            OutputAction::AddHyperlink { .. } | OutputAction::EndHyperlink => OutputOscKind::Hyperlink,
            OutputAction::ConEmuAction(_) => OutputOscKind::ConEmuAction,
            OutputAction::FinalTermAction(_) => OutputOscKind::FinalTermAction,
            OutputAction::VsCodeAction(_) => OutputOscKind::VsCodeAction,
            OutputAction::UrxvtAction(_) => OutputOscKind::UrxvtAction,
            OutputAction::ITerm2Action(_) => OutputOscKind::ITerm2Action,
            OutputAction::WtAction(_) => OutputOscKind::WtAction,
            _ => return,
        } as u32;
    }
}

fn representative_text(parameter: i32) -> Vec<u16> {
    let text = match parameter {
        4 => "0;?",
        10 | 11 | 12 | 17 => "?",
        52 => "c;Zm9v",
        8 => ";",
        _ => "",
    };
    text.encode_utf16().collect()
}

/// Classifies one Windows Terminal OSC parameter through the existing safe
/// Rust output engine. OSC payload parsing and native dispatch materialization
/// remain outside this narrow classification seam.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_osc_plan(
    parameter: u64,
    out_plan: *mut OutputOscPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }

        let Ok(parameter) = i32::try_from(parameter) else {
            // SAFETY: `out_plan` was checked non-null above and the ABI requires it
            // to reference one writable `OutputOscPlan` for this call.
            unsafe { ptr::write(out_plan, OutputOscPlan::default()) };
            return FfiStatus::Ok;
        };

        let text = representative_text(parameter);
        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        let _ = engine.action_osc_dispatch(parameter, &text);
        let dispatch = engine.into_dispatch();

        // SAFETY: `out_plan` was checked non-null above and the ABI requires it
        // to reference one writable `OutputOscPlan` for this call.
        unsafe { ptr::write(out_plan, dispatch.plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{OutputOscKind, OutputOscPlan, terminal_parser_ffi_output_osc_plan};
    use crate::FfiStatus;

    fn expect(parameter: u64, kind: OutputOscKind) {
        let mut result = OutputOscPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_osc_plan(parameter, &mut result),
            FfiStatus::Ok
        );
        assert_eq!(result.kind, kind as u32, "parameter={parameter}");
    }

    #[test]
    fn output_osc_ffi_replays_all_cpp_classification_cases() {
        for parameter in [0, 1, 2, 21] {
            expect(parameter, OutputOscKind::SetWindowTitle);
        }
        expect(4, OutputOscKind::SetColorTable);
        for parameter in [10, 11, 12, 17] {
            expect(parameter, OutputOscKind::SetDynamicColor);
        }
        expect(52, OutputOscKind::SetClipboard);
        expect(104, OutputOscKind::ResetColorTable);
        for parameter in [110, 111, 112, 117] {
            expect(parameter, OutputOscKind::ResetDynamicColor);
        }
        expect(7, OutputOscKind::SetCurrentWorkingDirectory);
        expect(8, OutputOscKind::Hyperlink);
        expect(9, OutputOscKind::ConEmuAction);
        expect(133, OutputOscKind::FinalTermAction);
        expect(633, OutputOscKind::VsCodeAction);
        expect(777, OutputOscKind::UrxvtAction);
        expect(1337, OutputOscKind::ITerm2Action);
        expect(9001, OutputOscKind::WtAction);
        expect(999, OutputOscKind::None);
    }

    #[test]
    fn output_osc_ffi_rejects_narrowing_aliases() {
        expect(u64::MAX, OutputOscKind::None);
        expect((i32::MAX as u64) + 1, OutputOscKind::None);
    }

    #[test]
    fn output_osc_ffi_validates_pointer() {
        assert_eq!(
            terminal_parser_ffi_output_osc_plan(0, std::ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
    }

    #[test]
    fn output_osc_plan_layout_is_stable() {
        assert_eq!(std::mem::size_of::<OutputOscPlan>(), 4);
        assert_eq!(std::mem::align_of::<OutputOscPlan>(), 4);
    }
}
