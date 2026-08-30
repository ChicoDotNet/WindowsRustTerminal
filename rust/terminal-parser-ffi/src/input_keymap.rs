use terminal_parser::input_keymap::{cursor_virtual_key, generic_virtual_key, ss3_virtual_key};

/// Maps a CSI final character to a Windows virtual-key value.
/// Returns zero when the sequence is not one of the supported deterministic keys.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_input_cursor_vkey(final_character: u16) -> u16 {
    cursor_virtual_key(final_character).unwrap_or(0)
}

/// Maps a CSI `~` generic-key identifier to a Windows virtual-key value.
/// Returns zero when the identifier is not mapped.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_input_generic_vkey(identifier: i32) -> u16 {
    generic_virtual_key(identifier).unwrap_or(0)
}

/// Maps an SS3 final character to a Windows virtual-key value.
/// Returns zero when the sequence is not one of the supported deterministic keys.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_input_ss3_vkey(final_character: u16) -> u16 {
    ss3_virtual_key(final_character).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{
        terminal_parser_ffi_input_cursor_vkey, terminal_parser_ffi_input_generic_vkey,
        terminal_parser_ffi_input_ss3_vkey,
    };

    #[test]
    fn input_keymap_ffi_preserves_zero_as_not_found() {
        assert_eq!(terminal_parser_ffi_input_cursor_vkey(u16::from(b'A')), 0x26);
        assert_eq!(terminal_parser_ffi_input_cursor_vkey(u16::from(b'X')), 0);
        assert_eq!(terminal_parser_ffi_input_generic_vkey(24), 0x7b);
        assert_eq!(terminal_parser_ffi_input_generic_vkey(22), 0);
        assert_eq!(terminal_parser_ffi_input_ss3_vkey(u16::from(b'P')), 0x70);
        assert_eq!(terminal_parser_ffi_input_ss3_vkey(u16::from(b'X')), 0);
    }
}
