# Leonard Hecker branch archaeology — Cohort 001

This ledger records the first namespace-wide compression pass over `dev/lhecker/**`.

Starting population: **71 refs including `dev/lhecker/main`**.

The rule for this lane is behavioral preservation, not branch preservation. A historical ref can be retired when its unique value is either already present in an upstream successor/current architecture, or reduced to an explicit recovery contract here. Contemporary PR heads and substantial unique implementations are retained.

## Active upstream work — do not delete

These fork refs are exact heads of currently open upstream PRs and remain authoritative while those PRs are open:

- `dev/lhecker/theme-quality` — microsoft/terminal#20508, new Ottosson Dark/Light themes.
- `dev/lhecker/generate-256-colors` — #19883, draft, dynamic 8-bit palettes.
- `dev/lhecker/1410-large-scrollback` — #18290, large/infinite scrollback.
- `dev/lhecker/dcs-perf` — #19640, DCS batch-processing performance.
- `dev/lhecker/14165-conhost-font-size` — #19905, conhost font sizing for AtlasEngine.
- `dev/lhecker/osc-7-wsl` — #20094, OSC 7 UNC path mangling via WSL.

## Substantial/recent recovery refs held for Cohort 002

Do not delete these in this cohort. They either contain contemporary work, unresolved bug fixes, or enough unique implementation that a short prose contract is not yet an adequate substitute:

- `dev/lhecker/theme-quality-test` — one WIP commit on top of the active `theme-quality` line.
- `dev/lhecker/18928-wip` — substantial tmux Control Mode prototype (`TmuxControl`, `TmuxConnection`, app/control/core plumbing); preserve implementation.
- `dev/lhecker/renderer-overhaul-2nd-attempt` — later renderer-overhaul recovery implementation; retained while the first attempt is retired.
- `dev/lhecker/atlas-engine-compute-shader` — substantial renderer experiment.
- `dev/lhecker/cleanup` — recent tokenization/integer-parsing cleanup experiment.
- `dev/lhecker/dark-mode` — theming experiment requiring comparison with the modern theme line.
- `dev/lhecker/attach-thread-input` — 2025 robustness work around `AttachThreadInput` without a confirmed successor.
- `dev/lhecker/15689-tab-drag-crash-fix` — issue #15689 remains open; branch contains fixes for tab-drag crash/two-pane close behavior.
- `dev/lhecker/17656-win32im-double-encoding` — issue #17656 remains open; branch avoids encoding plain-text paste as Win32 input-mode sequences.
- `dev/lhecker/1860-horizontal-scrollbar` — contemporary WIP.
- `dev/lhecker/benchcat-fix` — 2026 v145 build fix.
- `dev/lhecker/wellp2-alt` — 2025 WIP.
- `dev/lhecker/window-thread-climate-control` — window-thread experiment, not yet compressed.
- `dev/lhecker/client-context-input-output-mode` — input/output-mode experiment, not yet compressed.
- `dev/lhecker/bugbash` — large integration branch made from multiple historical feature branches; its high ahead count is genealogical noise until semantic families are separated.

Other refs not listed as SAFE below are simply **unclassified in Cohort 001**, not implicitly safe.

# SAFE TO RETIRE — Cohort 001

## Successor/merge chains

### `dev/lhecker/11509-kitty-keyboard-protocol-wip`

Early February 2026 WIP. Leonard rebuilt the feature on `dev/lhecker/11509-kitty-keyboard-protocol`; upstream PR #19817 merged 2026-02-17 and closed #11509. The clean merged line is authoritative.

Disposition: **SAFE — superseded by merged successor**.

### `dev/lhecker/20149-hotfix`

The work became PR #20153, which closed without merge. PR #20213 explicitly superseded #20153, merged 2026-05-12, and closed both the issue and superseded PR.

Disposition: **SAFE — superseded by merged successor**.

### `dev/lhecker/issue-4015-til-rect`
### `dev/lhecker/4015-cursor`

Both are 2022 WIP precursors for the 32-bit-coordinate migration. Upstream PR #13025 merged 2022-06-03 and migrated the project to 32-bit `til::point`, `til::size`, `til::rect`, and related coordinate types, closing #4015.

