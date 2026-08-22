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

## Safety boundary

`terminal-host` uses `#![forbid(unsafe_code)]`. R06a does not own Windows handles, create threads, call Win32, modify C++, or introduce FFI. Those boundaries remain deferred until a concrete compatibility facade is required and will then make the relevant Microsoft host/ConPTY contract tests blocking.

## Next slices

Continue upward through deterministic host/server/interactivity state and ConPTY behavior, reusing the migrated parser, input, buffer, adapter, and core crates rather than creating parallel representations.
