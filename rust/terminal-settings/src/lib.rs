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
pub mod command_expansion;
pub mod command_model;
pub mod deserialization_actions;
pub mod deserialization_copy;
pub mod deserialization_fragments;
pub mod deserialization_profile_properties;
pub mod deserialization_profiles;
pub mod deserialization_validation;
pub mod elevate;
pub mod json_utils;
pub mod keybindings;
pub mod keybindings_model;
pub mod media_resource;
pub mod new_tab_menu;
pub mod profile;
pub mod profile_collection;
pub mod profile_duplication;
pub mod profile_identity;
pub mod profile_lookup;
pub mod serialization;
pub mod settings_fixup;
pub mod settings_json;
pub mod terminal_settings;
pub mod theme;
