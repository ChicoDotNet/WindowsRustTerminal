//! Deterministic Unicode width classification and text measurement.
//!
//! Windows Terminal keeps codepoint-width detection separate from the row
//! storage engine. This module preserves that boundary while providing a
//! platform-neutral implementation for both cell classification and the
//! Unicode text-measurement policies exercised by the Microsoft contract.

use crate::output_cell::GlyphWidthDetector;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

/// Stateless width detector used by the output-cell iterator.
///
/// Ambiguous-width characters remain narrow here. Stateful text measurement
/// lives in [`TextMeasurementEngine`] so existing buffer consumers do not gain
/// process-global or mutable width policy.
#[derive(Debug, Clone, Copy, Default)]
pub struct CodepointWidthDetector;

impl GlyphWidthDetector for CodepointWidthDetector {
    fn is_full_width(&self, glyph: &[u16]) -> bool {
        decode_first_scalar(glyph).is_some_and(is_unambiguous_wide)
    }
}

/// The three measurement policies exposed by Microsoft's
/// `CodepointWidthDetector`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextMeasurementMode {
    /// UAX #29 extended grapheme clustering with terminal cell widths.
    #[default]
    Graphemes,
    /// A non-zero-width scalar followed by zero-width scalars forms a cluster.
    Wcswidth,
    /// Legacy conhost-style scalar measurement.
    Console,
}

/// One measured cluster, expressed in UTF-16 units just like the Microsoft
/// source contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextMeasurement {
    pub utf16_len: usize,
    pub width: u8,
}

/// Stateful policy owner corresponding to the mutable portion of Microsoft's
/// `CodepointWidthDetector`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextMeasurementEngine {
    mode: TextMeasurementMode,
    ambiguous_width: u8,
}

impl Default for TextMeasurementEngine {
    fn default() -> Self {
        Self {
            mode: TextMeasurementMode::Graphemes,
            ambiguous_width: 1,
        }
    }
}

impl TextMeasurementEngine {
    #[must_use]
    pub const fn mode(self) -> TextMeasurementMode {
        self.mode
    }

    #[must_use]
    pub const fn ambiguous_width(self) -> u8 {
        self.ambiguous_width
    }

    pub const fn set_ambiguous_width(&mut self, width: u8) {
        self.ambiguous_width = width;
    }

    pub const fn reset(&mut self, mode: TextMeasurementMode) {
        self.mode = mode;
    }

    #[must_use]
    pub fn scalar_width(self, value: char) -> u8 {
        let narrow = UnicodeWidthChar::width(value).unwrap_or(0) as u8;
        let cjk = UnicodeWidthChar::width_cjk(value).unwrap_or(0) as u8;
        if narrow != cjk {
            self.ambiguous_width
        } else {
            narrow
        }
    }

    #[must_use]
    pub fn grapheme_width(self, grapheme: &str) -> u8 {
        grapheme
            .chars()
            .map(|value| {
                if value == '\u{fe0f}' {
                    2
                } else {
                    self.scalar_width(value)
                }
            })
            .fold(0_u8, u8::saturating_add)
            .min(2)
    }

    /// Measures a complete UTF-16 string as UAX #29 grapheme clusters.
    ///
    /// Invalid UTF-16 is replaced by U+FFFD, matching Microsoft's explicit
    /// malformed-scalar fallback in the detector implementation.
    #[must_use]
    pub fn graphemes_forward(self, text: &[u16]) -> Vec<TextMeasurement> {
        let text = String::from_utf16_lossy(text);
        UnicodeSegmentation::graphemes(text.as_str(), true)
            .map(|grapheme| TextMeasurement {
                utf16_len: grapheme.encode_utf16().count(),
                width: self.grapheme_width(grapheme),
            })
            .collect()
    }

    #[must_use]
    pub fn graphemes_backward(self, text: &[u16]) -> Vec<TextMeasurement> {
        let mut measured = self.graphemes_forward(text);
        measured.reverse();
        measured
    }
}

fn decode_first_scalar(glyph: &[u16]) -> Option<u32> {
    let first = u32::from(*glyph.first()?);
    if (0xd800..=0xdbff).contains(&first) {
        let second = u32::from(*glyph.get(1)?);
        if !(0xdc00..=0xdfff).contains(&second) {
            return None;
        }
        return Some(0x1_0000 + ((first - 0xd800) << 10) + (second - 0xdc00));
    }
    if (0xdc00..=0xdfff).contains(&first) {
        return None;
    }
    Some(first)
}

