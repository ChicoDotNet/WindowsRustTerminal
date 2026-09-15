use terminal_parser::state_machine::{Parameters, State, StateMachine, StateMachineEngine, VtId};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Event {
    Print(Vec<u16>),
    Execute(u16),
    Csi { id: u64, params: Vec<Option<i32>>, sub_params: Vec<Vec<Option<i32>>> },
}

#[derive(Default)]
struct WitnessEngine {
    events: Vec<Event>,
}

impl StateMachineEngine for WitnessEngine {
    fn action_execute(&mut self, code_unit: u16) -> bool {
        self.events.push(Event::Execute(code_unit));
        true
    }

    fn action_print_string(&mut self, text: &[u16]) -> bool {
        self.events.push(Event::Print(text.to_vec()));
        true
    }

    fn action_csi_dispatch(&mut self, id: VtId, parameters: &Parameters) -> bool {
        let sub_params = (0..parameters.size())
            .map(|index| parameters.sub_params_for(index).to_vec())
            .collect();
        self.events.push(Event::Csi {
            id: id.value(),
            params: parameters.values().to_vec(),
            sub_params,
        });
        true
    }
}

fn ascii_id(text: &str) -> u64 {
    VtId::from_ascii(text).value()
}

#[test]
fn ground_escape_csi_contract_preserves_state_across_split_writes() {
    let mut machine = StateMachine::new(WitnessEngine::default());

    machine.process_str("prefix\u{1b}[");
    assert_eq!(machine.state(), State::CsiEntry);
    assert_eq!(machine.engine().events, [Event::Print("prefix".encode_utf16().collect())]);

    machine.process_str("12;34");
    assert_eq!(machine.state(), State::CsiParam);
    assert_eq!(machine.engine().events.len(), 1);

    machine.process_str("m");
    assert_eq!(machine.state(), State::Ground);
    assert_eq!(
        machine.engine().events,
        [
            Event::Print("prefix".encode_utf16().collect()),
            Event::Csi {
                id: ascii_id("m"),
                params: vec![Some(12), Some(34)],
                sub_params: vec![vec![], vec![]],
            },
        ]
    );
}

#[test]
fn csi_subparameter_contract_preserves_empty_and_numeric_fields() {
    let mut machine = StateMachine::new(WitnessEngine::default());

    machine.process_str("\u{1b}[38:2::255:128:0m");

    assert_eq!(machine.state(), State::Ground);
    assert_eq!(
        machine.engine().events,
        [Event::Csi {
            id: ascii_id("m"),
            params: vec![Some(38)],
            sub_params: vec![vec![Some(2), None, Some(255), Some(128), Some(0)]],
        }]
    );
}

#[test]
fn cancel_from_csi_executes_control_and_returns_to_ground() {
    let mut machine = StateMachine::new(WitnessEngine::default());

    machine.process_str("\u{1b}[12\u{18}");

    assert_eq!(machine.state(), State::Ground);
    assert_eq!(machine.engine().events, [Event::Execute(0x18)]);
}
