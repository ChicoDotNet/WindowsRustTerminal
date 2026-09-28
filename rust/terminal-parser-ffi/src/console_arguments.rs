use std::{ptr, slice};

use terminal_host::console_argument_parser::parse_console_arguments;

use super::{FfiStatus, ffi_guard};

const FLAG_AMBIGUOUS_IS_WIDE: u16 = 1 << 0;
const FLAG_FORCE_V1: u16 = 1 << 1;
const FLAG_FORCE_NO_HANDOFF: u16 = 1 << 2;
const FLAG_HEADLESS: u16 = 1 << 3;
const FLAG_RUN_AS_COM_SERVER: u16 = 1 << 4;
const FLAG_CREATE_SERVER_HANDLE: u16 = 1 << 5;
const FLAG_INHERIT_CURSOR: u16 = 1 << 6;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ConsoleArgumentToken {
    pub data: *const u16,
    pub len: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ConsoleArgumentPlan {
    pub width: i16,
    pub height: i16,
    pub flags: u16,
    pub has_server_handle: u8,
    pub has_signal_handle: u8,
    pub server_handle: u32,
    pub signal_handle: u32,
    pub client_commandline_len: usize,
    pub text_measurement_len: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_console_arguments_plan(
    tokens: *const ConsoleArgumentToken,
    token_count: usize,
    client_commandline: *mut u16,
    client_commandline_capacity: usize,
    text_measurement: *mut u16,
    text_measurement_capacity: usize,
    out_plan: *mut ConsoleArgumentPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null()
            || (tokens.is_null() && token_count != 0)
            || (client_commandline.is_null() && client_commandline_capacity != 0)
            || (text_measurement.is_null() && text_measurement_capacity != 0)
        {
            return FfiStatus::InvalidArgument;
        }

        let token_views = if token_count == 0 {
            &[]
        } else {
            // SAFETY: the ABI requires `tokens` to reference `token_count`
            // readable token descriptors for the duration of this call.
            unsafe { slice::from_raw_parts(tokens, token_count) }
        };
        let mut owned_tokens = Vec::with_capacity(token_views.len());
        for token in token_views {
            if token.data.is_null() && token.len != 0 {
                return FfiStatus::InvalidArgument;
            }
            let units = if token.len == 0 {
                &[]
            } else {
                // SAFETY: each non-empty token descriptor must reference its
                // declared number of readable UTF-16 code units.
                unsafe { slice::from_raw_parts(token.data, token.len) }
            };
            let Ok(value) = String::from_utf16(units) else {
                return FfiStatus::InvalidArgument;
            };
            owned_tokens.push(value);
        }

        let Ok(state) = parse_console_arguments(&owned_tokens) else {
            return FfiStatus::InvalidArgument;
        };
        let client_units = state.client_commandline.encode_utf16().collect::<Vec<_>>();
        let measurement_units = state.text_measurement.encode_utf16().collect::<Vec<_>>();
        let mut flags = 0u16;
        flags |= u16::from(state.ambiguous_is_wide()) * FLAG_AMBIGUOUS_IS_WIDE;
        flags |= u16::from(state.force_v1()) * FLAG_FORCE_V1;
        flags |= u16::from(state.force_no_handoff()) * FLAG_FORCE_NO_HANDOFF;
        flags |= u16::from(state.headless()) * FLAG_HEADLESS;
        flags |= u16::from(state.run_as_com_server()) * FLAG_RUN_AS_COM_SERVER;
        flags |= u16::from(state.create_server_handle()) * FLAG_CREATE_SERVER_HANDLE;
        flags |= u16::from(state.inherit_cursor()) * FLAG_INHERIT_CURSOR;

        let plan = ConsoleArgumentPlan {
            width: state.width,
            height: state.height,
            flags,
            has_server_handle: u8::from(state.server_handle.is_some()),
            has_signal_handle: u8::from(state.signal_handle.is_some()),
            server_handle: state.server_handle.unwrap_or_default(),
            signal_handle: state.signal_handle.unwrap_or_default(),
            client_commandline_len: client_units.len(),
            text_measurement_len: measurement_units.len(),
        };
        // SAFETY: `out_plan` was checked non-null above.
        unsafe { ptr::write(out_plan, plan) };

        if client_commandline_capacity < client_units.len()
            || text_measurement_capacity < measurement_units.len()
        {
            return FfiStatus::BufferTooSmall;
        }
        if !client_units.is_empty() {
            if client_commandline.is_null() {
                return FfiStatus::InvalidArgument;
            }
            // SAFETY: capacity was checked above and the ABI requires a
            // writable caller-owned buffer of that size.
            unsafe { ptr::copy_nonoverlapping(client_units.as_ptr(), client_commandline, client_units.len()) };
        }
        if !measurement_units.is_empty() {
            if text_measurement.is_null() {
                return FfiStatus::InvalidArgument;
            }
            // SAFETY: capacity was checked above and the ABI requires a
            // writable caller-owned buffer of that size.
            unsafe { ptr::copy_nonoverlapping(measurement_units.as_ptr(), text_measurement, measurement_units.len()) };
        }

        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_views(tokens: &[Vec<u16>]) -> Vec<ConsoleArgumentToken> {
        tokens
            .iter()
            .map(|token| ConsoleArgumentToken { data: token.as_ptr(), len: token.len() })
            .collect()
    }

    #[test]
    fn ffi_replays_console_argument_owner_and_sizes_outputs() {
        let tokens = ["--server", "0x40", "--headless", "--textMeasurement", "graphemes", "--", "cmd.exe", "/c", "echo hi"]
            .map(|token| token.encode_utf16().collect::<Vec<_>>());
        let views = token_views(&tokens);
        let mut plan = ConsoleArgumentPlan::default();
        assert_eq!(
            terminal_parser_ffi_console_arguments_plan(views.as_ptr(), views.len(), ptr::null_mut(), 0, ptr::null_mut(), 0, &mut plan),
            FfiStatus::BufferTooSmall
        );
        assert_eq!(plan.server_handle, 0x40);
        assert_eq!(plan.has_server_handle, 1);
        assert_ne!(plan.flags & FLAG_HEADLESS, 0);
        let mut commandline = vec![0u16; plan.client_commandline_len];
        let mut measurement = vec![0u16; plan.text_measurement_len];
        assert_eq!(
            terminal_parser_ffi_console_arguments_plan(views.as_ptr(), views.len(), commandline.as_mut_ptr(), commandline.len(), measurement.as_mut_ptr(), measurement.len(), &mut plan),
            FfiStatus::Ok
        );
        assert_eq!(String::from_utf16(&commandline).unwrap(), "cmd.exe /c \"echo hi\"");
        assert_eq!(String::from_utf16(&measurement).unwrap(), "graphemes");
    }

    #[test]
    fn ffi_fails_closed_for_invalid_tokens_and_arguments() {
        let invalid = "--width".encode_utf16().collect::<Vec<_>>();
        let view = ConsoleArgumentToken { data: invalid.as_ptr(), len: invalid.len() };
        let mut plan = ConsoleArgumentPlan::default();
        assert_eq!(terminal_parser_ffi_console_arguments_plan(&view, 1, ptr::null_mut(), 0, ptr::null_mut(), 0, &mut plan), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_console_arguments_plan(ptr::null(), 1, ptr::null_mut(), 0, ptr::null_mut(), 0, &mut plan), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_console_arguments_plan(ptr::null(), 0, ptr::null_mut(), 0, ptr::null_mut(), 0, ptr::null_mut()), FfiStatus::InvalidArgument);
    }
}