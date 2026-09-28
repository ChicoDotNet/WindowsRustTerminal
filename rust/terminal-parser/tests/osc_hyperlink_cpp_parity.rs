use terminal_parser::output_engine::{
    MAX_URL_LENGTH, OutputAction, OutputStateMachineEngine, TermDispatch,
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

fn hyperlink_action(parameters: &str, uri: &str) -> OutputAction {
    let mut machine = StateMachine::new(OutputStateMachineEngine::new(RecordingDispatch::default()));
    machine.process_str(&format!("\u{1b}]8;{parameters};{uri}\u{7}"));
    machine
        .engine()
        .dispatch()
        .actions
        .last()
        .cloned()
        .expect("OSC 8 should dispatch a semantic hyperlink action")
}

#[test]
fn osc8_uses_the_last_cpp_id_parameter() {
    assert_eq!(
        hyperlink_action("id=first:ignored:id=last", "https://example.test"),
        OutputAction::AddHyperlink {
            uri: "https://example.test".to_owned(),
            custom_id: "last".to_owned(),
        }
    );
}

#[test]
fn osc8_matches_cpp_id_search_within_a_parameter_segment() {
    assert_eq!(
        hyperlink_action("ignored-id=embedded", "https://example.test"),
        OutputAction::AddHyperlink {
            uri: "https://example.test".to_owned(),
            custom_id: "embedded".to_owned(),
        }
    );
}

#[test]
fn osc8_cpp_uri_limit_is_measured_in_utf16_code_units() {
    let mut uri = "a".repeat(MAX_URL_LENGTH - 1);
    uri.push('😀');
    uri.push('z');

    let OutputAction::AddHyperlink { uri, custom_id } = hyperlink_action("id=utf16", &uri) else {
        panic!("expected hyperlink action");
    };

    assert_eq!(custom_id, "utf16");
    assert_eq!(uri.encode_utf16().count(), MAX_URL_LENGTH);
    assert!(uri.ends_with('\u{fffd}'));
}
