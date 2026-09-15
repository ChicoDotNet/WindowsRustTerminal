//! Stateful VT parser seam for incremental product routing.
//!
//! Rust owns parser state and parameter mechanics. The native host supplies
//! synchronous callbacks for the semantic actions that are routed through this
//! first Ground/Escape/CSI slice. Callback pointers and borrowed buffers are
//! valid only for the duration of the call; consumers must copy data they need
//! to retain.

use std::{ffi::c_void, ptr, slice};

use terminal_parser::state_machine::{Parameters, StateMachine, StateMachineEngine, VtId};

use crate::{FfiStatus, ffi_guard};

pub type ExecuteCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type PrintCallback = unsafe extern "C" fn(*mut c_void, u16) -> bool;
pub type PrintStringCallback = unsafe extern "C" fn(*mut c_void, *const u16, usize) -> bool;
pub type CsiCallback = unsafe extern "C" fn(
    *mut c_void,
    u64,
    *const i32,
    *const u8,
    usize,
) -> bool;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StateMachineCallbacks {
    pub user_data: *mut c_void,
    pub execute: Option<ExecuteCallback>,
    pub print: Option<PrintCallback>,
    pub print_string: Option<PrintStringCallback>,
    pub csi: Option<CsiCallback>,
}

struct CallbackEngine {
    callbacks: StateMachineCallbacks,
}

impl StateMachineEngine for CallbackEngine {
    fn action_execute(&mut self, code_unit: u16) -> bool {
        let Some(callback) = self.callbacks.execute else {
            return false;
        };
        // SAFETY: The callback and context are supplied by the C ABI caller and
        // are required to remain valid for the lifetime of the parser handle.
        unsafe { callback(self.callbacks.user_data, code_unit) }
    }

    fn action_print(&mut self, code_unit: u16) -> bool {
        let Some(callback) = self.callbacks.print else {
            return false;
        };
        // SAFETY: Same lifetime contract as `action_execute`.
        unsafe { callback(self.callbacks.user_data, code_unit) }
    }

    fn action_print_string(&mut self, text: &[u16]) -> bool {
        let Some(callback) = self.callbacks.print_string else {
            return false;
        };
        // SAFETY: `text` remains readable for the duration of this synchronous
        // callback. The consumer must copy it to retain it.
        unsafe { callback(self.callbacks.user_data, text.as_ptr(), text.len()) }
    }

    fn action_csi_dispatch(&mut self, id: VtId, parameters: &Parameters) -> bool {
        let Some(callback) = self.callbacks.csi else {
            return false;
        };
        let values = parameters.values();
        let mut raw_values = Vec::with_capacity(values.len());
        let mut present = Vec::with_capacity(values.len());
        for value in values {
            raw_values.push(value.unwrap_or_default());
            present.push(u8::from(value.is_some()));
        }
        // SAFETY: Both vectors remain alive and immutable for the duration of
        // the synchronous callback. Presence is carried separately so an empty
        // parameter cannot be confused with numeric zero.
        unsafe {
            callback(
                self.callbacks.user_data,
                id.value(),
                raw_values.as_ptr(),
                present.as_ptr(),
                raw_values.len(),
            )
        }
    }
}

