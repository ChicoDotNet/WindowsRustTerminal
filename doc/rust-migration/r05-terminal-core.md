# R05 — TerminalCore

R05 begins from the fully validated R04 checkpoint on `rust/main` and migrates the deterministic state and behavior currently owned by `src/cascadia/TerminalCore`.

## R05a — core key-state foundation

The first slice introduces a safe `terminal-core` workspace crate and ports `ControlKeyStates` without depending on Win32 headers, WinRT, COM, XAML, C++, or FFI.

The Rust representation deliberately preserves the numeric `KEY_EVENT_RECORD` flag values used by the NT console subsystem, plus Windows Terminal's two Windows-key extension bits. It also preserves the C++ queries for Shift, Alt, Ctrl, Windows, AltGr, and generic modifier state, including unknown-bit round-tripping.

## R05b — deterministic selection state

The second slice ports the platform-neutral selection state machine from `TerminalSelection.cpp`.

It adds row-major `BufferPoint` ordering compatible with the `til::point` comparisons used by TerminalCore, `SelectionInfo`, selection expansion and interaction enums, the exact `_PivotSelection` behavior (including equality targeting the start side), block-selection state, clearing, and Mark Mode endpoint switching/pivot updates. Unit tests cover forward/backward pivot crossing, pivot equality, inactive selection behavior, and each endpoint transition.

## R05c — buffer-backed selection expansion

The third slice connects `terminal-core` directly to the safe R04 `terminal-buffer` crate and ports the deterministic expansion path from `TerminalSelection.cpp`:

- `Line` expansion climbs to the complete forced-wrap chain and uses left/right-exclusive line anchors.
- `Word` expansion uses the R04 row delimiter classes and can cross row boundaries only when the preceding row is force-wrapped.
- Forward word expansion preserves the GH#5099 one-cell backoff before calculating the word end.
- Shift+Click overrides the active expansion mode for the operation, expands only the moving endpoint, and restores the opposite endpoint to the immutable pivot.
- Forward character Shift+Click advances one cell before pivoting so the clicked cell is included.
- Multi-click selection restores its pivot to the expanded selection start for future Shift+Click operations.

Tests cover wrapped-line expansion, delimiter-aware word expansion, cross-wrap words, forward character Shift+Click, and target-side-only word expansion.

## Safety

`terminal-core` uses `#![forbid(unsafe_code)]`.

R05a–R05c add no product C++, no FFI, and no platform-specific dependency. The ordinary blocking gate is therefore workspace fmt, Clippy with `-D warnings`, Linux and Windows check/test, repository quality gates, and the TAEF harness self-test.

## Next slices

Continue upward through deterministic TerminalCore API behavior, reusing the R04 buffer, R02 input, and R03 adapter contracts. A C++ compatibility facade is deferred until a concrete boundary is required and then becomes subject to the relevant Microsoft C++ contract tests.
