# R06 — Host, server, interactivity, and ConPTY

R06 migrates host-side deterministic contracts before introducing operating-system handle ownership or thread boundaries.

## R06a — ConPTY signal wire contract

The first slice introduces the safe `terminal-host` crate and ports the private signal protocol consumed by `PtySignalInputThread`.

The Rust representation preserves the C++ signal discriminators exactly:

- `ShowHideWindow = 1`
- `ClearBuffer = 2`
- `SetParent = 3`
- `ResizeWindow = 8`

Payload decoding is explicit little-endian byte parsing rather than native-structure reinterpretation. Resize dimensions remain two 16-bit values, show/hide and keep-cursor-row fields retain their raw 16-bit wire values with nonzero boolean interpretation, and parent handles retain all 64 bits. Payload sizes are exact and unknown signal values are rejected.

## R06b — client command-line escaping

The second slice ports the deterministic `EscapeArgument` behavior from `ConsoleArguments.cpp` without taking ownership of `CommandLineToArgvW` or Win32 handles. Empty arguments are explicitly quoted, space/tab-containing arguments are wrapped in quotes, quote-adjacent backslashes are doubled, and trailing backslashes inside quoted arguments are escaped so Windows tokenization reconstructs the original value.

A small `join_client_arguments` helper mirrors the host path that rebuilds the client command line after host-only switches have been consumed. Tests cover empty/simple/Unicode arguments, spaces and tabs, embedded quotes, consecutive backslashes before quotes, trailing backslashes, and multi-argument reconstruction.

## R06c — tokenized ConsoleArguments parsing

The third slice ports the deterministic portion of `ConsoleArguments::ParseCommandline` while deliberately leaving `CommandLineToArgvW` on the Windows side. It consumes already-tokenized arguments and preserves server/signal handle forms, ForceV1/ForceNoHandoff/Embedding flags, width/height, `--feature pty`, headless/inherit-cursor, text measurement, ambiguous-width state, the historical `\\??\\` path token, explicit `--`, and the fallback where the first unrecognized argument begins the client command line.

Handle parsing mirrors the existing `wcstoul` behavior used by conhost, including nonzero enforcement, duplicate-handle rejection, prefix consumption and 32-bit saturation. Dimension parsing preserves the current C++ upper-bound behavior and full-token numeric validation.

## R06d — deterministic VtIo lifecycle

The fourth slice extracts the platform-neutral lifecycle decisions from `VtIo`. It preserves the `Uninitialized`, `Initialized`, `Starting`, `StartupFailed`, and `Running` states, the no-op path outside ConPTY mode, single initialization, non-reentrant start, startup failure when a close arrives while starting, close-event deduplication after startup, and the rule that shutdown reset sequences are emitted only while running.

Handle ownership, I/O threads, renderer construction, console locking, and actual close-event delivery remain on the C++/Win32 side. The Rust type models only the deterministic transition contract so later compatibility plumbing can delegate these decisions without duplicating state logic.

## Safety boundary

`terminal-host` uses `#![forbid(unsafe_code)]`. R06a–R06d do not own Windows handles, create threads, call Win32, modify C++, or introduce FFI. `CommandLineToArgvW` remains an explicit platform boundary for a later compatibility slice.

## Next slices

Continue through deterministic host/server/interactivity state and ConPTY lifecycle decisions, reusing the migrated parser, input, buffer, adapter, and core crates rather than creating parallel representations.
