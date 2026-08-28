//! Portable `ApiRoutines::SetConsoleInputModeImpl` semantics.
//!
//! The native API stores ordinary input-mode bits on the input buffer while
//! keeping insert, quick-edit and auto-position as console-global extended
//! state. Those extended flags may be enabled without `ENABLE_EXTENDED_FLAGS`,
//! but clearing them requires that flag. Invalid combinations are still
//! applied and reported as `InvalidArgument`, matching the Microsoft contract.

use crate::input_buffer::InputBuffer;

pub const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
pub const ENABLE_LINE_INPUT: u32 = 0x0002;
pub const ENABLE_ECHO_INPUT: u32 = 0x0004;
pub const ENABLE_WINDOW_INPUT: u32 = 0x0008;
pub const ENABLE_MOUSE_INPUT: u32 = 0x0010;
pub const ENABLE_INSERT_MODE: u32 = 0x0020;
pub const ENABLE_QUICK_EDIT_MODE: u32 = 0x0040;
pub const ENABLE_EXTENDED_FLAGS: u32 = 0x0080;
pub const ENABLE_AUTO_POSITION: u32 = 0x0100;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;

const EXTENDED_STATE_MASK: u32 = ENABLE_INSERT_MODE
    | ENABLE_QUICK_EDIT_MODE
    | ENABLE_EXTENDED_FLAGS
    | ENABLE_AUTO_POSITION;
const VALID_INPUT_MODE_MASK: u32 = ENABLE_PROCESSED_INPUT
    | ENABLE_LINE_INPUT
    | ENABLE_ECHO_INPUT
    | ENABLE_WINDOW_INPUT
    | ENABLE_MOUSE_INPUT
    | EXTENDED_STATE_MASK
    | ENABLE_VIRTUAL_TERMINAL_INPUT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputModeStatus {
    Success,
    InvalidArgument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleInputModeState {
    input_buffer: InputBuffer,
    quick_edit_mode: bool,
    auto_position: bool,
    insert_mode: bool,
    cursor_double_mode: bool,
    cooked_read_insert_mode: Option<bool>,
}

impl ConsoleInputModeState {
    #[must_use]
    pub fn from_mode(mode: u32) -> Self {
        let mut input_buffer = InputBuffer::new();
        input_buffer.set_input_mode(mode & !EXTENDED_STATE_MASK);
        Self {
            input_buffer,
            quick_edit_mode: mode & ENABLE_QUICK_EDIT_MODE != 0,
            auto_position: mode & ENABLE_AUTO_POSITION != 0,
            insert_mode: mode & ENABLE_INSERT_MODE != 0,
            cursor_double_mode: true,
            cooked_read_insert_mode: None,
        }
    }

    pub fn begin_cooked_read(&mut self) {
        self.cooked_read_insert_mode = Some(self.insert_mode);
    }

    #[must_use]
    pub const fn input_mode(&self) -> u32 {
        self.input_buffer.input_mode()
    }

    #[must_use]
    pub const fn quick_edit_mode(&self) -> bool {
        self.quick_edit_mode
    }

    #[must_use]
    pub const fn auto_position(&self) -> bool {
        self.auto_position
    }

    #[must_use]
    pub const fn insert_mode(&self) -> bool {
        self.insert_mode
    }

    #[must_use]
    pub const fn cursor_double_mode(&self) -> bool {
        self.cursor_double_mode
    }

    #[must_use]
    pub const fn cooked_read_insert_mode(&self) -> Option<bool> {
        self.cooked_read_insert_mode
    }

    pub fn set_console_input_mode(&mut self, requested_mode: u32) -> InputModeStatus {
        let can_clear_extended = requested_mode & ENABLE_EXTENDED_FLAGS != 0;
        let new_quick_edit = next_extended_flag(
            self.quick_edit_mode,
            requested_mode,
            ENABLE_QUICK_EDIT_MODE,
            can_clear_extended,
        );
        let new_auto_position = next_extended_flag(
            self.auto_position,
            requested_mode,
            ENABLE_AUTO_POSITION,
            can_clear_extended,
        );
        let new_insert_mode = next_extended_flag(
            self.insert_mode,
            requested_mode,
            ENABLE_INSERT_MODE,
            can_clear_extended,
        );

        if new_insert_mode != self.insert_mode {
            self.cursor_double_mode = false;
        }
        self.quick_edit_mode = new_quick_edit;
        self.auto_position = new_auto_position;
        self.insert_mode = new_insert_mode;
        if let Some(cooked_insert) = self.cooked_read_insert_mode.as_mut() {
            *cooked_insert = new_insert_mode;
        }

        self.input_buffer
            .set_input_mode(requested_mode & !EXTENDED_STATE_MASK);

        if requested_mode & !VALID_INPUT_MODE_MASK != 0
            || requested_mode & ENABLE_ECHO_INPUT != 0
                && requested_mode & ENABLE_LINE_INPUT == 0
        {
            InputModeStatus::InvalidArgument
        } else {
            InputModeStatus::Success
        }
    }
}

fn next_extended_flag(
    previous: bool,
    requested_mode: u32,
    flag: u32,
    can_clear_extended: bool,
) -> bool {
    if requested_mode & flag != 0 {
        true
    } else if can_clear_extended {
        false
    } else {
        previous
    }
}
