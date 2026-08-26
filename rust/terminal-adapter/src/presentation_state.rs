//! Product-level adapter presentation state for cursor and text attributes.
//!
//! Microsoft `AdaptDispatch` saves text attributes together with the cursor,
//! applies DECTCEM directly to the active cursor, and mutates the active text
//! attributes for SGR. This owner keeps those deterministic semantics in Rust
//! while native drawing remains outside the portable core.

use terminal_buffer::{
    text_attribute::{TextAttribute, UnderlineStyle},
    text_color::TextColor,
};
use terminal_parser::{
    output_engine::{OutputAction, TermDispatch},
    state_machine::Parameters,
};

use crate::adapt_dispatch::{AdaptDispatchCore, PageGeometry};

const DECTCEM_TEXT_CURSOR_ENABLE_MODE: i32 = 25;

#[derive(Debug, Clone, PartialEq, Eq)]
enum RenditionStackEntry {
    Full(TextAttribute),
    DeferredSelective,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptDispatchPresentationState {
    core: AdaptDispatchCore,
    current_attributes: TextAttribute,
    saved_attributes: Option<TextAttribute>,
    cursor_visible: bool,
    rendition_stack: Vec<RenditionStackEntry>,
}

impl AdaptDispatchPresentationState {
    #[must_use]
    pub fn new(geometry: PageGeometry) -> Self {
        Self {
            core: AdaptDispatchCore::new(geometry),
            current_attributes: TextAttribute::default(),
            saved_attributes: None,
            cursor_visible: true,
            rendition_stack: Vec::new(),
        }
    }

    #[must_use]
    pub const fn core(&self) -> &AdaptDispatchCore {
        &self.core
    }

    pub const fn core_mut(&mut self) -> &mut AdaptDispatchCore {
        &mut self.core
    }

    #[must_use]
    pub const fn current_attributes(&self) -> TextAttribute {
        self.current_attributes
    }

    pub const fn set_current_attributes(&mut self, attributes: TextAttribute) {
        self.current_attributes = attributes;
    }

    #[must_use]
    pub const fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn set_mode(&mut self, private: bool, mode: i32, enabled: bool) -> bool {
        if private && mode == DECTCEM_TEXT_CURSOR_ENABLE_MODE {
            self.cursor_visible = enabled;
            true
        } else {
            self.core.set_mode(private, mode, enabled)
        }
    }

    fn underline_style_from_subparams(sub_params: &[Option<i32>]) -> UnderlineStyle {
        match sub_params.first().copied().flatten().unwrap_or(1) {
            0 => UnderlineStyle::None,
            2 => UnderlineStyle::Double,
            3 => UnderlineStyle::Curly,
            4 => UnderlineStyle::Dotted,
            5 => UnderlineStyle::Dashed,
            _ => UnderlineStyle::Single,
        }
    }

    fn extended_color_from_subparams(sub_params: &[Option<i32>]) -> Option<TextColor> {
        match sub_params.first().copied().flatten()? {
            2 => {
                let red = u8::try_from(sub_params.get(1).copied().flatten()?).ok()?;
                let green = u8::try_from(sub_params.get(2).copied().flatten()?).ok()?;
                let blue = u8::try_from(sub_params.get(3).copied().flatten()?).ok()?;
                Some(TextColor::rgb(red, green, blue))
            }
            5 => {
                let index = u8::try_from(sub_params.get(1).copied().flatten()?).ok()?;
                Some(TextColor::index256(index))
            }
            _ => None,
        }
    }

