use std::{ptr, slice};

use terminal_parser::output_engine::{OutputAction, OutputStateMachineEngine, TermDispatch};
use terminal_parser::state_machine::{MAX_PARAMETER_COUNT, Parameters, StateMachineEngine, VtId};

use super::{FfiStatus, ffi_guard};

#[derive(Default)]
struct PlanDispatch {
    matched: bool,
    values: Vec<i32>,
}

impl TermDispatch for PlanDispatch {
    fn dispatch(&mut self, action: OutputAction) {
        if let OutputAction::SetGraphicsRendition(parameters) = action {
            self.matched = true;
            self.values = (0..parameters.size())
                .map(|index| parameters.at(index).unwrap_or(0))
                .collect();
        }
    }
}

fn vt_id_from_value(identifier: u64) -> Option<VtId> {
    if identifier & 0xff00_0000_0000_0000 != 0 {
        return None;
    }

    let bytes = identifier.to_le_bytes();
    let length = bytes[..7]
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(7);
    if bytes[length..7].iter().any(|byte| *byte != 0) || !bytes[..length].is_ascii() {
        return None;
    }
    let text = std::str::from_utf8(&bytes[..length]).ok()?;
    Some(VtId::from_ascii(text))
}

/// Replays CSI SGR classification and its complete flat parameter payload
/// through the Rust output engine. All input and output storage remains owned
/// by the caller; this seam does not allocate across the ABI boundary.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_sgr_values(
    identifier: u64,
    values: *const i32,
    value_count: usize,
    out_values: *mut i32,
    output_capacity: usize,
    out_count: *mut usize,
    out_matched: *mut u32,
) -> FfiStatus {
    ffi_guard(|| {
        if out_count.is_null()
            || out_matched.is_null()
            || value_count > MAX_PARAMETER_COUNT
            || (values.is_null() && value_count != 0)
            || (out_values.is_null() && output_capacity != 0)
        {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let values = if value_count == 0 {
            &[]
        } else {
            // SAFETY: The ABI contract requires `values` to reference
            // `value_count` readable i32 values; null with non-zero count was
            // rejected above.
            unsafe { slice::from_raw_parts(values, value_count) }
        };
        let parameters = Parameters::from_values(values.iter().copied().map(Some).collect());
        let mut engine = OutputStateMachineEngine::new(PlanDispatch::default());
        let _ = engine.action_csi_dispatch(id, &parameters);
        let dispatch = engine.into_dispatch();
        let required = dispatch.values.len();

        // SAFETY: `out_count` and `out_matched` were checked non-null above;
        // the ABI requires one writable value of each type for this call.
        unsafe {
            ptr::write(out_count, required);
            ptr::write(out_matched, u32::from(dispatch.matched));
        }

        if output_capacity < required {
            return FfiStatus::BufferTooSmall;
        }
        if required != 0 {
            if out_values.is_null() {
                return FfiStatus::InvalidArgument;
            }
            // SAFETY: `output_capacity >= required`; caller guarantees a
            // writable array and it cannot overlap Rust-owned storage.
            unsafe { ptr::copy_nonoverlapping(dispatch.values.as_ptr(), out_values, required) }
        }

        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::terminal_parser_ffi_output_csi_sgr_values;
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn replay(id: &str, values: &[i32]) -> (bool, Vec<i32>) {
        let mut required = 0usize;
        let mut matched = 0u32;
        let status = terminal_parser_ffi_output_csi_sgr_values(
            VtId::from_ascii(id).value(),
            values.as_ptr(),
            values.len(),
            std::ptr::null_mut(),
            0,
            &mut required,
            &mut matched,
        );
        if required == 0 {
            assert_eq!(status, FfiStatus::Ok);
            return (matched != 0, Vec::new());
        }
        assert_eq!(status, FfiStatus::BufferTooSmall);
        let mut output = vec![0; required];
        assert_eq!(
            terminal_parser_ffi_output_csi_sgr_values(
                VtId::from_ascii(id).value(),
                values.as_ptr(),
                values.len(),
                output.as_mut_ptr(),
                output.len(),
                &mut required,
                &mut matched,
            ),
            FfiStatus::Ok
        );
        (matched != 0, output)
    }

    #[test]
    fn csi_sgr_ffi_replays_default_single_and_multi_parameter_payloads() {
        assert_eq!(replay("m", &[]), (true, vec![0]));
        assert_eq!(replay("m", &[1]), (true, vec![1]));
        assert_eq!(replay("m", &[1, 31, 44]), (true, vec![1, 31, 44]));
        assert_eq!(replay("#{", &[1]), (false, Vec::new()));
    }

    #[test]
    fn csi_sgr_ffi_reports_capacity_and_validates_arguments() {
        let values = [1, 31];
        let mut required = 0usize;
        let mut matched = 0u32;
        let mut one = 0i32;
        assert_eq!(
            terminal_parser_ffi_output_csi_sgr_values(
                VtId::from_ascii("m").value(),
                values.as_ptr(),
                values.len(),
                &mut one,
                1,
                &mut required,
                &mut matched,
            ),
            FfiStatus::BufferTooSmall
        );
        assert_eq!(required, 2);
        assert_eq!(matched, 1);
        assert_eq!(
            terminal_parser_ffi_output_csi_sgr_values(
                VtId::from_ascii("m").value(),
                std::ptr::null(),
                1,
                std::ptr::null_mut(),
                0,
                &mut required,
                &mut matched,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_output_csi_sgr_values(
                VtId::from_ascii("m").value(),
                values.as_ptr(),
                values.len(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                &mut matched,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_output_csi_sgr_values(
                VtId::from_ascii("m").value(),
                values.as_ptr(),
                values.len(),
                std::ptr::null_mut(),
                0,
                &mut required,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );
    }
}
