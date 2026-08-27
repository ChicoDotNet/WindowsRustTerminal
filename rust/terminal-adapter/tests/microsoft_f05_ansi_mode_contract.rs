use terminal_adapter::{
    adapt_dispatch::PageGeometry, parser_control::set_ansi_mode,
    product_dispatch::AdaptDispatchProductState,
};
use terminal_parser::{
    output_engine::OutputStateMachineEngine,
    state_machine::{ParserMode, StateMachine},
};

#[test]
fn microsoft_ansi_mode_test_mutates_the_live_state_machine_mode() {
    let dispatch = AdaptDispatchProductState::new(PageGeometry::new(20, 100, 29));
    let mut machine = StateMachine::new(OutputStateMachineEngine::new(dispatch));

    machine.set_parser_mode(ParserMode::Ansi, false);
    assert!(!machine.get_parser_mode(ParserMode::Ansi));

    set_ansi_mode(&mut machine, true);
    assert!(machine.get_parser_mode(ParserMode::Ansi));

    set_ansi_mode(&mut machine, false);
    assert!(!machine.get_parser_mode(ParserMode::Ansi));
}
