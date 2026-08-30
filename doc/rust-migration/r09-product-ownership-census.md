# R09 product ownership census

R09 changes the migration question from behavioral parity to product ownership. The canonical product remains the existing `CascadiaPackage` / `WindowsTerminal.vcxproj` graph. C++/WinRT, XAML, Win32, COM, DirectX/DWrite/GDI and packaging surfaces remain native where they are the legitimate platform or UI owner. Portable behavior already certified in Rust should no longer have a second C++ implementation compiled into the product.

The classifications used here are:

- **DELETE** — Rust owns the behavior and the C++ implementation is redundant.
- **KEEP-NATIVE/UI** — the surface legitimately owns Windows/platform/UI behavior.
- **SPLIT** — the file currently mixes portable behavior with a required native seam; portable ownership should move behind the Rust boundary while the seam remains native.
- **UNRESOLVED** — more build/ownership evidence is required before changing the surface.

## Parser product slice

| Surface | Current classification | Evidence / next action |
|---|---|---|
| `src/terminal/parser/base64.cpp` | **DELETE — complete** | Deleted after `Base64::Decode` was routed through `terminal-parser-ffi`; canonical `CascadiaPackage x64 Debug` run #5 passed on commit `b68392a4`. |
| `src/terminal/parser/base64.hpp` | **KEEP-NATIVE/ABI** | Minimal compatibility seam retaining the existing C++ API while delegating decoding to `terminal_parser_ffi_base64_decode_utf16`. |
| `rust/terminal-parser/src/base64.rs` | **Rust owner** | Safe deterministic implementation certified during R08 and now used by the product graph. |
| `rust/terminal-parser-ffi` | **KEEP-ABI** | Narrow C ABI/static-library boundary. Raw pointer handling stays here; product semantics stay in safe Rust crates. |
| `src/terminal/parser/InputStateMachineEngine.cpp` | **SPLIT** | Contains substantial deterministic VT/input policy already represented by `terminal-parser`, but also Windows keyboard-layout, `INPUT_RECORD`, ConPTY/interactivity and timing integration. Migrate deterministic helpers incrementally; keep Win32/interactivity seams native. |
| `src/terminal/parser/OutputStateMachineEngine.cpp` | **SPLIT** | Deterministic parser/dispatch policy is represented in Rust; existing C++ surface still participates in native dispatch/product integration. Promote narrow behaviors before attempting translation-unit removal. |
| `src/terminal/parser/stateMachine.cpp` | **SPLIT** | Rust owns the portable state-machine semantics, but C++ consumers still depend on the native class/API shape. Replace ownership through a product seam before removing the C++ implementation. |
| `src/terminal/parser/tracing.cpp` | **KEEP-NATIVE** | Telemetry/tracing is a platform integration concern, not a reason to duplicate parser semantics in Rust. |
| `src/terminal/parser/precomp.cpp` / `precomp.h` | **KEEP-NATIVE** | Native build infrastructure while C++ parser seams remain. |

## Mechanical promotion rule

A surface moves from `SPLIT` to `DELETE` only after all of the following are true:

1. the existing product consumer routes the relevant behavior through the Rust owner;
2. Rust and Microsoft contract tests remain green;
3. the canonical `CascadiaPackage` product build remains green;
4. the C++ implementation is removed from MSBuild/source lists;
5. a repository ownership gate prevents the obsolete implementation from being reintroduced.

`tools/rust/Test-R09ParserOwnership.ps1` is the first such gate. It locks the completed Base64 promotion by verifying that `base64.cpp` remains absent, no build/source reference to it exists, the C++ compatibility header still delegates to Rust, and the parser project still builds/links `terminal-parser-ffi`.

## Next parser candidate

The next low-blast-radius target is the deterministic key-mapping policy embedded in `InputStateMachineEngine.cpp` (`CSI`, generic and `SS3` key-to-VKEY mappings). Rust already contains equivalent mappings in `terminal-parser::input_engine`. Promote these mappings through the existing parser FFI while leaving keyboard-layout lookup, scan-code generation, `INPUT_RECORD` synthesis and ConPTY dispatch in C++/Win32. This advances `InputStateMachineEngine.cpp` as a **SPLIT** without rewriting its legitimate native responsibilities.
