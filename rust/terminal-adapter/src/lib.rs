//! Safe Rust migration track for Windows Terminal's adapter layer.
//!
//! R03 starts with protocol-heavy components that can be isolated from the C++
//! `TextBuffer`, renderer, and platform integration surfaces.

#![forbid(unsafe_code)]

pub mod macro_buffer;
pub mod sixel;
