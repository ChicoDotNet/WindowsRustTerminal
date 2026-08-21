//! Safe, platform-neutral foundations for Windows Terminal text buffers.
//!
//! R04 ports the deterministic storage and geometry semantics beneath the C++
//! `TextBuffer` before any C++ facade or FFI integration is introduced.

#![forbid(unsafe_code)]

pub mod geometry;
pub mod image_slice;
pub mod line_rendition;
pub mod output_cell;
pub mod rle;
pub mod row;
pub mod row_writer;
pub mod sixel_store;
pub mod text_attribute;
pub mod text_buffer;
pub mod text_color;
pub mod width_detector;