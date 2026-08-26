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
pub struct AdaptDispatchPresentationState {
    core: AdaptDispatchCore,
    current_attributes: TextAttribute,
    saved_attributes: Option<TextAttribute>,
    cursor_visible: bool,
}

impl AdaptDispatchPresentationState {
    #[must_use]
    pub fn new(geometry: PageGeometry) -> Self {
        Self {
            core: AdaptDispatchCore::new(geometry),
            current_attributes: TextAttribute::default(),
            saved_attributes: None,
            cursor_visible: true,
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

    fn apply_graphics_rendition(&mut self, parameters: &Parameters) {
        for index in 0..parameters.size() {
            let option = parameters.at(index).unwrap_or(0);
            match option {
                0 => self.current_attributes = TextAttribute::default(),
                1 => self.current_attributes.set_intense(true),
                2 => self.current_attributes.set_faint(true),
                4 => self
                    .current_attributes
                    .set_underline_style(UnderlineStyle::Single),
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
                39 => self.current_attributes.set_default_foreground(),
                40..=47 => self.current_attributes.set_background(TextColor::index16(
                    u8::try_from(option - 40).unwrap_or_default(),
                )),
                49 => self.current_attributes.set_default_background(),
                53 => self.current_attributes.set_overlined(true),
                55 => self.current_attributes.set_overlined(false),
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
}
