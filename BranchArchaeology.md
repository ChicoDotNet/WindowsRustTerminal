# Branch Archaeology — niels9001

This branch is the curated durable home for historical knowledge from `dev/niels9001/**`.

Baseline at creation: fork `main` = `15a36f470c9c9f8a6e5822b9836f4ea53052b7a6`.

The three legacy refs present during the first namespace-compression pass were investigated together. None should be transplanted wholesale onto the current tree. Their useful value is behavioral intent and recovery guidance.

## dev/niels9001/page-transitions

- Relevant SHA: `6e05d9ad5c85f6fa9d8aee8187c51ead8bc390e1`
- Functional age: 2026-05-16.
- Exclusive work vs current fork `main`: 1 commit.
- Upstream issue: microsoft/terminal#20235, **still open** as of 2026-09-07.
- Upstream PR: microsoft/terminal#20236, closed without merge on 2026-06-14 after becoming stale / needing author feedback.
- Intent: Settings navigation should use transitions according to navigation semantics rather than one `DrillInNavigationTransitionInfo` for every page change:
  - top-level NavigationView changes -> Entrance/default;
  - breadcrumb/back -> slide from left;
  - drill-in/forward -> slide from right.
- The old implementation introduced `NavDirection`, `_MakeTransitionInfo()`, and propagated transition info through Settings `Navigate(...)` callsites.

### Disposition

`NO-PORT / recovery candidate` — **SAFE TO RETIRE**.

The bug/UX contract remains relevant because #20235 is still open, but the PR was never accepted and the Settings implementation has continued to evolve. Do not cherry-pick the May 2026 patch onto modern code.

Recovery path: reproduce #20235 against current Settings navigation, then add a focused navigation-transition contract around top-level/back/forward intent and implement it using the current Settings navigation architecture.

---

## dev/niels9001/fontweight-fixes

- Tip SHA: `53103f6488e7ad5b5b77788b4fd602d17da24e25`
- Functional age: 2026-05-16 through 2026-05-18.
- Exclusive work vs current fork `main`: 3 commits.
- Upstream issue: microsoft/terminal#20228, **still open** as of 2026-09-07.
- Upstream PR: microsoft/terminal#20237, closed without merge on 2026-06-14 after becoming stale / needing author feedback.
- Intent: align Terminal UI typography more closely with the Fluent type ramp. The PR's concrete first pass replaced selected `Bold` uses with `SemiBold`, removed some italic shortcut hints, and cleaned spacing in new-tab tooltips.
- Validation contract from the PR included Suggestions, Command Palette highlighted matches, new-tab button tooltip, and per-profile dropdown tooltip.

### Disposition

`NO-PORT / recovery candidate` — **SAFE TO RETIRE**.

The underlying UX issue is still open, but the three-commit patch is a point-in-time first pass, not an accepted architectural contract. Preserve the intent rather than the exact XAML/C++ substitutions.

Recovery path: audit current TerminalApp/Settings UI against the current Fluent typography guidance, convert the expected typography into focused visual/resource-level contracts, and make targeted modern changes. Use microsoft/terminal#20228 and #20237 as historical evidence rather than transplanting these commits.

---

## dev/niels9001/inactive-tab-foreground

- Relevant SHA: `cf67c8e92e07a3f8dc4b803089775ffaf9fd4663`
- Functional age: 2026-05-28.
- Exclusive work vs current fork `main`: 1 commit touching only `src/cascadia/TerminalApp/Tab.cpp`.
- No matching upstream PR by Niels was found in the May/June 2026 PR cohort.
- Intent: when an inactive tab's effective background is fully transparent (for example the default theme's transparent `tab.unfocusedBackground`), do **not** override the stock deselected tab foreground/close-button foreground. Let the Fluent `TabViewItemHeaderForeground` / secondary text treatment show through instead of forcing the black/white contrast brush chosen for an opaque custom tab color.
- Historical implementation gated the `TabViewItemHeaderForeground` and `TabViewItemHeaderCloseButtonForeground` resource overrides on `deselectedTabColor.a != 0`.
- Current fork `main` still computes a `deselectedFontBrush` from the composited inactive color and unconditionally inserts it into both resources, so the historical behavioral question is not already absorbed verbatim.

### Disposition

`NO-PORT / recovery candidate` — **SAFE TO RETIRE**.

The behavior is worth preserving, but the one-off alpha check is not automatically the correct modern fix: current theming, tab-row compositing, custom tab colors, hover/pressed states, and high-contrast behavior all need to remain coherent.

Recovery path: add a modern contract for a transparent inactive-tab background verifying that stock Fluent secondary foreground is preserved, plus opaque-custom-color and high-contrast controls. Implement the smallest current-architecture change in `Tab.cpp` only after those contracts demonstrate the gap.

---

# Namespace result

First-pass namespace compression result: **3 legacy refs -> 1 curated branch**.

Safe legacy refs:

- `dev/niels9001/page-transitions`
- `dev/niels9001/fontweight-fixes`
- `dev/niels9001/inactive-tab-foreground`

Keep:

- `dev/niels9001/main`

No historical implementation was ported. All remaining value is captured as issue/PR provenance plus recovery contracts suitable for modern Contract Replay/TDD.
