# R09 product ownership census

R09 changes the migration question from behavioral parity to product ownership. The canonical product remains the existing `CascadiaPackage` / `WindowsTerminal.vcxproj` graph. C++/WinRT, XAML, Win32, COM, DirectX/DWrite/GDI and packaging surfaces remain native where they are the legitimate platform or UI owner. Portable behavior already certified in Rust should no longer have a second C++ implementation compiled into the product.

The classifications used here are:

- **DELETE** — Rust owns the behavior and the C++ implementation is redundant.
- **KEEP-NATIVE/UI** — the surface legitimately owns Windows/platform/UI behavior.
- **SPLIT** — the file currently mixes portable behavior with a required native seam; portable ownership should move behind the Rust boundary while the seam remains native.
- **UNRESOLVED** — more build/ownership evidence is required before changing the surface.

## Certified product owners

| Surface / behavior | Current classification | Evidence / next action |
|---|---|---|
| Base64 decoding | **DELETE C++ / Rust owner** | `src/terminal/parser/base64.cpp` is deleted. `base64.hpp` remains a narrow ABI-compatible facade over `terminal_parser_ffi_base64_decode_utf16`. |
| Input CSI cursor key mapping | **DELETE C++ / Rust owner** | Product routing uses `terminal_parser_ffi_input_cursor_vkey`; the legacy C++ map/table is absent and guarded by `Test-R09ParserOwnership.ps1`. |
| Input CSI generic key mapping | **DELETE C++ / Rust owner** | Product routing uses `terminal_parser_ffi_input_generic_vkey`; the legacy C++ map/table is absent and guarded. |
| Input SS3 key mapping | **DELETE C++ / Rust owner** | Product routing uses `terminal_parser_ffi_input_ss3_vkey`; the legacy C++ map/table is absent and guarded. |
| VT modifier normalization / enhanced-key composition | **DELETE C++ / Rust owner** | Cursor, generic and SGR modifier translation route through `terminal-parser-ffi`; duplicate C++ bit-composition patterns are prohibited by the ownership gate. |
| Win32 key-field normalization | **DELETE portable C++ / Rust owner** | `_GenerateWin32Key` remains as the native `INPUT_RECORD` adapter, while deterministic parameter/default/saturation policy routes through `terminal_parser_ffi_input_win32_key_fields`. |
| Control-character classification | **DELETE portable C++ / Rust owner** | `_DoControlCharacter` routes Ctrl+C/C0/DEL/print classification through `terminal_parser_ffi_input_control_character_plan`; C++ retains only keyboard-layout and Win32 event materialization. |
| Command-palette FZF matching | **DELETE portable C++ / Rust owner** | `CommandPalette`/`FilteredCommand` still expose the product/UI shape, while `fzf.cpp` is a narrow adapter over `terminal-app-ffi`; score and UTF-16 runs come from Rust. `Test-R09FzfOwnership.ps1` guards the real consumer route. |

## Remaining parser product surfaces

| Surface | Current classification | Evidence / next action |
|---|---|---|
| `rust/terminal-parser-ffi` | **KEEP-ABI** | Narrow C ABI/static-library boundary. Raw pointer handling stays here; product semantics stay in safe Rust crates. |
| `src/terminal/parser/InputStateMachineEngine.cpp` | **SPLIT** | SGR mouse deterministic policy now has a Rust owner plus C ABI, native replay and ownership sensor certified by R09 Product Build #83. The remaining promotion step is to route `_UpdateSGRMouseButtonState` through `terminal_parser_ffi_input_sgr_mouse_plan`, remove its duplicate portable decision tree, and retain only native double-click timing/position plus Windows event materialization. |
| `src/terminal/parser/OutputStateMachineEngine.cpp` | **SPLIT** | Deterministic parser/dispatch policy is represented in Rust; existing C++ surface still participates in native dispatch/product integration. Promote narrow behaviors before attempting translation-unit removal. |
| `src/terminal/parser/stateMachine.cpp` | **SPLIT** | Rust owns portable state-machine semantics, but C++ consumers still depend on the native class/API shape. Replace ownership through product seams before removing the implementation. |
| `src/terminal/parser/tracing.cpp` | **KEEP-NATIVE** | Telemetry/tracing is a platform integration concern, not a reason to duplicate parser semantics in Rust. |
| `src/terminal/parser/precomp.cpp` / `precomp.h` | **KEEP-NATIVE** | Native build infrastructure while C++ parser seams remain. |

## Mechanical promotion rule

A surface moves from `SPLIT` to `DELETE` only after all of the following are true:

1. the existing product consumer routes the relevant behavior through the Rust owner;
2. Rust and Microsoft contract tests remain green;
3. the canonical `CascadiaPackage` product build remains green;
4. the redundant C++ implementation is removed from the product path or reduced to a native/ABI adapter;
5. a repository ownership gate prevents the obsolete implementation from being reintroduced.

`tools/rust/Test-R09ParserOwnership.ps1` records the parser promotions already completed: Base64, CSI/generic/SS3 key maps, modifier translation and enhanced-key composition, Win32 key normalization, and control-character classification. It also verifies that the product C++ consumer still routes through those Rust owners and that known legacy implementations remain absent. The SGR mouse seam is already guarded at the Rust/ABI/replay boundary; after the product swap, the same gate must also reject reintroduction of the removed C++ decision tree.

## Next parser candidate

The next low-blast-radius parser target remains the **deterministic portion of SGR mouse button/state decoding** in `InputStateMachineEngine::_UpdateSGRMouseButtonState`, but the VERIFY phase is complete: press/release/drag/wheel and multi-button state are covered through Rust Contract Replay and the native C ABI probe, and canonical `CascadiaPackage x64 Debug` passed in R09 Product Build #83 on `4be9a7340746ab8cb0a2c83f8de299fb0979f26b`.

The remaining change is ownership, not semantics. Route the existing product consumer through `terminal_parser_ffi_input_sgr_mouse_plan`; copy `button_state`, `persistent_button_state` and `event_flags` from the Rust plan; preserve C++ only for `steady_clock`, click position/history, `_doubleClickTime` and subsequent `INPUT_RECORD` emission; then prohibit the legacy button/wheel/drag decision tree in the ownership gate and recertify the canonical product.
