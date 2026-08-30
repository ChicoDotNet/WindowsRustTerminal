//! Safe, platform-neutral foundations for Windows Terminal text buffers.
//!
//! R04 ports the deterministic storage and geometry semantics beneath the C++
//! `TextBuffer` before any C++ facade or FFI integration is introduced.

#![forbid(unsafe_code)]

pub mod alternate_buffer;
pub mod clipboard_text;
pub mod color_table;
pub mod command_regions;
pub mod cursor_movement;
pub mod cursor_properties;
pub mod deferred_resize;
pub mod delayed_wrap;
pub mod extended_attributes;
pub mod geometry;
pub mod host_write;
pub mod hyperlink;
pub mod image_slice;
pub mod line_edit;
pub mod line_rendition;
pub mod output_cell;
pub mod output_cell_runs;
pub mod rect_ops;
pub mod reflow;
pub mod repeat_character;
pub mod resize_integrity;
pub mod rle;
pub mod rle_ops;
pub mod row;
pub mod row_writer;
pub mod rtf_text;
pub mod saved_cursor;
pub mod screen_alignment;
pub mod screen_erase;
pub mod search;
pub mod sgr;
pub mod sixel_store;
pub mod soft_reset;
pub mod tab_stops;
pub mod terminal_modes;
pub mod text_attribute;
pub mod text_buffer;
pub mod text_buffer_iterator;
pub mod text_buffer_queries;
pub mod text_buffer_write;
pub mod text_color;
pub mod til_color;
pub mod til_operators;
pub mod til_point;
pub mod til_rect;
pub mod til_rect_index;
pub mod til_replace;
pub mod til_string;
pub mod til_utf_convert;
pub mod uia_text_range;
pub mod url_patterns;
pub mod vertical_scroll;
pub mod viewport;
pub mod viewport_index;
pub mod virtual_bottom;
pub mod vt_resize;
pub mod width_detector;
pub mod word_boundary;

#[cfg(test)]
mod microsoft_text_buffer_tests;