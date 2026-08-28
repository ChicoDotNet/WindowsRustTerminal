//! Safe, platform-neutral foundations for Windows Terminal text buffers.
//!
//! R04 ports the deterministic storage and geometry semantics beneath the C++
//! `TextBuffer` before any C++ facade or FFI integration is introduced.

#![forbid(unsafe_code)]

pub mod alternate_buffer;
pub mod clipboard_text;
pub mod geometry;
pub mod image_slice;
pub mod line_edit;
pub mod line_rendition;
pub mod output_cell;
pub mod rect_ops;
pub mod reflow;
pub mod rle;
pub mod row;
pub mod row_writer;
pub mod search;
pub mod sixel_store;
pub mod tab_stops;
pub mod text_attribute;
pub mod text_buffer;
pub mod text_color;
pub mod uia_text_range;
pub mod viewport;
pub mod virtual_bottom;
pub mod width_detector;
