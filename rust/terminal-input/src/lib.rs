//! Safe, platform-neutral port of Windows Terminal's `TerminalInput` state and
//! deterministic VT keyboard encoding.
//!
//! R02a intentionally excludes keyboard-layout translation and mouse encoding.
//! Those are isolated behind later increments so this crate stays deterministic
//! on Linux and Windows.

#![forbid(unsafe_code)]

const ESC: char = '\u{1b}';
const CSI_8BIT: char = '\u{009b}';
const SS3_8BIT: char = '\u{008f}';
const KITTY_STACK_MAX_SIZE: usize = 8;
const FUNCTION_KEY_NUMBERS: [u16; 16] = [15, 17, 18, 19, 20, 21, 23, 24, 25, 26, 28, 29, 31, 32, 33, 34];
const NUMERIC_CTRLS: [u32; 7] = [0, 27, 28, 29, 30, 31, 127];

pub mod control_state {
    pub const RIGHT_ALT_PRESSED: u32 = 0x0001;
    pub const LEFT_ALT_PRESSED: u32 = 0x0002;
    pub const RIGHT_CTRL_PRESSED: u32 = 0x0004;
    pub const LEFT_CTRL_PRESSED: u32 = 0x0008;
    pub const SHIFT_PRESSED: u32 = 0x0010;
    pub const NUMLOCK_ON: u32 = 0x0020;
    pub const ENHANCED_KEY: u32 = 0x0100;
    pub const ALT_PRESSED: u32 = RIGHT_ALT_PRESSED | LEFT_ALT_PRESSED;
    pub const CTRL_PRESSED: u32 = RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED;
}

pub mod virtual_key {
    pub const CANCEL: u16 = 0x03;
    pub const BACK: u16 = 0x08;
    pub const TAB: u16 = 0x09;
    pub const CLEAR: u16 = 0x0c;
    pub const RETURN: u16 = 0x0d;
    pub const SHIFT: u16 = 0x10;
    pub const CONTROL: u16 = 0x11;
    pub const MENU: u16 = 0x12;
    pub const PAUSE: u16 = 0x13;
    pub const ESCAPE: u16 = 0x1b;
    pub const SPACE: u16 = 0x20;
    pub const PRIOR: u16 = 0x21;
    pub const NEXT: u16 = 0x22;
    pub const END: u16 = 0x23;
    pub const HOME: u16 = 0x24;
    pub const LEFT: u16 = 0x25;
    pub const UP: u16 = 0x26;
    pub const RIGHT: u16 = 0x27;
    pub const DOWN: u16 = 0x28;
    pub const INSERT: u16 = 0x2d;
    pub const DELETE: u16 = 0x2e;
    pub const NUMPAD0: u16 = 0x60;
    pub const NUMPAD9: u16 = 0x69;
    pub const MULTIPLY: u16 = 0x6a;
    pub const ADD: u16 = 0x6b;
    pub const SEPARATOR: u16 = 0x6c;
    pub const SUBTRACT: u16 = 0x6d;
    pub const DECIMAL: u16 = 0x6e;
    pub const DIVIDE: u16 = 0x6f;
    pub const F1: u16 = 0x70;
    pub const F4: u16 = 0x73;
    pub const F5: u16 = 0x74;
    pub const F11: u16 = 0x7a;
    pub const F12: u16 = 0x7b;
    pub const F13: u16 = 0x7c;
    pub const F20: u16 = 0x83;
    pub const LSHIFT: u16 = 0xa0;
    pub const RSHIFT: u16 = 0xa1;
    pub const LCONTROL: u16 = 0xa2;
    pub const RCONTROL: u16 = 0xa3;
    pub const LMENU: u16 = 0xa4;
    pub const RMENU: u16 = 0xa5;
    pub const PACKET: u16 = 0xe7;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    LineFeed,
    Ansi,
    AutoRepeat,
    Keypad,
    CursorKey,
    BackarrowKey,
    Win32,
    SendC1,
    Utf8MouseEncoding,
    SgrMouseEncoding,
    DefaultMouseTracking,
    ButtonEventMouseTracking,
    AnyEventMouseTracking,
    FocusEvent,
    AlternateScroll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KittyKeyboardProtocolMode {
    Replace,
    Set,
    Reset,
}

pub struct KittyKeyboardProtocolFlags;

impl KittyKeyboardProtocolFlags {
    pub const NONE: u8 = 0;
    pub const DISAMBIGUATE_ESCAPE_CODES: u8 = 1 << 0;
    pub const REPORT_EVENT_TYPES: u8 = 1 << 1;
    pub const REPORT_ALTERNATE_KEYS: u8 = 1 << 2;
    pub const REPORT_ALL_KEYS_AS_ESCAPE_CODES: u8 = 1 << 3;
    pub const REPORT_ASSOCIATED_TEXT: u8 = 1 << 4;
    pub const ALL: u8 = (1 << 5) - 1;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KeyEvent {
    pub virtual_key: u16,
    pub scan_code: u16,
    pub codepoint: u32,
    pub control_key_state: u32,
    pub key_down: bool,
    pub repeat_count: u16,
}

#[derive(Debug, Clone)]
pub struct TerminalInput {
    input_modes: u32,
    force_disable_win32_input_mode: bool,
    in_alternate_buffer: bool,
    last_virtual_key: Option<u16>,
    force_disable_kitty_keyboard_protocol: bool,
    kitty_flags: u8,
    kitty_main_stack: Vec<u8>,
    kitty_alt_stack: Vec<u8>,
}

impl Default for TerminalInput {
    fn default() -> Self {
        let mut input = Self {
            input_modes: 0,
            force_disable_win32_input_mode: false,
            in_alternate_buffer: false,
            last_virtual_key: None,
            force_disable_kitty_keyboard_protocol: false,
            kitty_flags: 0,
            kitty_main_stack: Vec::new(),
            kitty_alt_stack: Vec::new(),
        };
        input.reset_input_modes();
        input
    }
}

impl TerminalInput {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn use_main_screen_buffer(&mut self) {
        if !self.in_alternate_buffer {
            return;
        }
        self.in_alternate_buffer = false;
        self.kitty_alt_stack.clear();
        self.kitty_flags = self.kitty_main_stack.last().copied().unwrap_or(0);
    }

