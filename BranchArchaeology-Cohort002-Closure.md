# Cazamor branch archaeology — Cohort 002 closure

Date: 2026-09-08

This document closes the second-pass archaeology for the remaining `dev/cazamor/**` namespace after Cohort 001 removed 47 of 75 refs.

## Closure result

Starting state for this pass: **28 refs**.

Disposition after this pass:

- **14 refs SAFE to retire** after this ledger is persisted.
- **12 refs remain because they map to open/current upstream work or unresolved product contracts already identified in Cohort 001, plus `toast/activity` discovered here.**
- **1 ref remains as a reserved recovery implementation:** `copilot/playground`.
- **1 curated ledger ref remains permanently:** `dev/cazamor/main`.

Expected namespace after SAFE deletion: **14 refs**.

The closure invariant is satisfied: no unidentified Cazamor legacy branch remains. Every surviving lateral ref has an explicit reason to exist.

---

# SAFE TO RETIRE

## `dev/cazamor/color-picker-redesign`

**Disposition:** NO-PORT / recovery contract.

Historical implementation PR: microsoft/terminal#13552, closed without merge in 2022.

The branch tried to replace the accessibility-hostile magic `Custom` toggle in the tab-color picker with explicit `Standard` / `Custom` modes and more descriptive reset semantics. Issue #12044 is closed. Issue #12045 remains open, but the 2022 XAML implementation is stale and should not be preserved as the implementation authority.

**Contract Replay if #12045 is revisited:**

1. The custom-color entry point must expose a descriptive accessible name.
2. Its role/state must match its visual interaction model; do not present an expanded/collapsed surface as an opaque on/off toggle.
3. Standard/custom modes should be explicit to both visual and assistive-technology users.
4. Reset actions must name the object being reset (for example, tab color), not merely say `Reset`.
5. Rebuild against the current color-picker/SUI architecture rather than reviving #13552.

---

## `dev/cazamor/drag-panes`

**Disposition:** NO-PORT / superseded precursor.

The only meaningful Cazamor commit is the 2020 spike `Allow mouse resizing on panes`. The live successor is microsoft/terminal#16895, head `dev/migrie/f/992-redux-redux`, which implements pane resizing with modern manipulation events and pane-tree propagation.

The old Cazamor ref is not the authority for this behavior.

---

## `dev/cazamor/eim/mvvm`

**Disposition:** ALREADY ABSORBED / engineering precursor.

PR microsoft/terminal#11853 closed without merge, but its parent engineering issue #9207 (`Apply MVVM Pattern to Settings UI`) is now closed as completed. Modern Settings UI extensively uses ViewModels.

**Durable design contract:**

- Keep XAML-only concerns and display metadata out of the settings model.
- Prefer stable ViewModels over storing navigation-state objects on page instances.
- ViewModels are the correct place for UI-only enum lists, special display values, derived state, and change tracking.

No reason remains to preserve the 2022 partial refactor.

---

## `dev/cazamor/ks/switchSelectionEndpoint`

**Disposition:** ALREADY ABSORBED / renamed precursor.

This branch introduced the `switchSelectionEndpoint` action as a one-commit experiment. The work continued under `dev/cazamor/kbd-sln/switchSelectionEndpoint` and became microsoft/terminal#13370, merged 2022-07-01.

The merged PR is the canonical implementation and behavioral record.

---

## `dev/cazamor/mcs/viewport-selection`

**Disposition:** NO-PORT / recovery candidate.

PR microsoft/terminal#1302 closed without merge in 2019. It proposed whole-viewport selection plus configurable triple-click behavior (line / viewport / disabled), and referenced #1084, which remains open in the Icebox.

**Contract Replay if revived:**

- Whole-viewport selection is a product behavior, not an implementation detail.
- Triple-click semantics may be configurable between line selection, viewport selection, or disabled behavior.
- Re-evaluate against the modern exclusive-range selection model, mark mode, mouse-selection rules, and current profile/settings architecture.
- Do not port the 2019 `TerminalSettings` / `ICoreSettings` implementation mechanically.

---

## `dev/cazamor/revert-dwm`

**Disposition:** ALREADY ABSORBED.

Exact PR microsoft/terminal#13098 was merged 2022-05-13. It reverted the DWM-hiding change because persisted windows on secondary monitors could shrink slightly on each restore.

The merged upstream change is authoritative.

---

## `dev/cazamor/sui/bugfix-reload-crash`

**Disposition:** NO-PORT / recovery contract.

