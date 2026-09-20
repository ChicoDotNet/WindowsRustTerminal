//! Stateful VT parser seam for incremental product routing.
//!
//! Rust owns parser state and parameter mechanics. The native host supplies
//! synchronous callbacks for semantic actions. Callback pointers and borrowed
//! buffers are valid only for the duration of the call; consumers must copy
//! data they need to retain.

use std::{ffi::c_void, ptr, slice};

use terminal_parser::state_machine::{Parameters, ParserMode, StateMachine, StateMachineEngine, VtId};

use crate::{FfiStatus, ffi_guard, parameter_view::ParameterView};

pub type ExecuteCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type PrintCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type PrintStringCallback = unsafe extern "C" fn(*mut c_void, *const u16, usize) -> bool;
pub type PassThroughCallback = unsafe extern "C" fn(*mut c_void, *const u16, usize) -> bool;
pub type EscCallback = unsafe extern "C" fn(*mut c_void, u64) -> bool;
pub type ExecuteFromEscapeCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type Vt52EscCallback = unsafe extern "C" fn(*mut c_void, u64, *const i32, *const u8, usize) -> bool;
pub type Ss3Callback = unsafe extern "C" fn(*mut c_void, u16, *const i32, *const u8, usize) -> bool;
pub type CsiCallback = unsafe extern "C" fn(*mut c_void, u64, *const i32, *const u8, usize) -> bool;
pub type CsiLosslessCallback = unsafe extern "C" fn(
    *mut c_void,
    u64,
    *const i32,
    *const u8,
    usize,
    *const i32,
    *const u8,
    usize,
    *const usize,
    *const usize,
) -> bool;
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
    csi_lossless: Option<CsiLosslessCallback>,
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
        if let Some(callback) = self.csi_lossless {
            let view = ParameterView::from_parameters(parameters);
            return unsafe {
                callback(
                    self.callbacks.user_data,
                    id.value(),
                    view.values.as_ptr(),
                    view.present.as_ptr(),
                    view.values.len(),
                    view.sub_values.as_ptr(),
                    view.sub_present.as_ptr(),
                    view.sub_values.len(),
                    view.sub_offsets.as_ptr(),
                    view.sub_counts.as_ptr(),
                )
            };
        }
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
    CallbackEngine { callbacks, pass_through: None, execute_from_escape: None, vt52_esc: None, ss3: None, csi_lossless: None }
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
pub extern "C" fn terminal_parser_ffi_state_machine_set_csi_lossless_callback(handle: *mut StateMachineHandle, callback: Option<CsiLosslessCallback>) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() { return FfiStatus::InvalidArgument; }
        unsafe { &mut *handle }.machine.engine_mut().csi_lossless = callback;
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
pub extern "C" fn terminal_parser_ffi_state_machine_reset_state(handle: *mut StateMachineHandle) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() { return FfiStatus::InvalidArgument; }
        unsafe { &mut *handle }.machine.reset_state();
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
