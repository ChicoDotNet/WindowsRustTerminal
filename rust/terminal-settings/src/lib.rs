//! Safe Rust owners for deterministic Windows Terminal settings semantics.
//!
//! The crate deliberately excludes XAML/WinRT projection. R08 moves portable
//! `SettingsModel` behavior here while the existing managed/native UI surfaces
//! remain responsible for presentation and ABI boundaries.

#![forbid(unsafe_code)]

pub mod action_map;
pub mod application_state;
pub mod cascadia_settings;
pub mod color_scheme;
pub mod command_model;
pub mod keybindings;
pub mod keybindings_model;
pub mod new_tab_menu;
pub mod profile;
pub mod profile_collection;
pub mod profile_duplication;
pub mod profile_identity;
pub mod serialization;
pub mod settings_fixup;
pub mod settings_json;
pub mod theme;
