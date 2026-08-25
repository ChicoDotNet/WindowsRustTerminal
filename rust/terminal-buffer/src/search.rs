//! Unicode text search over terminal-buffer rows with cell-coordinate results.
//!
//! Windows Terminal stores row text as UTF-16 while exposing search results in
//! terminal cell coordinates. A Unicode scalar may therefore consume two UTF-16
//! code units but one cell, while an East Asian wide glyph consumes one scalar
//! and two cells. This module searches the stored UTF-16 representation and maps
//! each match back through the row's validated character-to-cell offsets.

use crate::text_buffer::{TextBuffer, TextBufferPoint};

/// An end-exclusive match span in logical terminal-buffer coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextSearchSpan {
    pub start: TextBufferPoint,
    pub end: TextBufferPoint,
}

impl TextSearchSpan {
    #[must_use]
    pub const fn new(start: TextBufferPoint, end: TextBufferPoint) -> Self {
        Self { start, end }
    }
}

impl TextBuffer {
    /// Finds exact UTF-16 substring matches and returns terminal-cell spans.
    ///
    /// Matches are currently row-local, which is the portable ownership seam
    /// exercised by Microsoft's `UTextAdapterTests::Unicode` contract. Host-level
    /// search flags, case folding and cross-row orchestration remain separate
    /// integration responsibilities.
    #[must_use]
    pub fn search_text(&self, needle: &[u16]) -> Vec<TextSearchSpan> {
        if needle.is_empty() {
            return Vec::new();
        }

        let mut matches = Vec::new();
        for y in 0..self.height() {
            let row = self.row(i32::from(y));
            let haystack = row.text();
            if needle.len() > haystack.len() {
                continue;
            }

            for char_begin in 0..=haystack.len() - needle.len() {
                let char_end = char_begin + needle.len();
                if haystack[char_begin..char_end] != *needle {
                    continue;
                }

                let start_x = row.leading_column_at_char_offset(
                    isize::try_from(char_begin).unwrap_or(isize::MAX),
                );
                let end_x = row
                    .leading_column_at_char_offset(isize::try_from(char_end).unwrap_or(isize::MAX));
                matches.push(TextSearchSpan::new(
                    TextBufferPoint::new(start_x, y),
                    TextBufferPoint::new(end_x, y),
                ));
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::row_writer::replace_text;
    use crate::text_attribute::TextAttribute;

    fn utf16(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    fn span(begin: u16, end: u16) -> TextSearchSpan {
        TextSearchSpan::new(TextBufferPoint::new(begin, 0), TextBufferPoint::new(end, 0))
    }

    #[test]
    fn microsoft_utext_adapter_unicode_matches_source_contract() {
        let mut buffer = TextBuffer::new(24, 1, TextAttribute::default()).expect("valid buffer");
        replace_text(buffer.row_mut(0), 0, &utf16("abc 𝒶𝒷𝒸 abc ネコちゃん"))
            .expect("Microsoft source text fits the 24-cell row");

        assert_eq!(
            buffer.search_text(&utf16("abc")),
            vec![span(0, 3), span(8, 11)]
        );
        assert_eq!(buffer.search_text(&utf16("𝒷")), vec![span(5, 6)]);
        assert_eq!(buffer.search_text(&utf16("ネコ")), vec![span(12, 16)]);
    }

    #[test]
    fn empty_needle_does_not_create_synthetic_matches() {
        let buffer = TextBuffer::new(4, 1, TextAttribute::default()).expect("valid buffer");
        assert!(buffer.search_text(&[]).is_empty());
    }
}
