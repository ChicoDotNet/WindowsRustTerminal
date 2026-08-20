//! Rust implementation track for Windows Terminal's VT parser.
//!
//! R01 ports Base64 and the VT state machine behind compatibility and
//! differential tests before introducing the C ABI boundary.

#![forbid(unsafe_code)]

pub mod base64;
pub mod state_machine;
