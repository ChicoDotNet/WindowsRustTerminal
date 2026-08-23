use terminal_parser::input_engine::{InputAction, InputDispatch, InputStateMachineEngine};
use terminal_parser::state_machine::Parameters;

#[derive(Default)]
struct NoopDispatch;

impl InputDispatch for NoopDispatch {
    fn dispatch(&mut self, _action: InputAction) {}
}

#[test]
fn microsoft_win32_input_optionals_matrix() {
    // InputEngineTest::TestWin32InputOptionals varies six independent booleans
    // and the number of supplied parameters from 0 through 6. Exercise the
    // complete 64 * 7 Cartesian product deterministically.
    for mask in 0u8..64 {
        let provide_virtual_key = mask & 0b00_0001 != 0;
        let provide_scan_code = mask & 0b00_0010 != 0;
        let provide_char_data = mask & 0b00_0100 != 0;
        let provide_key_down = mask & 0b00_1000 != 0;
        let provide_modifiers = mask & 0b01_0000 != 0;
        let provide_repeat_count = mask & 0b10_0000 != 0;

        let complete = [
            if provide_virtual_key { 1 } else { 0 },
            if provide_scan_code { 2 } else { 0 },
            if provide_char_data { 3 } else { 0 },
            if provide_key_down { 4 } else { 0 },
            if provide_modifiers { 5 } else { 0 },
            if provide_repeat_count { 6 } else { 0 },
        ];

        for parameter_count in 0usize..=6 {
            let parameters = Parameters::from_values(
                complete[..parameter_count]
                    .iter()
                    .copied()
                    .map(Some)
                    .collect(),
            );
            let key = InputStateMachineEngine::<NoopDispatch>::generate_win32_key(&parameters);

            assert_eq!(
                key.virtual_key,
                if provide_virtual_key && parameter_count > 0 {
                    1
                } else {
                    0
                },
                "mask={mask:#08b}, parameter_count={parameter_count}: virtual key"
            );
            assert_eq!(
                key.scan_code,
                if provide_scan_code && parameter_count > 1 {
                    2
                } else {
                    0
                },
                "mask={mask:#08b}, parameter_count={parameter_count}: scan code"
            );
            assert_eq!(
                key.unicode_char,
                if provide_char_data && parameter_count > 2 {
                    3
                } else {
                    0
                },
                "mask={mask:#08b}, parameter_count={parameter_count}: character"
            );
            assert_eq!(
                key.key_down,
                provide_key_down && parameter_count > 3,
                "mask={mask:#08b}, parameter_count={parameter_count}: key-down"
            );
            assert_eq!(
                key.control_key_state,
                if provide_modifiers && parameter_count > 4 {
                    5
                } else {
                    0
                },
                "mask={mask:#08b}, parameter_count={parameter_count}: modifiers"
            );

            let expected_repeat = if parameter_count == 6 {
                if provide_repeat_count { 6 } else { 0 }
            } else {
                1
            };
            assert_eq!(
                key.repeat_count, expected_repeat,
                "mask={mask:#08b}, parameter_count={parameter_count}: repeat count"
            );
        }
    }
}
