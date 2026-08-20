//! Rust implementation track for Windows Terminal's VT parser.
//!
//! R01 begins by porting Base64 and then the parser state machine behind
//! compatibility and differential tests.

#![forbid(unsafe_code)]

pub mod base64;
