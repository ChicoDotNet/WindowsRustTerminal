use terminal_input::{KeyEvent, TerminalInput, virtual_key};

fn key_down(virtual_key: u16) -> KeyEvent {
    KeyEvent {
        virtual_key,
        scan_code: 0,
        // Microsoft TerminalInputTests populates UnicodeChar with
        // MapVirtualKeyW(..., MAPVK_VK_TO_CHAR) for key-down events. Escape is
        // the one fixed-key case where the portable Rust fallback consumes it.
        codepoint: if virtual_key == virtual_key::ESCAPE {
            u32::from(b'\x1b')
        } else {
            0
        },
        control_key_state: 0,
        key_down: true,
        repeat_count: 1,
    }
}

fn key_up(virtual_key: u16) -> KeyEvent {
    KeyEvent {
        virtual_key,
        scan_code: 0,
        codepoint: 0,
        control_key_state: 0,
        key_down: false,
        repeat_count: 1,
    }
}

#[test]
fn microsoft_terminal_input_tests_fixed_special_key_table() {
    let cases = [
        (virtual_key::TAB, "\t"),
        (virtual_key::BACK, "\u{7f}"),
        (virtual_key::ESCAPE, "\u{1b}"),
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
        (0x70, "\u{1b}OP"),
        (0x71, "\u{1b}OQ"),
        (0x72, "\u{1b}OR"),
        (0x73, "\u{1b}OS"),
        (0x74, "\u{1b}[15~"),
        (0x75, "\u{1b}[17~"),
        (0x76, "\u{1b}[18~"),
        (0x77, "\u{1b}[19~"),
        (0x78, "\u{1b}[20~"),
        (0x79, "\u{1b}[21~"),
        (0x7a, "\u{1b}[23~"),
        (0x7b, "\u{1b}[24~"),
        (0x7c, "\u{1b}[25~"),
        (0x7d, "\u{1b}[26~"),
        (0x7e, "\u{1b}[28~"),
        (0x7f, "\u{1b}[29~"),
        (0x80, "\u{1b}[31~"),
        (0x81, "\u{1b}[32~"),
        (0x82, "\u{1b}[33~"),
        (0x83, "\u{1b}[34~"),
        (virtual_key::CANCEL, "\u{3}"),
    ];

    for (virtual_key, expected) in cases {
        let mut input = TerminalInput::new();
        assert_eq!(
            input.handle_key(key_down(virtual_key)),
            expected,
            "virtual_key={virtual_key:#x}"
        );
    }
}

#[test]
fn microsoft_terminal_input_tests_all_key_up_events_are_silent() {
    for virtual_key in 0_u16..u16::from(u8::MAX) {
        let mut input = TerminalInput::new();
        assert_eq!(
            input.handle_key(key_up(virtual_key)),
            "",
            "virtual_key={virtual_key:#x}"
        );
    }
}
