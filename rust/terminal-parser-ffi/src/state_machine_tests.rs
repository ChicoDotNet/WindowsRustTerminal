use std::{ffi::c_void, ptr, slice};

use crate::{FfiStatus, state_machine::*};

#[derive(Default)]
struct CsiWitness {
    legacy_calls: usize,
    lossless_calls: usize,
    id: u64,
    values: Vec<i32>,
    present: Vec<u8>,
    sub_values: Vec<i32>,
    sub_present: Vec<u8>,
    sub_offsets: Vec<usize>,
    sub_counts: Vec<usize>,
}

unsafe extern "C" fn legacy_csi(
    user_data: *mut c_void,
    _id: u64,
    _values: *const i32,
    _present: *const u8,
    _parameter_count: usize,
) -> bool {
    let witness = unsafe { &mut *(user_data.cast::<CsiWitness>()) };
    witness.legacy_calls += 1;
    true
}

unsafe extern "C" fn lossless_csi(
    user_data: *mut c_void,
    id: u64,
    values: *const i32,
    present: *const u8,
    parameter_count: usize,
    sub_values: *const i32,
    sub_present: *const u8,
    sub_parameter_count: usize,
    sub_offsets: *const usize,
    sub_counts: *const usize,
) -> bool {
    let witness = unsafe { &mut *(user_data.cast::<CsiWitness>()) };
    witness.lossless_calls += 1;
    witness.id = id;
    witness.values = unsafe { slice::from_raw_parts(values, parameter_count) }.to_vec();
    witness.present = unsafe { slice::from_raw_parts(present, parameter_count) }.to_vec();
    witness.sub_values = unsafe { slice::from_raw_parts(sub_values, sub_parameter_count) }.to_vec();
    witness.sub_present = unsafe { slice::from_raw_parts(sub_present, sub_parameter_count) }.to_vec();
    witness.sub_offsets = unsafe { slice::from_raw_parts(sub_offsets, parameter_count) }.to_vec();
    witness.sub_counts = unsafe { slice::from_raw_parts(sub_counts, parameter_count) }.to_vec();
    true
}

fn callbacks(witness: &mut CsiWitness) -> StateMachineCallbacks {
    StateMachineCallbacks {
        user_data: ptr::from_mut(witness).cast(),
        execute: None,
        print: None,
        print_string: None,
        esc: None,
        csi: Some(legacy_csi),
        osc: None,
        dcs_dispatch: None,
        dcs_put: None,
    }
}

#[test]
fn lossless_csi_callback_preserves_subparameters_and_supersedes_v1_when_installed() {
    let mut witness = CsiWitness::default();
    let callbacks = callbacks(&mut witness);
    let mut handle = ptr::null_mut();
    assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);
    assert_eq!(terminal_parser_ffi_state_machine_set_csi_lossless_callback(handle, Some(lossless_csi)), FfiStatus::Ok);

    let input = "\u{1b}[1:2::3;4:5m".encode_utf16().collect::<Vec<_>>();
    assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, input.as_ptr(), input.len()), FfiStatus::Ok);
    assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);

    assert_eq!(witness.lossless_calls, 1);
    assert_eq!(witness.legacy_calls, 0);
    assert_eq!(witness.values, [1, 4]);
    assert_eq!(witness.present, [1, 1]);
    assert_eq!(witness.sub_values, [2, 0, 3, 5]);
    assert_eq!(witness.sub_present, [1, 0, 1, 1]);
    assert_eq!(witness.sub_offsets, [0, 3]);
    assert_eq!(witness.sub_counts, [3, 1]);
}

#[test]
fn csi_v1_remains_the_fallback_and_lossless_setter_is_fail_closed() {
    assert_eq!(terminal_parser_ffi_state_machine_set_csi_lossless_callback(ptr::null_mut(), Some(lossless_csi)), FfiStatus::InvalidArgument);

    let mut witness = CsiWitness::default();
    let callbacks = callbacks(&mut witness);
    let mut handle = ptr::null_mut();
    assert_eq!(terminal_parser_ffi_state_machine_create(&callbacks, &mut handle), FfiStatus::Ok);

    let input = "\u{1b}[31m".encode_utf16().collect::<Vec<_>>();
    assert_eq!(terminal_parser_ffi_state_machine_process_utf16(handle, input.as_ptr(), input.len()), FfiStatus::Ok);
    assert_eq!(terminal_parser_ffi_state_machine_destroy(handle), FfiStatus::Ok);

    assert_eq!(witness.legacy_calls, 1);
    assert_eq!(witness.lossless_calls, 0);
}
