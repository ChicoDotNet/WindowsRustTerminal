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

## Cohort 003 — maintenance/tooling branches with no remaining product contract

### `dev/duhowett/eyebeam`

- Sole functional branch-specific commit: `61e46e31595da09363c897eaad5f3ec3a07114fa` (2020-03-25), `whitelist IBeam`.
- The commit changes only the historical spell-check whitelist by adding the token `IBeam`; the remaining branch-tip commits are mechanical spelling migrations.
- No application behavior, API, compatibility rule, test contract, or data shape is present in this branch.
- The old spell-check infrastructure has itself evolved, so preserving a branch for a single historical dictionary token has no recovery value.
- Classification: **NO-PORT / obsolete spelling-only maintenance branch**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/clang`

- Functional commit: `3ea1ae242a27df9023d713a13cd4af05d4175f36` (2020-11-13), `HAX HAX HAX clang tidy`; later tip commits are spelling migrations.
- Intent: force AuditMode/projects through `ClangCl` and clang-tidy, experiment with analyzer configuration, and expose compiler-compatibility diagnostics.
- The branch contains deliberate diagnostic scaffolding and compiler-only hacks (including an artificial `wmemchr` call through a null pointer and temporary disabling/rewriting of telemetry-related code); these are evidence that it is a toolchain laboratory, not candidate product code.
- Current `main` does not retain the branch's `EnableClangTidyCodeAnalysis`, `clang-tidy-wrapper`, or global `ClangCl` configuration.
- No product behavior contract or test behavior needs replaying.
- Classification: **NO-PORT / abandoned compiler-tooling experiment**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/fix-tracing-2`

- Functional commits: `c7a1b257f2a70f8cf3663aaf3cd497988cb4b484` (`Rip out RIPMSG`) and `cb6d4aa402b1f73e14ded27af26fc595decb92ef` (`WIP: Remove the other dbg macros`), July 2023, built on top of the merged tracing cleanup #15737.
- Intent: remove legacy debug/tracing residue: `RIPMSG*`, `Telemetry::LogRipMessage`, `DBGCHARS`, `DBGOUTPUT`, `_DBGFONTS`, `gDebugFlag`, and their callsites.
- Current `main` contains none of those symbols. The desired cleanup state is therefore already present in the maintained tree even though these branch commits were WIP archaeology rather than the authoritative final history.
- There is no runtime feature or compatibility contract to recover from the deleted debug macros.
- Classification: **ALREADY ABSORBED / cleanup state present in current tree**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/dead-loc`

- Functional commits: `4ac3a98f2a06e441244d25071490d99f5d628329` (`Remove old loc keys from TermApp and Settings`) through `a5dbcb014c3c6cab6ec25db94ab8a2e850554d66` (`Fix more loc issues`), March 2024.
- Intent: delete stale localization resources and experiment with avoiding UID collisions by renaming caption-button `x:Uid` values.
- Representative resources removed by the branch (`KeyboardServiceDisabledDialog.*`, `LegacyGlobalsProperty*`, `AddProfile_AddNewButton.Tag`, `ColorScheme_DeleteButton2.Text`, etc.) are absent from current `main`, so the dead-resource cleanup intent has already landed through the maintained localization stream.
- The final experimental renames (`MinimizeButton` -> `WindowMinimizeButton`, etc.) were not adopted: current `MinMaxCloseControl.xaml` intentionally retains `x:Uid="MinimizeButton"`/`MaximizeButton`/`CloseButton` while using distinct `Window*ButtonToolTip` UIDs for tooltips. The current resource layout is authoritative.
- Classification: **ALREADY ABSORBED for dead-resource cleanup; NO-PORT for abandoned UID rename experiment**.
- Disposition: **SAFE TO DELETE**.

## Cohort 003 result

Safe refs:

- `dev/duhowett/eyebeam`
- `dev/duhowett/clang`
- `dev/duhowett/fix-tracing-2`
- `dev/duhowett/dead-loc`

Explicitly retained:

- `dev/duhowett/font-64` — **CONSERVAR / NO BORRAR**. Its checkpoint `49691f891aeabf02dba506d4c5080c49eac3aaba` is explicitly referenced by still-open upstream issue #3123 (long font names through conhost settings/property sheet/TrueTypeFontList). This branch remains a recovery reference for unresolved work.
- `dev/duhowett/conpty-flags` — unresolved mixed ConPTY behavior prototype; successor contracts not yet proven.
- `dev/duhowett/hax-selection-exclusive` — selection experiment still requires lineage reconstruction.

## Cohort 004 — diagnostics and compiler-architecture experiments

### `dev/duhowett/hax/tsm-graphviz`

- Functional commit: `94b295ab724d746845cd0ca3d8c9ceec967e1272` (2020-10-21), `HAX: Expose the profile inheritance tree as a graphviz document`; later tip commits are spelling migrations.
- Intent: temporarily make profile-parent internals accessible, walk the inheritance graph after settings load, emit a Graphviz DOT document, and send it to `OutputDebugStringW` for developer inspection.
- It is a diagnostic visualization instrument rather than product behavior. It changes no persisted settings contract and adds no user-facing supported API or test invariant.
- The graph dump also intentionally weakens encapsulation (`_parents` moved out of `protected`) solely to support the diagnostic.
- Classification: **NO-PORT / disposable diagnostic tooling**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/128-bit-compiler`

