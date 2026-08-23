use terminal_parser::state_machine::{Parameters, StateMachine, StateMachineEngine, VtId};

#[derive(Default)]
struct CaptureEngine {
    printed: Vec<u16>,
    passed_through: Vec<u16>,
}

impl StateMachineEngine for CaptureEngine {
    fn action_print_string(&mut self, text: &[u16]) -> bool {
        self.printed.extend_from_slice(text);
        true
    }

    fn action_pass_through_string(&mut self, text: &[u16]) -> bool {
        self.passed_through.extend_from_slice(text);
        true
    }

    fn action_csi_dispatch(&mut self, _id: VtId, _parameters: &Parameters) -> bool {
        false
    }
}

#[test]
fn microsoft_passthrough_unhandled_sequence_before_printable_text() {
    let mut machine = StateMachine::new(CaptureEngine::default());

    machine.process_str("\u{1b}[?999h 12345 Hello World");

    assert_eq!(
        String::from_utf16(&machine.engine().passed_through)
            .expect("test sequence is valid UTF-16"),
        "\u{1b}[?999h"
    );
    assert_eq!(
        String::from_utf16(&machine.engine().printed).expect("test text is valid UTF-16"),
        " 12345 Hello World"
    );
}