Unique functional commit: `d913907...` — `Fix Settings UI crash from reloading JSON` (2021).

**Contract Replay:**

- Reloading `settings.json` while Settings UI is open must not leave pages, navigation state, ViewModels, or controls pointing at the prior settings object graph.
- Any reload boundary must invalidate/rebind stale object references coherently.
- JSON reload must not produce a use-after-reload style crash simply because Settings UI remains open.
- Test reload while nested Settings pages are active, not just while the root page is displayed.

The current Settings architecture is too different for the 2021 patch to be useful as code.

---

## `dev/cazamor/sui/inheritance-hyperlinks-test`

**Disposition:** ALREADY ABSORBED / precursor.

This was an intermediate branch in the inheritance-display work. The canonical line became `dev/cazamor/sui/inheritance-hyperlinks`, PR microsoft/terminal#9079, merged 2021-02-19.

That merged work introduced override-source tracking, origin tags, reset visibility, and improved inheritance messages. The `*-test` branch is redundant.

---

## `dev/cazamor/sui/invert-cursor-color`

**Disposition:** NO-PORT / recovery contract.

Unique commit: `171daf47...` — `[WIP] Represent Invert for Cursor Color in SUI` (2025-10-08).

The branch follows the merged nullable-color work in microsoft/terminal#17870, whose follow-up list explicitly called for representing the cursor-color special value as `invert` instead of merely showing `#FFFFFF`.

**Contract Replay:**

- If the settings model still uses a special/sentinel cursor color to mean inversion, Settings UI must present that semantic state as **Invert**, not as an ordinary literal color.
- The accessible representation must convey the semantic state as well.
- Before implementation, verify whether current model semantics can distinguish literal white from inversion; do not encode a UI-only assumption that silently aliases two distinct user intents.
- Implement against current `NullableColorPicker` / appearance ViewModel code, not this WIP patch.

One WIP commit is not worth retaining as a branch once the behavior is ledgered.

---

## `dev/cazamor/sui/tab-color-old`

**Disposition:** ALREADY ABSORBED / predecessor.

The old two-commit prototype (`Add tab color setting to settings UI`, followed by a bugged null/theme attempt) was superseded by `dev/cazamor/SUI/tab-color`, microsoft/terminal#19351, merged 2025-09-25.

The merged implementation correctly models null as `Use theme color` and resolves the active theme/background cases. It is the canonical implementation.

---

## `dev/cazamor/tile-background`

**Disposition:** NO-PORT / old visual prototype.

Meaningful 2020 commits:

- `0e98b5c...` — `add tile setting + use ImageBrush`
- `c450551...` — `cache imageBrush`
- `36357d6...` — `bugfix: BI opacity`

No canonical PR/successor was found for this exact experiment.

**Recovery contract:**

- If tiled background images are revisited, treat tiling as an explicit background-image layout mode.
- Avoid rebuilding expensive image/brush resources on every incidental update; cache/reuse visual resources when their inputs have not changed.
- Background-image opacity must apply consistently to the image layer rather than accidentally changing unrelated terminal content.
- Rebuild on the current renderer/composition stack; the 2020 XAML `ImageBrush` implementation is obsolete.

---

## `dev/cazamor/wpf/uia-events`

**Disposition:** ALREADY ABSORBED.

Exact PR microsoft/terminal#14097 was merged 2022-10-06. It added UIA text/selection/cursor event plumbing for the WPF terminal control.

The merged upstream implementation is canonical.

---

## `dev/cazamor/wpf/uia-expose-enable-events`

**Disposition:** NO-PORT / post-merge recovery contract.

Unique follow-up commit: `85ab8141111cbae57c651275c7cde1c935e13a4c` — `Expose control over UIA events` (2022-10-07), one day after #14097 merged.

It decoupled focus from UIA event-engine enablement and exported explicit host calls analogous to:

- `TerminalTryEnableUiaEvents`
- `TerminalTryDisableUiaEvents`

through PublicTerminalCore -> WPF `TerminalContainer` -> WPF `TerminalControl`.

**Contract Replay if host-controlled event lifetime becomes necessary:**

- Focus state and UIA event subscription lifetime are separate concerns.
- A host should be able to explicitly enable/disable expensive UIA event production without falsifying focus state.
- The operation must tolerate the UIA engine not existing yet (`Try...` semantics).
- Re-evaluate ownership/lifetime against the current hosting architecture before implementation.

The branch is not a current product line and need not survive once this contract is recorded.