/// Returns whether a Unicode scalar has an unambiguous two-column terminal
/// presentation according to the East Asian wide/full-width repertoire plus
/// emoji blocks that Windows Terminal renders as wide cells.
const fn is_unambiguous_wide(codepoint: u32) -> bool {
    matches!(
        codepoint,
        0x1100..=0x115f
            | 0x231a..=0x231b
            | 0x2329..=0x232a
            | 0x23e9..=0x23ec
            | 0x23f0
            | 0x23f3
            | 0x25fd..=0x25fe
            | 0x2614..=0x2615
            | 0x2648..=0x2653
            | 0x267f
            | 0x2693
            | 0x26a1
            | 0x26aa..=0x26ab
            | 0x26bd..=0x26be
            | 0x26c4..=0x26c5
            | 0x26ce
            | 0x26d4
            | 0x26ea
            | 0x26f2..=0x26f3
            | 0x26f5
            | 0x26fa
            | 0x26fd
            | 0x2705
            | 0x270a..=0x270b
            | 0x2728
            | 0x274c
            | 0x274e
            | 0x2753..=0x2755
            | 0x2757
            | 0x2795..=0x2797
            | 0x27b0
            | 0x27bf
            | 0x2b1b..=0x2b1c
            | 0x2b50
            | 0x2b55
            | 0x2e80..=0x303e
            | 0x3040..=0xa4cf
            | 0xac00..=0xd7a3
            | 0xf900..=0xfaff
            | 0xfe10..=0xfe19
            | 0xfe30..=0xfe6f
            | 0xff01..=0xff60
            | 0xffe0..=0xffe6
            | 0x16fe0..=0x16fff
            | 0x17000..=0x187ff
            | 0x18800..=0x18cff
            | 0x18d00..=0x18d8f
            | 0x1aff0..=0x1afff
            | 0x1b000..=0x1b2ff
            | 0x1f004
            | 0x1f0cf
            | 0x1f18e
            | 0x1f191..=0x1f19a
            | 0x1f200..=0x1f251
            | 0x1f300..=0x1f64f
            | 0x1f680..=0x1f6ff
            | 0x1f900..=0x1f9ff
            | 0x1fa70..=0x1faff
            | 0x20000..=0x2fffd
            | 0x30000..=0x3fffd
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utf16(input: char) -> Vec<u16> {
        let mut storage = [0; 2];
        input.encode_utf16(&mut storage).to_vec()
    }

    #[test]
    fn ascii_and_ambiguous_characters_remain_narrow() {
        let detector = CodepointWidthDetector;
        assert!(!detector.is_full_width(&utf16('A')));
        assert!(!detector.is_full_width(&utf16('·')));
    }

    #[test]
    fn cjk_and_fullwidth_forms_are_wide() {
        let detector = CodepointWidthDetector;
        assert!(detector.is_full_width(&utf16('界')));
        assert!(detector.is_full_width(&utf16('Ａ')));
    }

    #[test]
    fn supplementary_plane_emoji_is_wide() {
        let detector = CodepointWidthDetector;
        assert!(detector.is_full_width(&utf16('🚀')));
    }

    #[test]
    fn malformed_utf16_is_not_classified_as_wide() {
        let detector = CodepointWidthDetector;
        assert!(!detector.is_full_width(&[0xd83d]));
        assert!(!detector.is_full_width(&[0xde80]));
        assert!(!detector.is_full_width(&[0xd83d, u16::from(b'A')]));
    }

    #[test]
    fn microsoft_ambiguous_width_policy_switches_arrow_width() {
        let mut detector = TextMeasurementEngine::default();
        for mode in [TextMeasurementMode::Graphemes, TextMeasurementMode::Wcswidth] {
            detector.reset(mode);
            detector.set_ambiguous_width(1);
            assert_eq!(detector.grapheme_width("→"), 1);
            detector.set_ambiguous_width(2);
            assert_eq!(detector.grapheme_width("→"), 2);
        }
    }

    #[test]
    fn variation_selector_sixteen_promotes_cluster_to_wide() {
        let detector = TextMeasurementEngine::default();
        assert_eq!(detector.grapheme_width("🏳\u{fe0f}"), 2);
    }

    #[test]
    fn microsoft_basic_graphemes_match_forward_and_reverse_contract() {
        let detector = TextMeasurementEngine::default();
        let text = "a\u{0363}e\u{0364}\u{0364}i\u{0365}"
            .encode_utf16()
            .collect::<Vec<_>>();
        let expected = vec![
            TextMeasurement {
                utf16_len: 2,
                width: 1,
            },
            TextMeasurement {
                utf16_len: 3,
                width: 1,
            },
            TextMeasurement {
                utf16_len: 2,
                width: 1,
            },
        ];

        assert_eq!(detector.graphemes_forward(&text), expected);
        let mut reverse_expected = expected;
        reverse_expected.reverse();
        assert_eq!(detector.graphemes_backward(&text), reverse_expected);
    }
}
