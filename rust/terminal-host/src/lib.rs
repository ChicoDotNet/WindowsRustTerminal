//! Safe host/server/ConPTY foundations for the Windows Terminal Rust migration.

#![forbid(unsafe_code)]

pub mod console_argument_parser;
pub mod console_arguments;
pub mod pty_signal;
