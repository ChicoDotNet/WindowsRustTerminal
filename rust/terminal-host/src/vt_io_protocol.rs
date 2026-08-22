//! Deterministic protocol/configuration decisions from host `VtIo`.
//!
//! Writing bytes to Windows handles, waiting for DA1, and mutating global
//! console services remain platform-owned boundaries. This module preserves the
//! pure choices that precede those operations.

/// Text measurement modes selected by `VtIo::Initialize`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextMeasurementMode {
    Graphemes,
    Wcswidth,
    Console,
}

/// Maps the optional conhost text-measurement argument to the mode selected by
/// `VtIo::Initialize`.
///
/// Empty input leaves the existing mode untouched. Any non-empty value not
/// explicitly recognized by the C++ implementation falls back to graphemes.
#[must_use]
pub fn text_measurement_mode(value: &str) -> Option<TextMeasurementMode> {
    if value.is_empty() {
        None
    } else {
        Some(match value {
            "wcswidth" => TextMeasurementMode::Wcswidth,
            "console" => TextMeasurementMode::Console,
            _ => TextMeasurementMode::Graphemes,
        })
    }
}

/// Width override applied to ambiguous codepoints by `VtIo::Initialize`.
#[must_use]
pub const fn ambiguous_width_override(ambiguous_is_wide: bool) -> Option<u8> {
    if ambiguous_is_wide { Some(2) } else { None }
}

/// Produces the startup negotiation written by `VtIo::StartIfNeeded`.
///
/// When cursor inheritance is requested, the cursor-position report request is
/// emitted before DA1 so the host can use the later DA1 response as the wait
/// boundary for both requests.
#[must_use]
pub fn startup_negotiation(inherit_cursor: bool) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(if inherit_cursor { 26 } else { 22 });
    if inherit_cursor {
        bytes.extend_from_slice(b"\x1b[6n");
    }
    bytes.extend_from_slice(b"\x1b[c\x1b[?1004h\x1b[?9001h");
    bytes
}

/// Reset sequences written by `VtIo::Shutdown` while the lifecycle is running.
#[must_use]
pub const fn shutdown_negotiation() -> &'static [u8] {
    b"\x1b[?1004l\x1b[?9001l"
}

/// Returns true for C0 controls and single-character C1 controls.
///
/// This is the semantic equivalent of the local `IsControlCharacter` helper in
/// `VtIo.cpp`; the C++ bitwise expression is an optimization of these ranges.
#[must_use]
pub const fn is_control_character(value: u16) -> bool {
    value <= 0x1f || (value >= 0x7f && value <= 0x9f)
}

/// Sanitizes one UTF-16 code unit using the legacy host `SanitizeUCS2` contract.
///
/// C0 controls and DEL use the historical code page 437 display glyphs, C1
/// controls become `?`, and isolated surrogate code units become U+FFFD.
#[must_use]
pub const fn sanitize_ucs2(value: u16) -> u16 {
    const C0_GLYPHS: [u16; 32] = [
        0x0020, 0x263a, 0x263b, 0x2665, 0x2666, 0x2663, 0x2660, 0x2022, 0x25d8, 0x25cb,
        0x25d9, 0x2642, 0x2640, 0x266a, 0x266b, 0x263c, 0x25ba, 0x25c4, 0x2195, 0x203c,
        0x00b6, 0x00a7, 0x25ac, 0x21a8, 0x2191, 0x2193, 0x2192, 0x2190, 0x221f, 0x2194,
        0x25b2, 0x25bc,
    ];

    if value < 0x20 {
        C0_GLYPHS[value as usize]
    } else if value == 0x7f {
        0x2302
    } else if value > 0x7f && value < 0xa0 {
        0x003f
    } else if value.wrapping_sub(0xd800) <= 0x07ff {
        0xfffd
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_measurement_mapping_matches_vt_io() {
        assert_eq!(text_measurement_mode(""), None);
        assert_eq!(
            text_measurement_mode("wcswidth"),
            Some(TextMeasurementMode::Wcswidth)
        );
        assert_eq!(
            text_measurement_mode("console"),
            Some(TextMeasurementMode::Console)
        );
        assert_eq!(
            text_measurement_mode("graphemes"),
            Some(TextMeasurementMode::Graphemes)
        );
        assert_eq!(
            text_measurement_mode("future-value"),
            Some(TextMeasurementMode::Graphemes)
        );
    }

    #[test]
    fn ambiguous_width_override_only_applies_when_requested() {
        assert_eq!(ambiguous_width_override(false), None);
        assert_eq!(ambiguous_width_override(true), Some(2));
    }

    #[test]
    fn startup_negotiation_preserves_sequence_order() {
        assert_eq!(
            startup_negotiation(false),
            b"\x1b[c\x1b[?1004h\x1b[?9001h"
        );
        assert_eq!(
            startup_negotiation(true),
            b"\x1b[6n\x1b[c\x1b[?1004h\x1b[?9001h"
        );
    }

    #[test]
    fn shutdown_negotiation_disables_focus_and_win32_input() {
        assert_eq!(shutdown_negotiation(), b"\x1b[?1004l\x1b[?9001l");
    }

    #[test]
    fn control_character_ranges_match_cpp_semantics() {
        assert!(is_control_character(0x00));
        assert!(is_control_character(0x1f));
        assert!(!is_control_character(0x20));
        assert!(!is_control_character(0x7e));
        assert!(is_control_character(0x7f));
        assert!(is_control_character(0x9f));
        assert!(!is_control_character(0xa0));
        assert!(!is_control_character(0xffff));
    }

    #[test]
    fn sanitize_ucs2_matches_legacy_display_contract() {
        assert_eq!(sanitize_ucs2(0x00), 0x0020);
        assert_eq!(sanitize_ucs2(0x01), 0x263a);
        assert_eq!(sanitize_ucs2(0x1f), 0x25bc);
        assert_eq!(sanitize_ucs2(0x20), 0x20);
        assert_eq!(sanitize_ucs2(0x7f), 0x2302);
        assert_eq!(sanitize_ucs2(0x80), 0x003f);
        assert_eq!(sanitize_ucs2(0x9f), 0x003f);
        assert_eq!(sanitize_ucs2(0xa0), 0xa0);
        assert_eq!(sanitize_ucs2(0xd800), 0xfffd);
        assert_eq!(sanitize_ucs2(0xdfff), 0xfffd);
        assert_eq!(sanitize_ucs2(0x2603), 0x2603);
    }
}