- Functional commits: `52ae8c1124b140b3d4e4012429a2d57ba692606e` (`Remove PreferredToolArchitecture`) and `2837d8d0040bba8be304c146bd6f5c088e816335` (`I hate it.`), November 2022; branch tip then received a spelling migration.
- Intent: resolve the tension between 64-bit-native VS2022 tooling and native ARM64 builds. The branch first removed the override, then restored `PreferredToolArchitecture=x64` conditionally under WOW64 because contemporary Azure DevOps tasks still launched 32-bit MSBuild.
- Definitive successor: upstream PR #20518, `Remove PreferredToolArchitecture override now that we require VS 2026`, merged 2026-08-06. Its rationale explicitly revisits the same historical workaround and removes it now that the supported toolchain makes the old compromise unnecessary.
- Current `main` contains no `PreferredToolArchitecture` setting.
- Classification: **NO-PORT / superseded by final toolchain decision #20518**.
- Disposition: **SAFE TO DELETE**.

## Cohort 004 result

Safe refs:

- `dev/duhowett/hax/tsm-graphviz`
- `dev/duhowett/128-bit-compiler`

Explicitly retained/recovery candidates discovered while building this cohort:

- `dev/duhowett/no-private-registry-keys` — **CONSERVAR / ADAPT candidate**. Its 2022 prototype replaces direct reads of private DWM accent-color registry state with the public `Windows.UI.ViewManagement::UISettings` API. Current `main` still reads `HKCU\\Software\\Microsoft\\Windows\\DWM\\AccentColor`, so this intent is not absorbed and deserves a future contract/port decision.
- `dev/duhowett/font-64` — remains protected by still-open issue #3123.
- Recent experimental branches such as `dev/duhowett/asan-for-all`, `dev/duhowett/hax/clogs`, and `dev/duhowett/interface-projects` are not early-cleanup candidates and remain untouched.

## Cohort 005 — superseded runtime prototypes and abandoned build/release experiments

### `dev/duhowett/graph`

- Sole branch-specific commit: `b6fc9297f5b186818dd3ed9d93c15e923b0d9dce` (2024-04-02), `build: try out the MSBuild static graph`.
- The entire experiment is one line: add `/graph` to the VSBuild/MSBuild invocation in `job-build-project.yml`.
- Current `main` does not use `/graph`, and the branch is one commit ahead of its 2024 merge base while more than 900 maintained commits have moved the pipeline onward.
- No product behavior, compatibility contract, or unique test is present.
- Classification: **NO-PORT / abandoned build experiment**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/fgb`

- Sole branch-specific commit: `d881eaedb7f180bed884f09418c545f4a304f5e5` (2023-06-23), `swiggity swooty i'm comin for that FBGoost`.
- Intent: propagate Terminal focus to the ConPTY child process using a provisional `ITerminalConnectionWithWindowAffinity` interface and dynamically resolved `SetAdditionalForegroundBoostProcesses`.
- Definitive upstream successor: PR #19192 / commit `0d23624fa9d620b13155f491c8b1c5d91dbf0ba4` (2025-08-13), `Use a new API to propagate foreground state to child processes`.
- The maintained implementation solves the same QoS/foreground contract at the Terminal window/tab/pane level with `TerminalTrySetWindowAssociatedProcesses`, handles active versus background tabs, and exposes the root process handle through the maintained connection model.
- Classification: **NO-PORT / superseded by formal foreground-QoS architecture #19192**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/net8`