    pub fn use_alternate_screen_buffer(&mut self) {
        if self.in_alternate_buffer {
            return;
        }
        self.in_alternate_buffer = true;
        self.kitty_alt_stack.clear();
        self.kitty_flags = 0;
    }

    pub fn set_input_mode(&mut self, mode: Mode, enabled: bool) {
        if is_tracking_mode(mode) {
            self.set_mode(Mode::DefaultMouseTracking, false);
            self.set_mode(Mode::ButtonEventMouseTracking, false);
            self.set_mode(Mode::AnyEventMouseTracking, false);
        }
        if enabled && matches!(mode, Mode::Utf8MouseEncoding | Mode::SgrMouseEncoding) {
            self.set_mode(Mode::Utf8MouseEncoding, false);
            self.set_mode(Mode::SgrMouseEncoding, false);
        }
        self.set_mode(mode, enabled);
    }

    #[must_use]
    pub fn get_input_mode(&self, mode: Mode) -> bool {
        self.input_modes & mode_bit(mode) != 0
    }

    pub fn reset_input_modes(&mut self) {
        self.input_modes = 0;
        self.set_mode(Mode::Ansi, true);
        self.set_mode(Mode::AutoRepeat, true);
        self.set_mode(Mode::AlternateScroll, true);
        self.last_virtual_key = None;
        self.reset_kitty_keyboard_protocols();
    }

    pub fn force_disable_win32_input_mode(&mut self, disable: bool) {
        self.force_disable_win32_input_mode = disable;
    }

    pub fn force_disable_kitty_keyboard_protocol(&mut self, disable: bool) {
        self.force_disable_kitty_keyboard_protocol = disable;
        if disable {
            self.reset_kitty_keyboard_protocols();
        }
    }

    pub fn set_kitty_keyboard_protocol(&mut self, mut flags: u8, mode: KittyKeyboardProtocolMode) {
        if self.force_disable_kitty_keyboard_protocol {
            return;
        }
        flags &= KittyKeyboardProtocolFlags::ALL;
        match mode {
            KittyKeyboardProtocolMode::Replace => self.kitty_flags = flags,
            KittyKeyboardProtocolMode::Set => self.kitty_flags |= flags,
            KittyKeyboardProtocolMode::Reset => self.kitty_flags &= !flags,
        }
    }

