use std::ptr;

use terminal_renderer::{
    AttributeColorFlags, AttributeColors, apply_attribute_alpha, apply_attribute_effects,
};

use super::{FfiStatus, ffi_guard};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderAttributeColors {
    pub foreground: u32,
    pub background: u32,
}

fn bool_from_abi(value: u32) -> Option<bool> {
    match value {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_render_attribute_effects(
    foreground: u32,
    background: u32,
    dim_foreground: u32,
    reverse_video: u32,
    screen_reversed: u32,
    invisible: u32,
    out_colors: *mut RenderAttributeColors,
) -> FfiStatus {
    ffi_guard(|| {
        if out_colors.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let (Some(dim_foreground), Some(reverse_video), Some(screen_reversed), Some(invisible)) = (
            bool_from_abi(dim_foreground),
            bool_from_abi(reverse_video),
            bool_from_abi(screen_reversed),
            bool_from_abi(invisible),
        ) else {
            return FfiStatus::InvalidArgument;
        };

        let flags = AttributeColorFlags::default()
            .with_dim_foreground(dim_foreground)
            .with_reverse_video(reverse_video)
            .with_screen_reversed(screen_reversed)
            .with_invisible(invisible);
        let colors = apply_attribute_effects(foreground, background, flags);

        // SAFETY: `out_colors` was checked non-null and the ABI requires one
        // writable result value for the duration of this call.
        unsafe {
            ptr::write(
                out_colors,
                RenderAttributeColors {
                    foreground: colors.foreground,
                    background: colors.background,
                },
            )
        };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_render_attribute_alpha(
    colors: RenderAttributeColors,
    background_default: u32,
    reverse_video: u32,
    screen_reversed: u32,
    invisible: u32,
    out_colors: *mut RenderAttributeColors,
) -> FfiStatus {
    ffi_guard(|| {
        if out_colors.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let (Some(background_default), Some(reverse_video), Some(screen_reversed), Some(invisible)) = (
            bool_from_abi(background_default),
            bool_from_abi(reverse_video),
            bool_from_abi(screen_reversed),
            bool_from_abi(invisible),
        ) else {
            return FfiStatus::InvalidArgument;
        };

        let flags = AttributeColorFlags::default()
            .with_background_default(background_default)
            .with_reverse_video(reverse_video)
            .with_screen_reversed(screen_reversed)
            .with_invisible(invisible);
        let colors = apply_attribute_alpha(
            AttributeColors {
                foreground: colors.foreground,
                background: colors.background,
            },
            flags,
        );

        // SAFETY: `out_colors` was checked non-null and the ABI requires one
        // writable result value for the duration of this call.
        unsafe {
            ptr::write(
                out_colors,
                RenderAttributeColors {
                    foreground: colors.foreground,
                    background: colors.background,
                },
            )
        };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        FfiStatus, RenderAttributeColors, terminal_parser_ffi_render_attribute_alpha,
        terminal_parser_ffi_render_attribute_effects,
    };

    #[test]
    fn ffi_replays_attribute_effect_order() {
        let mut colors = RenderAttributeColors::default();
        assert_eq!(
            terminal_parser_ffi_render_attribute_effects(
                0x0060_4020,
                0x0011_2233,
                1,
                1,
                0,
                1,
                &mut colors,
            ),
            FfiStatus::Ok
        );
        assert_eq!(
            colors,
            RenderAttributeColors {
                foreground: 0x0030_2010,
                background: 0x0030_2010,
            }
        );
    }

    #[test]
    fn ffi_replays_reverse_xor_and_alpha_rules() {
        let mut colors = RenderAttributeColors::default();
        assert_eq!(
            terminal_parser_ffi_render_attribute_effects(
                0x0011_2233,
                0x0044_5566,
                0,
                1,
                1,
                0,
                &mut colors,
            ),
            FfiStatus::Ok
        );
        assert_eq!(
            colors,
            RenderAttributeColors {
                foreground: 0x0011_2233,
                background: 0x0044_5566,
            }
        );

        assert_eq!(
            terminal_parser_ffi_render_attribute_alpha(colors, 1, 0, 0, 0, &mut colors),
            FfiStatus::Ok
        );
        assert_eq!(colors.foreground, 0xFF11_2233);
        assert_eq!(colors.background, 0x0044_5566);
    }

    #[test]
    fn ffi_rejects_invalid_booleans_and_pointers() {
        let mut colors = RenderAttributeColors::default();
        assert_eq!(
            terminal_parser_ffi_render_attribute_effects(0, 0, 2, 0, 0, 0, &mut colors),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_render_attribute_alpha(colors, 0, 0, 0, 0, std::ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
    }
}