    fn apply_graphics_rendition(&mut self, parameters: &Parameters) {
        for index in 0..parameters.size() {
            let option = parameters.at(index).unwrap_or(0);
            let sub_params = parameters.sub_params_for(index);
            match option {
                0 => self.current_attributes = TextAttribute::default(),
                1 => self.current_attributes.set_intense(true),
                2 => self.current_attributes.set_faint(true),
                4 => self
                    .current_attributes
                    .set_underline_style(Self::underline_style_from_subparams(sub_params)),
                7 => self.current_attributes.set_reverse_video(true),
                8 => self.current_attributes.set_invisible(true),
                9 => self.current_attributes.set_crossed_out(true),
                21 => self
                    .current_attributes
                    .set_underline_style(UnderlineStyle::Double),
                22 => {
                    self.current_attributes.set_intense(false);
                    self.current_attributes.set_faint(false);
                }
                24 => self
                    .current_attributes
                    .set_underline_style(UnderlineStyle::None),
                27 => self.current_attributes.set_reverse_video(false),
                28 => self.current_attributes.set_invisible(false),
                29 => self.current_attributes.set_crossed_out(false),
                30..=37 => self.current_attributes.set_foreground(TextColor::index16(
                    u8::try_from(option - 30).unwrap_or_default(),
                )),
                38 => {
                    if let Some(color) = Self::extended_color_from_subparams(sub_params) {
                        self.current_attributes.set_foreground(color);
                    }
                }
                39 => self.current_attributes.set_default_foreground(),
                40..=47 => self.current_attributes.set_background(TextColor::index16(
                    u8::try_from(option - 40).unwrap_or_default(),
                )),
                48 => {
                    if let Some(color) = Self::extended_color_from_subparams(sub_params) {
                        self.current_attributes.set_background(color);
                    }
                }
                49 => self.current_attributes.set_default_background(),
                53 => self.current_attributes.set_overlined(true),
                55 => self.current_attributes.set_overlined(false),
                58 => {
                    if let Some(color) = Self::extended_color_from_subparams(sub_params) {
                        self.current_attributes.set_underline_color(color);
                    }
                }
                90..=97 => self.current_attributes.set_foreground(TextColor::index16(
                    u8::try_from(option - 90 + 8).unwrap_or_default(),
                )),
                100..=107 => self.current_attributes.set_background(TextColor::index16(
                    u8::try_from(option - 100 + 8).unwrap_or_default(),
                )),
                _ => {}
            }
        }
    }

    fn push_graphics_rendition(&mut self, parameters: Parameters) {
        let saves_all = parameters.values().is_empty()
            || (parameters.values().len() == 1 && parameters.at(0).unwrap_or(0) == 0);
        if saves_all {
            self.rendition_stack
                .push(RenditionStackEntry::Full(self.current_attributes));
        } else {
            self.rendition_stack
                .push(RenditionStackEntry::DeferredSelective);
            self.core
                .dispatch(OutputAction::PushGraphicsRendition(parameters));
        }
    }

