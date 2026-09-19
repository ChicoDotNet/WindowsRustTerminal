//! Stateful VT parser seam for incremental product routing.
//!
//! Rust owns parser state and parameter mechanics. The native host supplies
//! synchronous callbacks for semantic actions. Callback pointers and borrowed
//! buffers are valid only for the duration of the call; consumers must copy
//! data they need to retain.

use std::{ffi::c_void, ptr, slice};

use terminal_parser::state_machine::{Parameters, ParserMode, StateMachine, StateMachineEngine, VtId};

use crate::{FfiStatus, ffi_guard};

pub type ExecuteCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type PrintCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type PrintStringCallback = unsafe extern "C" fn(*mut c_void, *const u16, usize) -> bool;
pub type PassThroughCallback = unsafe extern "C" fn(*mut c_void, *const u16, usize) -> bool;
pub type EscCallback = unsafe extern "C" fn(*mut c_void, u64) -> bool;
pub type ExecuteFromEscapeCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type Vt52EscCallback = unsafe extern "C" fn(*mut c_void, u64, *const i32, *const u8, usize) -> bool;
pub type Ss3Callback = unsafe extern "C" fn(*mut c_void, u16, *const i32, *const u8, usize) -> bool;
pub type CsiCallback = unsafe extern "C" fn(*mut c_void, u64, *const i32, *const u8, usize) -> bool;
pub type OscCallback = unsafe extern "C" fn(*mut c_void, i32, *const u16, usize) -> bool;
pub type DcsDispatchCallback = unsafe extern "C" fn(*mut c_void, u64, *const i32, *const u8, usize) -> bool;
pub type DcsPutCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;

const PARSER_MODE_ACCEPT_C1: u32 = 0;
const PARSER_MODE_ANSI: u32 = 1;

