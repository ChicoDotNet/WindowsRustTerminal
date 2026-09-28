//! Safe decoding for the private `ConPTY` signal pipe used by `PtySignalInputThread`.
//!
//! The C++ implementation reads native Windows `unsigned short`/`uint64_t`
//! payloads from the pipe. Windows Terminal targets little-endian Windows, so
//! this module makes that byte order explicit instead of reinterpreting memory.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum PtySignal {
    ShowHideWindow = 1,
    ClearBuffer = 2,
    SetParent = 3,
    ResizeWindow = 8,
}

impl PtySignal {
    /// Decodes the two-byte signal discriminator used by the `ConPTY` signal pipe.
    ///
    /// # Errors
    /// Returns [`PtySignalError::UnknownSignal`] for unsupported signal values.
    pub fn decode(bytes: [u8; 2]) -> Result<Self, PtySignalError> {
        match u16::from_le_bytes(bytes) {
            1 => Ok(Self::ShowHideWindow),
            2 => Ok(Self::ClearBuffer),
            3 => Ok(Self::SetParent),
            8 => Ok(Self::ResizeWindow),
            value => Err(PtySignalError::UnknownSignal(value)),
        }
    }

    #[must_use]
    pub const fn payload_len(self) -> usize {
        match self {
            Self::ShowHideWindow | Self::ClearBuffer => 2,
            Self::ResizeWindow => 4,
            Self::SetParent => 8,
        }
    }
}

/// The deterministic read contract for one PTY signal frame.
///
/// Native code may own the pipe and the exact `ReadFile` calls, while this
/// plan keeps the signal discriminator and required payload size under one
/// Rust owner. That prevents an eventual C ABI seam from duplicating the wire
/// table in C++.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PtySignalPlan {
    pub signal: PtySignal,
    pub payload_len: usize,
}

/// Classifies a PTY signal discriminator and returns its exact payload size.
///
/// # Errors
/// Returns [`PtySignalError::UnknownSignal`] for unsupported signal values.
pub fn plan_signal(bytes: [u8; 2]) -> Result<PtySignalPlan, PtySignalError> {
    let signal = PtySignal::decode(bytes)?;
    Ok(PtySignalPlan {
        signal,
        payload_len: signal.payload_len(),
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResizeWindowData {
    pub columns: u16,
    pub rows: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShowHideData {
    pub show: u16,
}

impl ShowHideData {
    #[must_use]
    pub const fn is_visible(self) -> bool {
        self.show != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClearBufferData {
    pub keep_cursor_row: u16,
}

impl ClearBufferData {
    #[must_use]
    pub const fn keep_cursor_row(self) -> bool {
        self.keep_cursor_row != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SetParentData {
    pub handle: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PtySignalData {
    ShowHideWindow(ShowHideData),
    ClearBuffer(ClearBufferData),
    SetParent(SetParentData),
    ResizeWindow(ResizeWindowData),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PtySignalError {
    UnknownSignal(u16),
    InvalidPayloadLength { expected: usize, actual: usize },
}

/// Decodes the payload associated with one `ConPTY` signal.
///
/// # Errors
/// Returns [`PtySignalError::InvalidPayloadLength`] unless the payload has the
/// exact native-structure size expected by the C++ signal reader.
pub fn decode_payload(signal: PtySignal, payload: &[u8]) -> Result<PtySignalData, PtySignalError> {
    let expected = signal.payload_len();
    if payload.len() != expected {
        return Err(PtySignalError::InvalidPayloadLength {
            expected,
            actual: payload.len(),
        });
    }

    let data = match signal {
        PtySignal::ShowHideWindow => PtySignalData::ShowHideWindow(ShowHideData {
            show: u16::from_le_bytes([payload[0], payload[1]]),
        }),
        PtySignal::ClearBuffer => PtySignalData::ClearBuffer(ClearBufferData {
            keep_cursor_row: u16::from_le_bytes([payload[0], payload[1]]),
        }),
        PtySignal::SetParent => PtySignalData::SetParent(SetParentData {
            handle: u64::from_le_bytes([
                payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
                payload[7],
            ]),
        }),
        PtySignal::ResizeWindow => PtySignalData::ResizeWindow(ResizeWindowData {
            columns: u16::from_le_bytes([payload[0], payload[1]]),
            rows: u16::from_le_bytes([payload[2], payload[3]]),
        }),
    };

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_values_match_cpp_contract() {
        assert_eq!(PtySignal::decode([1, 0]), Ok(PtySignal::ShowHideWindow));
        assert_eq!(PtySignal::decode([2, 0]), Ok(PtySignal::ClearBuffer));
        assert_eq!(PtySignal::decode([3, 0]), Ok(PtySignal::SetParent));
        assert_eq!(PtySignal::decode([8, 0]), Ok(PtySignal::ResizeWindow));
        assert_eq!(
            PtySignal::decode([4, 0]),
            Err(PtySignalError::UnknownSignal(4))
        );
    }

    #[test]
    fn signal_plan_owns_discriminator_and_payload_size() {
        assert_eq!(
            plan_signal([1, 0]),
            Ok(PtySignalPlan {
                signal: PtySignal::ShowHideWindow,
                payload_len: 2,
            })
        );
        assert_eq!(
            plan_signal([2, 0]),
            Ok(PtySignalPlan {
                signal: PtySignal::ClearBuffer,
                payload_len: 2,
            })
        );
        assert_eq!(
            plan_signal([3, 0]),
            Ok(PtySignalPlan {
                signal: PtySignal::SetParent,
                payload_len: 8,
            })
        );
        assert_eq!(
            plan_signal([8, 0]),
            Ok(PtySignalPlan {
                signal: PtySignal::ResizeWindow,
                payload_len: 4,
            })
        );
        assert_eq!(plan_signal([4, 0]), Err(PtySignalError::UnknownSignal(4)));
    }

    #[test]
    fn resize_payload_is_two_little_endian_u16_values() {
        assert_eq!(
            decode_payload(PtySignal::ResizeWindow, &[0x50, 0x00, 0x18, 0x00]),
            Ok(PtySignalData::ResizeWindow(ResizeWindowData {
                columns: 80,
                rows: 24,
            }))
        );
    }

    #[test]
    fn boolean_wire_fields_preserve_raw_u16_semantics() {
        let show = ShowHideData { show: 7 };
        let clear = ClearBufferData { keep_cursor_row: 2 };
        assert!(show.is_visible());
        assert!(clear.keep_cursor_row());
        assert!(!ShowHideData { show: 0 }.is_visible());
        assert!(!ClearBufferData { keep_cursor_row: 0 }.keep_cursor_row());
    }

    #[test]
    fn parent_handle_preserves_all_64_bits() {
        assert_eq!(
            decode_payload(
                PtySignal::SetParent,
                &[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
            ),
            Ok(PtySignalData::SetParent(SetParentData {
                handle: 0x0102_0304_0506_0708,
            }))
        );
    }

    #[test]
    fn payload_lengths_are_exact() {
        assert_eq!(
            decode_payload(PtySignal::ResizeWindow, &[1, 2]),
            Err(PtySignalError::InvalidPayloadLength {
                expected: 4,
                actual: 2,
            })
        );
    }
}
