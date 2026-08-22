//! Deterministic screen-state serialization from `VtIo::Writer::WriteScreenInfo`.
//!
//! Windows still owns buffer reads, resizing, alternate-buffer discovery, and
//! output handles. This module ports only the ordering and VT byte generation
//! once a platform-neutral screen snapshot has already been captured.

use crate::attribute_format::format_attributes;
use crate::vt_char_info::{HostCharInfo, write_infos};
use crate::vt_writer_sequences::{alternate_screen_buffer, cup, decawm, dectcem};
use terminal_buffer::text_attribute::{LegacyColorDefaults, TextAttribute};

/// Platform-neutral screen state consumed by the deterministic writer path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScreenSnapshot {
    pub rows: Vec<Vec<HostCharInfo>>,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub attributes: u16,
    pub cursor_visible: bool,
    pub wrap_at_eol: bool,
}

impl ScreenSnapshot {
    #[must_use]
    pub fn new(
        rows: Vec<Vec<HostCharInfo>>,
        cursor_x: i32,
        cursor_y: i32,
        attributes: u16,
        cursor_visible: bool,
        wrap_at_eol: bool,
    ) -> Self {
        Self {
            rows,
            cursor_x,
            cursor_y,
            attributes,
            cursor_visible,
            wrap_at_eol,
        }
    }
}

/// Serializes the main screen and optional alternate screen in the same order
/// as `VtIo::Writer::WriteScreenInfo` after the Windows-owned read/resize work.
#[must_use]
pub fn write_screen_info(main: &ScreenSnapshot, alternate: Option<&ScreenSnapshot>) -> Vec<u8> {
    let mut output = alternate_screen_buffer(false);
    append_snapshot(&mut output, main);

    if let Some(alternate) = alternate {
        output.extend_from_slice(&alternate_screen_buffer(true));
        append_snapshot(&mut output, alternate);
    }

    output
}

fn append_snapshot(output: &mut Vec<u8>, snapshot: &ScreenSnapshot) {
    for (y, row) in snapshot.rows.iter().enumerate() {
        let y = i32::try_from(y).expect("screen row index must fit in an i32 coordinate");
        output.extend_from_slice(&write_infos(0, y, row));
    }

    output.extend_from_slice(&cup(snapshot.cursor_x, snapshot.cursor_y));
    let attributes =
        TextAttribute::from_legacy(snapshot.attributes, LegacyColorDefaults::default());
    output.extend_from_slice(format_attributes(attributes).as_bytes());
    output.extend_from_slice(&dectcem(snapshot.cursor_visible));
    output.extend_from_slice(&decawm(snapshot.wrap_at_eol));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_ATTRS: u16 = 0x0007;

    fn snapshot(ch: u8, cursor_visible: bool, wrap_at_eol: bool) -> ScreenSnapshot {
        ScreenSnapshot::new(
            vec![vec![HostCharInfo::new(u16::from(ch), DEFAULT_ATTRS)]],
            0,
            0,
            DEFAULT_ATTRS,
            cursor_visible,
            wrap_at_eol,
        )
    }

    #[test]
    fn main_buffer_is_selected_before_dump() {
        let main = snapshot(b'M', true, true);

        assert_eq!(
            write_screen_info(&main, None),
            b"\x1b[?1049l\x1b[1;1H\x1b[0mM\x1b[1;1H\x1b[0m\x1b[?25h\x1b[?7h"
        );
    }

    #[test]
    fn alternate_buffer_is_selected_and_dumped_after_main() {
        let main = snapshot(b'M', true, true);
        let alternate = snapshot(b'A', false, false);

        assert_eq!(
            write_screen_info(&main, Some(&alternate)),
            b"\x1b[?1049l\x1b[1;1H\x1b[0mM\x1b[1;1H\x1b[0m\x1b[?25h\x1b[?7h\x1b[?1049h\x1b[1;1H\x1b[0mA\x1b[1;1H\x1b[0m\x1b[?25l\x1b[?7l"
        );
    }

    #[test]
    fn rows_are_emitted_top_to_bottom_before_cursor_and_modes() {
        let screen = ScreenSnapshot::new(
            vec![
                vec![HostCharInfo::new(u16::from(b'A'), DEFAULT_ATTRS)],
                vec![HostCharInfo::new(u16::from(b'B'), DEFAULT_ATTRS)],
            ],
            3,
            1,
            DEFAULT_ATTRS,
            true,
            false,
        );

        assert_eq!(
            write_screen_info(&screen, None),
            b"\x1b[?1049l\x1b[1;1H\x1b[0mA\x1b[2;1H\x1b[0mB\x1b[2;4H\x1b[0m\x1b[?25h\x1b[?7l"
        );
    }
}
