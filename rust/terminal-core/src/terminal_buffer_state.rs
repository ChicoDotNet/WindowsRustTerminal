//! Aggregate Terminal buffer/viewport state above the portable `TextBuffer` owner.

use terminal_buffer::row::RowError;
use terminal_buffer::row_writer::replace_text;
use terminal_buffer::text_attribute::TextAttribute;
use terminal_buffer::text_buffer::{TextBuffer, TextBufferError};

#[derive(Debug, Clone)]
pub struct TerminalBufferState {
    buffer: TextBuffer,
    viewport_height: u16,
    history_rows: u16,
    mutable_viewport_top: u16,
    scroll_offset: u16,
    line_feeds: u32,
}

impl TerminalBufferState {
    /// Creates a terminal buffer with a visible viewport and scrollback history.
    ///
    /// # Errors
    ///
    /// Returns [`TextBufferError`] when the requested backing buffer dimensions
    /// cannot be represented by the portable `TextBuffer` owner.
    pub fn new(
        width: u16,
        viewport_height: u16,
        history_rows: u16,
    ) -> Result<Self, TextBufferError> {
        let total_height = viewport_height.saturating_add(history_rows).max(1);
        Ok(Self {
            buffer: TextBuffer::new(width, total_height, TextAttribute::default())?,
            viewport_height: viewport_height.max(1),
            history_rows,
            mutable_viewport_top: 0,
            scroll_offset: 0,
            line_feeds: 0,
        })
    }

    #[must_use]
    pub const fn buffer(&self) -> &TextBuffer {
        &self.buffer
    }

    #[must_use]
    pub const fn viewport_top(&self) -> u16 {
        self.mutable_viewport_top.saturating_sub(self.scroll_offset)
    }

    #[must_use]
    pub const fn viewport_bottom_exclusive(&self) -> u16 {
        self.viewport_top().saturating_add(self.viewport_height)
    }

    #[must_use]
    pub const fn scroll_offset(&self) -> u16 {
        self.scroll_offset
    }

    pub fn write_text_at(&mut self, x: i32, y: i32, text: &[u16]) -> Result<u16, RowError> {
        replace_text(self.buffer.row_mut(y), x, text)
    }

    pub fn line_feed(&mut self) {
        self.line_feeds = self.line_feeds.saturating_add(1);
        if self.line_feeds < u32::from(self.viewport_height) {
            return;
        }

        if self.mutable_viewport_top < self.history_rows {
            self.mutable_viewport_top += 1;
            if self.scroll_offset > 0 {
                self.scroll_offset = self.scroll_offset.saturating_add(1).min(self.history_rows);
            }
        } else if self.scroll_offset > 0 {
            self.scroll_offset = self.scroll_offset.saturating_add(1).min(self.history_rows);
        }
    }

    pub fn set_scroll_offset(&mut self, offset: u16) {
        self.scroll_offset = offset.min(self.history_rows).min(self.mutable_viewport_top);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utf16(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    #[test]
    fn microsoft_terminal_buffer_simple_writing_contract() {
        let mut terminal = TerminalBufferState::new(80, 32, 100).expect("valid terminal");
        assert_eq!(terminal.viewport_top(), 0);
        assert_eq!(terminal.viewport_bottom_exclusive(), 32);

        let expected = utf16("Hello World");
        terminal
            .write_text_at(0, 0, &expected)
            .expect("source text fits");

        assert_eq!(terminal.viewport_top(), 0);
        assert_eq!(terminal.viewport_bottom_exclusive(), 32);
        for (column, code_unit) in expected.into_iter().enumerate() {
            assert_eq!(
                terminal.buffer().row(0).glyph_at(column as i32),
                &[code_unit],
                "column {column} must preserve Microsoft's source string"
            );
        }
    }

    #[test]
    fn microsoft_terminal_buffer_dont_snap_to_output_contract() {
        const HEIGHT: u16 = 32;
        const HISTORY: u16 = 100;
        let mut terminal = TerminalBufferState::new(80, HEIGHT, HISTORY).expect("valid terminal");

        assert_eq!(terminal.viewport_top(), 0);
        assert_eq!(terminal.viewport_bottom_exclusive(), HEIGHT);
        assert_eq!(terminal.scroll_offset(), 0);

        for _ in 0..(u32::from(HEIGHT) + 8 - 1) {
            terminal.line_feed();
        }
        assert_eq!(terminal.viewport_top(), 8);
        assert_eq!(terminal.viewport_bottom_exclusive(), HEIGHT + 8);
        assert_eq!(terminal.scroll_offset(), 0);

        terminal.set_scroll_offset(1);
        assert_eq!(terminal.viewport_top(), 7);
        assert_eq!(terminal.viewport_bottom_exclusive(), HEIGHT + 7);
        assert_eq!(terminal.scroll_offset(), 1);

        for _ in 0..8 {
            terminal.line_feed();
        }
        assert_eq!(terminal.viewport_top(), 7);
        assert_eq!(terminal.viewport_bottom_exclusive(), HEIGHT + 7);
        assert_eq!(terminal.scroll_offset(), 9);

        while terminal.mutable_viewport_top < HISTORY {
            terminal.line_feed();
        }
        assert_eq!(terminal.viewport_top(), 7);
        assert_eq!(terminal.viewport_bottom_exclusive(), HEIGHT + 7);
        assert_eq!(terminal.scroll_offset(), HISTORY - 7);

        for _ in 0..3 {
            terminal.line_feed();
        }
        assert_eq!(terminal.viewport_top(), 4);
        assert_eq!(terminal.viewport_bottom_exclusive(), HEIGHT + 4);
        assert_eq!(terminal.scroll_offset(), HISTORY - 4);

        for _ in 0..8 {
            terminal.line_feed();
        }
        assert_eq!(terminal.viewport_top(), 0);
        assert_eq!(terminal.viewport_bottom_exclusive(), HEIGHT);
        assert_eq!(terminal.scroll_offset(), HISTORY);
    }
}