fn parser_mode(mode: u32) -> Option<ParserMode> {
    match mode {
        PARSER_MODE_ACCEPT_C1 => Some(ParserMode::AcceptC1),
        PARSER_MODE_ANSI => Some(ParserMode::Ansi),
        _ => None,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StateMachineCallbacks {
    pub user_data: *mut c_void,
    pub execute: Option<ExecuteCallback>,
    pub print: Option<PrintCallback>,
    pub print_string: Option<PrintStringCallback>,
    pub esc: Option<EscCallback>,
    pub csi: Option<CsiCallback>,
    pub osc: Option<OscCallback>,
    pub dcs_dispatch: Option<DcsDispatchCallback>,
    pub dcs_put: Option<DcsPutCallback>,
}

fn raw_parameters(parameters: &Parameters) -> (Vec<i32>, Vec<u8>) {
    let values = parameters.values();
    let mut raw_values = Vec::with_capacity(values.len());
    let mut present = Vec::with_capacity(values.len());
    for value in values {
        raw_values.push(value.unwrap_or_default());
        present.push(u8::from(value.is_some()));
    }
    (raw_values, present)
}

struct CallbackEngine {
    callbacks: StateMachineCallbacks,
    pass_through: Option<PassThroughCallback>,
    execute_from_escape: Option<ExecuteFromEscapeCallback>,
    vt52_esc: Option<Vt52EscCallback>,
    ss3: Option<Ss3Callback>,
}

impl StateMachineEngine for CallbackEngine {
    fn action_execute(&mut self, code_unit: u16) -> bool {
        let Some(callback) = self.callbacks.execute else { return false; };
        unsafe { callback(self.callbacks.user_data, code_unit) }
    }

    fn action_execute_from_escape(&mut self, code_unit: u16) -> bool {
        let Some(callback) = self.execute_from_escape else { return false; };
        unsafe { callback(self.callbacks.user_data, code_unit) }
    }

    fn action_print(&mut self, code_unit: u16) -> bool {
        let Some(callback) = self.callbacks.print else { return false; };
        unsafe { callback(self.callbacks.user_data, code_unit) }
    }

    fn action_print_string(&mut self, text: &[u16]) -> bool {
        let Some(callback) = self.callbacks.print_string else { return false; };
        unsafe { callback(self.callbacks.user_data, text.as_ptr(), text.len()) }
    }

    fn action_pass_through_string(&mut self, text: &[u16]) -> bool {
        let Some(callback) = self.pass_through else { return false; };
        unsafe { callback(self.callbacks.user_data, text.as_ptr(), text.len()) }
    }

    fn action_esc_dispatch(&mut self, id: VtId) -> bool {
        let Some(callback) = self.callbacks.esc else { return false; };
        unsafe { callback(self.callbacks.user_data, id.value()) }
    }

    fn action_vt52_esc_dispatch(&mut self, id: VtId, parameters: &Parameters) -> bool {
        let Some(callback) = self.vt52_esc else { return false; };
        let (raw_values, present) = raw_parameters(parameters);
        unsafe { callback(self.callbacks.user_data, id.value(), raw_values.as_ptr(), present.as_ptr(), raw_values.len()) }
    }

    fn action_csi_dispatch(&mut self, id: VtId, parameters: &Parameters) -> bool {
        let Some(callback) = self.callbacks.csi else { return false; };
        let (raw_values, present) = raw_parameters(parameters);
        unsafe { callback(self.callbacks.user_data, id.value(), raw_values.as_ptr(), present.as_ptr(), raw_values.len()) }
    }

    fn action_osc_dispatch(&mut self, parameter: i32, text: &[u16]) -> bool {
        let Some(callback) = self.callbacks.osc else { return false; };
        unsafe { callback(self.callbacks.user_data, parameter, text.as_ptr(), text.len()) }
    }

    fn action_ss3_dispatch(&mut self, code_unit: u16, parameters: &Parameters) -> bool {
        let Some(callback) = self.ss3 else { return false; };
        let (raw_values, present) = raw_parameters(parameters);
        unsafe { callback(self.callbacks.user_data, code_unit, raw_values.as_ptr(), present.as_ptr(), raw_values.len()) }
    }

    fn action_dcs_dispatch(&mut self, id: VtId, parameters: &Parameters) -> bool {
        let Some(callback) = self.callbacks.dcs_dispatch else { return false; };
        let (raw_values, present) = raw_parameters(parameters);
        unsafe { callback(self.callbacks.user_data, id.value(), raw_values.as_ptr(), present.as_ptr(), raw_values.len()) }
    }

    fn action_dcs_put(&mut self, code_unit: u16) -> bool {
        let Some(callback) = self.callbacks.dcs_put else { return false; };
        unsafe { callback(self.callbacks.user_data, code_unit) }
    }
}

pub struct StateMachineHandle {
    machine: StateMachine<CallbackEngine>,
}

fn callback_engine(callbacks: StateMachineCallbacks) -> CallbackEngine {
    CallbackEngine { callbacks, pass_through: None, execute_from_escape: None, vt52_esc: None, ss3: None }
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_create(callbacks: *const StateMachineCallbacks, out_handle: *mut *mut StateMachineHandle) -> FfiStatus {
    ffi_guard(|| {
        if callbacks.is_null() || out_handle.is_null() { return FfiStatus::InvalidArgument; }
        let callbacks = unsafe { ptr::read(callbacks) };
        let handle = Box::new(StateMachineHandle { machine: StateMachine::new(callback_engine(callbacks)) });
        unsafe { ptr::write(out_handle, Box::into_raw(handle)) };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_create_input(callbacks: *const StateMachineCallbacks, out_handle: *mut *mut StateMachineHandle) -> FfiStatus {
    ffi_guard(|| {
        if callbacks.is_null() || out_handle.is_null() { return FfiStatus::InvalidArgument; }
        let callbacks = unsafe { ptr::read(callbacks) };
        let handle = Box::new(StateMachineHandle { machine: StateMachine::new_input(callback_engine(callbacks)) });
        unsafe { ptr::write(out_handle, Box::into_raw(handle)) };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_set_parser_mode(handle: *mut StateMachineHandle, mode: u32, enabled: u32) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() || enabled > 1 { return FfiStatus::InvalidArgument; }
        let Some(mode) = parser_mode(mode) else { return FfiStatus::InvalidArgument; };
        unsafe { &mut *handle }.machine.set_parser_mode(mode, enabled != 0);
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_set_pass_through_callback(handle: *mut StateMachineHandle, callback: Option<PassThroughCallback>) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() { return FfiStatus::InvalidArgument; }
        unsafe { &mut *handle }.machine.engine_mut().pass_through = callback;
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_set_execute_from_escape_callback(handle: *mut StateMachineHandle, callback: Option<ExecuteFromEscapeCallback>) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() { return FfiStatus::InvalidArgument; }
        unsafe { &mut *handle }.machine.engine_mut().execute_from_escape = callback;
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_set_vt52_esc_callback(handle: *mut StateMachineHandle, callback: Option<Vt52EscCallback>) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() { return FfiStatus::InvalidArgument; }
        unsafe { &mut *handle }.machine.engine_mut().vt52_esc = callback;
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_set_ss3_callback(handle: *mut StateMachineHandle, callback: Option<Ss3Callback>) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() { return FfiStatus::InvalidArgument; }
        unsafe { &mut *handle }.machine.engine_mut().ss3 = callback;
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_process_utf16(handle: *mut StateMachineHandle, text: *const u16, text_len: usize) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() || (text.is_null() && text_len != 0) { return FfiStatus::InvalidArgument; }
        let text = if text_len == 0 { &[] } else { unsafe { slice::from_raw_parts(text, text_len) } };
        unsafe { &mut *handle }.machine.process_utf16(text);
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_flush_to_terminal(handle: *mut StateMachineHandle) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() { return FfiStatus::InvalidArgument; }
        let _ = unsafe { &mut *handle }.machine.flush_to_terminal();
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_destroy(handle: *mut StateMachineHandle) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() { return FfiStatus::InvalidArgument; }
        drop(unsafe { Box::from_raw(handle) });
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Witness {
        printed: Vec<u16>,
        executed: Vec<u16>,
        execute_from_escape: Vec<u16>,
        esc: Vec<u64>,
        vt52: Vec<(u64, Vec<Option<i32>>)>,
        ss3: Vec<(u16, Vec<Option<i32>>)>,
        csi: Vec<(u64, Vec<Option<i32>>)>,
        osc: Vec<(i32, Vec<u16>)>,
        dcs: Vec<(u64, Vec<Option<i32>>)>,
        dcs_data: Vec<u16>,
        passed_through: Vec<u16>,
        accept_dcs: bool,
    }

    unsafe extern "C" fn execute(context: *mut c_void, code_unit: u16) -> bool { unsafe { &mut *context.cast::<Witness>() }.executed.push(code_unit); true }
    unsafe extern "C" fn execute_from_escape(context: *mut c_void, code_unit: u16) -> bool { unsafe { &mut *context.cast::<Witness>() }.execute_from_escape.push(code_unit); true }
    unsafe extern "C" fn print_string(context: *mut c_void, text: *const u16, len: usize) -> bool { let text = unsafe { slice::from_raw_parts(text, len) }; unsafe { &mut *context.cast::<Witness>() }.printed.extend_from_slice(text); true }
    unsafe extern "C" fn pass_through(context: *mut c_void, text: *const u16, len: usize) -> bool { let text = unsafe { slice::from_raw_parts(text, len) }; unsafe { &mut *context.cast::<Witness>() }.passed_through.extend_from_slice(text); true }
    unsafe extern "C" fn esc(context: *mut c_void, id: u64) -> bool { unsafe { &mut *context.cast::<Witness>() }.esc.push(id); true }
    unsafe fn decode_parameters(values: *const i32, present: *const u8, len: usize) -> Vec<Option<i32>> { let values = unsafe { slice::from_raw_parts(values, len) }; let present = unsafe { slice::from_raw_parts(present, len) }; values.iter().zip(present).map(|(&value, &is_present)| (is_present != 0).then_some(value)).collect() }
    unsafe extern "C" fn vt52(context: *mut c_void, id: u64, values: *const i32, present: *const u8, len: usize) -> bool { let parameters = unsafe { decode_parameters(values, present, len) }; unsafe { &mut *context.cast::<Witness>() }.vt52.push((id, parameters)); true }
    unsafe extern "C" fn ss3(context: *mut c_void, code_unit: u16, values: *const i32, present: *const u8, len: usize) -> bool { let parameters = unsafe { decode_parameters(values, present, len) }; unsafe { &mut *context.cast::<Witness>() }.ss3.push((code_unit, parameters)); true }
    unsafe extern "C" fn csi(context: *mut c_void, id: u64, values: *const i32, present: *const u8, len: usize) -> bool { let parameters = unsafe { decode_parameters(values, present, len) }; unsafe { &mut *context.cast::<Witness>() }.csi.push((id, parameters)); true }
    unsafe extern "C" fn osc(context: *mut c_void, parameter: i32, text: *const u16, len: usize) -> bool { let text = unsafe { slice::from_raw_parts(text, len) }; unsafe { &mut *context.cast::<Witness>() }.osc.push((parameter, text.to_vec())); true }
    unsafe extern "C" fn dcs_dispatch(context: *mut c_void, id: u64, values: *const i32, present: *const u8, len: usize) -> bool { let parameters = unsafe { decode_parameters(values, present, len) }; let witness = unsafe { &mut *context.cast::<Witness>() }; witness.dcs.push((id, parameters)); witness.accept_dcs }
    unsafe extern "C" fn dcs_put(context: *mut c_void, code_unit: u16) -> bool { unsafe { &mut *context.cast::<Witness>() }.dcs_data.push(code_unit); true }

    fn callbacks(witness: &mut Witness) -> StateMachineCallbacks { StateMachineCallbacks { user_data: (witness as *mut Witness).cast(), execute: Some(execute), print: None, print_string: Some(print_string), esc: Some(esc), csi: Some(csi), osc: Some(osc), dcs_dispatch: Some(dcs_dispatch), dcs_put: Some(dcs_put) } }

    #[test]
    fn stateful_ffi_preserves_parser_state_across_fragmented_writes() {
        let mut witness = Witness::default(); let callbacks = callbacks(&mut witness); let mut handle = ptr::null_mut(); assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        for fragment in ["ready", "\u{1b}[", "12;", "34H"] { let units = fragment.encode_utf16().collect::<Vec<_>>(); assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, units.as_ptr(), units.len()), FfiStatus::Ok); }
        assert_eq!(String::from_utf16(&witness.printed).unwrap(), "ready"); assert_eq!(witness.csi, vec![(u64::from(b'H'), vec![Some(12), Some(34)])]); assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_routes_escape_dispatch() {
        let mut witness = Witness::default(); let callbacks = callbacks(&mut witness); let mut handle = ptr::null_mut(); assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok); let units = "\u{1b}7".encode_utf16().collect::<Vec<_>>(); assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, units.as_ptr(), units.len()), FfiStatus::Ok); assert_eq!(witness.esc, vec![u64::from(b'7')]); assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_routes_input_only_actions() {
        let mut witness = Witness::default();
        let callbacks = callbacks(&mut witness);
        let mut handle = ptr::null_mut();
        assert_eq!(terminal_parser_ffi_state_machine_create_input(&callbacks, &mut handle), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_state_machine_set_execute_from_escape_callback(handle, Some(execute_from_escape)), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_state_machine_set_ss3_callback(handle, Some(ss3)), FfiStatus::Ok);
        for sequence in [[0x1b, 0x18].as_slice(), [0x1b, u16::from(b'O'), u16::from(b'1'), u16::from(b';'), u16::from(b'2'), u16::from(b'A')].as_slice()] {
            assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, sequence.as_ptr(), sequence.len()), FfiStatus::Ok);
        }
        assert_eq!(witness.execute_from_escape, vec![0x18]);
        assert_eq!(witness.ss3, vec![(u16::from(b'A'), vec![Some(1), Some(2)])]);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_routes_vt52_dispatch_when_ansi_is_disabled() {
        let mut witness = Witness::default();
        let callbacks = callbacks(&mut witness);
        let mut handle = ptr::null_mut();
        assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_state_machine_set_vt52_esc_callback(handle, Some(vt52)), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_state_machine_set_parser_mode(handle, PARSER_MODE_ANSI, 0), FfiStatus::Ok);
        let sequence = [0x1b, u16::from(b'A')];
        assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, sequence.as_ptr(), sequence.len()), FfiStatus::Ok);
        assert_eq!(witness.vt52, vec![(u64::from(b'A'), vec![])]);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_special_callbacks_fail_closed_on_invalid_handles() {
        assert_eq!(terminal_parser_ffi_state_machine_create_input(ptr::null(), ptr::null_mut()), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_set_execute_from_escape_callback(ptr::null_mut(), Some(execute_from_escape)), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_set_vt52_esc_callback(ptr::null_mut(), Some(vt52)), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_set_ss3_callback(ptr::null_mut(), Some(ss3)), FfiStatus::InvalidArgument);
    }

    #[test]
    fn stateful_ffi_routes_osc_dispatch_across_fragmented_writes() {
        let mut witness = Witness::default(); let callbacks = callbacks(&mut witness); let mut handle = ptr::null_mut(); assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        for fragment in ["\u{1b}]", "2;window ", "title", "\u{7}"] { let units = fragment.encode_utf16().collect::<Vec<_>>(); assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, units.as_ptr(), units.len()), FfiStatus::Ok); }
        assert_eq!(witness.osc.len(), 1); assert_eq!(witness.osc[0].0, 2); assert_eq!(String::from_utf16(&witness.osc[0].1).unwrap(), "window title"); assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_routes_dcs_put_only_when_dispatch_accepts_handler() {
        for accept_dcs in [false, true] {
            let mut witness = Witness { accept_dcs, ..Witness::default() }; let callbacks = callbacks(&mut witness); let mut handle = ptr::null_mut(); assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
            for fragment in ["\u{1b}P1;", "2qpay", "load", "\u{1b}\\"] { let units = fragment.encode_utf16().collect::<Vec<_>>(); assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, units.as_ptr(), units.len()), FfiStatus::Ok); }
            assert_eq!(witness.dcs, vec![(u64::from(b'q'), vec![Some(1), Some(2)])]);
            if accept_dcs { assert_eq!(witness.dcs_data, "payload\u{1b}".encode_utf16().collect::<Vec<_>>()); } else { assert!(witness.dcs_data.is_empty()); }
            assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
        }
    }

    #[test]
    fn stateful_ffi_exposes_pass_through_recovery_without_changing_callback_layout() {
        let mut witness = Witness::default();
        let callbacks = callbacks(&mut witness);
        let mut handle = ptr::null_mut();
        assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_state_machine_set_pass_through_callback(handle, Some(pass_through)), FfiStatus::Ok);
        let fragment = "\u{1b}[12;".encode_utf16().collect::<Vec<_>>();
        assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, fragment.as_ptr(), fragment.len()), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_state_machine_flush_to_terminal(handle), FfiStatus::Ok);
        assert_eq!(witness.passed_through, fragment);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_pass_through_operations_fail_closed_on_invalid_handles() {
        assert_eq!(terminal_parser_ffi_state_machine_set_pass_through_callback(ptr::null_mut(), Some(pass_through)), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_flush_to_terminal(ptr::null_mut()), FfiStatus::InvalidArgument);
    }

    #[test]
    fn stateful_ffi_exposes_accept_c1_without_changing_default_output_mode() {
        let mut witness = Witness::default();
        let callbacks = callbacks(&mut witness);
        let mut handle = ptr::null_mut();
        assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        let csi_c1 = [0x009b, u16::from(b'H')];
        assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, csi_c1.as_ptr(), csi_c1.len()), FfiStatus::Ok);
        assert!(witness.csi.is_empty());
        assert_eq!(terminal_parser_ffi_state_machine_set_parser_mode(handle, PARSER_MODE_ACCEPT_C1, 1), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, csi_c1.as_ptr(), csi_c1.len()), FfiStatus::Ok);
        assert_eq!(witness.csi, vec![(u64::from(b'H'), vec![])]);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_parser_modes_fail_closed_on_invalid_arguments() {
        let mut witness = Witness::default();
        let callbacks = callbacks(&mut witness);
        let mut handle = ptr::null_mut();
        assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_state_machine_set_parser_mode(handle, 99, 1), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_set_parser_mode(handle, PARSER_MODE_ANSI, 2), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_set_parser_mode(ptr::null_mut(), PARSER_MODE_ACCEPT_C1, 1), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_rejects_invalid_ownership_arguments() { assert_eq!(terminal_parser_ffi_state_machine_create(ptr::null(), ptr::null_mut()), FfiStatus::InvalidArgument); assert_eq!(terminal_parser_ffi_state_machine_process_utf16(ptr::null_mut(), ptr::null(), 0), FfiStatus::InvalidArgument); assert_eq!(terminal_parser_ffi_state_machine_destroy(ptr::null_mut()), FfiStatus::InvalidArgument); }
}
