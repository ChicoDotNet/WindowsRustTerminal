const HALF_COMPONENT_MASK: u32 = 0x007F_7F7F;
const OPAQUE_ALPHA: u32 = 0xFF00_0000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttributeColors {
    pub foreground: u32,
    pub background: u32,
}

#[must_use]
pub fn apply_attribute_effects(
    mut foreground: u32,
    mut background: u32,
    dim_foreground: bool,
    reverse_video: bool,
    screen_reversed: bool,
    invisible: bool,
) -> AttributeColors {
    if dim_foreground {
        foreground = (foreground >> 1) & HALF_COMPONENT_MASK;
    }

    if reverse_video ^ screen_reversed {
        core::mem::swap(&mut foreground, &mut background);
    }

    if invisible {
        foreground = background;
    }

    AttributeColors {
        foreground,
        background,
    }
}

#[must_use]
pub const fn apply_attribute_alpha(
    mut colors: AttributeColors,
    background_is_default: bool,
    reverse_video: bool,
    screen_reversed: bool,
    invisible: bool,
) -> AttributeColors {
    colors.foreground |= OPAQUE_ALPHA;

    if !background_is_default || (reverse_video ^ screen_reversed) || invisible {
        colors.background |= OPAQUE_ALPHA;
    }

    colors
}

#[cfg(test)]
mod tests {
    use super::{AttributeColors, apply_attribute_alpha, apply_attribute_effects};

    #[test]
    fn dim_halves_each_foreground_component() {
        let colors = apply_attribute_effects(0x0060_4020, 0x0011_2233, true, false, false, false);

        assert_eq!(
            colors,
            AttributeColors {
                foreground: 0x0030_2010,
                background: 0x0011_2233,
            }
        );
    }

    #[test]
    fn reverse_video_and_screen_reverse_cancel_each_other() {
        let original = AttributeColors {
            foreground: 0x0011_2233,
            background: 0x0044_5566,
        };

        assert_eq!(
            apply_attribute_effects(
                original.foreground,
                original.background,
                false,
                true,
                true,
                false,
            ),
            original
        );
    }

    #[test]
    fn one_reverse_source_swaps_foreground_and_background() {
        assert_eq!(
            apply_attribute_effects(0x0011_2233, 0x0044_5566, false, true, false, false),
            AttributeColors {
                foreground: 0x0044_5566,
                background: 0x0011_2233,
            }
        );
    }

    #[test]
    fn invisible_text_uses_the_final_background_as_foreground() {
        assert_eq!(
            apply_attribute_effects(0x0011_2233, 0x0044_5566, false, true, false, true),
            AttributeColors {
                foreground: 0x0011_2233,
                background: 0x0011_2233,
            }
        );
    }

    #[test]
    fn default_background_keeps_transparency_when_not_reversed_or_invisible() {
        assert_eq!(
            apply_attribute_alpha(
                AttributeColors {
                    foreground: 0x0011_2233,
                    background: 0x0044_5566,
                },
                true,
                false,
                false,
                false,
            ),
            AttributeColors {
                foreground: 0xFF11_2233,
                background: 0x0044_5566,
            }
        );
    }

    #[test]
    fn custom_reversed_and_invisible_backgrounds_are_opaque() {
        for (background_is_default, reverse_video, screen_reversed, invisible) in [
            (false, false, false, false),
            (true, true, false, false),
            (true, false, true, false),
            (true, false, false, true),
        ] {
            let colors = apply_attribute_alpha(
                AttributeColors {
                    foreground: 0x0011_2233,
                    background: 0x0044_5566,
                },
                background_is_default,
                reverse_video,
                screen_reversed,
                invisible,
            );

            assert_eq!(colors.foreground, 0xFF11_2233);
            assert_eq!(colors.background, 0xFF44_5566);
        }
    }
}
