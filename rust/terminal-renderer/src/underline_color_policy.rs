#[must_use]
pub const fn apply_underline_invisibility(
    underline: u32,
    background: u32,
    invisible: bool,
) -> u32 {
    if invisible { background } else { underline }
}

#[cfg(test)]
mod tests {
    use super::apply_underline_invisibility;

    #[test]
    fn invisible_text_uses_background_for_explicit_underline() {
        assert_eq!(
            apply_underline_invisibility(0x0011_2233, 0x0044_5566, true),
            0x0044_5566
        );
    }

    #[test]
    fn visible_text_preserves_explicit_underline() {
        assert_eq!(
            apply_underline_invisibility(0x0011_2233, 0x0044_5566, false),
            0x0011_2233
        );
    }
}