---

## `dev/cazamor/1.15/scroll-to-point`

**Disposition:** ALREADY ABSORBED.

Exact release-1.15 PR microsoft/terminal#13660 was merged 2022-08-03. It scrolls the selection marker into view for Mark Mode and SelectAll.

The merged release history is authoritative.

---

# KEEP / WAIT — newly classified in Cohort 002

## `dev/cazamor/toast/activity`

**Disposition:** ACTIVE — DO NOT DELETE.

Exact upstream head of microsoft/terminal#20014, still OPEN as of this archaeology pass and updated 2026-09-01.

The PR contains the current activity-notification settings line (`notifyOnActivity`, thresholds, next-prompt notification, running-command detection) and is the authoritative implementation while open.

---

## `dev/cazamor/copilot/playground`

**Disposition:** RESERVED RECOVERY IMPLEMENTATION — DO NOT DELETE.

This is unique contemporary work from March 2026 with no upstream PR, no matching upstream commit, and no successor found.

Relative to current fork `main`, the branch has five exclusive commits and adds a substantial `src/tools/wt.mcp/**` C# MCP server plus Copilot guidance. It is not merely a small product contract: it contains thousands of lines of executable implementation.

Known exclusive development sequence includes:

1. `Add a simple MCP server for WT settings`
2. `Improve UX flow of modifying settings; remove extra tools`
3. `Add support for fragment extension management`
4. `Add support for OhMyPosh, .wt.json, and shell integration management`
5. final branch state with the accumulated MCP tool surface

Recovered capability map:

- inspect/modify Windows Terminal settings through structured MCP tools;
- validate against settings/schema knowledge rather than blind text edits;
- manage fragment extensions;
- manage Oh My Posh related configuration;
- manage `.wt.json` and shell-integration configuration;
- expose snippets/tooling intended for Copilot-assisted Terminal administration.

**Why the branch stays:** a prose ledger is not an adequate substitute for ~3.5K lines of unique, reasonably contemporary implementation. This branch is intentionally retained until one of these happens:

1. the MCP server is promoted/ported into a maintained line;
2. its source is deliberately archived into `dev/cazamor/main` or another durable recovery location; or
3. a newer canonical implementation supersedes it.

This is the only non-PR historical implementation intentionally retained by the Cazamor closure.

---

# KEEP / WAIT carried forward from Cohort 001

These refs were already classified and remain intentionally outside the SAFE set:

- `dev/cazamor/sui-rejuv/expander-groups` — microsoft/terminal#20200 OPEN.
- `dev/cazamor/sui-rejuv/profile-rejuv` — microsoft/terminal#20336 OPEN.
- `dev/cazamor/spec/auto-save` — microsoft/terminal#19678 OPEN.
- `dev/cazamor/bugfix/close-scratchpad` — implementation line closed, but issue #20187 remains open/assigned; recovery candidate.
- `dev/cazamor/fix/search-selection-off-by-one` — microsoft/terminal#20519 OPEN.
- `dev/cazamor/conhost/bugfix-a11y-find` — microsoft/terminal#20532 OPEN.
- `dev/cazamor/conhost/bugfix-a11y-find-1.24` — microsoft/terminal#20533 OPEN.
- `dev/cazamor/a11y/nav-by-page` — old PR closed, but #13756 remains open/assigned; recovery candidate.
- `dev/cazamor/a11y-sev3/new-profile-announcement` — PR closed 2026-09-03, but #12038 remains open; recovery candidate.
- `dev/cazamor/spec/a11y-vt-seq` — microsoft/terminal#14342 OPEN.
- `dev/cazamor/a11y/vt-seq-prototype` — contemporary prototype coupled to the still-open VT screen-reader-control spec.

---

# PERMANENT CURATED REF

- `dev/cazamor/main` — **DO NOT DELETE**.

---

# Namespace closure statement

After deleting the 14 SAFE refs in this cohort, the Cazamor namespace should contain only:

- the curated `main` ledger;
- active/open upstream work;
- explicitly documented unresolved recovery candidates; and
- the single reserved unique MCP implementation (`copilot/playground`).

No remaining lateral ref is an unexplained legacy branch.

Overall compression across the two cohorts:

**75 refs -> 47 SAFE in Cohort 001 -> 28 refs -> 14 SAFE in Cohort 002 -> 14 intentional survivors.**

That is an **81.3% reduction** in Cazamor refs while preserving all identified unique knowledge and all active work.