Disposition: **SAFE — absorbed by merged architectural successor**.

### `dev/lhecker/18584-part2`

Historical attempt to clean persisted layouts after disabling persistence. Leonard later described finding the missing piece in PR #18910; that PR merged 2025-05-14 and closed #18584.

Disposition: **SAFE — superseded by merged fix**.

### `dev/lhecker/fucking-service-locator`

Prototype of the conhost MSAA/UIA rewrite. The same work resurfaced under `dev/lhecker/BEGONE-FOUL-SPIRIT`, upstream PR #19344. That PR merged 2025-09-22, removed `ServiceLocator`, centralized accessibility event raising and debounced events, with a reported >10x improvement for the pathological MSAA case.

Disposition: **SAFE — precursor renamed into merged PR**.

### `dev/lhecker/16575-TerminateProcess`

Pre-merge variant of the shutdown fix around `TerminateProcess`. Upstream PR #16575 merged 2024-01-26 (co-authored by Leonard) to prevent a window thread holding the final Emperor reference from causing `E_WRONG_THREAD` during shutdown.

Disposition: **SAFE — merged behavior exists upstream**.

### `dev/lhecker/app-state-actually-hidden`

Prototype titled “Allow generated profiles to be deleted”. The work became `dev/lhecker/delete-generated-profiles`, upstream PR #11007, merged 2021-08-23. Merged behavior defines deletion/hidden semantics for generated profiles and filters Startup Profiles appropriately.

Disposition: **SAFE — precursor renamed into merged PR**.

## Superseded by a retained later implementation

### `dev/lhecker/renderer-overhaul`

The first renderer-overhaul experiment predates and is superseded as an archaeology artifact by the explicitly named `renderer-overhaul-2nd-attempt`, which is retained for Cohort 002. No claim is made that the old hashes were merged verbatim.

Disposition: **SAFE — earlier attempt superseded; later recovery implementation retained**.

### `dev/lhecker/ottosson-by-default`

One February 2026 semantic commit, “Default to Ottosson”, on an older line. The current design authority is the actively reviewed `theme-quality` / PR #20508 line.

Recovery contract: if defaults are reconsidered, evaluate Ottosson as a default explicitly against compatibility, readability, accessibility, existing user expectations and the final theme-quality implementation; do not resurrect this old patch mechanically.

Disposition: **SAFE — product question preserved, implementation superseded**.

## Mechanical/tooling experiments whose value is consumed

### `dev/lhecker/gsl-narrow`
### `dev/lhecker/remove-gsl`

The first branch incrementally replaced GSL types with standard-library equivalents; the later branch says “Remove gsl”. Current `main` no longer contains `dep/gsl`.

Disposition: **SAFE — architectural goal already realized**.

### `dev/lhecker/clang-tidy`

Single 2023 experiment creating a `.clang-tidy` from local IDE settings. Current `main` does not carry that file. This is historical tooling, not product behavior.

Disposition: **SAFE — NO-PORT tooling experiment**.

### `dev/lhecker/til-to-ulong-improvements`

Tiny post-#13766 cleanup (“Make `til::to_ulong` more compact”). No unique product contract exists beyond correctness of the conversion helper.

Disposition: **SAFE — mechanical cleanup**.

### `dev/lhecker/terminal-settings-cleanup`

2021 mechanical simplification of `TerminalSettings::ColorTable`; the settings model has since evolved substantially.

Disposition: **SAFE — stale mechanical refactor**.

### `dev/lhecker/til-env-cleanup`

Single 2023 `til::env` refactor with no unique externally observable behavior.

Disposition: **SAFE — stale mechanical refactor**.

### `dev/lhecker/wsl-distro-generator-cleanup`

2021 WIP cleanup of the WSL dynamic profile generator. Modern dynamic-profile/extension architecture is the authority.

Disposition: **SAFE — stale mechanical prototype**.

### `dev/lhecker/remove-chrome-math`

