//! DEC macro status-report serialization shared by the adapter response path.
//!
//! The macro buffer owns storage and checksum semantics. This module owns only
//! the exact VT report framing required by DSR 62 (macro space) and DSR 63
//! (macro memory checksum), keeping storage accounting out of the response
//! engine while making the eventual live dispatch wiring trivial.

use crate::macro_buffer::{MAX_SPACE, MacroBuffer};

/// DEC reports macro space in blocks of 16 bytes via `CSI Ps * {`.
#[must_use]
pub fn macro_space_report(buffer: &MacroBuffer) -> String {
    let available_blocks = buffer.space_available() / 16;
    format!("\u{1b}[{available_blocks}*{{")
}

/// DEC reports the macro-memory checksum as `DCS id ! ~ hhhh ST`.
#[must_use]
pub fn macro_checksum_report(buffer: &MacroBuffer, request_id: i32) -> String {
    let request_id = request_id.max(0);
    format!(
        "\u{1b}P{request_id}!~{:04X}\u{1b}\\",
        buffer.calculate_checksum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macro_buffer::{MacroDeleteControl, MacroEncoding};

    fn define_text_macro(buffer: &mut MacroBuffer, id: usize, text: &str) {
        assert!(buffer.init_parser(id, MacroDeleteControl::DeleteId, MacroEncoding::Text));
        for unit in text.encode_utf16() {
            assert!(buffer.parse_definition(unit));
        }
        assert!(!buffer.parse_definition(0x1b));
    }

    #[test]
    fn microsoft_macro_space_report_uses_sixteen_byte_blocks() {
        let mut buffer = MacroBuffer::default();
        assert_eq!(macro_space_report(&buffer), format!("\u{1b}[{}*{{", MAX_SPACE / 16));

        // Microsoft defines four eight-byte macros: 32 bytes total, therefore
        // two 16-byte report blocks are consumed.
        for id in 1..=4 {
            define_text_macro(&mut buffer, id, "12345678");
        }
        assert_eq!(
            macro_space_report(&buffer),
            format!("\u{1b}[{}*{{", (MAX_SPACE / 16) - 2)
        );
    }

    #[test]
    fn microsoft_macro_memory_checksum_report_matches_dsr_63_framing() {
        let mut buffer = MacroBuffer::default();
        assert_eq!(macro_checksum_report(&buffer, 12), "\u{1b}P12!~0000\u{1b}\\");

        define_text_macro(&mut buffer, 1, "ABC");
        let expected = format!(
            "\u{1b}P12!~{:04X}\u{1b}\\",
            buffer.calculate_checksum()
        );
        assert_eq!(macro_checksum_report(&buffer, 12), expected);
    }

    #[test]
    fn macro_checksum_report_clamps_negative_request_ids() {
        let buffer = MacroBuffer::default();
        assert_eq!(macro_checksum_report(&buffer, -1), "\u{1b}P0!~0000\u{1b}\\");
    }
}