    #[must_use]
    pub const fn kitty_flags(&self) -> u8 {
        self.kitty_flags
    }

    pub fn push_kitty_flags(&mut self, flags: u8) {
        if self.force_disable_kitty_keyboard_protocol {
            return;
        }
        let stack = if self.in_alternate_buffer {
            &mut self.kitty_alt_stack
        } else {
            &mut self.kitty_main_stack
        };
        if stack.len() >= KITTY_STACK_MAX_SIZE {
            stack.remove(0);
        }
        stack.push(self.kitty_flags);
        self.kitty_flags = flags & KittyKeyboardProtocolFlags::ALL;
    }

    pub fn pop_kitty_flags(&mut self, count: usize) {
        if count == 0 {
            return;
        }
        let stack = if self.in_alternate_buffer {
            &mut self.kitty_alt_stack
        } else {
            &mut self.kitty_main_stack
        };
        if count >= stack.len() {
            self.kitty_flags = 0;
            stack.clear();
        } else {
            let new_len = stack.len() - count;
            self.kitty_flags = stack[new_len];
            stack.truncate(new_len);
        }
    }

    pub fn reset_kitty_keyboard_protocols(&mut self) {
        self.kitty_flags = 0;
        self.kitty_main_stack.clear();
        self.kitty_alt_stack.clear();
    }

    #[must_use]
    pub fn handle_focus(&self, focused: bool) -> Option<String> {
        if !self.get_input_mode(Mode::FocusEvent) {
            return None;
        }
        let suffix = if focused { 'I' } else { 'O' };
        Some(format!("{}{suffix}", self.csi_prefix()))
    }

    /// Translates one typed key event into its VT input representation.
    ///
    /// An empty string means the event was handled but intentionally emitted nothing.
    #[must_use]
    pub fn handle_key(&mut self, event: KeyEvent) -> String {
        if self.get_input_mode(Mode::Win32)
            && !self.force_disable_win32_input_mode
            && self.kitty_flags == 0
        {
            return self.make_win32_output(event);
        }

        if !event.key_down {
            self.last_virtual_key = None;
            return String::new();
        }

        if event.virtual_key == virtual_key::PACKET || event.virtual_key == 0 {
            return codepoint_string(event.codepoint);
        }

        let key_repeat = self.last_virtual_key == Some(event.virtual_key);
        self.last_virtual_key = Some(event.virtual_key);
        if key_repeat && (is_modifier_key(event.virtual_key) || !self.get_input_mode(Mode::AutoRepeat)) {
            return String::new();
        }
        if is_modifier_key(event.virtual_key) {
            return String::new();
        }

        let ctrl = event.control_key_state & control_state::CTRL_PRESSED != 0;
        let alt = event.control_key_state & control_state::ALT_PRESSED != 0;
        let modifier = modifier_parameter(event.control_key_state);

        if let Some(sequence) = self.encode_special(event.virtual_key, event.control_key_state, modifier) {
            return sequence;
        }

        let mut codepoint = event.codepoint;
        if codepoint == 0 && ctrl && (u16::from(b'2')..=u16::from(b'Z')).contains(&event.virtual_key) {
            codepoint = u32::from(event.virtual_key);
        }
        if codepoint == 0 {
            return String::new();
        }
        if ctrl {
            codepoint = make_ctrl_char(codepoint);
        }

        let mut output = String::new();
        if alt && self.get_input_mode(Mode::Ansi) {
            output.push(ESC);
        }
        output.push_str(&codepoint_string(codepoint));
        output
    }

    fn set_mode(&mut self, mode: Mode, enabled: bool) {
        let bit = mode_bit(mode);
        if enabled {
            self.input_modes |= bit;
        } else {
            self.input_modes &= !bit;
        }
    }

    fn csi_prefix(&self) -> String {
        if self.get_input_mode(Mode::SendC1) {
            CSI_8BIT.to_string()
        } else {
            format!("{ESC}[")
        }
    }

    fn ss3_prefix(&self) -> String {
        if self.get_input_mode(Mode::SendC1) {
            SS3_8BIT.to_string()
        } else {
            format!("{ESC}O")
        }
    }

