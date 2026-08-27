//! Portable DEC presentation-state report support.
//!
//! Windows Terminal's DECRQPSR tabulation-stop report is deterministic and
//! depends only on the live text width plus the terminal's stored tab stops.
//! This owner keeps the default eight-column cadence, DCS restore semantics,
//! resize filtering, ordering, clear-all behavior, and exact response framing
//! in safe Rust.

use terminal_parser::{
    output_engine::{DcsAction, OutputAction, TermDispatch},
    state_machine::{Parameters, VtId},
};

use crate::vt_response::VtResponseEngine;

const ESC: u16 = 0x1b;
const TABULATION_STOP_REPORT: i32 = 2;
const MAX_RESTORE_PAYLOAD: usize = 4096;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TabulationStopState {
    restored_stops: Option<Vec<i32>>,
    restore_buffer: Option<String>,
}

impl TabulationStopState {
    #[must_use]
    pub fn restoring(&self) -> bool {
        self.restore_buffer.is_some()
    }

    pub fn begin_restore(&mut self, parameters: &Parameters) -> bool {
        if parameters.at(0).unwrap_or(0) != TABULATION_STOP_REPORT {
            return false;
        }
        self.restore_buffer = Some(String::new());
        true
    }

    pub fn put_restore(&mut self, code_unit: u16) -> bool {
        let Some(buffer) = self.restore_buffer.as_mut() else {
            return false;
        };

        if code_unit == ESC {
            let payload = self.restore_buffer.take().unwrap_or_default();
            self.restore(&payload);
            return false;
        }

        let Ok(byte) = u8::try_from(code_unit) else {
            self.restore_buffer = None;
            return false;
        };
        if !byte.is_ascii() || buffer.len() >= MAX_RESTORE_PAYLOAD {
            self.restore_buffer = None;
            return false;
        }

        buffer.push(char::from(byte));
        true
    }

    pub fn clear_all(&mut self) {
        self.restored_stops = Some(Vec::new());
    }

    #[must_use]
    pub fn report(&self, width: i32) -> String {
        let width = width.max(1);
        let stops = self.visible_stops(width);
        let payload = stops
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join("/");
        format!("\u{1b}P2$u{payload}\u{1b}\\")
    }

    fn restore(&mut self, payload: &str) {
        let mut stops = payload
            .split('/')
            .filter_map(|part| part.parse::<i32>().ok())
            .filter(|stop| *stop > 1)
            .collect::<Vec<_>>();
        stops.sort_unstable();
        stops.dedup();
        self.restored_stops = Some(stops);
    }

    fn visible_stops(&self, width: i32) -> Vec<i32> {
        if let Some(stops) = &self.restored_stops {
            return stops
                .iter()
                .copied()
                .filter(|stop| *stop <= width)
                .collect();
        }

        (9..=width).step_by(8).collect()
    }
}

#[derive(Debug, Clone)]
pub struct PresentationReportEngine {
    width: i32,
    tabulation_stops: TabulationStopState,
    responses: VtResponseEngine,
}

impl PresentationReportEngine {
    #[must_use]
    pub fn new(width: i32) -> Self {
        Self {
            width: width.max(1),
            tabulation_stops: TabulationStopState::default(),
            responses: VtResponseEngine::default(),
        }
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width.max(1);
    }

    #[must_use]
    pub const fn tabulation_stops(&self) -> &TabulationStopState {
        &self.tabulation_stops
    }

    #[must_use]
    pub fn response(&self) -> &str {
        self.responses.response()
    }

    pub fn clear_response(&mut self) {
        self.responses.clear();
    }

    pub const fn set_response_writable(&mut self, writable: bool) {
        self.responses.set_writable(writable);
    }

    #[must_use]
    pub fn is_tabulation_report(action: &OutputAction) -> bool {
        matches!(
            action,
            OutputAction::AdvancedCsi { id, parameters }
                if *id == VtId::from_ascii("$w")
                    && parameters.at(0).unwrap_or(0) == TABULATION_STOP_REPORT
        )
    }

    #[must_use]
    pub fn is_clear_all_tabs(action: &OutputAction) -> bool {
        matches!(action, OutputAction::TabClear(3))
    }

    #[must_use]
    pub fn handles_restore(action: &DcsAction) -> bool {
        matches!(
            action,
            DcsAction::RestorePresentationState(parameters)
                if parameters.at(0).unwrap_or(0) == TABULATION_STOP_REPORT
        )
    }

