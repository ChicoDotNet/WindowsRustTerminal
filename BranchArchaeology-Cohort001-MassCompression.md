# Carlos Zamora branch archaeology — Cohort 001 mass compression

Date: 2026-09-08

Starting namespace: **75 refs** under `dev/cazamor/**`, including curated `dev/cazamor/main`.

Goal: compress by semantic families, preserving active PR/issue heads and converting non-canonical prototypes/selfhost snapshots into durable recovery contracts instead of keeping implementation branches.

## Scorecard

- 75 starting refs
- 47 refs classified **SAFE TO RETIRE** in this cohort
- 11 lateral refs explicitly **KEEP / WAIT** because they remain tied to open PRs/issues/specs
- 1 curated `main` — never delete
- 16 lateral refs intentionally deferred to Pass 2 rather than guessed
- Expected physical namespace after SAFE deletion: **28 refs**

## SAFE TO RETIRE

### Selfhost integration snapshots — 19

These are non-canonical selfhost/pre-merge assemblies. The source PR/feature branch is the authority; preserving a selfhost merge does not preserve additional product history.

- `dev/cazamor/selfhost/sui-rejuv`
- `dev/cazamor/selfhost/11-18-v3`
- `dev/cazamor/selfhost/11-18`
- `dev/cazamor/selfhost/2026-01-12`
- `dev/cazamor/selfhost/2026-01-20`
- `dev/cazamor/selfhost/2026-01-29`
- `dev/cazamor/selfhost/2026-02-10`
- `dev/cazamor/selfhost/2026-04-06`
- `dev/cazamor/selfhost/2026-04-08`
- `dev/cazamor/selfhost/2026-05-04-workspaces`
- `dev/cazamor/selfhost/2026-05-04`
- `dev/cazamor/selfhost/2026-05-05`
- `dev/cazamor/selfhost/2026-05-13-sui`
- `dev/cazamor/selfhost/2026-05-13`
- `dev/cazamor/selfhost/2026-05-18`
- `dev/cazamor/selfhost/2026-05-19`
- `dev/cazamor/selfhost/2026-05-20-home`
- `dev/cazamor/selfhost/2026-05-20`
- `dev/cazamor/selfhost/2026-06-18`