    fn make_win32_output(&self, key: KeyEvent) -> String {
        let key_down = u8::from(key.key_down);
        let codepoint = key.codepoint.min(u32::from(u16::MAX));
        let control_state = key.control_key_state.min(u32::from(u16::MAX));
        format!(
            "{}{};{};{codepoint};{key_down};{control_state};{}_",
            self.csi_prefix(),
            key.virtual_key,
            key.scan_code,
            key.repeat_count
        )
    }

    fn encode_special(&self, virtual_key: u16, control_key_state: u32, modifier: u8) -> Option<String> {
        let shift = control_key_state & control_state::SHIFT_PRESSED != 0;
        let ctrl = control_key_state & control_state::CTRL_PRESSED != 0;
        let alt = control_key_state & control_state::ALT_PRESSED != 0;
        let enhanced = control_key_state & control_state::ENHANCED_KEY != 0;
        let ansi = self.get_input_mode(Mode::Ansi);

        match virtual_key {
            virtual_key::BACK => {
                let backarrow = self.get_input_mode(Mode::BackarrowKey);
                let character = if ctrl == backarrow { '\u{7f}' } else { '\u{8}' };
                Some(with_alt_prefix(character.to_string(), alt && ansi))
            }
            virtual_key::TAB => {
                let sequence = if shift {
                    format!("{}Z", self.csi_prefix())
                } else {
                    "\t".to_string()
                };
                Some(with_alt_prefix(sequence, alt && ansi))
            }
            virtual_key::RETURN => {
                let sequence = if self.get_input_mode(Mode::Keypad) && enhanced {
                    if ansi {
                        format!("{}M", self.ss3_prefix())
                    } else {
                        format!("{ESC}?M")
                    }
                } else if ctrl {
                    "\n".to_string()
                } else if self.get_input_mode(Mode::LineFeed) {
                    "\r\n".to_string()
                } else {
                    "\r".to_string()
                };
                Some(with_alt_prefix(sequence, alt && ansi))
            }
            virtual_key::PAUSE => Some("\u{1a}".to_string()),
            virtual_key::CANCEL => Some("\u{3}".to_string()),
            virtual_key::F1..=virtual_key::F4 => {
                Some(self.function_key_f1_f4(virtual_key, modifier, ansi))
            }
            virtual_key::F5..=virtual_key::F20 => {
                Some(self.function_key_f5_f20(virtual_key, modifier, ansi))
            }
            virtual_key::LEFT | virtual_key::UP | virtual_key::RIGHT | virtual_key::DOWN => {
                Some(self.cursor_key(virtual_key, modifier, ansi))
            }
            virtual_key::CLEAR | virtual_key::HOME | virtual_key::END => {
                Some(self.navigation_key(virtual_key, modifier, ansi))
            }
            virtual_key::INSERT | virtual_key::DELETE => {
                if ansi {
                    let number = 2 + (virtual_key - virtual_key::INSERT);
                    Some(self.csi_numeric(number, modifier, '~'))
                } else {
                    Some(String::new())
                }
            }
            virtual_key::PRIOR | virtual_key::NEXT => {
                if ansi {
                    let number = 5 + (virtual_key - virtual_key::PRIOR);
                    Some(self.csi_numeric(number, modifier, '~'))
                } else {
                    Some(String::new())
                }
            }
            virtual_key::NUMPAD0..=virtual_key::NUMPAD9 if self.get_input_mode(Mode::Keypad) => {
                let final_character = char::from_u32(
                    u32::from(b'p') + u32::from(virtual_key - virtual_key::NUMPAD0),
                )
                .expect("ASCII keypad final");
                Some(if ansi {
                    format!("{}{final_character}", self.ss3_prefix())
                } else {
                    format!("{ESC}?{final_character}")
                })
            }
            virtual_key::MULTIPLY..=virtual_key::DIVIDE if self.get_input_mode(Mode::Keypad) => {
                let final_character = char::from_u32(
                    u32::from(b'j') + u32::from(virtual_key - virtual_key::MULTIPLY),
                )
                .expect("ASCII keypad operator final");
                Some(if ansi {
                    format!("{}{final_character}", self.ss3_prefix())
                } else {
                    format!("{ESC}?{final_character}")
                })
            }
            _ => None,
        }
    }

    fn function_key_f1_f4(&self, virtual_key: u16, modifier: u8, ansi: bool) -> String {
        let final_character = char::from_u32(
            u32::from(b'P') + u32::from(virtual_key - virtual_key::F1),
        )
        .expect("ASCII function-key final");
        if !ansi {
            return format!("{ESC}{final_character}");
        }
        if modifier == 0 {
            format!("{}{final_character}", self.ss3_prefix())
        } else {
            self.csi_with_modifier(1, modifier, final_character)
        }
    }

