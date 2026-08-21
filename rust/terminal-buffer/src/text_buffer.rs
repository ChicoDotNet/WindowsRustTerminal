//! Safe row ownership and circular-buffer semantics for Windows Terminal text storage.
//!
//! The C++ `TextBuffer` keeps a fixed set of rows and rotates the logical top
//! through that storage as the viewport advances. This module keeps the same
//! ownership model without pointer arithmetic or shared mutable aliases.

use crate::row::{Row, RowError};
use crate::text_attribute::TextAttribute;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBufferError {
    EmptyWidth,
    EmptyHeight,
    HeightTooLarge,
    Row(RowError),
}

impl From<RowError> for TextBufferError {
    fn from(value: RowError) -> Self {
        Self::Row(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    rows: Vec<Row>,
    width: u16,
    height: u16,
    first_row: u16,
}

impl TextBuffer {
    /// Creates a fixed-size circular row store.
    ///
    /// # Errors
    ///
    /// Returns an error for zero dimensions, a height that cannot be represented
    /// by the buffer's `u16` row coordinates, or an invalid row width.
    pub fn new(
        width: u16,
        height: u16,
        fill_attribute: TextAttribute,
    ) -> Result<Self, TextBufferError> {
        if width == 0 {
            return Err(TextBufferError::EmptyWidth);
        }
        if height == 0 {
            return Err(TextBufferError::EmptyHeight);
        }

        let mut rows = Vec::with_capacity(usize::from(height));
        for _ in 0..height {
            rows.push(Row::new(width, fill_attribute)?);
        }

        Ok(Self {
            rows,
            width,
            height,
            first_row: 0,
        })
    }

    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }

    #[must_use]
    pub const fn first_row_index(&self) -> u16 {
        self.first_row
    }

    #[must_use]
    pub fn row(&self, logical_y: i32) -> &Row {
        &self.rows[self.physical_index(logical_y)]
    }

    #[must_use]
    pub fn row_mut(&mut self, logical_y: i32) -> &mut Row {
        let index = self.physical_index(logical_y);
        &mut self.rows[index]
    }

    /// Rotates the logical top upward by `count` rows and resets the rows that
    /// become newly visible at the logical bottom.
    pub fn rotate_up(&mut self, count: u16, fill_attribute: TextAttribute) {
        let count = count.min(self.height);
        for _ in 0..count {
            self.first_row = (self.first_row + 1) % self.height;
            let bottom = self.physical_index(i32::from(self.height) - 1);
            self.rows[bottom].reset(fill_attribute);
        }
    }

    /// Rotates the logical top downward by `count` rows and resets the rows that
    /// become newly visible at the logical top.
    pub fn rotate_down(&mut self, count: u16, fill_attribute: TextAttribute) {
        let count = count.min(self.height);
        for _ in 0..count {
            self.first_row = if self.first_row == 0 {
                self.height - 1
            } else {
                self.first_row - 1
            };
            let top = self.physical_index(0);
            self.rows[top].reset(fill_attribute);
        }
    }

    pub fn reset(&mut self, fill_attribute: TextAttribute) {
        self.first_row = 0;
        for row in &mut self.rows {
            row.reset(fill_attribute);
        }
    }

    /// Changes only the row count while preserving the oldest logical rows.
    /// Width-changing reflow remains a separate R04 operation because it must
    /// account for glyph widths and wrapped-line semantics.
    ///
    /// # Errors
    ///
    /// Returns an error for zero height or row allocation failure.
    pub fn resize_height(
        &mut self,
        new_height: u16,
        fill_attribute: TextAttribute,
    ) -> Result<(), TextBufferError> {
        if new_height == 0 {
            return Err(TextBufferError::EmptyHeight);
        }
        if new_height == self.height {
            return Ok(());
        }

        let preserve = self.height.min(new_height);
        let mut rows = Vec::with_capacity(usize::from(new_height));
        for logical_y in 0..preserve {
            rows.push(self.row(i32::from(logical_y)).clone());
        }
        for _ in preserve..new_height {
            rows.push(Row::new(self.width, fill_attribute)?);
        }

        self.rows = rows;
        self.height = new_height;
        self.first_row = 0;
        Ok(())
    }

    #[must_use]
    fn physical_index(&self, logical_y: i32) -> usize {
        let logical_y = logical_y.clamp(0, i32::from(self.height) - 1);
        let logical_y = u16::try_from(logical_y).unwrap_or_default();
        usize::from((self.first_row + logical_y) % self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attribute() -> TextAttribute {
        TextAttribute::default()
    }

    #[test]
    fn creates_fixed_owned_rows() {
        let buffer = TextBuffer::new(8, 3, attribute()).unwrap();
        assert_eq!(buffer.width(), 8);
        assert_eq!(buffer.height(), 3);
        assert_eq!(buffer.first_row_index(), 0);
        assert_eq!(buffer.row(0).size(), 8);
        assert_eq!(buffer.row(99).size(), 8);
    }

    #[test]
    fn rotate_up_reuses_storage_and_clears_new_bottom() {
        let mut buffer = TextBuffer::new(4, 3, attribute()).unwrap();
        buffer
            .row_mut(0)
            .replace_glyph(0, 1, &[u16::from(b'A')])
            .unwrap();
        buffer
            .row_mut(1)
            .replace_glyph(0, 1, &[u16::from(b'B')])
            .unwrap();
        buffer
            .row_mut(2)
            .replace_glyph(0, 1, &[u16::from(b'C')])
            .unwrap();

        buffer.rotate_up(1, attribute());

        assert_eq!(buffer.first_row_index(), 1);
        assert_eq!(buffer.row(0).glyph_at(0), &[u16::from(b'B')]);
        assert_eq!(buffer.row(1).glyph_at(0), &[u16::from(b'C')]);
        assert_eq!(buffer.row(2).glyph_at(0), &[u16::from(b' ')]);
    }

    #[test]
    fn rotate_down_reuses_storage_and_clears_new_top() {
        let mut buffer = TextBuffer::new(4, 3, attribute()).unwrap();
        buffer
            .row_mut(0)
            .replace_glyph(0, 1, &[u16::from(b'A')])
            .unwrap();
        buffer
            .row_mut(1)
            .replace_glyph(0, 1, &[u16::from(b'B')])
            .unwrap();

        buffer.rotate_down(1, attribute());

        assert_eq!(buffer.first_row_index(), 2);
        assert_eq!(buffer.row(0).glyph_at(0), &[u16::from(b' ')]);
        assert_eq!(buffer.row(1).glyph_at(0), &[u16::from(b'A')]);
        assert_eq!(buffer.row(2).glyph_at(0), &[u16::from(b'B')]);
    }

    #[test]
    fn resize_height_preserves_logical_order_across_rotation() {
        let mut buffer = TextBuffer::new(4, 3, attribute()).unwrap();
        buffer
            .row_mut(0)
            .replace_glyph(0, 1, &[u16::from(b'A')])
            .unwrap();
        buffer
            .row_mut(1)
            .replace_glyph(0, 1, &[u16::from(b'B')])
            .unwrap();
        buffer
            .row_mut(2)
            .replace_glyph(0, 1, &[u16::from(b'C')])
            .unwrap();
        buffer.rotate_up(1, attribute());

        buffer.resize_height(4, attribute()).unwrap();

        assert_eq!(buffer.first_row_index(), 0);
        assert_eq!(buffer.row(0).glyph_at(0), &[u16::from(b'B')]);
        assert_eq!(buffer.row(1).glyph_at(0), &[u16::from(b'C')]);
        assert_eq!(buffer.row(2).glyph_at(0), &[u16::from(b' ')]);
        assert_eq!(buffer.row(3).glyph_at(0), &[u16::from(b' ')]);
    }
}
