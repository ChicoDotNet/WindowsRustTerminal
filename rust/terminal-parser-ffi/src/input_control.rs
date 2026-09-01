use std::ptr;

use terminal_parser::input_control::{ControlCharacterKind, classify_control_character};

use super::{FfiStatus, ffi_guard};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TerminalParserFfiControlCharacterPlan {
    pub kind: u32,
    pub character: u16,
    pub forced_virtual_key: u16,
    pub write_ctrl: u32,
    pub clear_layout_modifiers: u32,
}

/// Classifies one input code unit without crossing keyboard-layout or Win32 types.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_input_control_character_plan(
    code_unit: u16,
    write_alt: u32,
    out_plan: *mut TerminalParserFfiControlCharacterPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() || write_alt > 1 {
            return FfiStatus::InvalidArgument;
        }

        let plan = classify_control_character(code_unit, write_alt != 0);
        let ffi_plan = TerminalParserFfiControlCharacterPlan {
            kind: plan.kind as u32,
            character: plan.character,
            forced_virtual_key: plan.forced_virtual_key,
            write_ctrl: u32::from(plan.write_ctrl),
            clear_layout_modifiers: u32::from(plan.clear_layout_modifiers),
        };
        unsafe { ptr::write(out_plan, ffi_plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replays_ctrl_c_and_alt_ctrl_c_split() {
        let mut plan = TerminalParserFfiControlCharacterPlan::default();
        assert_eq!(
            terminal_parser_ffi_input_control_character_plan(0x03, 0, &mut plan),
            FfiStatus::Ok
        );
        assert_eq!(plan.kind, ControlCharacterKind::CtrlC as u32);
        assert_eq!(plan.forced_virtual_key, u16::from(b'C'));

        assert_eq!(
            terminal_parser_ffi_input_control_character_plan(0x03, 1, &mut plan),
            FfiStatus::Ok
        );
        assert_eq!(plan.kind, ControlCharacterKind::MappedC0 as u32);
        assert_eq!(plan.write_ctrl, 1);
    }

    #[test]
    fn replays_backspace_escape_delete_and_printable_paths() {
        let mut plan = TerminalParserFfiControlCharacterPlan::default();

        assert_eq!(
            terminal_parser_ffi_input_control_character_plan(0x08, 0, &mut plan),
            FfiStatus::Ok
        );
        assert_eq!(plan.character, 0x7f);
        assert_eq!(plan.clear_layout_modifiers, 1);

        assert_eq!(
            terminal_parser_ffi_input_control_character_plan(0x1b, 0, &mut plan),
            FfiStatus::Ok
        );
        assert_eq!(plan.forced_virtual_key, 0x1b);

        assert_eq!(
            terminal_parser_ffi_input_control_character_plan(0x7f, 1, &mut plan),
            FfiStatus::Ok
        );
        assert_eq!(plan.kind, ControlCharacterKind::DeleteAsBackspace as u32);
        assert_eq!(plan.character, 0x08);

        assert_eq!(
            terminal_parser_ffi_input_control_character_plan(u16::from(b'A'), 0, &mut plan),
            FfiStatus::Ok
        );
        assert_eq!(plan.kind, ControlCharacterKind::Print as u32);
    }

    #[test]
    fn rejects_invalid_abi_arguments() {
        let mut plan = TerminalParserFfiControlCharacterPlan::default();
        assert_eq!(
            terminal_parser_ffi_input_control_character_plan(0, 2, &mut plan),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_input_control_character_plan(0, 0, std::ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
    }
}