    fn function_key_f5_f20(&self, virtual_key: u16, modifier: u8, ansi: bool) -> String {
        if !ansi {
            return match virtual_key {
                virtual_key::F11 => ESC.to_string(),
                virtual_key::F12 => "\u{8}".to_string(),
                virtual_key::F13 => "\n".to_string(),
                _ => String::new(),
            };
        }
        let number = FUNCTION_KEY_NUMBERS[usize::from(virtual_key - virtual_key::F5)];
        self.csi_numeric(number, modifier, '~')
    }

    fn cursor_key(&self, virtual_key: u16, modifier: u8, ansi: bool) -> String {
        let final_character = match virtual_key {
            virtual_key::LEFT => 'D',
            virtual_key::UP => 'A',
            virtual_key::RIGHT => 'C',
            virtual_key::DOWN => 'B',
            _ => unreachable!("validated cursor key"),
        };
        if !ansi {
            return format!("{ESC}{final_character}");
        }
        if modifier == 0 && self.get_input_mode(Mode::CursorKey) {
            format!("{}{final_character}", self.ss3_prefix())
        } else if modifier == 0 {
            format!("{}{final_character}", self.csi_prefix())
        } else {
            self.csi_with_modifier(1, modifier, final_character)
        }
    }

    fn navigation_key(&self, virtual_key: u16, modifier: u8, ansi: bool) -> String {
        let final_character = match virtual_key {
            virtual_key::CLEAR => 'E',
            virtual_key::HOME => 'H',
            virtual_key::END => 'F',
            _ => unreachable!("validated navigation key"),
        };
        if !ansi {
            return format!("{ESC}{final_character}");
        }
        if modifier == 0 && self.get_input_mode(Mode::CursorKey) {
            format!("{}{final_character}", self.ss3_prefix())
        } else if modifier == 0 {
            format!("{}{final_character}", self.csi_prefix())
        } else {
            self.csi_with_modifier(1, modifier, final_character)
        }
    }

    fn csi_numeric(&self, number: u16, modifier: u8, final_character: char) -> String {
        if modifier == 0 {
            format!("{}{number}{final_character}", self.csi_prefix())
        } else {
            self.csi_with_modifier(number, modifier, final_character)
        }
    }

    fn csi_with_modifier(&self, number: u16, modifier: u8, final_character: char) -> String {
        format!(
            "{}{number};{}{final_character}",
            self.csi_prefix(),
            modifier + 1
        )
    }
}

const fn mode_bit(mode: Mode) -> u32 {
    1u32 << mode as u8
}

const fn is_tracking_mode(mode: Mode) -> bool {
    matches!(
        mode,
        Mode::DefaultMouseTracking | Mode::ButtonEventMouseTracking | Mode::AnyEventMouseTracking
    )
}

fn is_modifier_key(virtual_key: u16) -> bool {
    (virtual_key::SHIFT..=virtual_key::MENU).contains(&virtual_key)
        || (virtual_key::LSHIFT..=virtual_key::RMENU).contains(&virtual_key)
}

fn modifier_parameter(control_key_state: u32) -> u8 {
    let shift = u8::from(control_key_state & control_state::SHIFT_PRESSED != 0);
    let alt = u8::from(control_key_state & control_state::ALT_PRESSED != 0);
    let ctrl = u8::from(control_key_state & control_state::CTRL_PRESSED != 0);
    shift | (alt << 1) | (ctrl << 2)
}

fn with_alt_prefix(sequence: String, alt: bool) -> String {
    if alt {
        format!("{ESC}{sequence}")
    } else {
        sequence
    }
}

fn codepoint_string(codepoint: u32) -> String {
    char::from_u32(codepoint).map_or_else(String::new, |character| character.to_string())
}

#[must_use]
pub fn make_ctrl_char(character: u32) -> u32 {
    if (u32::from(b'@')..=u32::from(b'~')).contains(&character) {
        return character & 0b1_1111;
    }
    if character == u32::from(b' ') {
        return 0;
    }
    if character == u32::from(b'/') {
        return 0x1f;
    }
    if character == u32::from(b'?') {
        return 0x7f;
    }
    if (u32::from(b'2')..=u32::from(b'8')).contains(&character) {
        let index = usize::try_from(character - u32::from(b'2')).expect("bounded Ctrl digit");
        return NUMERIC_CTRLS[index];
    }
    character
}

#[cfg(test)]
mod tests {
    use super::{
        control_state, virtual_key, KeyEvent, KittyKeyboardProtocolFlags,
        KittyKeyboardProtocolMode, Mode, TerminalInput,
    };

