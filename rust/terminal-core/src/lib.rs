//! Safe, platform-neutral state for Windows Terminal `TerminalCore`.
//!
//! R05 migrates deterministic core state before any C++ compatibility facade
//! or WinRT/COM boundary is introduced.

#![forbid(unsafe_code)]

pub mod control_key_states;
pub mod keyboard_selection;
pub mod selection;
