from pathlib import Path

path = Path("rust/terminal-input/src/keyboard.rs")
text = path.read_text()


def replace_once(old: str, new: str, label: str) -> None:
    global text
    if old not in text:
        raise SystemExit(f"R02b clippy marker not found: {label}")
    text = text.replace(old, new, 1)


text = text.replace("AltGr retained", "`AltGr` retained")

replace_once(
    '''#[derive(Debug, Clone, Copy)]
struct SanitizedKeyEvent {
    raw: KeyEvent,
    codepoint: u32,
    key_repeat: bool,
    alt_gr: bool,
    ctrl: bool,
    alt: bool,
    shift: bool,
}
''',
    '''#[derive(Debug, Clone, Copy, Default)]
struct Modifiers {
    alt_gr: bool,
    ctrl: bool,
    alt: bool,
    shift: bool,
}

#[derive(Debug, Clone, Copy)]
struct SanitizedKeyEvent {
    raw: KeyEvent,
    codepoint: u32,
    key_repeat: bool,
    modifiers: Modifiers,
}
''',
    "modifier grouping",
)

replace_once(
    '''    let key = SanitizedKeyEvent {
        raw: event,
        codepoint: event.codepoint,
        key_repeat,
        alt_gr,
        ctrl,
        alt,
        shift,
    };

    if input.kitty_flags != 0 {
        if let Some(sequence) = encode_kitty(input, &key, mapper) {
            return sequence;
        }
    }
''',
    '''    let key = SanitizedKeyEvent {
        raw: event,
        codepoint: event.codepoint,
        key_repeat,
        modifiers: Modifiers {
            alt_gr,
            ctrl,
            alt,
            shift,
        },
    };

    if input.kitty_flags != 0
        && let Some(sequence) = encode_kitty(input, &key, mapper)
    {
        return sequence;
    }
''',
    "sanitized construction",
)

replace_once(
    '''fn combine_surrogate(input: &mut TerminalInput, event: &mut KeyEvent) -> Option<String> {
    if (0xd800..=0xdbff).contains(&event.codepoint) {
        input.leading_surrogate = Some(event.codepoint as u16);
        return Some(String::new());
    }

    if let Some(leading) = input.leading_surrogate.take() {
        if (0xdc00..=0xdfff).contains(&event.codepoint) {
            let trailing = event.codepoint as u16;
            let high = u32::from(leading) - 0xd800;
            let low = u32::from(trailing) - 0xdc00;
            event.codepoint = 0x1_0000 + ((high << 10) | low);
        }
    }
    None
}
''',
    '''fn combine_surrogate(input: &mut TerminalInput, event: &mut KeyEvent) -> Option<String> {
    if (0xd800..=0xdbff).contains(&event.codepoint) {
        let Ok(code_unit) = u16::try_from(event.codepoint) else {
            return None;
        };
        input.leading_surrogate = Some(code_unit);
        return Some(String::new());
    }

    if let Some(leading) = input.leading_surrogate.take()
        && (0xdc00..=0xdfff).contains(&event.codepoint)
        && let Ok(trailing) = u16::try_from(event.codepoint)
    {
        let high = u32::from(leading) - 0xd800;
        let low = u32::from(trailing) - 0xdc00;
        event.codepoint = 0x1_0000 + ((high << 10) | low);
    }
    None
}
''',
    "checked surrogate conversion",
)

for old, new in [
    ("key.alt_gr", "key.modifiers.alt_gr"),
    ("key.ctrl", "key.modifiers.ctrl"),
    ("key.alt", "key.modifiers.alt"),
    ("key.shift", "key.modifiers.shift"),
]:
    text = text.replace(old, new)

