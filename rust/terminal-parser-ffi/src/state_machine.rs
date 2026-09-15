//! Stateful VT parser seam for incremental product routing.
//!
//! Rust owns parser state and parameter mechanics. The native host supplies
//! synchronous callbacks for semantic actions. Callback pointers and borrowed
//! buffers are valid only for the duration of the call; consumers must copy
//! data they need to retain.

use std::{ffi::c_void, ptr, slice};

use terminal_parser::state_machine::{Parameters, StateMachine, StateMachineEngine, VtId};

use crate::{FfiStatus, ffi_guard};

pub type ExecuteCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type PrintCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type PrintStringCallback = unsafe extern "C" fn(*mut c_void, *const u16, usize) -> bool;
pub type EscCallback = unsafe extern "C" fn(*mut c_void, u64) -> bool;
pub type CsiCallback = unsafe extern "C" fn(*mut c_void, u64, *const i32, *const u8, usize) -> bool;
pub type OscCallback = unsafe extern "C" fn(*mut c_void, i32, *const u16, usize) -> bool;

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
}

struct CallbackEngine {
    callbacks: StateMachineCallbacks,
}

impl StateMachineEngine for CallbackEngine {
    fn action_execute(&mut self, code_unit: u16) -> bool {
        let Some(callback) = self.callbacks.execute else { return false; };
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

    fn action_esc_dispatch(&mut self, id: VtId) -> bool {
        let Some(callback) = self.callbacks.esc else { return false; };
        unsafe { callback(self.callbacks.user_data, id.value()) }
    }

    fn action_csi_dispatch(&mut self, id: VtId, parameters: &Parameters) -> bool {
        let Some(callback) = self.callbacks.csi else { return false; };
        let values = parameters.values();
        let mut raw_values = Vec::with_capacity(values.len());
        let mut present = Vec::with_capacity(values.len());
        for value in values {
            raw_values.push(value.unwrap_or_default());
            present.push(u8::from(value.is_some()));
        }
        unsafe { callback(self.callbacks.user_data, id.value(), raw_values.as_ptr(), present.as_ptr(), raw_values.len()) }
    }

    fn action_osc_dispatch(&mut self, parameter: i32, text: &[u16]) -> bool {
        let Some(callback) = self.callbacks.osc else { return false; };
        unsafe { callback(self.callbacks.user_data, parameter, text.as_ptr(), text.len()) }
    }
}

pub struct StateMachineHandle {
    machine: StateMachine<CallbackEngine>,
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_create(callbacks: *const StateMachineCallbacks, out_handle: *mut *mut StateMachineHandle) -> FfiStatus {
    ffi_guard(|| {
        if callbacks.is_null() || out_handle.is_null() { return FfiStatus::InvalidArgument; }
        let callbacks = unsafe { ptr::read(callbacks) };
        let handle = Box::new(StateMachineHandle { machine: StateMachine::new(CallbackEngine { callbacks }) });
        unsafe { ptr::write(out_handle, Box::into_raw(handle)) };
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
        esc: Vec<u64>,
        csi: Vec<(u64, Vec<Option<i32>>)>,
        osc: Vec<(i32, Vec<u16>)>,
    }

    unsafe extern "C" fn execute(context: *mut c_void, code_unit: u16) -> bool {
        unsafe { &mut *context.cast::<Witness>() }.executed.push(code_unit);
        true
    }

    unsafe extern "C" fn print_string(context: *mut c_void, text: *const u16, len: usize) -> bool {
        let text = unsafe { slice::from_raw_parts(text, len) };
        unsafe { &mut *context.cast::<Witness>() }.printed.extend_from_slice(text);
        true
    }

    unsafe extern "C" fn esc(context: *mut c_void, id: u64) -> bool {
        unsafe { &mut *context.cast::<Witness>() }.esc.push(id);
        true
    }

    unsafe extern "C" fn csi(context: *mut c_void, id: u64, values: *const i32, present: *const u8, len: usize) -> bool {
        let values = unsafe { slice::from_raw_parts(values, len) };
        let present = unsafe { slice::from_raw_parts(present, len) };
        let parameters = values.iter().zip(present).map(|(&value, &is_present)| (is_present != 0).then_some(value)).collect();
        unsafe { &mut *context.cast::<Witness>() }.csi.push((id, parameters));
        true
    }

    unsafe extern "C" fn osc(context: *mut c_void, parameter: i32, text: *const u16, len: usize) -> bool {
        let text = unsafe { slice::from_raw_parts(text, len) };
        unsafe { &mut *context.cast::<Witness>() }.osc.push((parameter, text.to_vec()));
        true
    }

    fn callbacks(witness: &mut Witness) -> StateMachineCallbacks {
        StateMachineCallbacks {
            user_data: (witness as *mut Witness).cast(),
            execute: Some(execute),
            print: None,
            print_string: Some(print_string),
            esc: Some(esc),
            csi: Some(csi),
            osc: Some(osc),
        }
    }

    #[test]
    fn stateful_ffi_preserves_parser_state_across_fragmented_writes() {
        let mut witness = Witness::default();
        let callbacks = callbacks(&mut witness);
        let mut handle = ptr::null_mut();
        assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        for fragment in ["ready", "\u{1b}[", "12;", "34H"] {
            let units = fragment.encode_utf16().collect::<Vec<_>>();
            assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, units.as_ptr(), units.len()), FfiStatus::Ok);
        }
        assert_eq!(String::from_utf16(&witness.printed).unwrap(), "ready");
        assert_eq!(witness.csi, vec![(u64::from(b'H'), vec![Some(12), Some(34)])]);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_routes_escape_dispatch() {
        let mut witness = Witness::default();
        let callbacks = callbacks(&mut witness);
        let mut handle = ptr::null_mut();
        assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        let units = "\u{1b}7".encode_utf16().collect::<Vec<_>>();
        assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, units.as_ptr(), units.len()), FfiStatus::Ok);
        assert_eq!(witness.esc, vec![u64::from(b'7')]);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_routes_osc_dispatch_across_fragmented_writes() {
        let mut witness = Witness::default();
        let callbacks = callbacks(&mut witness);
        let mut handle = ptr::null_mut();
        assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
        for fragment in ["\u{1b}]", "2;window ", "title", "\u{7}"] {
            let units = fragment.encode_utf16().collect::<Vec<_>>();
            assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, units.as_ptr(), units.len()), FfiStatus::Ok);
        }
        assert_eq!(witness.osc.len(), 1);
        assert_eq!(witness.osc[0].0, 2);
        assert_eq!(String::from_utf16(&witness.osc[0].1).unwrap(), "window title");
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_rejects_invalid_ownership_arguments() {
        assert_eq!(terminal_parser_ffi_state_machine_create(ptr::null(), ptr::null_mut()), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_process_utf16(ptr::null_mut(), ptr::null(), 0), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(ptr::null_mut()), FfiStatus::InvalidArgument);
    }
}