    fn key(virtual_key: u16) -> KeyEvent {
        KeyEvent {
            virtual_key,
            key_down: true,
            repeat_count: 1,
            ..KeyEvent::default()
        }
    }

    #[test]
    fn defaults_match_microsoft_terminal_input() {
        let input = TerminalInput::new();
        assert!(input.get_input_mode(Mode::Ansi));
        assert!(input.get_input_mode(Mode::AutoRepeat));
        assert!(input.get_input_mode(Mode::AlternateScroll));
        assert!(!input.get_input_mode(Mode::CursorKey));
    }

    #[test]
    fn classic_special_keys_match_microsoft_contract() {
        let cases = [
            (virtual_key::TAB, "\t"),
            (virtual_key::BACK, "\u{7f}"),
            (virtual_key::PAUSE, "\u{1a}"),
            (virtual_key::UP, "\u{1b}[A"),
            (virtual_key::DOWN, "\u{1b}[B"),
            (virtual_key::RIGHT, "\u{1b}[C"),
            (virtual_key::LEFT, "\u{1b}[D"),
            (virtual_key::CLEAR, "\u{1b}[E"),
            (virtual_key::HOME, "\u{1b}[H"),
            (virtual_key::INSERT, "\u{1b}[2~"),
            (virtual_key::DELETE, "\u{1b}[3~"),
            (virtual_key::END, "\u{1b}[F"),
            (virtual_key::PRIOR, "\u{1b}[5~"),
            (virtual_key::NEXT, "\u{1b}[6~"),
            (virtual_key::F1, "\u{1b}OP"),
            (virtual_key::F1 + 1, "\u{1b}OQ"),
            (virtual_key::F1 + 2, "\u{1b}OR"),
            (virtual_key::F1 + 3, "\u{1b}OS"),
            (virtual_key::F5, "\u{1b}[15~"),
            (virtual_key::F20, "\u{1b}[34~"),
        ];
        for (virtual_key, expected) in cases {
            let mut input = TerminalInput::new();
            assert_eq!(input.handle_key(key(virtual_key)), expected);
        }
    }

    #[test]
    fn focus_mode_matches_microsoft_contract() {
        let mut input = TerminalInput::new();
        assert_eq!(input.handle_focus(false), None);
        assert_eq!(input.handle_focus(true), None);
        input.set_input_mode(Mode::FocusEvent, true);
        assert_eq!(input.handle_focus(false).as_deref(), Some("\u{1b}[O"));
        assert_eq!(input.handle_focus(true).as_deref(), Some("\u{1b}[I"));
    }

    #[test]
    fn backarrow_mode_and_ctrl_inversion_match_microsoft() {
        let mut input = TerminalInput::new();
        input.set_input_mode(Mode::BackarrowKey, true);
        assert_eq!(input.handle_key(key(virtual_key::BACK)), "\u{8}");

        let mut input = TerminalInput::new();
        input.set_input_mode(Mode::BackarrowKey, true);
        let mut ctrl = key(virtual_key::BACK);
        ctrl.control_key_state = control_state::LEFT_CTRL_PRESSED;
        assert_eq!(input.handle_key(ctrl), "\u{7f}");

        let mut input = TerminalInput::new();
        assert_eq!(input.handle_key(key(virtual_key::BACK)), "\u{7f}");
        let mut input = TerminalInput::new();
        assert_eq!(input.handle_key(ctrl), "\u{8}");
    }

    #[test]
    fn auto_repeat_mode_matches_microsoft() {
        let mut input = TerminalInput::new();
        let mut event = key(u16::from(b'A'));
        event.codepoint = u32::from(b'A');
        input.set_input_mode(Mode::AutoRepeat, false);
        assert_eq!(input.handle_key(event), "A");
        assert_eq!(input.handle_key(event), "");
        let mut up = event;
        up.key_down = false;
        assert_eq!(input.handle_key(up), "");
        assert_eq!(input.handle_key(event), "A");
    }

