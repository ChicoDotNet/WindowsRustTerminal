//! Product-level adapter response dispatch.
//!
//! This owner wires parser response-producing actions into the portable VT
//! response serializer while retaining the existing presentation-state owner
//! for cursor, modes, and rendition semantics.

use terminal_parser::output_engine::{DeviceAttributesKind, OutputAction, TermDispatch};

use crate::{
    adapt_dispatch::PageGeometry, presentation_state::AdaptDispatchPresentationState,
    vt_response::VtResponseEngine,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptDispatchResponseState {
    presentation: AdaptDispatchPresentationState,
    responses: VtResponseEngine,
    clipboard_supported: bool,
    viewport_left: i32,
    active_page: i32,
    visible_page: i32,
}

impl AdaptDispatchResponseState {
    #[must_use]
    pub fn new(geometry: PageGeometry) -> Self {
        let mut presentation = AdaptDispatchPresentationState::new(geometry);
        presentation.dispatch(OutputAction::SetMode {
            private: true,
            mode: 64,
            enabled: true,
        });
        Self {
            presentation,
            responses: VtResponseEngine::default(),
            clipboard_supported: true,
            viewport_left: 0,
            active_page: 1,
            visible_page: 1,
        }
    }

    #[must_use]
    pub const fn presentation(&self) -> &AdaptDispatchPresentationState {
        &self.presentation
    }
    pub const fn presentation_mut(&mut self) -> &mut AdaptDispatchPresentationState {
        &mut self.presentation
    }
    #[must_use]
    pub fn response(&self) -> &str {
        self.responses.response()
    }
    pub fn clear_response(&mut self) {
        self.responses.clear();
    }
    pub const fn set_clipboard_supported(&mut self, supported: bool) {
        self.clipboard_supported = supported;
    }
    pub const fn set_response_writable(&mut self, writable: bool) {
        self.responses.set_writable(writable);
    }
    pub fn set_viewport_left(&mut self, left: i32) {
        self.viewport_left = left.max(0);
    }

    fn device_status_report(&mut self, private: bool, status: i32, id: Option<i32>) -> bool {
        match (private, status) {
            (false, 5) => self.responses.operating_status(),
            (false, 6) => {
                let cursor = self.presentation.core().cursor();
                let viewport_top = self.presentation.core().geometry().top;
                self.responses
                    .cursor_position_report(cursor.x, cursor.y, viewport_top)
            }
            (true, 6) => {
                let cursor = self.presentation.core().cursor();
                let viewport_top = self.presentation.core().geometry().top;
                self.responses.extended_cursor_position_report(
                    cursor.x,
                    cursor.y,
                    viewport_top,
                    id.unwrap_or(1),
                )
            }
            _ => false,
        }
    }

    fn device_attributes(&mut self, kind: DeviceAttributesKind) -> bool {
        match kind {
            DeviceAttributesKind::Primary => self
                .responses
                .primary_device_attributes(self.clipboard_supported),
            DeviceAttributesKind::Secondary => self.responses.secondary_device_attributes(),
            DeviceAttributesKind::Tertiary => self.responses.tertiary_device_attributes(),
            DeviceAttributesKind::Vt52 => false,
        }
    }

    fn request_displayed_extent(&mut self) -> bool {
        let geometry = self.presentation.core().geometry();
        self.responses.displayed_extent(
            geometry.height,
            geometry.width,
            self.viewport_left,
            self.visible_page,
        )
    }

    fn page_position_absolute(&mut self, page: i32) {
        self.active_page = page.max(1);
        if self.presentation.core().page_cursor_coupling_mode() {
            self.visible_page = self.active_page;
        }
    }
}

impl TermDispatch for AdaptDispatchResponseState {
    fn dispatch(&mut self, action: OutputAction) {
        match action {
            OutputAction::DeviceStatusReport {
                private,
                status,
                id,
            } => {
                if !self.device_status_report(private, status, id) {
                    self.presentation
                        .dispatch(OutputAction::DeviceStatusReport {
                            private,
                            status,
                            id,
                        });
                }
            }
            OutputAction::DeviceAttributes(kind) => {
                if !self.device_attributes(kind) {
                    self.presentation
                        .dispatch(OutputAction::DeviceAttributes(kind));
                }
            }
            OutputAction::RequestTerminalParameters(permission) => {
                if !self.responses.terminal_parameters(permission) {
                    self.presentation
                        .dispatch(OutputAction::RequestTerminalParameters(permission));
                }
            }
            OutputAction::RequestDisplayedExtent => {
                if !self.request_displayed_extent() {
                    self.presentation
                        .dispatch(OutputAction::RequestDisplayedExtent);
                }
            }
            OutputAction::PagePositionAbsolute(page) => {
                self.page_position_absolute(page);
                self.presentation
                    .dispatch(OutputAction::PagePositionAbsolute(page));
            }
            OutputAction::SetMode {
                private: true,
                mode: 64,
                enabled,
            } => {
                let was_coupled = self.presentation.core().page_cursor_coupling_mode();
                self.presentation.dispatch(OutputAction::SetMode {
                    private: true,
                    mode: 64,
                    enabled,
                });
                if enabled && !was_coupled {
                    self.visible_page = self.active_page;
                }
            }
            other => self.presentation.dispatch(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapt_dispatch::Point;

    fn state() -> AdaptDispatchResponseState {
        let mut state = AdaptDispatchResponseState::new(PageGeometry::new(20, 100, 29));
        state
            .presentation_mut()
            .core_mut()
            .set_cursor(Point { x: 50, y: 34 });
        state
    }

    #[test]
    fn microsoft_operating_status_is_returned_through_adapter_dispatch() {
        let mut state = state();
        state.dispatch(OutputAction::DeviceStatusReport {
            private: false,
            status: 5,
            id: None,
        });
        assert_eq!(state.response(), "\u{1b}[0n");
    }

    #[test]
    fn microsoft_cpr_uses_live_cursor_and_viewport_state() {
        let mut state = state();
        state.dispatch(OutputAction::DeviceStatusReport {
            private: false,
            status: 6,
            id: None,
        });
        assert_eq!(state.response(), "\u{1b}[15;51R");
        state
            .presentation_mut()
            .core_mut()
            .set_cursor(Point { x: 51, y: 35 });
        state.dispatch(OutputAction::DeviceStatusReport {
            private: false,
            status: 6,
            id: None,
        });
        assert_eq!(state.response(), "\u{1b}[15;51R\u{1b}[16;52R");
    }

    #[test]
    fn microsoft_decxcpr_uses_reported_page_identifier() {
        let mut state = state();
        state.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 6,
            id: Some(1),
        });
        assert_eq!(state.response(), "\u{1b}[?15;51;1R");
        state.clear_response();
        state.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 6,
            id: Some(3),
        });
        assert_eq!(state.response(), "\u{1b}[?15;51;3R");
    }

    #[test]
    fn microsoft_primary_device_attributes_uses_live_clipboard_capability() {
        let mut state = state();
        state.dispatch(OutputAction::DeviceAttributes(
            DeviceAttributesKind::Primary,
        ));
        assert_eq!(
            state.response(),
            "\u{1b}[?61;4;6;7;14;21;22;23;24;28;32;42;52c"
        );
        state.clear_response();
        state.set_clipboard_supported(false);
        state.dispatch(OutputAction::DeviceAttributes(
            DeviceAttributesKind::Primary,
        ));
        assert_eq!(
            state.response(),
            "\u{1b}[?61;4;6;7;14;21;22;23;24;28;32;42c"
        );
    }

    #[test]
    fn microsoft_secondary_and_tertiary_attributes_flow_through_adapter_dispatch() {
        let mut state = state();
        state.dispatch(OutputAction::DeviceAttributes(
            DeviceAttributesKind::Secondary,
        ));
        assert_eq!(state.response(), "\u{1b}[>0;10;1c");
        state.clear_response();
        state.dispatch(OutputAction::DeviceAttributes(
            DeviceAttributesKind::Tertiary,
        ));
        assert_eq!(state.response(), "\u{1b}P!|00000000\u{1b}\\");
    }

    #[test]
    fn microsoft_terminal_parameters_flow_through_adapter_dispatch() {
        let mut state = state();
        state.dispatch(OutputAction::RequestTerminalParameters(0));
        assert_eq!(state.response(), "\u{1b}[2;1;1;128;128;1;0x");
        state.clear_response();
        state.dispatch(OutputAction::RequestTerminalParameters(1));
        assert_eq!(state.response(), "\u{1b}[3;1;1;128;128;1;0x");
    }

    #[test]
    fn microsoft_displayed_extent_tracks_pan_visible_page_and_coupling() {
        let mut state = AdaptDispatchResponseState::new(PageGeometry::new(0, 80, 24));
        state.dispatch(OutputAction::RequestDisplayedExtent);
        assert_eq!(state.response(), "\u{1b}[24;80;1;1;1\"w");

        state.clear_response();
        state.set_viewport_left(5);
        state.dispatch(OutputAction::RequestDisplayedExtent);
        assert_eq!(state.response(), "\u{1b}[24;80;6;1;1\"w");

        state.clear_response();
        state.dispatch(OutputAction::PagePositionAbsolute(3));
        state.dispatch(OutputAction::RequestDisplayedExtent);
        assert_eq!(state.response(), "\u{1b}[24;80;6;1;3\"w");

        state.clear_response();
        state.dispatch(OutputAction::SetMode {
            private: true,
            mode: 64,
            enabled: false,
        });
        state.dispatch(OutputAction::PagePositionAbsolute(1));
        state.dispatch(OutputAction::RequestDisplayedExtent);
        assert_eq!(state.response(), "\u{1b}[24;80;6;1;3\"w");

        state.clear_response();
        state.dispatch(OutputAction::SetMode {
            private: true,
            mode: 64,
            enabled: true,
        });
        state.dispatch(OutputAction::RequestDisplayedExtent);
        assert_eq!(state.response(), "\u{1b}[24;80;6;1;1\"w");
    }

    #[test]
    fn response_sink_failure_is_propagated_as_deferred_adapter_work() {
        let mut state = state();
        state.set_response_writable(false);
        state.dispatch(OutputAction::DeviceAttributes(
            DeviceAttributesKind::Primary,
        ));
        state.dispatch(OutputAction::DeviceAttributes(
            DeviceAttributesKind::Secondary,
        ));
        state.dispatch(OutputAction::DeviceAttributes(
            DeviceAttributesKind::Tertiary,
        ));
        state.dispatch(OutputAction::RequestTerminalParameters(0));
        state.dispatch(OutputAction::RequestDisplayedExtent);
        assert!(state.response().is_empty());
        assert_eq!(state.presentation().core().deferred_actions().len(), 5);
    }

    #[test]
    fn unsupported_reports_vt52_attributes_and_parameters_remain_deferred() {
        let mut state = state();
        state.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 15,
            id: None,
        });
        state.dispatch(OutputAction::DeviceAttributes(DeviceAttributesKind::Vt52));
        state.dispatch(OutputAction::RequestTerminalParameters(2));
        assert!(state.response().is_empty());
        assert_eq!(state.presentation().core().deferred_actions().len(), 3);
    }
}
