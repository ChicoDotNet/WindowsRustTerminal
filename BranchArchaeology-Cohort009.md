# Dustin Howett branch archaeology — Cohort 009

This ledger closes the namespace-level compression pass over the 59 refs that were still visible under `dev/duhowett/**` on 2026-09-07. It extends the primary archaeology ledger plus Cohorts 006–008.

The goal of this cohort is not to preserve every historical patch. It is to preserve every unique behavior, architectural lesson, recovery contract, active review head, or contemporary implementation that still matters. Branches whose useful content can be reconstructed from this ledger or from maintained upstream history are safe to retire.

## Result

Current namespace classification at the close of the pass:

- **59 refs total**
- **37 SAFE TO DELETE**
- **2 WAIT / active upstream review**
- **20 KEEP**, including permanent `dev/duhowett/main`
- **0 code PORTs required into `dev/duhowett/main`**

Pass 2 therefore consisted only of knowledge integration. No historical implementation was sufficiently authoritative to justify transplanting code into the curated lane.

## SAFE TO DELETE — 37 refs

- `dev/duhowett/cant-believe-gotta-do-this-shit`
- `dev/duhowett/chop`
- `dev/duhowett/coast-to-coast`
- `dev/duhowett/connection-utf8`
- `dev/duhowett/dev/duhowett/hax/appstate_remember`
- `dev/duhowett/eoy-25/underline-colors-in-atlas-bug`
- `dev/duhowett/errordialog`
- `dev/duhowett/feed-forward-variables`
- `dev/duhowett/fhl-2024/clang`
- `dev/duhowett/fhl-2024/merge-idls`
- `dev/duhowett/fhl-2025/bitmap-fonts`
- `dev/duhowett/fhl-2025/what-if-no-content-ids`
- `dev/duhowett/fhl-2025/wt-command-palette-cmdpal-integration`
- `dev/duhowett/hax/clogs`
- `dev/duhowett/hax/conhost_dump_replay`
- `dev/duhowett/hax/conhost-icon`
- `dev/duhowett/hax/cursor_stamp_foreground_background`
- `dev/duhowett/hax/embed-everything`
- `dev/duhowett/hax/l9`
- `dev/duhowett/hax/merge_idl`
- `dev/duhowett/hax/sui-color-chip-border`
- `dev/duhowett/hax/terminalsettings-as-a-lib-/with-types-merged-into-tsm`
- `dev/duhowett/hax/wpf-atlas`
- `dev/duhowett/i-have-a-burning-hatred-for-ntstatus-of-later-so-why-not-fix-it`
- `dev/duhowett/interface-projects`
- `dev/duhowett/is-pgo-broken-because-of-sui-being-slower`
- `dev/duhowett/learn/rewrite-highlights`
- `dev/duhowett/move-timers-down-into-core-interactivity-etc`
- `dev/duhowett/multi-blern`
- `dev/duhowett/osc-strided-table`
- `dev/duhowett/padding-in-atlas`
- `dev/duhowett/portable-shader-members`
- `dev/duhowett/sel-2-spans`
- `dev/duhowett/server-2025-vms`
- `dev/duhowett/sticky-control`
- `dev/duhowett/testing`
- `dev/duhowett/wellp2`

## WAIT — active upstream review

### `dev/duhowett/fhl-2026/remove-paste-hairpin-handler`

Exact upstream head for microsoft/terminal #20155, **“TermControl: Remove the ‘hairpin’ paste handler, consolidate string writers.”**

The reviewed direction is to collapse confusing `SendInput` / `PasteText` paths into `WriteInputString`, make paste request/response one-way instead of callback hairpins, and localize bracketed-paste encoding behavior to the control. The PR remains open and is authoritative while review continues.

**Disposition: WAIT / DO NOT DELETE.**

### `dev/duhowett/fhl-2026/rewrite-paste-and-dragdrop-handling-writeinputstring`

Exact upstream head for microsoft/terminal #20165, **“[WIP] Rewrite paste and drag/drop broadcasting.”** It explicitly targets #20155 and is stacked on that work.

**Disposition: WAIT / DO NOT DELETE.**

## KEEP — 20 refs

