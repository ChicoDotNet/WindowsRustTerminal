//! Safe host/server/ConPTY foundations for the Windows Terminal Rust migration.

#![forbid(unsafe_code)]

pub mod attribute_format;
pub mod console_argument_parser;
pub mod console_arguments;
pub mod pty_clear_buffer;
pub mod pty_signal;
pub mod pty_signal_session;
pub mod pty_signal_state;
pub mod pty_signal_stream;
pub mod vt_char_info;
pub mod vt_io_protocol;
pub mod vt_io_state;
pub mod vt_screen_dump;
pub mod vt_writer_sequences;