- Sole branch-specific commit: `21f91b1d7aa846d73023a588fe4b1768cc84630e` (2024-10-15), `Move WPFTerminalControl and TestNetCore to NET 8`.
- Intent: move the WPF terminal control/test projects to .NET 8.
- Current `main` has the maintained result: `WpfTerminalControl.csproj` targets `net472;net8.0-windows`, preserving compatibility while adding the .NET 8 target. The branch changes only the two WPF C# project files.
- Classification: **ALREADY ABSORBED / maintained dual-target implementation**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/rename-all-dlls`

- Functional commits: `3564c6cb54eaebb19ab7cfac1f2cddfeb5a3666a` (`Rename TerminalConnection to Microsoft.Terminal.Connection`), `24297f99d99e25c54a122ac5de20b4c3e978ef90` (`Rename WindowsTerminalShellExt to Microsoft.Terminal.ShellExtension`), and `238d848ecc5a9d685e3b2d1fbe68977a12ae438d` (`HAX: TerminalApp->M.T.App`), February 2023.
- Intent: mechanically rename assemblies/DLL identities and their namespace-qualified references across the Cascadia tree.
- The branch is exactly three commits ahead of its historical merge base. Current `main` does not adopt the proposed `Microsoft.Terminal.Connection` identity, so this is an abandoned naming/refactor experiment rather than an unmerged product capability.
- The wide diff is mechanical name propagation: manifests, project files, tests, imports, `.def` filenames, and namespace references; there is no independent runtime contract to replay.
- Classification: **NO-PORT / abandoned mechanical assembly-renaming experiment**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/onebranch-custom-pool`

- Functional work is confined to OneBranch/Azure DevOps pipeline files; the branch is seven commits ahead of its 2024 merge base and changes only `ob-nightly.yml`, `job-build-project.yml`, and `pipeline-onebranch-full-release-build.yml`.
- Representative branch commit `c0ec1968c6fa15f98e38e099e7ad2cd86ab436fd` (2024-08-20), `We still have to publish only one artifact tho`, forces logs into the output artifact and publishes it even on failure.
- Current `main` deliberately retains the other policy: when `publishArtifacts` is true it publishes build/cache logs separately; otherwise it copies them into `Terminal.BinDir`.
- This is pipeline-policy experimentation with no product/runtime contract, and the maintained pipeline chose a different design.
- Classification: **NO-PORT / abandoned OneBranch pipeline experiment**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/nuget-publication-with-aad-app-id`

- Branch-specific commits: `06f212f08e68b9c95faf3c0f7db20e987a7bbc7f` (`Try to publish the PGO package using AAD...`), `c0531c542692df29503452d431eaac32be3b7567` (`NFCI: Lock to a specific build to tighten test cycle`), and `06ccfd601d78181d6d28aa6c44dde5e29203a703` (`use -apikey instead of setapikey`), April 2024.
- The branch is exactly three commits ahead of its merge base and modifies only `build/pipelines/pgo.yml` and `build/pipelines/templates-v2/job-pgo-build-nuget-and-publish.yml`.
- Intent: trial an AAD/API-key authentication path for publishing the internal PGO NuGet package while shortening the CI feedback loop.
- This is credential/publication pipeline experimentation, not product behavior, and its historical test-cycle pinning/auth flow is not a recovery contract worth preserving as a live branch.
- Classification: **NO-PORT / obsolete release-pipeline experiment**.
- Disposition: **SAFE TO DELETE**.

## Cohort 005 result

Safe refs:

- `dev/duhowett/graph`
- `dev/duhowett/fgb`
- `dev/duhowett/net8`
- `dev/duhowett/rename-all-dlls`
- `dev/duhowett/onebranch-custom-pool`
- `dev/duhowett/nuget-publication-with-aad-app-id`

Explicitly retained/pending after this pass:

- `dev/duhowett/hax/tap_upgrade` — **CONSERVAR POR AHORA**. The historical HAX mutates the active `TermControl` connection to insert a tap and split a debug pane, while current `main` still contains a maintained `DebugTapConnection` implementation. The branch is also contaminated by a large spelling-infrastructure migration, so its exact functional lineage must be separated before deleting the ref.
- `dev/duhowett/conpty_first_frame_blug` — **CONSERVAR POR AHORA**. It carries a concrete first-frame/background-color regression test plus a renderer fix; the old renderer state has disappeared but no definitive successor contract has yet been established.
- `dev/duhowett/applicableactions` and `dev/duhowett/copylink` — intertwined 2024 Suggestions/selection lineage; not quick-delete candidates.
- `dev/duhowett/compiler-compliance` — several compiler-correctness fixes; requires per-commit absorption checks.
- `dev/duhowett/wprp` — unique WPR performance profile; diagnostic-only appearance is not enough by itself to discard potentially useful perf instrumentation.
- `dev/duhowett/server-2025-vms` and other recent/live 2025–2026 experiments remain outside early archaeology.
