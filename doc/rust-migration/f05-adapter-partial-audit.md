# F05 Adapter Partial audit

This audit records the remaining Microsoft `adapter` `Partial` contracts after the DECCIR (`CursorInformationReportTests`) Exact promotion, the Xterm SGR/color re-audit, and the live parser-control closeout for ANSI/VT52 and C1/coding-system semantics. It is a delivery-routing artifact, not a coverage exemption: every row remains part of the global R08 functional-debt gate until it becomes Exact/Stronger or is independently evidence-classified as a permitted non-functional boundary.

The modified sandwich order is:

1. F05 Adapter / Dispatch / VT responses
2. SettingsModel
3. TIL / Types / Foundation
4. Host / Console aggregate
5. TextBuffer / attributes / colors
6. Renderer / policy
7. TerminalCore
8. TerminalApp

## Current Adapter Partial inventory

| Microsoft contract | H11 class | Proposed delivery owner | F05 closeout? | Evidence/rationale |
| --- | --- | --- | --- | --- |
| `inputTest.cpp::TerminalInputTests` | platform-boundary | native keyboard boundary | no | Remaining branches are `MapVirtualKeyW`/default-character translation and non-KEY `INPUT_RECORD` dispatch. |
| `inputTest.cpp::TerminalInputNullKeyTests` | platform-boundary | native keyboard boundary | no | Remaining NUL VKEY lookup is `VkKeyScanExW` under a Windows layout. |
| `inputTest.cpp::DifferentModifiersTest` | platform-boundary | native keyboard boundary | no | Remaining identity observations depend on Windows keyboard-layout translation. |
| `kittyKeyboardProtocol.cpp::KeyPressTests` | functional | pre-existing input/R02 burn-down | no | Major semantic families are present, but the complete Microsoft data-source table is not yet reproduced row-for-row. |
| `kittyKeyboardProtocol.cpp::IgnoreDeadKey` | platform-boundary | native keyboard boundary | no | Portable no-output semantic exists; `ToUnicodeEx` adapter remains platform-owned. |
| `adapterTest.cpp::ColorTableReportTests` | functional | Renderer / policy | no | Requires renderer color-table projection plus response formatting. |
| `adapterTest.cpp::Osc4ColorPaletteReportTests` | functional | Renderer / policy | no | Query routing exists; live renderer color-table lookup/formatting remains. |
| `adapterTest.cpp::XtermColorResourceReportTests` | functional | Renderer / policy | no | Resource query semantics exist; renderer alias resolution/formatting remains. |
| `adapterTest.cpp::AllowBlinkingTest` | functional | TextBuffer / attributes / colors | no | Requires concrete cursor blinking mutation on the text-buffer/product state. |
| `adapterTest.cpp::LineFeedTest` | functional | Host / Console aggregate + TextBuffer | no | Typed actions exist; buffer movement and host LineFeed-mode coupling remain. |
| `adapterTest.cpp::SetConsoleTitleTest` | functional | Host / Console aggregate | no | Payload preservation exists; product/window-title side effect remains. |
| `adapterTest.cpp::SetColorTableValue` | functional | Renderer / policy | no | Action/index domain exists; live renderer palette mutation remains. |
| `adapterTest.cpp::SoftFontSizeDetection` | functional | TIL / Types / Foundation + Renderer | no | Requires DRCS/FontBuffer cell-size inference and bitmap sizing semantics. |
| `adapterTest.cpp::MacroInvokes` | functional | F05 Adapter / parser recursion | **yes** | Macro payload/depth semantics exist; CSI invocation still needs recursive execution through the live parser/product path. |
| `adapterTest.cpp::MenuCompletionsTests` | functional | TerminalApp | no | Payload is lossless; completion parsing and UI/menu dispatch are external product behavior. |
| `adapterTest.cpp::SendC1ControlTest` | functional | Renderer / policy | no | S7C1T/S8C1T and TerminalInput side effects are owned; remaining assertions cross color-report serialization paths. |

## Xterm SGR/color re-audit result

The three historical color Partials were stale after F04 introduced the live Rust `TextAttribute` owner. Existing Microsoft-derived parser-to-product witnesses exercise the full source vectors, including 256-color foreground/background indices, omitted/default RGB/indexed parameters, colon subparameters, color-space rejection, and out-of-range rejection. The following contracts are Exact in the historical ledger:

- `Xterm256ColorTest`
- `XtermExtendedColorDefaultParameterTest`
- `XtermExtendedSubParameterColorTest`

No downstream Renderer owner was required for these contracts because the Microsoft assertions terminate at the active `TextAttribute`; palette mutation/reporting contracts remain separately visible under Renderer/policy.

## Parser-control closeout result

`AnsiModeTest` and `TogglingC1ParserMode` are now Exact. The Adapter parser-control seam mutates the canonical Rust `StateMachine` modes directly instead of maintaining a reporting copy:

- ANSI/VT52 mode follows Microsoft's `false -> true -> false` `SetAnsiMode` observations.
- `AcceptC1Controls` toggles the live `AcceptC1` parser mode.
- ISO-2022 selects ISO-8859-1 / code page 28591 and enables C1 parsing.
- UTF-8 selects code page 65001 and disables C1 parsing.

The code-page result is returned to the native boundary rather than embedding Windows API ownership into the safe Rust semantic crate.

## F05 closeout interpretation

Adapter now has 16 Partial contracts. Four are already H11 `platform-boundary` exceptions, leaving 12 functional Adapter Partials. Only one is direct F05 response/parser integration debt:

- `MacroInvokes`

`KeyPressTests` is older input/R02 debt and remains globally blocking at R08 exit, but it is not response-engine F05 work. Every other functional Adapter Partial has an explicit downstream owner in the sandwich. Closing F05 therefore does **not** hide those rows or remove them from H11; it requires `MacroInvokes` to close while every downstream row remains visible in the global debt ledger.

## Next F05 sequence

1. Close `MacroInvokes` recursive parser/product execution.
2. Run the complete Rust CI gates.
3. If green, declare F05 delivery-complete and move immediately to SettingsModel while downstream Adapter rows stay visible in the global debt ledger.
