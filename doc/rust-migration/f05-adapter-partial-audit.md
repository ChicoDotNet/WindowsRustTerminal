# F05 Adapter Partial audit

This audit records the remaining Microsoft `adapter` `Partial` contracts after the DECCIR (`CursorInformationReportTests`) Exact promotion. It is a delivery-routing artifact, not a coverage exemption: every row remains part of the global R08 functional-debt gate until it becomes Exact/Stronger or is independently evidence-classified as a permitted non-functional boundary.

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
| `adapterTest.cpp::AnsiModeTest` | functional | F05 Adapter / parser coupling | **yes** | ANSI/VT52 grammar exists; dispatch-driven mutation of the live state-machine mode remains. |
| `adapterTest.cpp::AllowBlinkingTest` | functional | TextBuffer / attributes / colors | no | Requires concrete cursor blinking mutation on the text-buffer/product state. |
| `adapterTest.cpp::LineFeedTest` | functional | Host / Console aggregate + TextBuffer | no | Typed actions exist; buffer movement and host LineFeed-mode coupling remain. |
| `adapterTest.cpp::SetConsoleTitleTest` | functional | Host / Console aggregate | no | Payload preservation exists; product/window-title side effect remains. |
| `adapterTest.cpp::Xterm256ColorTest` | functional | TextBuffer / attributes / colors | no, but re-audit early | Historical note says `TextAttribute` mutation was deferred; F04 added a live presentation owner, so this is a candidate for an early evidence-only promotion if the Microsoft vectors now match exactly. |
| `adapterTest.cpp::XtermExtendedColorDefaultParameterTest` | functional | TextBuffer / attributes / colors | no, but re-audit early | Current SGR owner may already cover part of the omitted/default parameter behavior; exact rejection/default vectors need comparison before promotion. |
| `adapterTest.cpp::XtermExtendedSubParameterColorTest` | functional | TextBuffer / attributes / colors | no, but re-audit early | Colon subparameter parsing and indexed/RGB application now exist; compare the full Microsoft matrix before changing coverage. |
| `adapterTest.cpp::SetColorTableValue` | functional | Renderer / policy | no | Action/index domain exists; live renderer palette mutation remains. |
| `adapterTest.cpp::SoftFontSizeDetection` | functional | TIL / Types / Foundation + Renderer | no | Requires DRCS/FontBuffer cell-size inference and bitmap sizing semantics. |
| `adapterTest.cpp::TogglingC1ParserMode` | functional | F05 Adapter / parser coupling | **yes** | Parser semantics exist; Adapter-driven parser/code-page coupling remains. |
| `adapterTest.cpp::MacroInvokes` | functional | F05 Adapter / parser recursion | **yes** | Macro payload/depth semantics exist; CSI invocation still needs recursive execution through the live parser/product path. |
| `adapterTest.cpp::MenuCompletionsTests` | functional | TerminalApp | no | Payload is lossless; completion parsing and UI/menu dispatch are external product behavior. |
| `adapterTest.cpp::SendC1ControlTest` | functional | Renderer / policy | no | S7C1T/S8C1T and TerminalInput side effects are owned; remaining assertions cross color-report serialization paths. |

## F05 closeout interpretation

After DECCIR promotion, Adapter has 21 Partial contracts. Four are already H11 `platform-boundary` exceptions, leaving 17 functional Adapter Partials. Only three are presently classified as direct F05 response/parser integration debt:

- `AnsiModeTest`
- `TogglingC1ParserMode`
- `MacroInvokes`

`KeyPressTests` is older input/R02 debt and remains globally blocking at R08 exit, but it is not response-engine F05 work. The remaining functional Adapter contracts have an explicit downstream owner in the sandwich. Closing F05 therefore does **not** mean hiding those rows or removing them from H11; it means the three F05-specific rows are closed and every other Adapter Partial has a named later owner.

Before leaving F05, re-audit the three Xterm SGR/color contracts against the F04 live `TextAttribute` owner. If they are already exact, promote them rather than carrying stale metadata into the later TextBuffer slice.

## Next F05 sequence

1. Re-audit `Xterm256ColorTest`, `XtermExtendedColorDefaultParameterTest`, and `XtermExtendedSubParameterColorTest` against current Rust behavior.
2. Close `AnsiModeTest` live state-machine mutation.
3. Close `TogglingC1ParserMode` parser/code-page coupling.
4. Close `MacroInvokes` recursive parser/product execution.
5. Run the complete Rust CI gates and, if green, declare F05 delivery-complete and move to SettingsModel while the downstream rows stay visible in the global debt ledger.
