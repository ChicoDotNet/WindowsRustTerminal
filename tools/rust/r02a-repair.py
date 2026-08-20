from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if old not in text:
        raise SystemExit(f"R02a repair marker not found: {label}")
    return text.replace(old, new, 1)


path = Path("rust/terminal-input/src/lib.rs")
text = path.read_text()

text = replace_once(
    text,
    "(u16::from(b'2')..=u16::from(b'Z')).contains(&event.virtual_key)",
    "(u16::from(b'1')..=u16::from(b'Z')).contains(&event.virtual_key)",
    "Ctrl+1 fallback",
)

old_kitty_test = '''    #[test]
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
'''

new_kitty_test = '''    #[test]
    fn kitty_stack_and_screen_buffer_state_match_microsoft_rules() {
        let mut input = TerminalInput::new();
        input.set_kitty_keyboard_protocol(
            KittyKeyboardProtocolFlags::DISAMBIGUATE_ESCAPE_CODES,
            KittyKeyboardProtocolMode::Replace,
        );
        input.push_kitty_flags(KittyKeyboardProtocolFlags::REPORT_EVENT_TYPES);
        assert_eq!(input.kitty_flags(), KittyKeyboardProtocolFlags::REPORT_EVENT_TYPES);

        input.use_alternate_screen_buffer();
        assert_eq!(input.kitty_flags(), 0);
        input.push_kitty_flags(KittyKeyboardProtocolFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES);
        assert_eq!(
            input.kitty_flags(),
            KittyKeyboardProtocolFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        );

        input.use_main_screen_buffer();
        assert_eq!(
            input.kitty_flags(),
            KittyKeyboardProtocolFlags::DISAMBIGUATE_ESCAPE_CODES
        );

        input.pop_kitty_flags(1);
        assert_eq!(input.kitty_flags(), 0);
    }
'''

text = replace_once(text, old_kitty_test, new_kitty_test, "Kitty stack semantics")
path.write_text(text)
