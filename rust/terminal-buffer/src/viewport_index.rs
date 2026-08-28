//! Safe viewport indexing for LF/VT/FF/IND/NEL when output advances past the
//! visible bottom into scrollback.
//!
//! Vertical margin scrolling is owned by `vertical_scroll`; this module owns
//! the adjacent no-margin case where the viewport pans down over an existing
//! text buffer and the newly exposed row must be erased.

use crate::rect_ops::{erase_rect, scroll_rect, ScreenRect};
use crate::row::RowError;
use crate::text_attribute::TextAttribute;
use crate::text_buffer::{TextBuffer, TextBufferPoint};

/// Advances one row as LF/VT/FF/IND. If the cursor is on the viewport bottom,
/// the viewport pans into the next buffer row when possible; otherwise the
/// visible region scrolls in place at the physical buffer bottom.
pub fn index_down(
    buffer: &mut TextBuffer,
    viewport: &mut ScreenRect,
    cursor: &mut TextBufferPoint,
    erase_source_attribute: TextAttribute,
) -> Result<(), RowError> {
    let top = viewport.top.min(buffer.height());
    let bottom = viewport.bottom.min(buffer.height());
    if top >= bottom {
        return Ok(());
    }

    cursor.y = cursor.y.clamp(top, bottom - 1);
    if cursor.y < bottom - 1 {
        cursor.y += 1;
        return Ok(());
    }

    if bottom < buffer.height() {
        erase_rect(
            buffer,
            ScreenRect::new(0, bottom, buffer.width(), bottom + 1),
            erase_source_attribute,
        )?;
        viewport.top = top + 1;
        viewport.bottom = bottom + 1;
        cursor.y += 1;
        return Ok(());
    }

    let mut erase = erase_source_attribute;
    erase.set_standard_erase();
    if top + 1 < bottom {
        scroll_rect(
            buffer,
            ScreenRect::new(0, top + 1, buffer.width(), bottom),
            TextBufferPoint::new(0, top),
            erase,
        )?;
    }
    erase_rect(
        buffer,
        ScreenRect::new(0, bottom - 1, buffer.width(), bottom),
        erase_source_attribute,
    )?;
    Ok(())
}

/// NEL shares the same indexing behavior and additionally returns to column 0.
pub fn next_line(
    buffer: &mut TextBuffer,
    viewport: &mut ScreenRect,
    cursor: &mut TextBufferPoint,
    erase_source_attribute: TextAttribute,
) -> Result<(), RowError> {
    index_down(buffer, viewport, cursor, erase_source_attribute)?;
    cursor.x = 0;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rect_ops::fill_rect;
    use crate::text_color::Rgb;

    #[test]
    fn index_pans_and_erases_the_newly_exposed_row() {
        let original = TextAttribute::default();
        let active = TextAttribute::from_rgb(Rgb::new(1, 2, 3), Rgb::new(4, 5, 6));
        let mut expected = active;
        expected.set_standard_erase();
        let mut buffer = TextBuffer::new(8, 6, original).unwrap();
        fill_rect(&mut buffer, ScreenRect::new(0, 0, 8, 6), u16::from(b'X'), original)
            .unwrap();
        let mut viewport = ScreenRect::new(0, 0, 8, 4);
        let mut cursor = TextBufferPoint::new(3, 3);

        index_down(&mut buffer, &mut viewport, &mut cursor, active).unwrap();

        assert_eq!(viewport, ScreenRect::new(0, 1, 8, 5));
        assert_eq!(cursor, TextBufferPoint::new(3, 4));
        assert_eq!(buffer.row(4).glyph_at(0), &[u16::from(b' ')]);
        assert_eq!(buffer.row(4).attribute_at(0), expected);
    }
}
