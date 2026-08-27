//! Final portable product aggregate for adapter response-producing behavior.
//!
//! `AdaptDispatchResponseState` owns the ordinary VT response path while
//! `MacroReportEngine` owns DECDMAC storage plus DSR 62/63. This aggregate is
//! the single `TermDispatch` surface that composes both owners, preventing the
//! macro parser/report state from becoming a disconnected reporting copy.

use terminal_parser::output_engine::{DcsAction, OutputAction, TermDispatch};

use crate::{
    adapt_dispatch::PageGeometry, macro_reports::MacroReportEngine,
    response_dispatch::AdaptDispatchResponseState,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum DcsOwner {
    #[default]
    None,
    Response,
    Macro,
}

#[derive(Debug, Clone)]
pub struct AdaptDispatchProductState {
    responses: AdaptDispatchResponseState,
    macros: MacroReportEngine,
    outbound: String,
    writable: bool,
    dcs_owner: DcsOwner,
}

impl AdaptDispatchProductState {
    #[must_use]
    pub fn new(geometry: PageGeometry) -> Self {
        Self {
            responses: AdaptDispatchResponseState::new(geometry),
            macros: MacroReportEngine::default(),
            outbound: String::new(),
            writable: true,
            dcs_owner: DcsOwner::None,
        }
    }

    #[must_use]
    pub const fn response_state(&self) -> &AdaptDispatchResponseState {
        &self.responses
    }

    pub const fn response_state_mut(&mut self) -> &mut AdaptDispatchResponseState {
        &mut self.responses
    }

    #[must_use]
    pub const fn macro_reports(&self) -> &MacroReportEngine {
        &self.macros
    }

    #[must_use]
    pub fn response(&self) -> &str {
        &self.outbound
    }

    pub fn clear_response(&mut self) {
        self.outbound.clear();
        self.responses.clear_response();
        self.macros.clear_response();
    }

    pub const fn set_response_writable(&mut self, writable: bool) {
        self.writable = writable;
        self.responses.set_response_writable(writable);
        self.macros.set_response_writable(writable);
    }

    fn collect_responses(&mut self) {
        if !self.responses.response().is_empty() {
            self.outbound.push_str(self.responses.response());
            self.responses.clear_response();
        }
        if !self.macros.response().is_empty() {
            self.outbound.push_str(self.macros.response());
            self.macros.clear_response();
        }
    }

    fn dispatch_macro_report(&mut self, status: i32, id: Option<i32>) {
        let action = OutputAction::DeviceStatusReport {
            private: true,
            status,
            id,
        };

        if self.writable {
            self.macros.dispatch(action);
            self.collect_responses();
        } else {
            // Preserve the same fail-closed behavior as every other response:
            // an unwritable sink leaves the request visible as deferred work.
            self.responses.dispatch(action);
        }
    }
}

impl TermDispatch for AdaptDispatchProductState {
    fn dispatch(&mut self, action: OutputAction) {
        match action {
            OutputAction::DeviceStatusReport {
                private: true,
                status: status @ (62 | 63),
                id,
            } => self.dispatch_macro_report(status, id),
            other => {
                self.responses.dispatch(other);
                self.collect_responses();
            }
        }
    }

    fn begin_dcs(&mut self, action: DcsAction) -> bool {
        self.dcs_owner = DcsOwner::None;
        match action {
            action @ DcsAction::DefineMacro(_) => {
                if self.macros.begin_dcs(action) {
                    self.dcs_owner = DcsOwner::Macro;
                    true
                } else {
                    false
                }
            }
            other => {
                if self.responses.begin_dcs(other) {
                    self.dcs_owner = DcsOwner::Response;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn dcs_put(&mut self, code_unit: u16) -> bool {
        let result = match self.dcs_owner {
            DcsOwner::Response => self.responses.dcs_put(code_unit),
            DcsOwner::Macro => self.macros.dcs_put(code_unit),
            DcsOwner::None => false,
        };
        self.collect_responses();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macro_buffer::MAX_SPACE;
    use terminal_parser::{output_engine::OutputStateMachineEngine, state_machine::StateMachine};

    #[test]
    fn microsoft_macro_space_report_uses_macros_defined_through_product_dispatch() {
        let dispatch = AdaptDispatchProductState::new(PageGeometry::new(0, 80, 24));
        let mut machine = StateMachine::new(OutputStateMachineEngine::new(dispatch));

        for id in 1..=4 {
            machine.process_str(&format!("\u{1b}P{id};0;0!z12345678\u{1b}\\"));
        }

        let dispatch = machine.engine_mut().dispatch_mut();
        assert_eq!(
            dispatch.macro_reports().buffer().space_available(),
            MAX_SPACE - 32
        );
        dispatch.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 62,
            id: None,
        });
        assert_eq!(
            dispatch.response(),
            format!("\u{1b}[{}*{{", (MAX_SPACE / 16) - 2)
        );
        assert!(
            dispatch
                .response_state()
                .presentation()
                .core()
                .deferred_actions()
                .is_empty()
        );
    }

    #[test]
    fn microsoft_macro_checksum_report_uses_live_product_macro_memory_and_request_id() {
        let dispatch = AdaptDispatchProductState::new(PageGeometry::new(0, 80, 24));
        let mut machine = StateMachine::new(OutputStateMachineEngine::new(dispatch));
        machine.process_str("\u{1b}P1;0;0!zABC\u{1b}\\");

        let dispatch = machine.engine_mut().dispatch_mut();
        let checksum = dispatch.macro_reports().buffer().calculate_checksum();
        dispatch.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 63,
            id: Some(12),
        });
        assert_eq!(
            dispatch.response(),
            format!("\u{1b}P12!~{checksum:04X}\u{1b}\\")
        );
        assert!(
            dispatch
                .response_state()
                .presentation()
                .core()
                .deferred_actions()
                .is_empty()
        );
    }

    #[test]
    fn macro_report_sink_failure_remains_deferred_at_the_product_boundary() {
        let mut dispatch = AdaptDispatchProductState::new(PageGeometry::new(0, 80, 24));
        dispatch.set_response_writable(false);
        dispatch.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 62,
            id: None,
        });
        dispatch.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 63,
            id: Some(7),
        });

        assert!(dispatch.response().is_empty());
        assert_eq!(
            dispatch
                .response_state()
                .presentation()
                .core()
                .deferred_actions()
                .len(),
            2
        );
    }

    #[test]
    fn ordinary_response_order_is_preserved_across_the_composed_product_sink() {
        let mut dispatch = AdaptDispatchProductState::new(PageGeometry::new(0, 80, 24));
        dispatch.dispatch(OutputAction::DeviceStatusReport {
            private: false,
            status: 5,
            id: None,
        });
        dispatch.dispatch(OutputAction::DeviceStatusReport {
            private: true,
            status: 62,
            id: None,
        });
        dispatch.dispatch(OutputAction::RequestTerminalParameters(0));

        assert_eq!(
            dispatch.response(),
            format!(
                "\u{1b}[0n\u{1b}[{}*{{\u{1b}[2;1;1;128;128;1;0x",
                MAX_SPACE / 16
            )
        );
    }
}
