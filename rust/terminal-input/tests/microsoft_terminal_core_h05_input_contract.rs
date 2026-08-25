use terminal_input::{KeyEvent, TerminalInput};

#[test]
fn microsoft_terminal_core_invalid_key_event_is_unhandled() {
    for virtual_key in [0u16, 255u16] {
        let mut input = TerminalInput::new();
        let event = KeyEvent {
            virtual_key,
            scan_code: 123,
            codepoint: 0,
            key_down: true,
            repeat_count: 1,
            ..KeyEvent::default()
        };

        assert_eq!(input.handle_key(event), "");
    }
}
