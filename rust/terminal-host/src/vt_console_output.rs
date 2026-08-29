//! Cursor-preserving `WriteConsoleOutputW` projection for VT hosts.
//!
//! The native API still owns source rectangle validation and `CHAR_INFO` acquisition.
//! This module owns the deterministic writer transaction once those values reach Rust:
//! save the caller cursor, serialize the requested cells at the target origin, and restore it.

use crate::vt_char_info::{HostCharInfo, write_infos};
use crate::vt_writer_sequences::{restore_cursor, save_cursor};

/// Serializes a `CHAR_INFO` run while preserving the caller's cursor, matching
/// the corked writer transaction exercised by Microsoft's `WriteConsoleOutputW` test.
#[must_use]
pub fn write_infos_preserving_cursor(
    target_x: i32,
    target_y: i32,
    infos: &[HostCharInfo],
) -> Vec<u8> {
    let body = write_infos(target_x, target_y, infos);
    let mut output =
        Vec::with_capacity(save_cursor().len() + body.len() + restore_cursor().len());
    output.extend_from_slice(save_cursor());
    output.extend_from_slice(&body);
    output.extend_from_slice(restore_cursor());
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOREGROUND_BLUE: u16 = 0x0001;
    const FOREGROUND_RED: u16 = 0x0004;
    const BACKGROUND_GREEN: u16 = 0x0020;

    #[test]
    fn microsoft_vt_io_write_console_output_w_matches_exact_vector() {
        let red = FOREGROUND_RED | BACKGROUND_GREEN;
        let blue = FOREGROUND_BLUE | BACKGROUND_GREEN;
        let infos = [
            HostCharInfo::new(u16::from(b'a'), red),
            HostCharInfo::new(u16::from(b'b'), red),
            HostCharInfo::new(u16::from(b'A'), blue),
            HostCharInfo::new(u16::from(b'B'), blue),
        ];

        assert_eq!(
            write_infos_preserving_cursor(1, 1, &infos),
            b"\x1b\x37\x1b[2;2H\x1b[0;31;42mab\x1b[0;34;42mAB\x1b\x38"
        );
    }
}