Mechanical removal of Chromium math / rarely-used `til::rect` helpers on the `goodbye-vtengine` integration line. The major ConPTY architecture change itself landed as PR #17510 and the codebase has continued to evolve. The extra cleanup does not justify retaining a historical integration ref.

Disposition: **SAFE — NO-PORT mechanical cleanup**.

### `dev/lhecker/tracy`

Single 2023 commit adding Tracy profiling instrumentation. Useful as an experiment, not as durable product behavior.

Disposition: **SAFE — profiling experiment consumed**.

### `dev/lhecker/winrt-file-api-benchmark`

2022 benchmark WIP. No durable product contract requires the branch.

Disposition: **SAFE — benchmark experiment consumed**.

### `dev/lhecker/vsconfig`

One 2022 `vsconfig` update. Current repository/toolchain metadata is the authority.

Disposition: **SAFE — stale build metadata**.

## Recovery contracts: behavior preserved, old patch retired

### `dev/lhecker/5907-startup-perf`

Old November 2022 startup-performance WIP. Issue #5907 remains a broader performance concern, but Leonard subsequently landed multiple focused startup optimizations; the old branch is not the authority.

Recovery contract:
- measure startup/profile activation before changing behavior;
- defer expensive non-critical work and lazy-load where correctness permits;
- preserve first-window/profile correctness while reducing synchronous startup work;
- use current performance instrumentation and architecture, not the 2022 patch.

Disposition: **SAFE — recovery contract preserved**.

### `dev/lhecker/fix-window-count`

One-commit draft PR #19299, closed without merge: prevent logical window count from going out of sync when a headless path closes.

Recovery contract:
- headless operations must not decrement/count a UI window that was never materialized;
- window lifetime counters must represent the same entity at creation and destruction;
- test headless creation/close, regular windows, mixed sequences and final-process shutdown.

Disposition: **SAFE — recovery contract preserved**.

### `dev/lhecker/get-lang-id`

Draft PR #18565, closed without merge. It explored preventing `CP_UTF8` from changing the locale reported through `GetConsoleLangId`.

Recovery contract:
- code page/encoding selection is transport behavior and must not silently redefine the user's/thread's language identity;
- test non-English locale + `chcp 65001` and legacy code pages;
- preserve compatibility with callers that use language ID independently of UTF-8 transport.

Disposition: **SAFE — recovery contract preserved**.

### `dev/lhecker/pwsh-5.1`

Draft PR #20135, closed without merge, adding a version suffix to the Windows PowerShell profile.

Recovery contract: if Windows PowerShell profile naming is revisited, distinguish legacy Windows PowerShell from modern PowerShell without breaking stable profile GUIDs, user references, settings migrations or localization.

Disposition: **SAFE — recovery contract preserved**.

### `dev/lhecker/render-snapshot`

2022 renderer snapshot WIP. Renderer/ConPTY architecture has changed radically since then; PR #17510 explicitly opened cleaner future paths for snapshotting.

Recovery contract:
- snapshot a coherent render/buffer state rather than observing partially-mutated state;
- define ownership/lifetime and synchronization boundaries;
- validate legacy API output, VT passthrough, scrollback, resize/reflow and renderer invalidation against the modern Atlas/ConPTY architecture.

Disposition: **SAFE — recovery idea preserved, stale implementation retired**.

### `dev/lhecker/rgba`

2021 renderer/color-model WIP, far predating current Atlas/theme work.

Recovery contract: if color representation is changed, preserve channel precision and explicitly define alpha semantics (straight vs premultiplied) across parsing, settings, render data and GPU composition. Do not mechanically port this historical patch.

Disposition: **SAFE — recovery contract preserved**.

# Cohort 001 scorecard

- Starting refs: **71**
- SAFE in this cohort: **27**
- Active upstream PR heads protected: **6**
- Large/recent recovery refs explicitly held: **15**
- Remaining after deleting this cohort: **44 refs**, including `dev/lhecker/main`

This is **not namespace closure**. Cohort 002 should concentrate on the remaining renderer/Atlas, input, color/theme and miscellaneous WIP families, using the same semantic-base method that exposed false “very ahead” counts in earlier developer namespaces.
