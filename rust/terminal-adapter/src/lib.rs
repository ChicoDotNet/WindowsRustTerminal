//! Safe Rust migration track for Windows Terminal's adapter layer.
//!
//! R03 isolates protocol-heavy Adapter components from the C++ `TextBuffer`,
//! renderer, and platform surfaces. `PageManager` completes the deterministic
//! VT paging control plane while concrete page storage remains an R04 concern.

#![forbid(unsafe_code)]

pub mod adapt_dispatch;
pub mod dcs_dispatch;
pub mod decrqss;
pub mod decrqss_color_alias;
pub mod decrqss_cursor;
pub mod macro_buffer;
pub mod macro_reports;
pub mod page_manager;
pub mod page_storage;
pub mod presentation_state;
pub mod response_dispatch;
pub mod sixel;
pub mod vt_response;
