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

fn replay(id: &str, flags: Option<i32>, mode: Option<i32>) -> Option<OutputAction> {
    let parameters = Parameters::from_values(vec![flags, mode]);
    let mut engine = OutputStateMachineEngine::new(CaptureDispatch::default());
    let _ = engine.action_csi_dispatch(VtId::from_ascii(id), &parameters);
    engine.into_dispatch().action
}

#[test]
fn kitty_keyboard_set_replays_through_the_rust_output_engine() {
    assert_eq!(
        replay("=u", Some(3), Some(2)),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("=u"),
            parameters: Parameters::from_values(vec![Some(3), Some(2)]),
        })
    );
}

#[test]
fn kitty_keyboard_set_preserves_missing_vt_parameters() {
    assert_eq!(
        replay("=u", Some(5), None),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("=u"),
            parameters: Parameters::from_values(vec![Some(5), None]),
        })
    );
    assert_eq!(
        replay("=u", None, None),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("=u"),
            parameters: Parameters::from_values(vec![None, None]),
        })
    );
}

#[test]
fn kitty_keyboard_set_is_distinct_from_neighboring_kitty_sequences() {
    assert_eq!(
        replay(">u", Some(3), Some(2)),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii(">u"),
            parameters: Parameters::from_values(vec![Some(3), Some(2)]),
        })
    );
    assert_eq!(
        replay("<u", Some(3), Some(2)),
        Some(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("<u"),
            parameters: Parameters::from_values(vec![Some(3), Some(2)]),
        })
    );
}
