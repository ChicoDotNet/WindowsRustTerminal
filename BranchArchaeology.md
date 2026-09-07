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
