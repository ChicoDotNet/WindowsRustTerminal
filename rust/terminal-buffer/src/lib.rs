//! Safe, platform-neutral foundations for Windows Terminal text buffers.
//!
//! R04 ports the deterministic storage and geometry semantics beneath the C++
//! `TextBuffer` before any C++ facade or FFI integration is introduced.

#![forbid(unsafe_code)]

pub mod geometry;
pub mod line_rendition;
pub mod rle;
pub mod row;
pub mod text_attribute;
pub mod text_buffer;
pub mod text_color;
