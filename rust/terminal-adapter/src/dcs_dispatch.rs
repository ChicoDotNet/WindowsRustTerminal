//! DCS integration for the safe Rust Adapter migration.
//!
//! R03a-R03c deliberately kept Sixel, DECDMAC macro storage, and the
//! `AdaptDispatch` geometry core independent. This module composes those pieces
//! behind the parser's `TermDispatch` boundary so DCS payloads can now travel
//! from `StateMachine` through `OutputStateMachineEngine` into real Adapter
//! protocol handlers without C++ or FFI.

use crate::adapt_dispatch::{AdaptDispatchCore, MarginRange, PageGeometry};
use crate::macro_buffer::{MacroBuffer, MacroDeleteControl, MacroEncoding};
use crate::sixel::{Background, Config as SixelConfig, Parser as SixelParser, Size as SixelSize};
use terminal_parser::output_engine::{DcsAction, OutputAction, TermDispatch};
use terminal_parser::state_machine::Parameters;

const ESC: u16 = 0x1b;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcsSessionKind {
    Sixel,
    Macro,
}

/// Composite Rust Adapter dispatch surface.
///
/// Regular terminal actions continue to be handled by [`AdaptDispatchCore`].
/// DCS actions are negotiated here so only supported payloads enter the parser
/// pass-through state. Unsupported DCS actions are preserved in the core's
/// deferred-action queue for later Adapter slices instead of being discarded.
#[derive(Debug, Clone)]
pub struct AdapterDispatch {
    core: AdaptDispatchCore,
    macro_buffer: MacroBuffer,
    sixel_parser: Option<SixelParser>,
    active_dcs: Option<DcsSessionKind>,
}

impl AdapterDispatch {
    #[must_use]
    pub fn new(geometry: PageGeometry) -> Self {
        Self {
            core: AdaptDispatchCore::new(geometry),
            macro_buffer: MacroBuffer::default(),
            sixel_parser: None,
            active_dcs: None,
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
    pub const fn macro_buffer(&self) -> &MacroBuffer {
        &self.macro_buffer
    }

    pub const fn macro_buffer_mut(&mut self) -> &mut MacroBuffer {
        &mut self.macro_buffer
    }

    #[must_use]
    pub const fn sixel_parser(&self) -> Option<&SixelParser> {
        self.sixel_parser.as_ref()
    }

    #[must_use]
    pub const fn active_dcs(&self) -> Option<DcsSessionKind> {
        self.active_dcs
    }

    fn begin_sixel(&mut self, parameters: &Parameters) -> bool {
        let Some(canvas) = self.sixel_canvas() else {
            return false;
        };

        let mut config = SixelConfig::new(canvas);
        config.macro_parameter = numeric(parameters, 0);
        config.background = match selective(parameters, 1) {
            1 => Background::Transparent,
            2 => Background::Opaque,
            _ => Background::Default,
        };
        config.background_color = parameters.at(2);

        if let Some(parser) = self.sixel_parser.as_mut() {
            parser.restart_image(config);
            parser.set_display_mode(self.core.sixel_display_mode());
        } else {
            let mut parser = SixelParser::new(config);
            parser.set_display_mode(self.core.sixel_display_mode());
            self.sixel_parser = Some(parser);
        }
        self.active_dcs = Some(DcsSessionKind::Sixel);
        true
    }

    fn begin_macro(&mut self, parameters: &Parameters) -> bool {
        let Ok(macro_id) = usize::try_from(selective(parameters, 0)) else {
            return false;
        };
        let delete_control = match selective(parameters, 1) {
            0 => MacroDeleteControl::DeleteId,
            1 => MacroDeleteControl::DeleteAll,
            _ => return false,
        };
        let encoding = match selective(parameters, 2) {
            0 => MacroEncoding::Text,
            1 => MacroEncoding::HexPair,
            _ => return false,
        };

        if !self
            .macro_buffer
            .init_parser(macro_id, delete_control, encoding)
        {
            return false;
        }

        self.active_dcs = Some(DcsSessionKind::Macro);
        true
    }

    fn sixel_canvas(&self) -> Option<SixelSize> {
        // The default Windows Terminal Sixel conformance level uses a 10x20
        // protocol cell. Ask the parser for that value instead of duplicating
        // it here so this integration stays aligned with the R03a core.
        let probe = SixelParser::new(SixelConfig::new(SixelSize::new(1, 1)));
        let cell = probe.cell_size();
        let geometry = self.core.geometry();

        let (width_cells, height_cells) = if self.core.sixel_display_mode() {
            (geometry.width, geometry.height)
        } else {
            let horizontal = self
                .core
                .margins()
                .horizontal()
                .unwrap_or_else(|| MarginRange::new(0, geometry.right()));
            let vertical = self
                .core
                .margins()
                .vertical()
                .unwrap_or_else(|| MarginRange::new(0, geometry.height - 1));
            let bottom = geometry.top.saturating_add(vertical.end);
            let cursor = self.core.cursor();

            // This is the same origin validity rule used by the C++ Sixel
            // integration when display mode is reset: the cursor must be in
            // the horizontal margin area and not below the bottom margin.
            if cursor.x < horizontal.start || cursor.x > horizontal.end || cursor.y > bottom {
                return None;
            }

            let width = horizontal.end.saturating_sub(cursor.x).saturating_add(1);
            let height = bottom.saturating_sub(cursor.y).saturating_add(1);
            (width, height)
        };

        let width = usize::try_from(width_cells).ok()?.checked_mul(cell.width)?;
        let height = usize::try_from(height_cells)
            .ok()?
            .checked_mul(cell.height)?;
        if width == 0 || height == 0 {
            None
        } else {
            Some(SixelSize::new(width, height))
        }
    }

    fn finish_active_dcs(&mut self) {
        self.active_dcs = None;
    }
}

impl TermDispatch for AdapterDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        let sixel_display_change = match &action {
            OutputAction::SetMode {
                private: true,
                enabled,
                mode: 80,
            } => Some(*enabled),
            _ => None,
        };

        self.core.dispatch(action);

        if let Some(enabled) = sixel_display_change
            && let Some(parser) = self.sixel_parser.as_mut()
        {
            parser.set_display_mode(enabled);
        }
    }

