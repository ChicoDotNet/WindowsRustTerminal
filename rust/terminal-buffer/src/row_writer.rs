//! Bulk row writes driven by safe `OutputCellView` values.

use crate::output_cell::{OutputCellView, TextAttributeBehavior};
use crate::row::{DbcsAttribute, Row, RowError};

/// Writes output-cell views into one row and returns the first untouched column.
///
/// Full-width text is written on its leading cell. The corresponding trailing
/// view consumes the second destination column without duplicating the glyph in
/// UTF-16 storage, matching the C++ iterator/ROW split of responsibilities.
///
/// # Errors
///
/// Propagates row storage errors, including a full-width glyph that cannot fit
/// in the final column.
pub fn write_cells<'a, I>(row: &mut Row, start_column: i32, cells: I) -> Result<u16, RowError>
where
    I: IntoIterator<Item = OutputCellView<'a>>,
{
    let start = start_column.clamp(0, i32::from(row.size()));
    let mut column = u16::try_from(start).unwrap_or_default();

    for cell in cells {
        if column >= row.size() {
            break;
        }

        let next_column = column.saturating_add(1);
        match cell.text_attribute_behavior() {
            TextAttributeBehavior::Current => {}
            TextAttributeBehavior::Stored | TextAttributeBehavior::StoredOnly => {
                row.replace_attributes(
                    i32::from(column),
                    i32::from(next_column),
                    cell.text_attribute(),
                );
            }
        }

        if !matches!(cell.text_attribute_behavior(), TextAttributeBehavior::StoredOnly) {
            match cell.dbcs_attribute() {
                DbcsAttribute::Single => {
                    row.replace_glyph(i32::from(column), 1, cell.chars())?;
                }
                DbcsAttribute::Leading => {
                    row.replace_glyph(i32::from(column), 2, cell.chars())?;
                }
                DbcsAttribute::Trailing => {
                    // The leading view already stored the glyph across both columns.
                }
            }
        }

        column = next_column;
    }

    Ok(column)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output_cell::{GlyphWidthDetector, OutputCellIterator};
    use crate::text_attribute::TextAttribute;

    struct TestWidthDetector;

    impl GlyphWidthDetector for TestWidthDetector {
        fn is_full_width(&self, glyph: &[u16]) -> bool {
            glyph == [0x4e00]
        }
    }

    fn row(width: u16) -> Row {
        Row::new(width, TextAttribute::default()).expect("valid test row")
    }

    #[test]
    fn bulk_write_preserves_current_attributes_for_text_only_input() {
        let detector = TestWidthDetector;
        let mut row = row(5);
        let mut highlighted = TextAttribute::default();
        highlighted.set_intense(true);
        row.replace_attributes(0, 5, highlighted);

        let text = [u16::from(b'A'), u16::from(b'B')];
        let end = write_cells(
            &mut row,
            1,
            OutputCellIterator::text_only(&text, &detector),
        )
        .expect("bulk write succeeds");

        assert_eq!(end, 3);
        assert_eq!(row.glyph_at(1), &[u16::from(b'A')]);
        assert_eq!(row.glyph_at(2), &[u16::from(b'B')]);
        assert!(row.attribute_at(1).is_intense());
        assert!(row.attribute_at(2).is_intense());
    }

    #[test]
    fn bulk_write_stores_attribute_with_text_when_requested() {
        let detector = TestWidthDetector;
        let mut row = row(4);
        let mut highlighted = TextAttribute::default();
        highlighted.set_intense(true);
        let text = [u16::from(b'X')];

        write_cells(
            &mut row,
            2,
            OutputCellIterator::text_with_attribute(&text, highlighted, &detector),
        )
        .expect("bulk write succeeds");

        assert_eq!(row.glyph_at(2), &[u16::from(b'X')]);
        assert!(row.attribute_at(2).is_intense());
    }

    #[test]
    fn full_width_iterator_and_row_writer_consume_exactly_two_columns() {
        let detector = TestWidthDetector;
        let mut row = row(5);
        let text = [0x4e00, u16::from(b'Z')];

        let end = write_cells(
            &mut row,
            1,
            OutputCellIterator::text_only(&text, &detector),
        )
        .expect("bulk write succeeds");

        assert_eq!(end, 4);
        assert_eq!(row.glyph_at(1), &[0x4e00]);
        assert_eq!(row.dbcs_attribute_at(1), DbcsAttribute::Leading);
        assert_eq!(row.dbcs_attribute_at(2), DbcsAttribute::Trailing);
        assert_eq!(row.glyph_at(3), &[u16::from(b'Z')]);
    }

    #[test]
    fn bulk_write_stops_at_row_boundary_without_touching_later_input() {
        let detector = TestWidthDetector;
        let mut row = row(3);
        let text = [u16::from(b'A'), u16::from(b'B')];

        let end = write_cells(
            &mut row,
            2,
            OutputCellIterator::text_only(&text, &detector),
        )
        .expect("bulk write succeeds");

        assert_eq!(end, 3);
        assert_eq!(row.glyph_at(2), &[u16::from(b'A')]);
    }
}