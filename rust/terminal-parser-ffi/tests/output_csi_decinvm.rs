use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{Parameters, StateMachineEngine, VtId};

#[derive(Debug, Default, PartialEq, Eq)]
struct CaptureDispatch {
    action: Option<OutputAction>,
}

impl TermDispatch for CaptureDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        self.action = Some(action);
    }
}

fn replay(id: &str, macro_id: Option<i32>) -> Option<OutputAction> {
    let parameters = Parameters::from_values(vec![macro_id]);
    let mut engine = OutputStateMachineEngine::new(CaptureDispatch::default());
    let _ = engine.action_csi_dispatch(VtId::from_ascii(id), &parameters);
    engine.into_dispatch().action
}

#[test]
fn decinvm_replays_through_the_rust_output_engine() {
    assert_eq!(
        replay("*z", Some(63)),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("*z"),
            parameters: Parameters::from_values(vec![Some(63)]),
        })
    );
}

#[test]
fn decinvm_preserves_missing_and_zero_macro_ids() {
    assert_eq!(
        replay("*z", None),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("*z"),
            parameters: Parameters::from_values(vec![None]),
        })
    );
    assert_eq!(
        replay("*z", Some(0)),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("*z"),
            parameters: Parameters::from_values(vec![Some(0)]),
        })
    );
}

#[test]
fn decinvm_is_distinct_from_neighboring_advanced_csi() {
    assert_eq!(
        replay("*y", Some(1)),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("*y"),
            parameters: Parameters::from_values(vec![Some(1)]),
        })
    );
}