- `dev/duhowett/main` — permanent consolidation lane.
- `dev/duhowett/asan-for-all` — June 2026 all-project ASan/Fuzzing configuration experiment; still contemporary and unresolved enough to retain implementation.
- `dev/duhowett/atlas-draw-d2d-dots-curlies-consistently` — February 2026 renderer work making dotted/dashed underlines continuous across color changes and reusing curly geometry; no reviewed successor found.
- `dev/duhowett/eoy-25/allow-set-foreground` — January 2026 foreground-activation propagation through shim, emperor, window activation and ConPTY handoff; unique implementation, no authoritative successor found.
- `dev/duhowett/even-more-builtin-glyphs` — July 2026 follow-up work after the reviewed built-in-glyph line, not merely a precursor to it.
- `dev/duhowett/fhl-2024/asciicast-recorder` — late-2025 Start/Stop/Mark Recording capability with no reviewed successor.
- `dev/duhowett/fhl-2025/wt-json-relative-icons` — September 2025 product capability allowing `.wt.json` media paths to resolve relative to the settings resolver; no reviewed successor found.
- `dev/duhowett/font-64` — retained by the still-live font contract already documented in the primary ledger / upstream #3123.
- `dev/duhowett/fzf-vcxproj` — May 2026 projectization of fzf plus dependent project wiring.
- `dev/duhowett/hax/cmake` — reworked February 2026 CMake/vcpkg/HybridCRT/resource-dependency experiment; broad and contemporary.
- `dev/duhowett/hax/our-own-tabview` — May 2026 direct MUX 2 TabView import into UIHelpers, including XAML, automation peers, IDL, generated properties and theming; roughly thousands of lines of contemporary implementation.
- `dev/duhowett/hax/punchout` — unresolved reverse-video/transparency behavior, upstream #7014 remains the recovery contract; old geometry implementation remains useful evidence.
- `dev/duhowett/hax/serial-port-support` — December 2025 serial-port connection capability on the modern connection architecture.
- `dev/duhowett/hax/unix-pty` — February 2026 Unix PTY / PTY bridge implementation that progressed to an actually hooked-up bridge.
- `dev/duhowett/no-private-registry-keys` — retained because public UISettings accent color was not proven semantically equivalent to the DWM titlebar color; see Cohort 007 and PR #14188.
- `dev/duhowett/padding-in-atlas-rebase-20250729` — authoritative later member of the padding/Atlas family; includes explicit padding propagation and gutter handling. The older `padding-in-atlas` ref is redundant.
- `dev/duhowett/powershell-module-supercharger` — still received substantive PowerShell-module work in March 2026.
- `dev/duhowett/unicode-17` — Unicode 17 migration attempt remains unresolved: generated tables caused broad segmentation-test failures while maintained main remained on the prior data line. Preserve as evidence, do not port blindly.
- `dev/duhowett/vt-cache-changes` — late-2025 coherent VT/OSC parser safety/performance line, including in-flight OSC length limiting and OSC fast-path experiments.
- `dev/duhowett/win7-wpf-termcontrol-squash` — substantial WPF/legacy TerminalControl compatibility implementation refreshed in April 2026, including Flat-C surface, current settings/resource-loader adaptation and horizontal wheel support.

## Recovery contracts for retired families

### Connection UTF-8 experiment

`connection-utf8` explored changing `ITerminalConnection` stdin from UTF-16 `Char[]` to UTF-8 bytes and moving conversion to the `TermControl` boundary. Do not cherry-pick the old ABI. If revisited:

1. Define the encoding contract at each public/WinRT boundary explicitly.
2. Keep transport bytes separate from UI string semantics.
3. Replay embedded-NUL, non-BMP, malformed-sequence, paste and broadcast cases.
4. Change the ABI only with all connection implementations and consumers migrated atomically.

### SettingsModel / AppAdapter architecture

`hax/terminalsettings-as-a-lib-/with-types-merged-into-tsm` was a broad attempt to move shared settings types out of Control, move application-specific TerminalSettings behavior behind an adapter library, and keep generic TerminalControl independent of the app’s JSON model.

The branch is not a modern topology to transplant. Preserve the architectural criterion instead:

1. TerminalControl should consume narrow settings abstractions, not application JSON objects.
2. Shared settings types belong at a stable model/ABI boundary.
3. Application-specific conversion belongs in an adapter layer.
4. Settings Editor and Terminal App may share the model without forcing Control to depend on app serialization.
5. Re-evaluate current project boundaries before implementing this direction again.

### Layer-9 / condrv API experiment

`hax/l9` added an experimental layer-9 console API and a standalone client around `IOCTL_CONDRV_ISSUE_USER_IO`, including descriptors, an APC callback and control-handler signaling.

If similar protocol investigation is needed:

- start from current condrv message/header definitions;
- isolate experimental API numbers and request bodies from production dispatch;
- test input/output descriptor sizing and WOW64 layouts;
- separately validate synchronous reply, APC delivery and control notification semantics.

The 2024 implementation is diagnostic evidence, not production code.

### IDL consolidation experiments

`hax/merge_idl` and `fhl-2024/merge-idls` are one broad program of reducing repetitive WinRT IDL/project boilerplate and experimenting with cppwinrt build generation. The latter reached upstream WIP PR #16931 and closed without merge.

Recovery criterion: optimize IDL organization only when generated ABI, namespace identity, incremental build correctness and cppwinrt tooling behavior remain identical. Treat the old diffs as build-system evidence rather than patches.

### Renderer / UI micro-experiments

The following old branches reduce cleanly to replayable behavior rather than unique implementations:

