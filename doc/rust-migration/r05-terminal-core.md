# R05 — TerminalCore

R05 begins from the fully validated R04 checkpoint on `rust/main` and migrates the deterministic state and behavior currently owned by `src/cascadia/TerminalCore`.

## R05a — core key-state foundation

The first slice introduces a safe `terminal-core` workspace crate and ports `ControlKeyStates` without depending on Win32 headers, WinRT, COM, XAML, C++, or FFI.

The Rust representation deliberately preserves the numeric `KEY_EVENT_RECORD` flag values used by the NT console subsystem, plus Windows Terminal's two Windows-key extension bits. It also preserves the C++ queries for Shift, Alt, Ctrl, Windows, AltGr, and generic modifier state, including unknown-bit round-tripping.

## Safety

`terminal-core` uses `#![forbid(unsafe_code)]`.

R05a adds no product C++, no FFI, and no platform-specific dependency. The ordinary blocking gate is therefore workspace fmt, Clippy with `-D warnings`, Linux and Windows check/test, repository quality gates, and the TAEF harness self-test.

## Next slices

Subsequent R05 slices will move upward through deterministic TerminalCore state and selection/API behavior, reusing the R04 `terminal-buffer`, R02 input, and R03 adapter contracts. A C++ compatibility facade is deferred until a concrete boundary is required and then becomes subject to the relevant Microsoft C++ contract tests.
