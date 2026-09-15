use std::ptr;

use terminal_renderer::{RenderMode, RenderSettingsPolicy};

use super::{FfiStatus, ffi_guard};

const VALID_MODE_MASK: u32 = (1 << 6) - 1;
const MODES: [(RenderMode, u32); 6] = [
    (RenderMode::IndexedDistinguishableColors, 1 << 0),
    (RenderMode::AlwaysDistinguishableColors, 1 << 1),
    (RenderMode::IntenseIsBold, 1 << 2),
    (RenderMode::IntenseIsBright, 1 << 3),
    (RenderMode::ScreenReversed, 1 << 4),
    (RenderMode::SynchronizedOutput, 1 << 5),
];

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderSettingsState {
    pub modes: u32,
    pub blink_should_be_faint: u32,
}

fn bool_from_abi(value: u32) -> Option<bool> {
    match value {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

fn mode_from_abi(mode: u32) -> Option<RenderMode> {
    MODES
        .get(mode.checked_sub(1)? as usize)
        .map(|(render_mode, _)| *render_mode)
}

fn policy_from_state(state: RenderSettingsState) -> Option<RenderSettingsPolicy> {
    if state.modes & !VALID_MODE_MASK != 0 || state.blink_should_be_faint > 1 {
        return None;
    }

    let mut policy = RenderSettingsPolicy::default();
    for (mode, mask) in MODES {
        policy.set_mode(mode, state.modes & mask != 0);
    }
    if state.blink_should_be_faint != 0 {
        policy.toggle_blink_rendition();
    }
    Some(policy)
}

fn state_from_policy(policy: RenderSettingsPolicy) -> RenderSettingsState {
    let mut modes = 0;
    for (mode, mask) in MODES {
        if policy.mode(mode) {
            modes |= mask;
        }
    }
    RenderSettingsState {
        modes,
        blink_should_be_faint: u32::from(policy.blink_should_be_faint()),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_render_settings_default(
    out_state: *mut RenderSettingsState,
) -> FfiStatus {
    ffi_guard(|| {
        if out_state.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let state = state_from_policy(RenderSettingsPolicy::default());
        // SAFETY: `out_state` was checked non-null and the ABI requires one
        // writable state value for the duration of this call.
        unsafe { ptr::write(out_state, state) };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_render_settings_set_mode(
    state: *mut RenderSettingsState,
    mode: u32,
    enabled: u32,
) -> FfiStatus {
    ffi_guard(|| {
        if state.is_null() || enabled > 1 {
            return FfiStatus::InvalidArgument;
        }
        let Some(mode) = mode_from_abi(mode) else {
            return FfiStatus::InvalidArgument;
        };
        // SAFETY: `state` was checked non-null and the ABI requires one
        // readable/writable state value for the duration of this call.
        let current = unsafe { ptr::read(state) };
        let Some(mut policy) = policy_from_state(current) else {
            return FfiStatus::InvalidArgument;
        };
        policy.set_mode(mode, enabled != 0);
        // SAFETY: `state` remains valid under the same ABI contract.
        unsafe { ptr::write(state, state_from_policy(policy)) };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_render_settings_get_mode(
    state: *const RenderSettingsState,
    mode: u32,
    out_enabled: *mut u32,
) -> FfiStatus {
    ffi_guard(|| {
        if state.is_null() || out_enabled.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let Some(mode) = mode_from_abi(mode) else {
            return FfiStatus::InvalidArgument;
        };
        // SAFETY: both pointers were checked non-null and the ABI requires
        // readable state plus one writable result for the duration of this call.
        let current = unsafe { ptr::read(state) };
        let Some(policy) = policy_from_state(current) else {
            return FfiStatus::InvalidArgument;
        };
        unsafe { ptr::write(out_enabled, u32::from(policy.mode(mode))) };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_render_settings_should_adjust_contrast(
    state: *const RenderSettingsState,
    candidate: u32,
    background: u32,
    candidate_is_default_or_legacy: u32,
    background_is_default_or_legacy: u32,
    out_should_adjust: *mut u32,
) -> FfiStatus {
    ffi_guard(|| {
        if state.is_null() || out_should_adjust.is_null() {
            return FfiStatus::InvalidArgument;
        }
        let (Some(candidate_is_default_or_legacy), Some(background_is_default_or_legacy)) = (
            bool_from_abi(candidate_is_default_or_legacy),
            bool_from_abi(background_is_default_or_legacy),
        ) else {
            return FfiStatus::InvalidArgument;
        };
        // SAFETY: `state` was checked non-null and the ABI requires one
        // readable state value for the duration of this call.
        let current = unsafe { ptr::read(state) };
        let Some(policy) = policy_from_state(current) else {
            return FfiStatus::InvalidArgument;
        };
        let should_adjust = policy.should_adjust_contrast(
            candidate,
            background,
            candidate_is_default_or_legacy,
            background_is_default_or_legacy,
        );
        // SAFETY: `out_should_adjust` was checked non-null and the ABI requires
        // one writable result value for the duration of this call.
        unsafe { ptr::write(out_should_adjust, u32::from(should_adjust)) };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_render_settings_restore_programmable_defaults(
    state: *mut RenderSettingsState,
) -> FfiStatus {
    ffi_guard(|| {
        if state.is_null() {
            return FfiStatus::InvalidArgument;
        }
        // SAFETY: `state` was checked non-null and the ABI requires one
        // readable/writable state value for the duration of this call.
        let current = unsafe { ptr::read(state) };
        let Some(mut policy) = policy_from_state(current) else {
            return FfiStatus::InvalidArgument;
        };
        policy.restore_programmable_defaults();
        unsafe { ptr::write(state, state_from_policy(policy)) };
        FfiStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn terminal_parser_ffi_render_settings_toggle_blink(
    state: *mut RenderSettingsState,
) -> FfiStatus {
    ffi_guard(|| {
        if state.is_null() {
            return FfiStatus::InvalidArgument;
        }
        // SAFETY: `state` was checked non-null and the ABI requires one
        // readable/writable state value for the duration of this call.
        let current = unsafe { ptr::read(state) };
        let Some(mut policy) = policy_from_state(current) else {
            return FfiStatus::InvalidArgument;
        };
        policy.toggle_blink_rendition();
        unsafe { ptr::write(state, state_from_policy(policy)) };
        FfiStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::{
        FfiStatus, RenderSettingsState, terminal_parser_ffi_render_settings_default,
        terminal_parser_ffi_render_settings_get_mode,
        terminal_parser_ffi_render_settings_restore_programmable_defaults,
        terminal_parser_ffi_render_settings_set_mode,
        terminal_parser_ffi_render_settings_should_adjust_contrast,
        terminal_parser_ffi_render_settings_toggle_blink,
    };

    const INDEXED_DISTINGUISHABLE: u32 = 1;
    const ALWAYS_DISTINGUISHABLE: u32 = 2;
    const INTENSE_IS_BRIGHT: u32 = 4;
    const SCREEN_REVERSED: u32 = 5;
    const SYNCHRONIZED_OUTPUT: u32 = 6;

    fn default_state() -> RenderSettingsState {
        let mut state = RenderSettingsState::default();
        assert_eq!(terminal_parser_ffi_render_settings_default(&mut state), FfiStatus::Ok);
        state
    }

    fn mode(state: &RenderSettingsState, mode: u32) -> bool {
        let mut enabled = u32::MAX;
        assert_eq!(terminal_parser_ffi_render_settings_get_mode(state, mode, &mut enabled), FfiStatus::Ok);
        enabled != 0
    }

    fn should_adjust(
        state: &RenderSettingsState,
        candidate: u32,
        background: u32,
        candidate_is_default_or_legacy: u32,
        background_is_default_or_legacy: u32,
    ) -> bool {
        let mut should_adjust = u32::MAX;
        assert_eq!(
            terminal_parser_ffi_render_settings_should_adjust_contrast(
                state,
                candidate,
                background,
                candidate_is_default_or_legacy,
                background_is_default_or_legacy,
                &mut should_adjust,
            ),
            FfiStatus::Ok
        );
        should_adjust != 0
    }

    #[test]
    fn ffi_replays_default_and_independent_mode_updates() {
        let mut state = default_state();
        assert!(mode(&state, INTENSE_IS_BRIGHT));
        assert!(!mode(&state, SCREEN_REVERSED));
        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, ALWAYS_DISTINGUISHABLE, 1), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, INTENSE_IS_BRIGHT, 0), FfiStatus::Ok);
        assert!(mode(&state, ALWAYS_DISTINGUISHABLE));
        assert!(!mode(&state, INTENSE_IS_BRIGHT));
    }

    #[test]
    fn ffi_replays_contrast_adjustment_policy() {
        let mut state = default_state();
        assert!(!should_adjust(&state, 0x0011_2233, 0x0044_5566, 1, 1));

        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, INDEXED_DISTINGUISHABLE, 1), FfiStatus::Ok);
        assert!(should_adjust(&state, 0x0011_2233, 0x0044_5566, 1, 1));
        assert!(!should_adjust(&state, 0x0011_2233, 0x0044_5566, 0, 1));
        assert!(!should_adjust(&state, 0x0044_5566, 0x0044_5566, 1, 1));

        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, ALWAYS_DISTINGUISHABLE, 1), FfiStatus::Ok);
        assert!(should_adjust(&state, 0x0011_2233, 0x0044_5566, 0, 0));
        assert!(!should_adjust(&state, 0x0044_5566, 0x0044_5566, 0, 0));
    }

    #[test]
    fn ffi_replays_programmable_reset_and_blink() {
        let mut state = default_state();
        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, ALWAYS_DISTINGUISHABLE, 1), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, SCREEN_REVERSED, 1), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, SYNCHRONIZED_OUTPUT, 1), FfiStatus::Ok);
        assert_eq!(terminal_parser_ffi_render_settings_toggle_blink(&mut state), FfiStatus::Ok);
        assert_eq!(state.blink_should_be_faint, 1);
        assert_eq!(terminal_parser_ffi_render_settings_restore_programmable_defaults(&mut state), FfiStatus::Ok);
        assert!(mode(&state, ALWAYS_DISTINGUISHABLE));
        assert!(!mode(&state, SCREEN_REVERSED));
        assert!(!mode(&state, SYNCHRONIZED_OUTPUT));
        assert_eq!(state.blink_should_be_faint, 1);
    }

    #[test]
    fn ffi_rejects_invalid_modes_state_and_pointers() {
        let mut state = default_state();
        let mut should_adjust = 0;
        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, 0, 1), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, 7, 1), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_render_settings_set_mode(&mut state, SCREEN_REVERSED, 2), FfiStatus::InvalidArgument);
        assert_eq!(
            terminal_parser_ffi_render_settings_should_adjust_contrast(
                &state,
                0,
                1,
                2,
                0,
                &mut should_adjust,
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_render_settings_should_adjust_contrast(
                &state,
                0,
                1,
                0,
                0,
                std::ptr::null_mut(),
            ),
            FfiStatus::InvalidArgument
        );
        assert_eq!(
            terminal_parser_ffi_render_settings_should_adjust_contrast(
                std::ptr::null(),
                0,
                1,
                0,
                0,
                &mut should_adjust,
            ),
            FfiStatus::InvalidArgument
        );
        state.modes = 1 << 31;
        assert_eq!(terminal_parser_ffi_render_settings_toggle_blink(&mut state), FfiStatus::InvalidArgument);
        assert_eq!(terminal_parser_ffi_render_settings_default(std::ptr::null_mut()), FfiStatus::InvalidArgument);
    }
}
