use std::ptr;

use terminal_parser::output_csi_decps::{DecpsAction, plan as plan_decps};
use terminal_parser::state_machine::{Parameters, VtId};

use super::{FfiStatus, ffi_guard};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCsiDecpsKind {
    None = 0,
    PlaySounds = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OutputCsiDecpsPlan {
    pub kind: u32,
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

/// Replays DECPS classification through its safe Rust semantic owner.
/// Parameters intentionally remain on the native side of this narrow ABI:
/// the owner already proves parameter preservation, while the C++ product seam
/// continues passing the original VTParameters to ITermDispatch::PlaySounds.
#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_output_csi_decps_plan(
    identifier: u64,
    out_plan: *mut OutputCsiDecpsPlan,
) -> FfiStatus {
    ffi_guard(|| {
        if out_plan.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(id) = vt_id_from_value(identifier) else {
            return FfiStatus::InvalidArgument;
        };

        let kind = match plan_decps(id, &Parameters::default()) {
            Some(DecpsAction::PlaySounds(_)) => OutputCsiDecpsKind::PlaySounds,
            None => OutputCsiDecpsKind::None,
        };
        let plan = OutputCsiDecpsPlan { kind: kind as u32 };

        // SAFETY: `out_plan` was checked non-null above and the ABI requires
        // one writable plan value for the duration of this call.
        unsafe { ptr::write(out_plan, plan) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        OutputCsiDecpsKind, OutputCsiDecpsPlan, terminal_parser_ffi_output_csi_decps_plan,
    };
    use crate::FfiStatus;
    use terminal_parser::state_machine::VtId;

    fn plan(id: &str) -> OutputCsiDecpsPlan {
        let mut plan = OutputCsiDecpsPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decps_plan(VtId::from_ascii(id).value(), &mut plan),
            FfiStatus::Ok
        );
        plan
    }

    #[test]
    fn decps_ffi_replays_rust_semantic_owner() {
        assert_eq!(
            plan(",~"),
            OutputCsiDecpsPlan {
                kind: OutputCsiDecpsKind::PlaySounds as u32,
            }
        );
        assert_eq!(plan(",|"), OutputCsiDecpsPlan::default());
    }

    #[test]
    fn decps_ffi_rejects_invalid_identifier_and_null_output() {
        let mut plan = OutputCsiDecpsPlan::default();
        assert_eq!(
            terminal_parser_ffi_output_csi_decps_plan(0xff00_0000_0000_0000, &mut plan),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_output_csi_decps_plan(
                VtId::from_ascii(",~").value(),
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );
    }
}
