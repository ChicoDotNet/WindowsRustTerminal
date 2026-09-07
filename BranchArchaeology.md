# Dustin Howett branch archaeology

`dev/duhowett/main` is the permanent consolidation lane. Historical branches are evidence, not archives: preserve useful contracts/intent, prefer definitive upstream implementations, and delete superseded refs only after provenance is recorded here.

## Method

- Order by the earliest functional branch work, not by branch name, issue number, or a tip date contaminated by merges/mechanical migrations.
- Classify intent as COPY / TRANSFORM / ADAPT / NO-PORT / ALREADY ABSORBED.
- Prefer a merged upstream implementation or current-tree contract over a historical prototype.
- Never delete `dev/duhowett/main`.
- Recent/live experimental branches are retained until their lineage is analyzed; e.g. `dev/duhowett/hax/cmake` is active 2026 work and is not part of early archaeology.

## Cohort 001 — 2019–2020 prototypes

### `dev/duhowett/version_hack`

- Earliest functional commit: `00db8704b6a2246b068222c426702eb9452e7cf1` (2019-04-23), `HACK: Add Version to the propsheet`.
- Intent: bolt an external-build-only Version page onto the legacy conhost property sheet, manually read Win32 version resources, and hard-code external version-resource data.
- The implementation was explicitly a HACK and couples version display to the legacy properties surface.
- The current product has a dedicated `AboutDialog` that exposes the application display name and application version; TerminalPage documents About as the supported version/about surface.
- Classification: **NO-PORT / superseded UX**. The useful intent (make the running product version discoverable) exists in the maintained About experience; the legacy property-sheet hack should not be revived.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/hax/wpf_win_8_hax`

- Functional commit: `3a81694d395c9243cf9785f274cea2ff79402818` (2020-04-23), `_this is a complete hack, add a netcore app to the sln, win 8, etc.`.
- Intent: prove a .NET Core WPF host/test application around the reusable terminal control and wire a special solution platform to build the native dependencies.
- Definitive upstream successor: PR #6441 / commit `48b99faed1f27781b3200ee2537e7eed1d33257e` (2020-06-09), `wpf: add a .NET Core WPF Test project for the WPF Control`.
- That project survives in current `main` as `src/cascadia/WpfTerminalTestNetCore` and remains part of the solution/build graph; later WPF work continues to use it for validation.
- Classification: **ALREADY ABSORBED**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/hax/caption_buttons`

- Functional commit: `c1dfd660bd8f9bfc4bffa060e3e5f785b4e7aee7` (2020-10-13), `ugh, do some caption button garb`.
- Intent: prototype caption-button theming/glyph refresh behavior using BitmapIcon resources, resource-qualifier refresh, and custom XAML button states.
- The titlebar/caption-button architecture moved on substantially: PR #11680 introduced the maintained Win32/XAML interaction required for snap layouts, and PR #13341 / commit `67f6b29d63d5a2f2c6a2694b8760a3994fce718b` replaced caption paths with the Windows 10/11 font glyph model.
- The old BitmapIcon/resource-refresh prototype is therefore not the authoritative contract or implementation.
- Classification: **NO-PORT / superseded by maintained caption-button architecture**.
- Disposition: **SAFE TO DELETE**.

## Cohort 001 result

Safe refs:

- `dev/duhowett/version_hack`
- `dev/duhowett/hax/wpf_win_8_hax`
- `dev/duhowett/hax/caption_buttons`

## Cohort 002 — 2020–2023 superseded experiments

### `dev/duhowett/hax/attr_smuggling`

- Functional commit: `d34218217c2f253b5022c2823c62176f21d0fe7b` (2020-04-06), `HAX: What if I smuggle fg/bg defaultness in the legacy attributes?`.
- Intent: preserve foreground/background "defaultness" through legacy `WORD` attributes by repurposing otherwise-unused legacy meta bits, then recover `TextColor::IsDefault` on the other side.
- Definitive upstream successor: PR #6698 / commit `f0df154ba97ad54f26412b70cc7dbac1a9c0cb49` (2020-07-01) formally maps the configured legacy default indices to `TextColor::IsDefault` and adds roundtrip tests. PR #6506 / commit `ddbe370d222a8b9f363eedc36d2d4e279613ebfd` then preserves original color types through the VT/ConPTY renderer instead of reverse-engineering them from legacy/RGB values.
- The upstream design solves the same contract without smuggling private state through legacy attribute bits.
- Classification: **NO-PORT / superseded by formal default-color and VT propagation architecture**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/hax/command_palette_search`

- Functional commit: `de5165b85708c3490fbd3b7c93215635e1c9dda5` (2020-08-06), `wow I still hate this`; later tip commits are spelling migrations.
- Intent: prototype word-prefix matching plus highlighted match ranges for command-palette entries.
- Definitive upstream successor: PR #7977 / commit `1aff3bc216c7f98917314f5e0e4707664292f995` (2020-11-05), `Bold matching text in the command palette`, introduced the maintained `FilteredCommand` view model, highlighted presentation, `HighlightedTextControl`, and matching tests.
- Current code has evolved further to FZF-backed matching/scoring in `FilteredCommand`; the historical custom recursive/prefix matcher is no longer the behavioral authority.
- Classification: **NO-PORT / superseded by #7977 and later FZF filtering architecture**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/hax/build-with-wholearchive`

- Functional commit: `33646d9af1a68fd762ad2a7d3b65d2e19b7dbfba` (2021-03-18), `Make TerminalControl consume the entire TerminalControlLib`.
- Intent: diagnose/fix release builds where `Microsoft.Terminal.Control.dll` became effectively empty by forcing a whole static library into the final DLL; the commit explicitly targets #9529.
- Root cause and definitive fix: PR #9537, `Fix build break where Microsoft.Terminal.Control.dll is empty`, identified that the project rename left the `.def` filename mismatched. Without the module-definition exports, Release linking pruned the WinRT activation symbols and everything reachable from them. Renaming/fixing the `.def` closed #9529.
- `/WHOLEARCHIVE` was therefore a diagnostic/workaround path, not the correct retained solution.
- Classification: **NO-PORT / superseded by root-cause fix #9537**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/shorthand-namespaces`

- Functional work: `c72c0f5f89e9ab1da6c8b0e0c6627cc87bbf78f2` through `fafe65d6c1d57702eb24414e50a93a31cf4450e9` (2023-02-04).
- Intent: mechanically replace long WinRT namespace qualifications with convenience aliases such as `MTSM`, including enums and broad regex-driven rewrites.
- This is a style/refactor experiment, not a product behavior contract. The branch itself records fallout (`Regex too big, broke AppCommandlineArgs`), and the current tree does not adopt the proposed `MTSM` namespace convention globally.
- There is no unique functionality, test contract, data migration, or compatibility behavior to recover.
- Classification: **NO-PORT / mechanical style experiment not adopted**.
- Disposition: **SAFE TO DELETE**.

## Cohort 002 result

Safe refs:

- `dev/duhowett/hax/attr_smuggling`
- `dev/duhowett/hax/command_palette_search`
- `dev/duhowett/hax/build-with-wholearchive`
- `dev/duhowett/shorthand-namespaces`

Retained for later analysis:

- `dev/duhowett/conpty-flags` — old but mixes several ConPTY behavior flags; no definitive successor established yet.
- `dev/duhowett/hax-selection-exclusive` — incomplete selection experiment; selection lineage still needs reconstruction.
- Recent/live branches such as `dev/duhowett/hax/cmake`, `dev/duhowett/hax/unix-pty`, and `dev/duhowett/hax/our-own-tabview` remain out of early-cleanup cohorts.
