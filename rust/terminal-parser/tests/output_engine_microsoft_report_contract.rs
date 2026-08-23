use terminal_parser::output_engine::{
    DeviceAttributesKind, OutputAction, OutputStateMachineEngine, TermDispatch,
};
use terminal_parser::state_machine::StateMachine;

#[derive(Debug, Default)]
struct RecordingDispatch {
    actions: Vec<OutputAction>,
}

impl TermDispatch for RecordingDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.actions.push(action);
    }
}

fn machine() -> StateMachine<OutputStateMachineEngine<RecordingDispatch>> {
    StateMachine::new(OutputStateMachineEngine::new(RecordingDispatch::default()))
}

fn actions(machine: &StateMachine<OutputStateMachineEngine<RecordingDispatch>>) -> &[OutputAction] {
    &machine.engine().dispatch().actions
}

#[test]
fn microsoft_output_device_attributes_cover_primary_secondary_and_tertiary_reports() {
    let mut machine = machine();
    machine.process_str("\u{1b}[c\u{1b}[>c\u{1b}[=c");

    assert_eq!(
        actions(&machine),
        [
            OutputAction::DeviceAttributes(DeviceAttributesKind::Primary),
            OutputAction::DeviceAttributes(DeviceAttributesKind::Secondary),
            OutputAction::DeviceAttributes(DeviceAttributesKind::Tertiary),
        ]
    );
}

#[test]
fn microsoft_output_device_status_report_preserves_private_marker_status_and_id() {
    let mut machine = machine();
    machine.process_str("\u{1b}[5n\u{1b}[6n\u{1b}[?6n\u{1b}[?15;7n");

    assert_eq!(
        actions(&machine),
        [
            OutputAction::DeviceStatusReport {
                private: false,
                status: 5,
                id: None,
            },
            OutputAction::DeviceStatusReport {
                private: false,
                status: 6,
                id: None,
            },
            OutputAction::DeviceStatusReport {
                private: true,
                status: 6,
                id: None,
            },
            OutputAction::DeviceStatusReport {
                private: true,
                status: 15,
                id: Some(7),
            },
        ]
    );
}

#[test]
fn microsoft_output_request_terminal_parameters_preserves_reference_values() {
    let mut machine = machine();
    machine.process_str("\u{1b}[0x\u{1b}[1x\u{1b}[2x");

    assert_eq!(
        actions(&machine),
        [
            OutputAction::RequestTerminalParameters(0),
            OutputAction::RequestTerminalParameters(1),
            OutputAction::RequestTerminalParameters(2),
        ]
    );
}

#[test]
fn microsoft_output_tab_clear_preserves_default_and_explicit_modes() {
    let mut machine = machine();
    machine.process_str("\u{1b}[g\u{1b}[0g\u{1b}[3g");

    assert_eq!(
        actions(&machine),
        [
            OutputAction::TabClear(0),
            OutputAction::TabClear(0),
            OutputAction::TabClear(3),
        ]
    );
}

#[test]
fn microsoft_output_set_graphics_rendition_preserves_omitted_and_multiple_parameters() {
    let mut machine = machine();
    machine.process_str("\u{1b}[m\u{1b}[1;31;44m");

    let recorded = actions(&machine);
    assert_eq!(recorded.len(), 2);
    match &recorded[0] {
        OutputAction::SetGraphicsRendition(parameters) => {
            assert_eq!(parameters.values(), &[None]);
        }
        action => panic!("expected SGR action, got {action:?}"),
    }
    match &recorded[1] {
        OutputAction::SetGraphicsRendition(parameters) => {
            assert_eq!(parameters.values(), &[Some(1), Some(31), Some(44)]);
        }
        action => panic!("expected SGR action, got {action:?}"),
    }
}