- `hax/wpf-atlas`: validate any future WPF renderer swap against input latency, resize, DPI, theme and device-loss behavior; do not resurrect the 2023 one-commit switch.
- `hax/sui-color-chip-border`: color chips that visually disappear into the Settings background need an accessible contrast/border treatment derived from current theme/background.
- `hax/conhost-icon`: simplify icon ownership and cache representations by DPI where useful; replay DPI-change, ownership and resource-lifetime cases.
- `hax/cursor_stamp_foreground_background`: if cursor foreground/background behavior is revisited, replay inverse cursor, explicit cursor color, default colors and accessibility/high-contrast combinations.
- `hax/embed-everything`: PRI/resource embedding is a packaging/build question; re-measure startup, package size, localization lookup and servicing before choosing it.
- `portable-shader-members`: shader constant-buffer layout must be versioned/descriptor-driven if member order becomes movable; validate packing and compatibility against the current renderer.
- `learn/rewrite-highlights`: preserve zero-copy/generational highlight and selection behavior, not the learning implementation.

### Padding / Atlas family

The old `padding-in-atlas` is superseded inside this namespace by `padding-in-atlas-rebase-20250729`. Retain only the latter. Any eventual integration must replay:

- top/bottom/left/right padding propagation;
- atlas viewport/scissor calculations;
- gutters and edge-cell rendering;
- scrolling, resize, DPI and fractional scaling;
- cursor/selection/hyperlink decoration alignment.

### NTSTATUS cleanup

`i-have-a-burning-hatred-for-ntstatus-of-later-so-why-not-fix-it` is a broad 2023 mechanical conversion/encapsulation experiment across host, interactivity and propslib, especially RegistrySerialization.

Recovery criterion: expose HRESULT/WIL/Win32-friendly errors at normal library boundaries while preserving exact NT/Win32 conversion semantics where kernel-facing code requires them. Re-run behavior and failure-path tests; do not mechanically replay stale file-by-file edits.

### Timers / core-interactivity

`move-timers-down-into-core-interactivity-etc` explored moving cursor/VT blink timers into ControlCore, autoscroll into Interactivity, and pointer ownership closer to the layer that owns the behavior.

Recovery criterion: lifecycle and scheduling belong with the component that owns the state, but dispatcher/threading, focus/blur rules, pane teardown and nonexistent-buffer cases must be replayed before moving timers again.

### Terminal Settings UI / FHL prototypes

`fhl-2025/bitmap-fonts`, `fhl-2025/what-if-no-content-ids`, and `fhl-2025/wt-command-palette-cmdpal-integration` are lab branches, not reviewed product lines. Preserve their ideas only as questions for current architecture:

- bitmap-font support must be justified against modern shaping/rendering and DPI behavior;
- removal/avoidance of content IDs must preserve identity, persistence and automation contracts;
- Command Palette integration must be rebuilt against the current CmdPal/Terminal surface rather than the historical prototype.

### Build / CI experiments

The following are safe because the literal implementation is tied to old infrastructure:

- `cant-believe-gotta-do-this-shit`: temporary CI-agent pin/workaround.
- `coast-to-coast`: packaging/minimum-OS pipeline experiment.
- `feed-forward-variables`: matrix/output dependency propagation experiment.
- `fhl-2024/clang`: clang-cl compatibility exploration; re-run against current compiler/toolset instead.
- `server-2025-vms`: Windows Server 2025 image test, superseded by later toolchain/image movement.
- `interface-projects`: old project/interface factoring experiment.
- `is-pgo-broken-because-of-sui-being-slower`: performance/PGO investigation; reproduce with current binaries and traces rather than preserving the branch.

### Already absorbed / resolved product lines

- `chop`: functional upstream line culminated in merged PR #12724; later branch content is synchronization noise.
- `eoy-25/underline-colors-in-atlas-bug`: exact reviewed line merged upstream as #19872.
- `sel-2-spans`: selection architecture has moved forward through maintained span/generational work; replay selection behavior against current code if needed.
- `wellp2`: upstream PR #19322 closed without merge, but issue #19312 was later confirmed resolved in release 1.23.12811.0 and closed completed; the old catch-and-leak workaround is no longer authoritative.

### Diagnostic / scratch families

`hax/clogs`, `hax/conhost_dump_replay`, `multi-blern`, `osc-strided-table`, `sticky-control`, `testing`, `errordialog`, and the typo-namespaced `dev/duhowett/hax/appstate_remember` do not justify permanent refs. Their useful diagnostic or interaction ideas are recoverable from commit history and this ledger. New diagnostics should be built against current APIs, not by reviving stale integration branches.

## Namespace closure statement

For every ref marked SAFE, its unique useful knowledge is now one of:

- already present in maintained upstream history;
- superseded by a newer branch retained in this namespace;
- represented above as a behavioral or architectural Contract Replay rule; or
- diagnostic/build experimentation whose literal implementation is no longer authoritative.

No SAFE ref contains code that must be ported to `dev/duhowett/main` before deletion.

The namespace is intentionally **not reduced to only `main` yet** because 19 non-main KEEP refs still contain contemporary or unresolved implementations, and 2 WAIT refs are active upstream review heads. Those survivors should be revisited as their upstream/product status changes.
