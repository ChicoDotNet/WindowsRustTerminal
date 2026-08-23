const HALF_COMPONENT_MASK: u32 = 0x007F_7F7F;
const OPAQUE_ALPHA: u32 = 0xFF00_0000;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AttributeColorFlags(u8);

impl AttributeColorFlags {
    const DIM_FOREGROUND: u8 = 1 << 0;
    const REVERSE_VIDEO: u8 = 1 << 1;
    const SCREEN_REVERSED: u8 = 1 << 2;
    const INVISIBLE: u8 = 1 << 3;
    const BACKGROUND_DEFAULT: u8 = 1 << 4;

    #[must_use]
    pub const fn with_dim_foreground(self, enabled: bool) -> Self {
        self.with_flag(Self::DIM_FOREGROUND, enabled)
    }

    #[must_use]
    pub const fn with_reverse_video(self, enabled: bool) -> Self {
        self.with_flag(Self::REVERSE_VIDEO, enabled)
    }

    #[must_use]
    pub const fn with_screen_reversed(self, enabled: bool) -> Self {
        self.with_flag(Self::SCREEN_REVERSED, enabled)
    }

    #[must_use]
    pub const fn with_invisible(self, enabled: bool) -> Self {
        self.with_flag(Self::INVISIBLE, enabled)
    }

    #[must_use]
    pub const fn with_background_default(self, enabled: bool) -> Self {
        self.with_flag(Self::BACKGROUND_DEFAULT, enabled)
    }

    const fn with_flag(self, flag: u8, enabled: bool) -> Self {
        if enabled {
            Self(self.0 | flag)
        } else {
            Self(self.0 & !flag)
        }
    }

    const fn has(self, flag: u8) -> bool {
        self.0 & flag != 0
    }

    const fn dim_foreground(self) -> bool {
        self.has(Self::DIM_FOREGROUND)
    }

    const fn reverse_video(self) -> bool {
        self.has(Self::REVERSE_VIDEO)
    }

    const fn screen_reversed(self) -> bool {
        self.has(Self::SCREEN_REVERSED)
    }

    const fn invisible(self) -> bool {
        self.has(Self::INVISIBLE)
    }

    const fn background_default(self) -> bool {
        self.has(Self::BACKGROUND_DEFAULT)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttributeColors {
    pub foreground: u32,
    pub background: u32,
}

#[must_use]
pub fn apply_attribute_effects(
    mut foreground: u32,
    mut background: u32,
    flags: AttributeColorFlags,
) -> AttributeColors {
    if flags.dim_foreground() {
        foreground = (foreground >> 1) & HALF_COMPONENT_MASK;
    }

    if flags.reverse_video() ^ flags.screen_reversed() {
        core::mem::swap(&mut foreground, &mut background);
    }

    if flags.invisible() {
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
    flags: AttributeColorFlags,
) -> AttributeColors {
    colors.foreground |= OPAQUE_ALPHA;

    if !flags.background_default()
        || (flags.reverse_video() ^ flags.screen_reversed())
        || flags.invisible()
    {
        colors.background |= OPAQUE_ALPHA;
    }

    colors
}

#[cfg(test)]
mod tests {
    use super::{
        AttributeColorFlags, AttributeColors, apply_attribute_alpha, apply_attribute_effects,
    };

    #[test]
    fn dim_halves_each_foreground_component() {
        let colors = apply_attribute_effects(
            0x0060_4020,
            0x0011_2233,
            AttributeColorFlags::default().with_dim_foreground(true),
        );

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
        let flags = AttributeColorFlags::default()
            .with_reverse_video(true)
            .with_screen_reversed(true);

        assert_eq!(
            apply_attribute_effects(original.foreground, original.background, flags),
            original
        );
    }

    #[test]
    fn one_reverse_source_swaps_foreground_and_background() {
        let flags = AttributeColorFlags::default().with_reverse_video(true);

        assert_eq!(
            apply_attribute_effects(0x0011_2233, 0x0044_5566, flags),
            AttributeColors {
                foreground: 0x0044_5566,
                background: 0x0011_2233,
            }
        );
    }

    #[test]
    fn invisible_text_uses_the_final_background_as_foreground() {
        let flags = AttributeColorFlags::default()
            .with_reverse_video(true)
            .with_invisible(true);

        assert_eq!(
            apply_attribute_effects(0x0011_2233, 0x0044_5566, flags),
            AttributeColors {
                foreground: 0x0011_2233,
                background: 0x0011_2233,
            }
        );
    }

    #[test]
    fn default_background_keeps_transparency_when_not_reversed_or_invisible() {
        let flags = AttributeColorFlags::default().with_background_default(true);

        assert_eq!(
            apply_attribute_alpha(
                AttributeColors {
                    foreground: 0x0011_2233,
                    background: 0x0044_5566,
                },
                flags,
            ),
            AttributeColors {
                foreground: 0xFF11_2233,
                background: 0x0044_5566,
            }
        );
    }

    #[test]
    fn custom_reversed_and_invisible_backgrounds_are_opaque() {
        let cases = [
            AttributeColorFlags::default(),
            AttributeColorFlags::default()
                .with_background_default(true)
                .with_reverse_video(true),
            AttributeColorFlags::default()
                .with_background_default(true)
                .with_screen_reversed(true),
            AttributeColorFlags::default()
                .with_background_default(true)
                .with_invisible(true),
        ];

        for flags in cases {
            let colors = apply_attribute_alpha(
                AttributeColors {
                    foreground: 0x0011_2233,
                    background: 0x0044_5566,
                },
                flags,
            );

            assert_eq!(colors.foreground, 0xFF11_2233);
            assert_eq!(colors.background, 0xFF44_5566);
        }
    }
}
