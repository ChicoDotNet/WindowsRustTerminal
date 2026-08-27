//! Safe Rust owners for deterministic Windows Terminal settings semantics.
//!
//! The crate deliberately excludes XAML/WinRT projection. R08 moves portable
//! `SettingsModel` behavior here while the existing managed/native UI surfaces
//! remain responsible for presentation and ABI boundaries.

#![forbid(unsafe_code)]

pub mod application_state;
