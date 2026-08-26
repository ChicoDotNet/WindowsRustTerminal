//! Product-level adapter response dispatch.
//!
//! This owner wires parser `DeviceStatusReport` actions into the portable VT
//! response serializer while retaining the existing presentation-state owner
//! for cursor, modes, and rendition semantics.

use terminal_parser::output_engine::{OutputAction, TermDispatch};

use crate::{
    adapt_dispatch::PageGeometry, presentation_state::AdaptDispatchPresentationState,
    vt_response::VtResponseEngine,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptDispatchResponseState {
    presentation: AdaptDispatchPresentationState,
    responses: VtResponseEngine,
}

impl AdaptDispatchResponseState {
    #[must_use]
    pub fn new(geometry: PageGeometry) -> Self {
        Self {
            presentation: AdaptDispatchPresentationState::new(geometry),
            responses: VtResponseEngine::default(),
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

    fn device_status_report(&mut self, private: bool, status: i32, id: Option<i32>) -> bool {
        match (private, status) {
            (false, 5) => {
                self.responses.operating_status();
                true
            }
            (false, 6) => {
                let cursor = self.presentation.core().cursor();
                let viewport_top = self.presentation.core().geometry().top;
                self.responses
                    .cursor_position_report(cursor.x, cursor.y, viewport_top);
                true
            }
            (true, 6) => {
                let cursor = self.presentation.core().cursor();
                let viewport_top = self.presentation.core().geometry().top;
                self.responses.extended_cursor_position_report(
                    cursor.x,
                    cursor.y,
                    viewport_top,
                    id.unwrap_or(1),
                );
                true
            }
            _ => false,
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
    fn unsupported_reports_remain_explicitly_deferred() {
        let mut state = state();
        state.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 15,
            id: None,
        });
        assert!(state.response().is_empty());
        assert_eq!(state.presentation().core().deferred_actions().len(), 1);
    }
}
