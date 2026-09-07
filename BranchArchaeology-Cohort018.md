# Branch Archaeology — Cohort 018

This cohort closes four historical `dev/migrie/b/**` branches whose useful behavior has either been productized in a later architecture or whose experimental implementation is no longer part of the product model.

## `dev/migrie/b/dxd-marker`

- Functional commit: `7cde1407eb2eabc84b5259b8daacd1efaa5506a7` (`stash`, 2022-05-23); later tip movement is spelling maintenance.
- Historical intent: during default-terminal package selection, instantiate the handoff COM object and require `IDefaultTerminalMarker` before allowing Windows Terminal to receive a terminal-by-default handoff.
- Upstream foundation: `microsoft/terminal#13160` (`Use feature markers during terminal-by-default handoff`) merged as `1b81c6540fc33166b3498c8567e2613a7ab20b94` and formally introduced `IDefaultTerminalMarker` as the willingness/capability marker for default-terminal handoff.
- Modern reading: the marker is still a real interface. Current `CConsoleHandoff` implements `IConsoleHandoff` and `IDefaultTerminalMarker`; current server-side handoff code checks the marker and falls back to the conhost delegation pair when the marker is absent. The experimental check in `DelegationConfig` therefore evolved into a productized check at a better ownership boundary.
- Disposition: **TRANSFORM / superseded by the modern handoff architecture**.
- Recovery value: provenance only; do not restore the package-selection-time COM probe.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/json-patching-is-hard`

- Functional/head commit: `cdf62de18975e8c991f11fd60841b2e5a19d3547` (`Fix some issues patching in profiles`, 2019-09-18).
- Historical intent: make `_AppendDynamicProfilesToUserSettings()` robust when the user settings contained an empty `profiles` array or no `profiles` object at all, while maintaining correct commas/insertion offsets as generated profiles were textually appended.
- Historical value: the branch documents two edge cases that were important to the old settings-file patcher: `{ profiles: [] }` and a settings object with no `profiles` member.
- Modern reading: `_AppendDynamicProfilesToUserSettings()` and that textual profile-insertion mechanism no longer exist in current `main`; modern settings/profile layering does not patch generated profiles into the user's JSON by string offsets.
- Disposition: **NO-PORT / obsolete settings architecture**.
- Recovery value: the edge-case intent is recorded here, but porting the old offset-manipulation code or tests would test a mechanism that no longer exists.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/localtests-ci-2022`

- Functional commit: `aded3facb5434279f9c5a2ef3445224298b2052e` (`This cannot possibly work, right?`, 2022-06-22); later tip movement is spelling maintenance.
- Historical intent: add a dedicated Azure Pipelines stage/template that downloads the build, installs/runs `*LocalTest*.dll`, converts WTT logs to xUnit and publishes results.
- No PR was associated with the experimental commit.
- Modern contract replay: current `build/pipelines/ci.yml` explicitly preserves AppX dependencies because LocalTests need them. Current `templates-v2/job-test-project.yml` installs those dependencies and has an explicit `Run Local Tests` step matching `*LocalTests*.dll` for x64/arm64, followed by log conversion/publication.
- Disposition: **ALREADY ABSORBED / productized in modern CI**.
- Recovery value: provenance only; the 2022 pipeline template itself is obsolete (old hosted image/artifact/task layout), while its objective is now a permanent CI sensor.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/no-cloaky-cloak`

- Functional commits: `84f95d20c797bd25e47d0dfc4093dce21f2d6365` (explicit revert of `#12979`) and `dae74672f5e035b7e7785d27c5d10eaa119755f3` (`This is the bad merge from 69b77ca`); later tip movement is spelling maintenance.
- Historical context: `microsoft/terminal#12979` (`Hide the window from DWM until we're finished with initialization`) merged as `14098d71f22225765c5c71907893cd86cc1738a6`, attempted to avoid the transparent/black startup frame by delaying `ShowWindow` until a low-priority `Initialized` event.
- The experiment was later reverted; the `#11561` discussion explicitly records: `Whoops, we reverted this and never reopened`, and continued investigating compositor/WinComp-level solutions instead of the delayed-show mechanism.
- Modern reading: current `main` no longer contains the `_AppInitializedHandler` mechanism from `#12979`. Thus this branch is not a unique fix waiting to be restored; it is a historical revert/worktree used while diagnosing a startup-rendering approach that was abandoned.
- Disposition: **NO-PORT / rejected startup-window experiment**.
- Recovery value: preserve the lesson that merely delaying/showing the Win32 window was not the durable rendering solution; do not resurrect this branch wholesale.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 018 deletion set

- `dev/migrie/b/dxd-marker`
- `dev/migrie/b/json-patching-is-hard`
- `dev/migrie/b/localtests-ci-2022`
- `dev/migrie/b/no-cloaky-cloak`