    #[test]
    fn send_c1_mode_matches_microsoft() {
        let mut input = TerminalInput::new();
        input.set_input_mode(Mode::SendC1, true);
        assert_eq!(input.handle_key(key(virtual_key::HOME)), "\u{009b}H");

        let mut input = TerminalInput::new();
        input.set_input_mode(Mode::SendC1, true);
        assert_eq!(input.handle_key(key(virtual_key::F1)), "\u{008f}P");
    }

    #[test]
    fn modified_navigation_sequences_match_microsoft() {
        let mut input = TerminalInput::new();
        let mut delete = key(virtual_key::DELETE);
        delete.control_key_state = control_state::LEFT_CTRL_PRESSED;
        assert_eq!(input.handle_key(delete), "\u{1b}[3;5~");

        let mut input = TerminalInput::new();
        let mut tab = key(virtual_key::TAB);
        tab.control_key_state = control_state::SHIFT_PRESSED;
        assert_eq!(input.handle_key(tab), "\u{1b}[Z");
    }

    #[test]
    fn ctrl_numeric_fallback_matches_microsoft_contract() {
        for (virtual_key, expected) in [
            (b'1', "1"),
            (b'3', "\u{1b}"),
            (b'4', "\u{1c}"),
            (b'5', "\u{1d}"),
            (b'6', "\u{1e}"),
            (b'7', "\u{1f}"),
            (b'8', "\u{7f}"),
            (b'9', "9"),
        ] {
            let mut input = TerminalInput::new();
            let mut event = key(u16::from(virtual_key));
            event.control_key_state = control_state::LEFT_CTRL_PRESSED;
            assert_eq!(input.handle_key(event), expected);
        }
    }

    #[test]
    fn win32_input_mode_is_lossless_for_key_fields() {
        let mut input = TerminalInput::new();
        input.set_input_mode(Mode::Win32, true);
        let event = KeyEvent {
            virtual_key: 1,
            scan_code: 2,
            codepoint: 3,
            control_key_state: 5,
            key_down: true,
            repeat_count: 6,
        };
        assert_eq!(input.handle_key(event), "\u{1b}[1;2;3;1;5;6_");
    }

    #[test]
    fn kitty_stack_and_screen_buffer_state_match_microsoft_rules() {
        let mut input = TerminalInput::new();
        input.set_kitty_keyboard_protocol(
            KittyKeyboardProtocolFlags::DISAMBIGUATE_ESCAPE_CODES,
            KittyKeyboardProtocolMode::Replace,
        );
        input.push_kitty_flags(KittyKeyboardProtocolFlags::REPORT_EVENT_TYPES);
        assert_eq!(input.kitty_flags(), KittyKeyboardProtocolFlags::REPORT_EVENT_TYPES);
        input.pop_kitty_flags(1);
        assert_eq!(
            input.kitty_flags(),
            KittyKeyboardProtocolFlags::DISAMBIGUATE_ESCAPE_CODES
        );

        input.push_kitty_flags(KittyKeyboardProtocolFlags::REPORT_ASSOCIATED_TEXT);
        input.use_alternate_screen_buffer();
        assert_eq!(input.kitty_flags(), 0);
        input.push_kitty_flags(KittyKeyboardProtocolFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES);
        input.use_main_screen_buffer();
        assert_eq!(
            input.kitty_flags(),
            KittyKeyboardProtocolFlags::REPORT_ASSOCIATED_TEXT
        );
    }

    #[test]
    fn tracking_and_encoding_modes_are_mutually_exclusive() {
        let mut input = TerminalInput::new();
        input.set_input_mode(Mode::DefaultMouseTracking, true);
        input.set_input_mode(Mode::AnyEventMouseTracking, true);
        assert!(!input.get_input_mode(Mode::DefaultMouseTracking));
        assert!(input.get_input_mode(Mode::AnyEventMouseTracking));

        input.set_input_mode(Mode::Utf8MouseEncoding, true);
        input.set_input_mode(Mode::SgrMouseEncoding, true);
        assert!(!input.get_input_mode(Mode::Utf8MouseEncoding));
        assert!(input.get_input_mode(Mode::SgrMouseEncoding));
    }
}
