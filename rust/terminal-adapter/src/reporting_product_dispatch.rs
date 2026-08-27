//! Product composition for DEC presentation-state reports.
//!
//! R08 F05 keeps the established [`AdaptDispatchProductState`] intact while
//! adding DECRQPSR as a narrow response-producing decorator. The wrapper owns
//! DCS presentation-state restore sessions and tabulation-stop reports, while
//! every unrelated parser action continues through the existing product
//! aggregate. This gives the report owner a real parser-to-product path without
//! duplicating the underlying terminal semantics.

use terminal_parser::output_engine::{DcsAction, OutputAction, TermDispatch};

use crate::{
    adapt_dispatch::PageGeometry, presentation_reports::PresentationReportEngine,
    product_dispatch::AdaptDispatchProductState,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum DcsOwner {
    #[default]
    None,
    Product,
    PresentationReport,
}

pub struct AdaptDispatchReportingState {
    product: AdaptDispatchProductState,
    presentation_reports: PresentationReportEngine,
    outbound: String,
    writable: bool,
    dcs_owner: DcsOwner,
}

impl AdaptDispatchReportingState {
    #[must_use]
    pub fn new(geometry: PageGeometry) -> Self {
        Self {
            product: AdaptDispatchProductState::new(geometry),
            presentation_reports: PresentationReportEngine::new(geometry.width),
            outbound: String::new(),
            writable: true,
            dcs_owner: DcsOwner::None,
        }
    }

    #[must_use]
    pub const fn product(&self) -> &AdaptDispatchProductState {
        &self.product
    }

    pub const fn product_mut(&mut self) -> &mut AdaptDispatchProductState {
        &mut self.product
    }

    #[must_use]
    pub const fn presentation_reports(&self) -> &PresentationReportEngine {
        &self.presentation_reports
    }

    pub fn set_text_width(&mut self, width: i32) {
        self.presentation_reports.set_width(width);
    }

    #[must_use]
    pub fn response(&self) -> &str {
        &self.outbound
    }

    pub fn clear_response(&mut self) {
        self.outbound.clear();
        self.product.clear_response();
        self.presentation_reports.clear_response();
    }

    pub const fn set_response_writable(&mut self, writable: bool) {
        self.writable = writable;
        self.product.set_response_writable(writable);
        self.presentation_reports.set_response_writable(writable);
    }

    fn collect_responses(&mut self) {
        if !self.product.response().is_empty() {
            self.outbound.push_str(self.product.response());
            self.product.clear_response();
        }
        if !self.presentation_reports.response().is_empty() {
            self.outbound.push_str(self.presentation_reports.response());
            self.presentation_reports.clear_response();
        }
    }

    fn dispatch_presentation_report(&mut self, action: OutputAction) {
        if self.writable {
            self.presentation_reports.dispatch(action);
            self.collect_responses();
        } else {
            self.product.dispatch(action);
            self.collect_responses();
        }
    }
}

impl TermDispatch for AdaptDispatchReportingState {
    fn dispatch(&mut self, action: OutputAction) {
        if PresentationReportEngine::is_tabulation_report(&action) {
            self.dispatch_presentation_report(action);
        } else if PresentationReportEngine::is_clear_all_tabs(&action) {
            self.presentation_reports.dispatch(action);
        } else {
            self.product.dispatch(action);
            self.collect_responses();
        }
    }

    fn begin_dcs(&mut self, action: DcsAction) -> bool {
        self.dcs_owner = DcsOwner::None;
        if PresentationReportEngine::handles_restore(&action) {
            if self.presentation_reports.begin_dcs(action) {
                self.dcs_owner = DcsOwner::PresentationReport;
                return true;
            }
            return false;
        }

        if self.product.begin_dcs(action) {
            self.dcs_owner = DcsOwner::Product;
            true
        } else {
            false
        }
    }

    fn dcs_put(&mut self, code_unit: u16) -> bool {
        let result = match self.dcs_owner {
            DcsOwner::Product => self.product.dcs_put(code_unit),
            DcsOwner::PresentationReport => self.presentation_reports.dcs_put(code_unit),
            DcsOwner::None => false,
        };
        self.collect_responses();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use terminal_parser::{
        output_engine::OutputStateMachineEngine,
        state_machine::{Parameters, StateMachine, VtId},
    };

    #[test]
    fn microsoft_tabulation_stop_report_flows_parser_through_reporting_product_state() {
        let dispatch = AdaptDispatchReportingState::new(PageGeometry::new(0, 80, 24));
        let mut machine = StateMachine::new(OutputStateMachineEngine::new(dispatch));

        machine.process_str("\u{1b}[2$w");
        assert_eq!(
            machine.engine().dispatch().response(),
            "\u{1b}P2$u9/17/25/33/41/49/57/65/73\u{1b}\\"
        );

        machine.engine_mut().dispatch_mut().clear_response();
        machine.engine_mut().dispatch_mut().set_text_width(132);
        machine.process_str("\u{1b}[2$w");
        assert_eq!(
            machine.engine().dispatch().response(),
            "\u{1b}P2$u9/17/25/33/41/49/57/65/73/81/89/97/105/113/121/129\u{1b}\\"
        );

        machine.engine_mut().dispatch_mut().clear_response();
        machine.engine_mut().dispatch_mut().set_text_width(80);
        machine.process_str("\u{1b}P2$t30/60/120/240\u{1b}\\\u{1b}[2$w");
        assert_eq!(
            machine.engine().dispatch().response(),
            "\u{1b}P2$u30/60\u{1b}\\"
        );

        machine.engine_mut().dispatch_mut().clear_response();
        machine.engine_mut().dispatch_mut().set_text_width(132);
        machine.process_str("\u{1b}[2$w");
        assert_eq!(
            machine.engine().dispatch().response(),
            "\u{1b}P2$u30/60/120\u{1b}\\"
        );

        machine.engine_mut().dispatch_mut().clear_response();
        machine.engine_mut().dispatch_mut().set_text_width(80);
        for (restore, expected) in [
            ("44/22/66", "\u{1b}P2$u22/44/66\u{1b}\\"),
            ("3//7", "\u{1b}P2$u3/7\u{1b}\\"),
            ("0/5/10", "\u{1b}P2$u5/10\u{1b}\\"),
            ("1/8/18", "\u{1b}P2$u8/18\u{1b}\\"),
        ] {
            machine.process_str(&format!("\u{1b}P2$t{restore}\u{1b}\\\u{1b}[2$w"));
            assert_eq!(machine.engine().dispatch().response(), expected);
            machine.engine_mut().dispatch_mut().clear_response();
        }

        machine.process_str("\u{1b}[3g\u{1b}[2$w");
        assert_eq!(machine.engine().dispatch().response(), "\u{1b}P2$u\u{1b}\\");
    }

    #[test]
    fn tabulation_stop_report_sink_failure_remains_deferred_at_product_boundary() {
        let mut dispatch = AdaptDispatchReportingState::new(PageGeometry::new(0, 80, 24));
        dispatch.set_response_writable(false);
        dispatch.dispatch(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("$w"),
            parameters: Parameters::from_values(vec![Some(2)]),
        });

        assert!(dispatch.response().is_empty());
        assert_eq!(
            dispatch
                .product()
                .response_state()
                .presentation()
                .core()
                .deferred_actions()
                .len(),
            1
        );
    }

    #[test]
    fn existing_product_responses_keep_order_with_presentation_reports() {
        let mut dispatch = AdaptDispatchReportingState::new(PageGeometry::new(0, 80, 24));
        dispatch.dispatch(OutputAction::DeviceStatusReport {
            private: false,
            status: 5,
            id: None,
        });
        dispatch.dispatch(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("$w"),
            parameters: Parameters::from_values(vec![Some(2)]),
        });

        assert_eq!(
            dispatch.response(),
            "\u{1b}[0n\u{1b}P2$u9/17/25/33/41/49/57/65/73\u{1b}\\"
        );
    }
}
