//! Narrow compatibility boundary between the existing C++ code and Rust.
//!
//! R00 intentionally exposes no ABI. Future unsafe/FFI code belongs in this
//! crate rather than in the safe `terminal-parser` implementation.

#![deny(unsafe_op_in_unsafe_fn)]
