//! Rust implementation track for Windows Terminal's VT parser.
//!
//! R01 ports Base64, the VT state machine, and the input/output dispatch layers
//! behind compatibility and differential tests before introducing the C ABI
//! boundary.

#![forbid(unsafe_code)]

pub mod base64;
pub mod input_engine;
pub mod output_engine;
pub mod state_machine;
