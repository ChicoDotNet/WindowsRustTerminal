//! Global color-table and DEC color-alias state for terminal presentation.
//!
//! `TextColor` deliberately stores symbolic/default/indexed colors. This owner
//! keeps the mutable palette those colors resolve through, so changing an OSC
//! color entry immediately changes the rendered color of existing indexed cells
//! without rewriting their stored attributes.

use crate::text_attribute::TextAttribute;
use crate::text_color::{
    Rgb, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND, FRAME_BACKGROUND, FRAME_FOREGROUND, TABLE_SIZE,
};

const PALETTE_SIZE: usize = 256;

const CAMPBELL: [Rgb; 16] = [
    Rgb::new(0x0c, 0x0c, 0x0c),
    Rgb::new(0xc5, 0x0f, 0x1f),
    Rgb::new(0x13, 0xa1, 0x0e),
    Rgb::new(0xc1, 0x9c, 0x00),
    Rgb::new(0x00, 0x37, 0xda),
    Rgb::new(0x88, 0x17, 0x98),
    Rgb::new(0x3a, 0x96, 0xdd),
    Rgb::new(0xcc, 0xcc, 0xcc),
    Rgb::new(0x76, 0x76, 0x76),
    Rgb::new(0xe7, 0x48, 0x56),
    Rgb::new(0x16, 0xc6, 0x0c),
    Rgb::new(0xf9, 0xf1, 0xa5),
    Rgb::new(0x3b, 0x78, 0xff),
    Rgb::new(0xb4, 0x00, 0x9e),
    Rgb::new(0x61, 0xd6, 0xd6),
    Rgb::new(0xf2, 0xf2, 0xf2),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorAlias {
    DefaultForeground,
    DefaultBackground,
    FrameForeground,
    FrameBackground,
}

impl ColorAlias {
    const fn slot(self) -> usize {
        match self {
            Self::DefaultForeground => 0,
            Self::DefaultBackground => 1,
            Self::FrameForeground => 2,
            Self::FrameBackground => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorTableState {
    table: [Rgb; TABLE_SIZE],
    aliases: [usize; 4],
}

impl Default for ColorTableState {
    fn default() -> Self {
        Self {
            table: initial_color_table(),
            aliases: initial_aliases(),
        }
    }
}

impl ColorTableState {
    #[must_use]
    pub fn table(&self) -> &[Rgb; TABLE_SIZE] {
        &self.table
    }

    #[must_use]
    pub fn color(&self, index: usize) -> Option<Rgb> {
        self.table.get(index).copied()
    }

    #[must_use]
    pub fn alias_index(&self, alias: ColorAlias) -> usize {
        self.aliases[alias.slot()]
    }

    /// Applies the product side of OSC 4, OSC 10 and OSC 11 after the VT parser
    /// has removed framing/terminators. Invalid payloads leave state unchanged.
    pub fn apply_osc(&mut self, command: u16, payload: &str) -> bool {
        match command {
            4 => self.apply_palette_payload(payload),
            10 => self.apply_special_color(DEFAULT_FOREGROUND, payload),
            11 => self.apply_special_color(DEFAULT_BACKGROUND, payload),
            _ => false,
        }
    }

    /// Applies DEC item color assignment. Item 1 owns normal-text aliases and
    /// item 2 owns frame aliases; unsupported items are ignored.
    pub fn assign_color_aliases(&mut self, item: u16, foreground: usize, background: usize) -> bool {
        if foreground >= TABLE_SIZE || background >= TABLE_SIZE {
            return false;
        }

        let pair = match item {
            1 => (ColorAlias::DefaultForeground, ColorAlias::DefaultBackground),
            2 => (ColorAlias::FrameForeground, ColorAlias::FrameBackground),
            _ => return false,
        };
        self.aliases[pair.0.slot()] = foreground;
        self.aliases[pair.1.slot()] = background;
        true
    }

    /// RIS restores the initial palette and alias assignments together.
    pub fn reset_to_initial(&mut self) {
        self.table = initial_color_table();
        self.aliases = initial_aliases();
    }

    /// Resolves stored symbolic/indexed attributes through the current global
    /// table. Reverse-video is applied after foreground/background resolution.
    #[must_use]
    pub fn attribute_colors(&self, attribute: TextAttribute) -> (Rgb, Rgb) {
        let foreground = attribute.foreground().resolve(
            &self.table,
            self.alias_index(ColorAlias::DefaultForeground),
            attribute.is_intense(),
        );
        let background = attribute.background().resolve(
            &self.table,
            self.alias_index(ColorAlias::DefaultBackground),
            false,
        );

        if attribute.is_reverse_video() {
            (background, foreground)
        } else {
            (foreground, background)
        }
    }

    fn apply_palette_payload(&mut self, payload: &str) -> bool {
        let fields: Vec<_> = payload.split(';').collect();
        if fields.len() < 2 || fields.len() % 2 != 0 {
            return false;
        }

        let mut updates = Vec::with_capacity(fields.len() / 2);
        for pair in fields.chunks_exact(2) {
            let Ok(index) = pair[0].parse::<usize>() else {
                return false;
            };
            if index >= PALETTE_SIZE {
                return false;
            }
            let Some(color) = parse_xterm_rgb(pair[1]) else {
                return false;
            };
            updates.push((index, color));
        }

        for (index, color) in updates {
            self.table[index] = color;
        }
        true
    }

    fn apply_special_color(&mut self, index: usize, payload: &str) -> bool {
        let Some(color) = parse_xterm_rgb(payload) else {
            return false;
        };
        self.table[index] = color;
        true
    }
}

fn parse_xterm_rgb(specification: &str) -> Option<Rgb> {
    let body = specification.strip_prefix("rgb:")?;
    let components: Vec<_> = body.split('/').collect();
    if components.len() != 3 {
        return None;
    }

    Some(Rgb::new(
        parse_xterm_component(components[0])?,
        parse_xterm_component(components[1])?,
        parse_xterm_component(components[2])?,
    ))
}

fn parse_xterm_component(component: &str) -> Option<u8> {
    if component.is_empty() || component.len() > 4 {
        return None;
    }

    let value = u32::from_str_radix(component, 16).ok()?;
    let bits = u32::try_from(component.len()).ok()?.checked_mul(4)?;
    let maximum = (1_u32.checked_shl(bits)?).checked_sub(1)?;
    let scaled = (value.checked_mul(255)? + maximum / 2) / maximum;
    u8::try_from(scaled).ok()
}

fn initial_aliases() -> [usize; 4] {
    [
        DEFAULT_FOREGROUND,
        DEFAULT_BACKGROUND,
        FRAME_FOREGROUND,
        FRAME_BACKGROUND,
    ]
}

fn initial_color_table() -> [Rgb; TABLE_SIZE] {
    let mut table = [Rgb::new(0, 0, 0); TABLE_SIZE];
    table[..CAMPBELL.len()].copy_from_slice(&CAMPBELL);

    let levels = [0_u8, 95, 135, 175, 215, 255];
    let mut index = 16;
    for red in levels {
        for green in levels {
            for blue in levels {
                table[index] = Rgb::new(red, green, blue);
                index += 1;
            }
        }
    }

    for gray_index in 0..24_usize {
        let value = u8::try_from(8 + gray_index * 10).expect("xterm grayscale is in byte range");
        table[232 + gray_index] = Rgb::new(value, value, value);
    }

    table[DEFAULT_FOREGROUND] = CAMPBELL[7];
    table[DEFAULT_BACKGROUND] = CAMPBELL[0];
    table[FRAME_FOREGROUND] = CAMPBELL[7];
    table[FRAME_BACKGROUND] = CAMPBELL[0];
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xterm_rgb_components_scale_by_declared_hex_precision() {
        assert_eq!(parse_xterm_rgb("rgb:1/23/12"), Some(Rgb::new(0x11, 0x23, 0x12)));
        assert_eq!(parse_xterm_rgb("rgb:ff/a1/1b"), Some(Rgb::new(0xff, 0xa1, 0x1b)));
        assert_eq!(parse_xterm_rgb("rgb:/1/1"), None);
        assert_eq!(parse_xterm_rgb("rgb:1/1/1/1"), None);
    }

    #[test]
    fn palette_updates_are_transactional_for_invalid_payloads() {
        let mut state = ColorTableState::default();
        let before = state.clone();
        assert!(!state.apply_osc(4, "5;rgb:09/09/09;6;rgb://"));
        assert_eq!(state, before);
    }
}