    fn begin_dcs(&mut self, action: DcsAction) -> bool {
        self.finish_active_dcs();
        match action {
            DcsAction::DefineSixelImage(parameters) => self.begin_sixel(&parameters),
            DcsAction::DefineMacro(parameters) => self.begin_macro(&parameters),
            other => {
                self.core.dispatch(OutputAction::DcsBegin(other));
                false
            }
        }
    }

    fn dcs_put(&mut self, code_unit: u16) -> bool {
        match self.active_dcs {
            Some(DcsSessionKind::Sixel) => {
                let Some(parser) = self.sixel_parser.as_mut() else {
                    self.finish_active_dcs();
                    return false;
                };
                parser.put(code_unit);
                if code_unit == ESC {
                    self.finish_active_dcs();
                }
                true
            }
            Some(DcsSessionKind::Macro) => {
                let keep_parsing = self.macro_buffer.parse_definition(code_unit);
                if code_unit == ESC || !keep_parsing {
                    self.finish_active_dcs();
                }
                keep_parsing
            }
            None => false,
        }
    }
}

fn numeric(parameters: &Parameters, index: usize) -> i32 {
    match parameters.at(index) {
        Some(value) if value > 0 => value,
        _ => 1,
    }
}

fn selective(parameters: &Parameters, index: usize) -> i32 {
    parameters.at(index).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapt_dispatch::Point as TextPoint;
    use terminal_parser::output_engine::OutputStateMachineEngine;
    use terminal_parser::state_machine::{State, StateMachine};

    fn geometry() -> PageGeometry {
        PageGeometry::new(0, 80, 24)
    }

    fn machine() -> StateMachine<OutputStateMachineEngine<AdapterDispatch>> {
        StateMachine::new(OutputStateMachineEngine::new(AdapterDispatch::new(
            geometry(),
        )))
    }

    fn dispatch(
        machine: &StateMachine<OutputStateMachineEngine<AdapterDispatch>>,
    ) -> &AdapterDispatch {
        machine.engine().dispatch()
    }

    #[test]
    fn sixel_dcs_flows_end_to_end_into_the_safe_parser() {
        let mut machine = machine();
        machine.process_str("\u{1b}P1;1q@\u{1b}\\");

        assert_eq!(machine.state(), State::Ground);
        assert_eq!(dispatch(&machine).active_dcs(), None);
        let parser = dispatch(&machine)
            .sixel_parser()
            .expect("Sixel parser was negotiated");
        assert_eq!(parser.image_width(), 1);
        assert!(!parser.pixel(0, 0).expect("first pixel exists").transparent);
        assert!(
            !parser
                .pixel(0, 1)
                .expect("aspect-ratio pixel exists")
                .transparent
        );
        assert!(parser.pixel(0, 2).expect("unset pixel exists").transparent);
    }

    #[test]
    fn sixel_parameters_and_display_mode_reach_the_protocol_core() {
        let mut machine = machine();
        machine.process_str("\u{1b}[?80h\u{1b}P7;1q?\u{1b}\\");

        let parser = dispatch(&machine)
            .sixel_parser()
            .expect("Sixel parser was negotiated");
        assert!(parser.display_mode());
        assert_eq!(parser.pixel_aspect_ratio(), 1);
        assert_eq!(parser.image_height(), 6);
    }

    #[test]
    fn sixel_palette_changes_persist_across_image_dcs_sessions() {
        let mut machine = machine();
        machine.process_str("\u{1b}P1;1q#1;2;100;0;0@\u{1b}\\");
        machine.process_str("\u{1b}P1;1q#1@\u{1b}\\");

        let parser = dispatch(&machine)
            .sixel_parser()
            .expect("Sixel parser was reused");
        assert_eq!(
            parser.palette_color(1),
            Some(crate::sixel::Rgb::new(255, 0, 0))
        );
        assert_eq!(parser.pixel(0, 0).expect("pixel exists").color_index, 1);
    }

    #[test]
    fn text_encoded_macro_dcs_persists_in_macro_buffer() {
        let mut machine = machine();
        machine.process_str("\u{1b}P3;0;0!zhello\u{1b}\\");

        assert_eq!(machine.state(), State::Ground);
        assert_eq!(dispatch(&machine).active_dcs(), None);
        let expected = "hello".encode_utf16().collect::<Vec<_>>();
        assert_eq!(
            dispatch(&machine).macro_buffer().macro_contents(3),
            Some(expected.as_slice())
        );
    }

    #[test]
    fn hex_macro_dcs_uses_cpp_selective_parameter_defaults() {
        let mut machine = machine();
        machine.process_str("\u{1b}P4;;1!z4869\u{1b}\\");

        let expected = "Hi".encode_utf16().collect::<Vec<_>>();
        assert_eq!(
            dispatch(&machine).macro_buffer().macro_contents(4),
            Some(expected.as_slice())
        );
    }

    #[test]
    fn invalid_macro_parameters_reject_the_payload_without_mutation() {
        let mut machine = machine();
        machine.process_str("\u{1b}P2;0;0!zkept\u{1b}\\");
        let expected = "kept".encode_utf16().collect::<Vec<_>>();
        assert_eq!(
            dispatch(&machine).macro_buffer().macro_contents(2),
            Some(expected.as_slice())
        );

        machine.process_str("\u{1b}P2;9;0!zdropped\u{1b}\\");
        assert_eq!(
            dispatch(&machine).macro_buffer().macro_contents(2),
            Some(expected.as_slice())
        );
    }

    #[test]
    fn unsupported_dcs_is_deferred_and_its_payload_is_not_misrouted() {
        let mut machine = machine();
        machine.process_str("\u{1b}P1$pignored\u{1b}\\");

        assert_eq!(dispatch(&machine).active_dcs(), None);
        let deferred = dispatch(&machine).core().deferred_actions();
        assert_eq!(deferred.len(), 1);
        assert!(matches!(
            deferred[0],
            OutputAction::DcsBegin(DcsAction::RestoreTerminalState(_))
        ));
    }

    #[test]
    fn regular_output_actions_still_flow_into_adapt_dispatch_core() {
        let mut machine = machine();
        machine.process_str("\u{1b}[10;20H\u{1b}P1;1q@\u{1b}\\");

        assert_eq!(
            dispatch(&machine).core().cursor(),
            TextPoint { x: 19, y: 9 }
        );
        assert!(dispatch(&machine).sixel_parser().is_some());
    }
}
