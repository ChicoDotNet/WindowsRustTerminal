//! Safe Rust migration track for Windows Terminal's adapter layer.
//!
//! R03 isolates protocol-heavy Adapter components from the C++ `TextBuffer`,
//! renderer, and platform surfaces. `PageManager` completes the deterministic
//! VT paging control plane while concrete page storage remains an R04 concern.

#![forbid(unsafe_code)]

pub mod adapt_dispatch;
pub mod dcs_dispatch;
pub mod macro_buffer;
pub mod page_manager;
pub mod sixel;