Representative canonical work assembled by these snapshots includes Actions UI (#18917), SUI search (#19519), screen-reader search announcements (#19726), tab-row acrylic (#19647), Kitty Keyboard Protocol (#19817), actions-page alignment (#19822), pane-title header (#20068), workspaces (#20162, later reverted by #20291), expander groups (#20200), activity notifications (#20014), and profile subpages (#20336).

### Selfhost-only recovery contracts

Preserve these ideas, not their old assembled branches:

1. **Merge-resolution correctness** (`2026-04-06`)
   - Explicit OSC777 notification should remain a non-transient `OutputNotificationStyle::Notification` when that semantic is selected.
   - Profile AutoDetectRunningCommand resource-key construction depends on the correct trailing separator/prefix convention.

2. **Color-scheme cards prototype** (`2026-05-18`)
   - If Settings revisits color-scheme browsing, test a card/gallery presentation with background preview and compact palette chips rather than porting the old XAML.

3. **SettingContainer styling crash** (`2026-05-19`)
   - Styling/template changes must be replayed against the modern SettingsCard/SettingsExpander architecture; the old `SettingContainer` path was subsequently removed.

4. **Consistent SUI spacing/home styling** (`2026-05-20-home`)
   - Preserve the product intent—consistent spacing/alignment across Actions, Dropdown and Profiles—but derive values from the modern design system.

5. **2026-05-20 selfhost Contract Replay matrix**
   - Re-test the integrated scenarios rather than preserving the integration branch itself: Settings navigation/search, Actions, Dropdown, Profiles, workspaces, pane/title behavior, notifications, and known-issue interactions recorded by that selfhost report.

### Settings UI rejuvenation — 6

Canonical/open lines supersede these refs:

- `dev/cazamor/sui-rejuv/profiles` — #20199 merged
- `dev/cazamor/sui-rejuv/setting-container` — #20232 merged
- `dev/cazamor/sui-rejuv/actions` — #20215 merged
- `dev/cazamor/sui-rejuv/expander-groups-new` — one-commit precursor to canonical expander-groups line
- `dev/cazamor/sui-rejuv/profiles-new` — one-commit precursor to final Profiles line
- `dev/cazamor/sui-rejuv/profiles-v2` — intermediate precursor to final Profiles line

### Auto-save implementation experiments — 3

The design spec remains open; these implementation branches do not.

- `dev/cazamor/auto-save/settings-model` — #20290 closed without merge
- `dev/cazamor/auto-save/model/json-manager`
- `dev/cazamor/auto-save/refresh-settings`

Recovery contracts:

- Centralize JSON persistence/write notifications behind one authority instead of allowing model objects to write independently.
- On settings reload, distinguish configured defaults from **live runtime deltas**. Reapply/preserve ephemeral overrides such as always-on-top, opacity/acrylic and VT-originated color overrides (OSC 4/10/11/12) instead of silently resetting live state.

### Extensions Settings page experiments — 3

- `dev/cazamor/sui/ext-page/badge` — old badge experiment; final Extensions page #18559 incorporated badge behavior and later #19637 removed the no-longer-new badge
- `dev/cazamor/sui/ext-page/lazy-load-objects` — lazy object/settings-loader intent incorporated into final #18559
- `dev/cazamor/sui/ext-page/powershell-stub` — #18639 closed without merge

Recovery contract for the PowerShell stub: when the latest PowerShell is absent, an Extensions/NTM-style surface may offer an installation stub; presence detection, acquisition UX and security must be redesigned against the current extension architecture.

### Accessibility/UIA historical experiments — 5

- `dev/cazamor/spec/a11y-vt-seq-v2` — superseded side revision; canonical `spec/a11y-vt-seq` remains open
- `dev/cazamor/a11y/system-menu-support` — #14217; issue #11970 is completed / Fix Committed
- `dev/cazamor/a11y/fastpass` — #16055 closed without merge
- `dev/cazamor/a11y/fake-uia-data` — debug spike returning literal fake UIA text
- `dev/cazamor/a11y/expand-line-under-viewport` — old boundary-fix/test branch on pre-exclusive-range UIA architecture

Recovery contracts:

- Accessibility CI: automated navigation may run Accessibility Insights/Axe.Windows FastPass and retain `.a11ytest` artifacts when such tooling is reintroduced.
- UIA boundary replay: degenerate range expansion at line start and expansion at buffer end must remain correct after any future UIA range rewrite.
- Fake-data injection is diagnostic technique only; never product behavior.

### Other absorbed / operational / test refs — 11

- `dev/cazamor/bot/deprecate-area-atlasengine` — one-off label migration, #19766 closed
- `dev/cazamor/uia-leak` — early diagnostic lab; maintained fix landed via #19950
- `dev/cazamor/confirmCloseOn/dont-ask-me-again` — predecessor to merged #20055 enum-based confirm-on-close behavior
- `dev/cazamor/sui/confirmation-announcements` — broken prototype; preserve announcement contract below
- `dev/cazamor/sui/dropdown-page` — old Dropdown prototype superseded by merged #20203 redesign
- `dev/cazamor/upgrade-settings-containers` — first SettingsCard precursor superseded by merged #20232
- `dev/cazamor/actions-page/template` — 2021 keybindings/ActionMap predecessor; `ActionMap` is maintained on modern main and Actions UI was subsequently redesigned
- `dev/cazamor/acc/ch/word-nav-perf` — historical branch around merged #7789 word-navigation optimization/WPR work
- `dev/cazamor/test/11440` — test integration branch for external PR #11440, which merged 2021-10-20
- `dev/cazamor/tag-youre-it` — #15687 telemetry privacy/performance-tag experiment; operational metadata, not product lineage
- `dev/cazamor/move-scratch` — only functional change moved `Scratch.sln`; that scratch solution no longer exists in modern tree

Recovery contracts:

- Color-scheme add/rename/delete operations should expose appropriate automation notifications when the UI requires confirmation announcements; rebuild against the current SUI rather than the old `[broken]` prototype.
- On app/window shutdown with layout saving enabled, revoke/throttle save callbacks before teardown and make close-time persistence tolerant of already-disposed state (behavior proven by merged #11440).
- Performance telemetry events should use the privacy/data classification appropriate to system/product performance rather than pretending they are user actions.

## KEEP / WAIT — active or unresolved lateral refs

These are **not** SAFE in Cohort 001.

- `dev/cazamor/sui-rejuv/expander-groups` — PR #20200 OPEN
- `dev/cazamor/sui-rejuv/profile-rejuv` — PR #20336 OPEN
- `dev/cazamor/spec/auto-save` — PR #19678 OPEN
- `dev/cazamor/bugfix/close-scratchpad` — implementation PR closed, but issue #20187 remains OPEN/assigned
- `dev/cazamor/fix/search-selection-off-by-one` — PR #20519 OPEN
- `dev/cazamor/conhost/bugfix-a11y-find` — PR #20532 OPEN
- `dev/cazamor/conhost/bugfix-a11y-find-1.24` — PR #20533 OPEN
- `dev/cazamor/a11y/nav-by-page` — old PR #14179 closed, but issue #13756 remains OPEN/assigned; viewport-as-UIA-Page contract unresolved
- `dev/cazamor/a11y-sev3/new-profile-announcement` — PR #13575 closed 2026-09-03, but issue #12038 remains OPEN; keep recovery reference
- `dev/cazamor/spec/a11y-vt-seq` — PR/spec #14342 OPEN
- `dev/cazamor/a11y/vt-seq-prototype` — 2026 implementation/debug prototype tied to the still-open VT screen-reader-control spec

## Curated ref — NEVER DELETE

- `dev/cazamor/main`

## Deferred to Pass 2 — no delete decision yet

These 16 refs were intentionally left unresolved rather than guessed:

- `dev/cazamor/color-picker-redesign`
- `dev/cazamor/copilot/playground`
- `dev/cazamor/drag-panes`
- `dev/cazamor/eim/mvvm`
- `dev/cazamor/ks/switchSelectionEndpoint`
- `dev/cazamor/mcs/viewport-selection`
- `dev/cazamor/revert-dwm`
- `dev/cazamor/sui/bugfix-reload-crash`
- `dev/cazamor/sui/inheritance-hyperlinks-test`
- `dev/cazamor/sui/invert-cursor-color`
- `dev/cazamor/sui/tab-color-old`
- `dev/cazamor/tile-background`
- `dev/cazamor/toast/activity`
- `dev/cazamor/wpf/uia-events`
- `dev/cazamor/wpf/uia-expose-enable-events`
- `dev/cazamor/1.15/scroll-to-point`

## Closure status

Cohort 001 deliberately compresses the high-confidence families first. Once the 47 SAFE refs are removed, only **27 lateral refs + curated main** should remain. Pass 2 can therefore reason over a namespace less than 40% of its starting size, with active/unresolved lines already separated from archaeology.