    fn pop_graphics_rendition(&mut self) {
        match self.rendition_stack.pop() {
            Some(RenditionStackEntry::Full(attributes)) => {
                self.current_attributes = attributes;
            }
            Some(RenditionStackEntry::DeferredSelective) | None => {
                self.core.dispatch(OutputAction::PopGraphicsRendition);
            }
        }
    }
}

impl TermDispatch for AdaptDispatchPresentationState {
    fn dispatch(&mut self, action: OutputAction) {
        match action {
            OutputAction::CursorSaveState => {
                self.saved_attributes = Some(self.current_attributes);
                self.core.dispatch(OutputAction::CursorSaveState);
            }
            OutputAction::CursorRestoreState => {
                self.current_attributes = self.saved_attributes.unwrap_or_default();
                self.core.dispatch(OutputAction::CursorRestoreState);
            }
            OutputAction::SetMode {
                private,
                enabled,
                mode,
            } => {
                if !self.set_mode(private, mode, enabled) {
                    self.core.dispatch(OutputAction::SetMode {
                        private,
                        enabled,
                        mode,
                    });
                }
            }
            OutputAction::SetGraphicsRendition(parameters) => {
                self.apply_graphics_rendition(&parameters);
            }
            OutputAction::PushGraphicsRendition(parameters) => {
                self.push_graphics_rendition(parameters);
            }
            OutputAction::PopGraphicsRendition => {
                self.pop_graphics_rendition();
            }
            other => self.core.dispatch(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapt_dispatch::Point;
    use terminal_buffer::text_attribute::LegacyColorDefaults;

    #[test]
    fn restore_without_save_resets_cursor_and_attributes() {
        let mut state = AdaptDispatchPresentationState::new(PageGeometry::new(20, 100, 29));
        state.core_mut().set_cursor(Point { x: 50, y: 34 });
        state.set_current_attributes(TextAttribute::from_legacy(
            0x0041,
            LegacyColorDefaults::default(),
        ));

        state.dispatch(OutputAction::CursorRestoreState);

        assert_eq!(state.core().cursor(), Point { x: 0, y: 20 });
        assert_eq!(state.current_attributes(), TextAttribute::default());
    }

    #[test]
    fn save_restore_preserves_text_attributes_with_cursor_state() {
        let mut state = AdaptDispatchPresentationState::new(PageGeometry::new(20, 100, 29));
        let saved = TextAttribute::from_legacy(0x0041, LegacyColorDefaults::default());
        state.core_mut().set_cursor(Point { x: 50, y: 34 });
        state.set_current_attributes(saved);
        state.dispatch(OutputAction::CursorSaveState);

        state.core_mut().set_cursor(Point { x: 0, y: 48 });
        state.set_current_attributes(TextAttribute::default());
        state.dispatch(OutputAction::CursorRestoreState);

        assert_eq!(state.core().cursor(), Point { x: 50, y: 34 });
        assert_eq!(state.current_attributes(), saved);
    }

    #[test]
    fn dectcem_updates_effective_cursor_visibility() {
        let mut state = AdaptDispatchPresentationState::new(PageGeometry::new(20, 100, 29));
        for starting_visibility in [false, true] {
            state.cursor_visible = starting_visibility;
            for ending_visibility in [false, true] {
                state.dispatch(OutputAction::SetMode {
                    private: true,
                    enabled: ending_visibility,
                    mode: DECTCEM_TEXT_CURSOR_ENABLE_MODE,
                });
                assert_eq!(state.cursor_visible(), ending_visibility);
            }
        }
    }

    #[test]
    fn empty_sgr_resets_attributes() {
        let mut state = AdaptDispatchPresentationState::new(PageGeometry::new(20, 100, 29));
        let mut attributes = TextAttribute::default();
        attributes.set_intense(true);
        attributes.set_reverse_video(true);
        state.set_current_attributes(attributes);

        state.dispatch(OutputAction::SetGraphicsRendition(Parameters::default()));

        assert_eq!(state.current_attributes(), TextAttribute::default());
    }

    #[test]
    fn full_rendition_stack_restores_nested_states() {
        let mut state = AdaptDispatchPresentationState::new(PageGeometry::new(20, 100, 29));
        state.dispatch(OutputAction::SetGraphicsRendition(Parameters::default()));
        state.dispatch(OutputAction::PushGraphicsRendition(Parameters::default()));

        state.dispatch(OutputAction::SetGraphicsRendition(Parameters::from_values(vec![Some(31)])));
        let red = state.current_attributes();
        state.dispatch(OutputAction::PushGraphicsRendition(Parameters::default()));

        state.dispatch(OutputAction::SetGraphicsRendition(Parameters::from_values(vec![Some(32)])));
        assert_eq!(
            state.current_attributes().foreground(),
            TextColor::index16(TextColor::DARK_GREEN)
        );

        state.dispatch(OutputAction::PopGraphicsRendition);
        assert_eq!(state.current_attributes(), red);
        state.dispatch(OutputAction::PopGraphicsRendition);
        assert_eq!(state.current_attributes(), TextAttribute::default());
    }
}