replace_once(
    '''        } else if functional == KITTY_TEXT_SENTINEL {
            if let Some(codepoint) = mapper.kitty_base_key(&key.raw, key.modifiers.alt_gr) {
                if codepoint < INVALID_CODEPOINT {
                    enc.unicode_key = codepoint;
                }
            }
        }
''',
    '''        } else if functional == KITTY_TEXT_SENTINEL
            && let Some(codepoint) = mapper.kitty_base_key(&key.raw, key.modifiers.alt_gr)
            && codepoint < INVALID_CODEPOINT
        {
            enc.unicode_key = codepoint;
        }
''',
    "base key collapse",
)

replace_once(
    '''        if functional == KITTY_TEXT_SENTINEL && key.modifiers.shift {
            if let Some(codepoint) = mapper.kitty_shifted_key(&key.raw, key.modifiers.alt_gr) {
                if codepoint < INVALID_CODEPOINT {
                    enc.shifted_key = codepoint;
                }
            }
        }
        if key.raw.scan_code != 0 {
            if let Some(codepoint) = mapper.kitty_us_base_key(&key.raw) {
                if codepoint < INVALID_CODEPOINT && codepoint != enc.unicode_key {
                    enc.base_layout_key = codepoint;
                }
            }
        }
''',
    '''        if functional == KITTY_TEXT_SENTINEL
            && key.modifiers.shift
            && let Some(codepoint) = mapper.kitty_shifted_key(&key.raw, key.modifiers.alt_gr)
            && codepoint < INVALID_CODEPOINT
        {
            enc.shifted_key = codepoint;
        }
        if key.raw.scan_code != 0
            && let Some(codepoint) = mapper.kitty_us_base_key(&key.raw)
            && codepoint < INVALID_CODEPOINT
            && codepoint != enc.unicode_key
        {
            enc.base_layout_key = codepoint;
        }
''',
    "alternate key collapse",
)

replace_once(
    "fn encode_regular_special(input: &TerminalInput, key: &SanitizedKeyEvent) -> Option<String> {",
    '''#[expect(
    clippy::too_many_lines,
    reason = "VT key dispatch stays contiguous for direct Microsoft parity review"
)]
fn encode_regular_special(input: &TerminalInput, key: &SanitizedKeyEvent) -> Option<String> {''',
    "protocol dispatch expectation",
)

replace_once(
    '''            } else if modifier == 0 && event_type == 0 && !kitty_regular {
                Some(format!("{}{final_character}", input.csi_prefix()))
            } else if modifier == 0 && event_type == 0 {
                Some(format!("{}{final_character}", input.csi_prefix()))
            } else {
''',
    '''            } else if modifier == 0 && event_type == 0 {
                Some(format!("{}{final_character}", input.csi_prefix()))
            } else {
''',
    "duplicate cursor branch",
)

replace_once(
    '''            let final_character = char::from(b'p' + (key.raw.virtual_key - virtual_key::NUMPAD0) as u8);
''',
    '''            let final_character = char::from_u32(
                u32::from(b'p') + u32::from(key.raw.virtual_key - virtual_key::NUMPAD0),
            )
            .unwrap_or('p');
''',
    "keypad digit conversion",
)
replace_once(
    '''            let final_character = char::from(b'j' + (key.raw.virtual_key - virtual_key::MULTIPLY) as u8);
''',
    '''            let final_character = char::from_u32(
                u32::from(b'j') + u32::from(key.raw.virtual_key - virtual_key::MULTIPLY),
            )
            .unwrap_or('j');
''',
    "keypad operator conversion",
)

replace_once(
    '''fn modifier_bits(key: &SanitizedKeyEvent) -> u32 {
    u32::from(key.modifiers.shift) | (u32::from(key.modifiers.alt) << 1) | (u32::from(key.modifiers.ctrl) << 2)
}
''',
    '''fn modifier_bits(key: &SanitizedKeyEvent) -> u32 {
    u32::from(key.modifiers.shift)
        | (u32::from(key.modifiers.alt) << 1)
        | (u32::from(key.modifiers.ctrl) << 2)
}
''',
    "modifier formatting",
)

path.write_text(text)
