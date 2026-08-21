# R05 — TerminalCore

R05 begins from the fully validated R04 checkpoint on `rust/main` and migrates the deterministic state and behavior currently owned by `src/cascadia/TerminalCore`.

## R05a — core key-state foundation

The first slice introduces a safe `terminal-core` workspace crate and ports `ControlKeyStates` without depending on Win32 headers, WinRT, COM, XAML, C++, or FFI.

The Rust representation deliberately preserves the numeric `KEY_EVENT_RECORD` flag values used by the NT console subsystem, plus Windows Terminal's two Windows-key extension bits. It also preserves the C++ queries for Shift, Alt, Ctrl, Windows, AltGr, and generic modifier state, including unknown-bit round-tripping.

## R05b — deterministic selection state

The second slice ports the platform-neutral selection state machine from `TerminalSelection.cpp` without pulling text-buffer-dependent word or line expansion across the boundary yet.

It adds row-major `BufferPoint` ordering compatible with the `til::point` comparisons used by TerminalCore, `SelectionInfo`, selection expansion and interaction enums, the exact `_PivotSelection` behavior (including equality targeting the start side), block-selection state, clearing, and Mark Mode endpoint switching/pivot updates. Unit tests cover forward/backward pivot crossing, pivot equality, inactive selection behavior, and each endpoint transition.

## Safety

`terminal-core` uses `#![forbid(unsafe_code)]`.

R05a/R05b add no product C++, no FFI, and no platform-specific dependency. The ordinary blocking gate is therefore workspace fmt, Clippy with `-D warnings`, Linux and Windows check/test, repository quality gates, and the TAEF harness self-test.

## Next slices

Continue upward through deterministic TerminalCore selection expansion/API behavior, reusing the R04 `terminal-buffer`, R02 input, and R03 adapter contracts. Text-buffer-dependent word/line expansion should reuse the migrated buffer rather than duplicate it. A C++ compatibility facade is deferred until a concrete boundary is required and then becomes subject to the relevant Microsoft C++ contract tests.
