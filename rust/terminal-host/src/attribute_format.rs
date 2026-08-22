//! Deterministic host formatting of legacy console attributes as VT SGR.

use terminal_buffer::text_attribute::TextAttribute;

const SGR_FOREGROUND: [u8; 16] = [
    30, 31, 32, 33, 34, 35, 36, 37, 90, 91, 92, 93, 94, 95, 96, 97,
];

/// Formats the subset of `TextAttribute` emitted by host `VtIo::FormatAttributes`.
///
/// The host always starts with SGR 0 because `SetConsoleTextAttribute` replaces
/// the full active rendition, including VT-only state that is not represented in
/// the legacy API. Reverse video and legacy foreground/background colors are then
/// appended using the same ANSI indices as the C++ implementation.
#[must_use]
pub fn format_attributes(attributes: TextAttribute) -> String {
    let mut output = String::from("\x1b[0");

    if attributes.is_reverse_video() {
        output.push_str(";7");
    }

    let foreground = attributes.foreground();
    if foreground.is_legacy() {
        let index = usize::from(foreground.index());
        output.push(';');
        output.push_str(&SGR_FOREGROUND[index].to_string());
    }

    let background = attributes.background();
    if background.is_legacy() {
        let index = usize::from(background.index());
        output.push(';');
        output.push_str(&(SGR_FOREGROUND[index] + 10).to_string());
    }

    output.push('m');
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use terminal_buffer::text_attribute::LegacyColorDefaults;
    use terminal_buffer::text_color::TextColor;

    #[test]
    fn default_attributes_only_reset_rendition() {
        assert_eq!(format_attributes(TextAttribute::default()), "\x1b[0m");
    }

    #[test]
    fn reverse_video_is_emitted_after_reset() {
        let mut attributes = TextAttribute::default();
        attributes.set_reverse_video(true);
        assert_eq!(format_attributes(attributes), "\x1b[0;7m");
    }

    #[test]
    fn ansi_legacy_colors_match_host_sgr_table() {
        let mut attributes = TextAttribute::default();
        attributes.set_foreground(TextColor::index16(TextColor::BRIGHT_RED));
        attributes.set_background(TextColor::index16(TextColor::DARK_BLUE));
        assert_eq!(format_attributes(attributes), "\x1b[0;91;44m");
    }

    #[test]
    fn legacy_windows_color_order_is_transposed_before_formatting() {
        let attributes = TextAttribute::from_legacy(0x0041, LegacyColorDefaults::default());
        assert_eq!(format_attributes(attributes), "\x1b[0;34;41m");
    }

    #[test]
    fn nonlegacy_colors_are_not_emitted_by_this_legacy_host_contract() {
        let mut attributes = TextAttribute::default();
        attributes.set_foreground(TextColor::rgb(1, 2, 3));
        attributes.set_background(TextColor::index256(42));
        assert_eq!(format_attributes(attributes), "\x1b[0m");
    }
}
