from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if old not in text:
        raise SystemExit(f"R01d repair marker not found: {label}")
    return text.replace(old, new, 1)


state_path = Path("rust/terminal-parser/src/state_machine.rs")
state = state_path.read_text()

state = replace_once(
    state,
    """        if code_unit == ESC && !matches!(self.state, State::OscString | State::OscParam) {
            self.action_interrupt();
            self.enter_escape();
            return;
        }""",
    """        if code_unit == ESC && !matches!(self.state, State::OscString | State::OscParam) {
            let preserve_sequence =
                self.state == State::DcsIgnore && !self.sequence_buffer.is_empty();
            self.action_interrupt();
            if preserve_sequence {
                self.enter_escape_preserving_sequence();
            } else {
                self.enter_escape();
            }
            return;
        }""",
    "preserve unhandled DCS across ESC",
)

state = replace_once(
    state,
    """        if self.runtime.dcs_handler_active {
            self.state = State::DcsPassThrough;
        } else {
            self.state = State::DcsIgnore;
        }
        self.sequence_buffer.clear();""",
    """        if self.runtime.dcs_handler_active {
            self.state = State::DcsPassThrough;
            self.sequence_buffer.clear();
        } else {
            self.state = State::DcsIgnore;
        }""",
    "retain unhandled DCS buffer",
)

state = replace_once(
    state,
    """    fn enter_escape(&mut self) {
        self.state = State::Escape;
        self.action_clear();
        self.sequence_buffer.clear();
        self.sequence_buffer.push(ESC);
    }

    fn enter_csi_entry(&mut self) {""",
    """    fn enter_escape(&mut self) {
        self.state = State::Escape;
        self.action_clear();
        self.sequence_buffer.clear();
        self.sequence_buffer.push(ESC);
    }

    fn enter_escape_preserving_sequence(&mut self) {
        self.state = State::Escape;
        self.action_clear();
        self.sequence_buffer.push(ESC);
    }

    fn enter_csi_entry(&mut self) {""",
    "escape entry that preserves buffered DCS",
)

state = replace_once(
    state,
    """    fn event_escape(&mut self, code_unit: u16) {
        if is_c0(code_unit) {
            if self.config.input_engine {
                let _ = self.engine.action_execute_from_escape(code_unit);
                self.enter_ground();
            } else {
                let _ = self.engine.action_execute(code_unit);
            }
        } else if code_unit == DEL {
        } else if is_intermediate(code_unit) {""",
    """    fn event_escape(&mut self, code_unit: u16) {
        if is_c0(code_unit)
            || (self.config.input_engine && matches!(code_unit, CAN | SUB))
        {
            if self.config.input_engine {
                if !self.engine.action_execute_from_escape(code_unit) {
                    let _ = self.flush_to_terminal();
                }
                self.enter_ground();
            } else {
                let _ = self.engine.action_execute(code_unit);
            }
        } else if code_unit == DEL {
            if self.config.input_engine {
                self.action_esc_dispatch(code_unit);
                self.enter_ground();
            }
        } else if is_intermediate(code_unit) {""",
    "input ESC controls and Alt+Backspace",
)

state_path.write_text(state)

input_path = Path("rust/terminal-parser/src/input_engine.rs")
input_text = input_path.read_text()

input_text = replace_once(
    input_text,
    """        assert_eq!(mice[3].button_state, SCROLL_DELTA_FORWARD);
        assert_eq!(mice[3].event_flags, MOUSE_WHEELED);""",
    """        assert_eq!(mice[3].button_state, SCROLL_DELTA_BACKWARD);
        assert_eq!(mice[3].event_flags, MOUSE_HWHEELED);""",
    "first horizontal wheel expectation",
)
input_text = replace_once(
    input_text,
    """        assert_eq!(mice[4].event_flags, MOUSE_WHEELED);""",
    """        assert_eq!(mice[4].event_flags, MOUSE_HWHEELED);""",
    "second horizontal wheel expectation",
)
input_text = replace_once(
    input_text,
    r"\u{1b}[<2;1;1M\u{1b}[<2;1;1m\u{1b}[<2;1;1M\u{1b}[<65;2;2M\u{1b}[<64;3;3M",
    r"\u{1b}[<2;1;1M\u{1b}[<2;1;1m\u{1b}[<2;1;1M\u{1b}[<66;2;2M\u{1b}[<67;3;3M",
    "horizontal wheel test encoding",
)
input_text = replace_once(
    input_text,
    """        assert_eq!(mice[3].event_flags, MOUSE_WHEELED);
        assert_eq!(mice[4].button_state & 0xffff_0000, SCROLL_DELTA_FORWARD);
        assert_eq!(mice[4].event_flags, MOUSE_WHEELED);""",
    """        assert_eq!(mice[3].event_flags, MOUSE_HWHEELED);
        assert_eq!(mice[4].button_state & 0xffff_0000, SCROLL_DELTA_FORWARD);
        assert_eq!(mice[4].event_flags, MOUSE_HWHEELED);""",
    "horizontal wheel flags",
)

input_path.write_text(input_text)