pub struct StateMachineHandle {
    machine: StateMachine<CallbackEngine>,
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_create(
    callbacks: *const StateMachineCallbacks,
    out_handle: *mut *mut StateMachineHandle,
) -> FfiStatus {
    ffi_guard(|| {
        if callbacks.is_null() || out_handle.is_null() {
            return FfiStatus::InvalidArgument;
        }
        // SAFETY: Both pointers were checked non-null. The caller must provide
        // readable callbacks and writable handle storage for this call.
        let callbacks = unsafe { ptr::read(callbacks) };
        let handle = Box::new(StateMachineHandle {
            machine: StateMachine::new(CallbackEngine { callbacks }),
        });
        // SAFETY: `out_handle` is writable by ABI contract. Ownership of the
        // allocation transfers to the caller until destroy is called.
        unsafe { ptr::write(out_handle, Box::into_raw(handle)) };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_process_utf16(
    handle: *mut StateMachineHandle,
    text: *const u16,
    text_len: usize,
) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() || (text.is_null() && text_len != 0) {
            return FfiStatus::InvalidArgument;
        }
        let text = if text_len == 0 {
            &[]
        } else {
            // SAFETY: Non-null pointer with `text_len` readable code units is
            // required by the ABI for the duration of this call.
            unsafe { slice::from_raw_parts(text, text_len) }
        };
        // SAFETY: The handle originates from create and must be exclusively
        // borrowed by the caller for each process call.
        unsafe { &mut *handle }.machine.process_utf16(text);
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_state_machine_destroy(
    handle: *mut StateMachineHandle,
) -> FfiStatus {
    ffi_guard(|| {
        if handle.is_null() {
            return FfiStatus::InvalidArgument;
        }
        // SAFETY: The handle must be returned exactly once by its owner.
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
        csi: Vec<(u64, Vec<Option<i32>>)>,
    }

    unsafe extern "C" fn execute(context: *mut c_void, code_unit: u16) -> bool {
        // SAFETY: Tests pass a live Witness as context for the handle lifetime.
        unsafe { &mut *context.cast::<Witness>() }
            .executed
            .push(code_unit);
        true
    }

    unsafe extern "C" fn print_string(
        context: *mut c_void,
        text: *const u16,
        len: usize,
    ) -> bool {
        // SAFETY: The adapter guarantees a readable callback-local slice.
        let text = unsafe { slice::from_raw_parts(text, len) };
        // SAFETY: Tests pass a live Witness as context for the handle lifetime.
        unsafe { &mut *context.cast::<Witness>() }
            .printed
            .extend_from_slice(text);
        true
    }

    unsafe extern "C" fn csi(
        context: *mut c_void,
        id: u64,
        values: *const i32,
        present: *const u8,
        len: usize,
    ) -> bool {
        // SAFETY: The adapter guarantees readable callback-local slices.
        let values = unsafe { slice::from_raw_parts(values, len) };
        let present = unsafe { slice::from_raw_parts(present, len) };
        let parameters = values
            .iter()
            .zip(present)
            .map(|(&value, &is_present)| (is_present != 0).then_some(value))
            .collect();
        // SAFETY: Tests pass a live Witness as context for the handle lifetime.
        unsafe { &mut *context.cast::<Witness>() }
            .csi
            .push((id, parameters));
        true
    }

    #[test]
    fn stateful_ffi_preserves_parser_state_across_fragmented_writes() {
        let mut witness = Witness::default();
        let callbacks = StateMachineCallbacks {
            user_data: (&mut witness as *mut Witness).cast(),
            execute: Some(execute),
            print: None,
            print_string: Some(print_string),
            csi: Some(csi),
        };
        let mut handle = ptr::null_mut();
        assert_eq!(
            terminal_parser_ffi_state_machine_create(&callbacks, &mut handle),
            FfiStatus::Ok
        );

        for fragment in ["ready", "\u{1b}[", "12;", "34H"] {
            let units = fragment.encode_utf16().collect::<Vec<_>>();
            assert_eq!(
                terminal_parser_ffi_state_machine_process_utf16(
                    handle,
                    units.as_ptr(),
                    units.len()
                ),
                FfiStatus::Ok
            );
        }

        assert_eq!(String::from_utf16(&witness.printed).unwrap(), "ready");
        assert_eq!(witness.csi, vec![(u64::from(b'H'), vec![Some(12), Some(34)])]);
        assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);
    }

    #[test]
    fn stateful_ffi_rejects_invalid_ownership_arguments() {
        assert_eq!(
            terminal_parser_ffi_state_machine_create(ptr::null(), ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_state_machine_process_utf16(ptr::null_mut(), ptr::null(), 0),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_state_machine_destroy(ptr::null_mut()),
            FfiStatus::InvalidArgument
        );
    }
}