    fn request_tabulation_report(&mut self) -> bool {
        let response = self.tabulation_stops.report(self.width);
        self.responses.return_response(&response)
    }
}

impl TermDispatch for PresentationReportEngine {
    fn dispatch(&mut self, action: OutputAction) {
        if Self::is_tabulation_report(&action) {
            let _ = self.request_tabulation_report();
        } else if Self::is_clear_all_tabs(&action) {
            self.tabulation_stops.clear_all();
        }
    }

    fn begin_dcs(&mut self, action: DcsAction) -> bool {
        match action {
            DcsAction::RestorePresentationState(parameters) => {
                self.tabulation_stops.begin_restore(&parameters)
            }
            _ => false,
        }
    }

    fn dcs_put(&mut self, code_unit: u16) -> bool {
        self.tabulation_stops.put_restore(code_unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use terminal_parser::{output_engine::OutputStateMachineEngine, state_machine::StateMachine};

    fn restore(state: &mut TabulationStopState, payload: &str) {
        assert!(state.begin_restore(&Parameters::from_values(vec![Some(2)])));
        for unit in payload.encode_utf16() {
            assert!(state.put_restore(unit));
        }
        assert!(!state.put_restore(ESC));
    }

    #[test]
    fn microsoft_tabulation_stop_report_matches_default_restore_resize_and_clear_contract() {
        let mut state = TabulationStopState::default();
        assert_eq!(
            state.report(80),
            "\u{1b}P2$u9/17/25/33/41/49/57/65/73\u{1b}\\"
        );
        assert_eq!(
            state.report(132),
            "\u{1b}P2$u9/17/25/33/41/49/57/65/73/81/89/97/105/113/121/129\u{1b}\\"
        );

        restore(&mut state, "30/60/120/240");
        assert_eq!(state.report(80), "\u{1b}P2$u30/60\u{1b}\\");
        assert_eq!(state.report(132), "\u{1b}P2$u30/60/120\u{1b}\\");

        restore(&mut state, "44/22/66");
        assert_eq!(state.report(80), "\u{1b}P2$u22/44/66\u{1b}\\");

        restore(&mut state, "3//7");
        assert_eq!(state.report(80), "\u{1b}P2$u3/7\u{1b}\\");

        restore(&mut state, "0/5/10");
        assert_eq!(state.report(80), "\u{1b}P2$u5/10\u{1b}\\");

        restore(&mut state, "1/8/18");
        assert_eq!(state.report(80), "\u{1b}P2$u8/18\u{1b}\\");

        state.clear_all();
        assert_eq!(state.report(80), "\u{1b}P2$u\u{1b}\\");
    }

    #[test]
    fn microsoft_tabulation_stop_report_runs_through_the_real_vt_parser() {
        let mut machine = StateMachine::new(OutputStateMachineEngine::new(
            PresentationReportEngine::new(80),
        ));

        machine.process_str("\u{1b}[2$w");
        assert_eq!(
            machine.engine().dispatch().response(),
            "\u{1b}P2$u9/17/25/33/41/49/57/65/73\u{1b}\\"
        );
        machine.engine_mut().dispatch_mut().clear_response();

        machine.process_str("\u{1b}P2$t30/60/120/240\u{1b}\\");
        machine.process_str("\u{1b}[2$w");
        assert_eq!(
            machine.engine().dispatch().response(),
            "\u{1b}P2$u30/60\u{1b}\\"
        );

        machine.engine_mut().dispatch_mut().clear_response();
        machine.engine_mut().dispatch_mut().set_width(132);
        machine.process_str("\u{1b}[2$w");
        assert_eq!(
            machine.engine().dispatch().response(),
            "\u{1b}P2$u30/60/120\u{1b}\\"
        );

        machine.engine_mut().dispatch_mut().clear_response();
        machine.process_str("\u{1b}[3g\u{1b}[2$w");
        assert_eq!(machine.engine().dispatch().response(), "\u{1b}P2$u\u{1b}\\");
    }

    #[test]
    fn unrelated_presentation_restore_selector_is_not_consumed() {
        let mut state = TabulationStopState::default();
        assert!(!state.begin_restore(&Parameters::from_values(vec![Some(1)])));
        assert!(!state.restoring());
    }
}
