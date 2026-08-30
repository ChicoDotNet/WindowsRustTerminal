//! Deterministic VT input key mappings shared by the Rust parser and product FFI.
//!
//! These tables correspond to the portable portions of the C++
//! `InputStateMachineEngine` CSI/Generic/SS3 key maps. Windows-specific scan
//! code generation and `INPUT_RECORD` synthesis remain native.

const VK_PRIOR: u16 = 0x21;
const VK_NEXT: u16 = 0x22;
const VK_END: u16 = 0x23;
const VK_HOME: u16 = 0x24;
const VK_LEFT: u16 = 0x25;
const VK_UP: u16 = 0x26;
const VK_RIGHT: u16 = 0x27;
const VK_DOWN: u16 = 0x28;
const VK_INSERT: u16 = 0x2d;
const VK_DELETE: u16 = 0x2e;
const VK_F1: u16 = 0x70;
const VK_F2: u16 = 0x71;
const VK_F3: u16 = 0x72;
const VK_F4: u16 = 0x73;
const VK_F5: u16 = 0x74;
const VK_F6: u16 = 0x75;
const VK_F7: u16 = 0x76;
const VK_F8: u16 = 0x77;
const VK_F9: u16 = 0x78;
const VK_F10: u16 = 0x79;
const VK_F11: u16 = 0x7a;
const VK_F12: u16 = 0x7b;

/// Maps a single-byte CSI final character to the Windows virtual-key value.
#[must_use]
pub const fn cursor_virtual_key(final_character: u16) -> Option<u16> {
    match final_character {
        b'A' as u16 => Some(VK_UP),
        b'B' as u16 => Some(VK_DOWN),
        b'C' as u16 => Some(VK_RIGHT),
        b'D' as u16 => Some(VK_LEFT),
        b'H' as u16 => Some(VK_HOME),
        b'F' as u16 => Some(VK_END),
        b'P' as u16 => Some(VK_F1),
        b'Q' as u16 => Some(VK_F2),
        b'R' as u16 => Some(VK_F3),
        b'S' as u16 => Some(VK_F4),
        _ => None,
    }
}

/// Maps a CSI `~` generic-key identifier to the Windows virtual-key value.
#[must_use]
pub const fn generic_virtual_key(identifier: i32) -> Option<u16> {
    match identifier {
        1 => Some(VK_HOME),
        2 => Some(VK_INSERT),
        3 => Some(VK_DELETE),
        4 => Some(VK_END),
        5 => Some(VK_PRIOR),
        6 => Some(VK_NEXT),
        15 => Some(VK_F5),
        17 => Some(VK_F6),
        18 => Some(VK_F7),
        19 => Some(VK_F8),
        20 => Some(VK_F9),
        21 => Some(VK_F10),
        23 => Some(VK_F11),
        24 => Some(VK_F12),
        _ => None,
    }
}

/// Maps a single-byte SS3 final character to the Windows virtual-key value.
#[must_use]
pub const fn ss3_virtual_key(final_character: u16) -> Option<u16> {
    cursor_virtual_key(final_character)
}

#[cfg(test)]
mod tests {
    use super::{cursor_virtual_key, generic_virtual_key, ss3_virtual_key};

    #[test]
    fn cursor_map_matches_microsoft_input_engine_table() {
        let expected = [
            (b'A', 0x26), (b'B', 0x28), (b'C', 0x27), (b'D', 0x25),
            (b'H', 0x24), (b'F', 0x23), (b'P', 0x70), (b'Q', 0x71),
            (b'R', 0x72), (b'S', 0x73),
        ];
        for (final_character, vkey) in expected {
            assert_eq!(cursor_virtual_key(u16::from(final_character)), Some(vkey));
        }
        assert_eq!(cursor_virtual_key(u16::from(b'X')), None);
    }

    #[test]
    fn generic_map_matches_microsoft_input_engine_table() {
        let expected = [
            (1, 0x24), (2, 0x2d), (3, 0x2e), (4, 0x23), (5, 0x21), (6, 0x22),
            (15, 0x74), (17, 0x75), (18, 0x76), (19, 0x77), (20, 0x78),
            (21, 0x79), (23, 0x7a), (24, 0x7b),
        ];
        for (identifier, vkey) in expected {
            assert_eq!(generic_virtual_key(identifier), Some(vkey));
        }
        assert_eq!(generic_virtual_key(22), None);
    }

    #[test]
    fn ss3_map_matches_microsoft_input_engine_table() {
        assert_eq!(ss3_virtual_key(u16::from(b'A')), Some(0x26));
        assert_eq!(ss3_virtual_key(u16::from(b'S')), Some(0x73));
        assert_eq!(ss3_virtual_key(u16::from(b'X')), None);
    }
}
