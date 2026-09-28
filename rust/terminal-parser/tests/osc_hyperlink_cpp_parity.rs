use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
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
