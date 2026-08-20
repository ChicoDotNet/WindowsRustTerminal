from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if old not in text:
        raise SystemExit(f"R02a repair marker not found: {label}")
    return text.replace(old, new, 1)


path = Path("rust/terminal-input/src/lib.rs")
text = path.read_text()

text = replace_once(
    text,
    "const NUMERIC_CTRLS: [u32; 7] = [0, 27, 28, 29, 30, 31, 127];\n",
    "",
    "remove numeric Ctrl lookup table",
)

text = replace_once(
    text,
    "(u16::from(b'2')..=u16::from(b'Z')).contains(&event.virtual_key)",
    "(u16::from(b'1')..=u16::from(b'Z')).contains(&event.virtual_key)",
    "Ctrl+1 fallback",
)

text = replace_once(
    text,
    '''    if (u32::from(b'2')..=u32::from(b'8')).contains(&character) {
        let index = usize::try_from(character - u32::from(b'2')).expect("bounded Ctrl digit");
        return NUMERIC_CTRLS[index];
    }
''',
    '''    if (u32::from(b'2')..=u32::from(b'8')).contains(&character) {
        return match character {
            value if value == u32::from(b'2') => 0,
            value if value == u32::from(b'3') => 27,
            value if value == u32::from(b'4') => 28,
            value if value == u32::from(b'5') => 29,
            value if value == u32::from(b'6') => 30,
            value if value == u32::from(b'7') => 31,
            value if value == u32::from(b'8') => 127,
            _ => character,
        };
    }
''',
    "non-panicking Ctrl digit mapping",
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
