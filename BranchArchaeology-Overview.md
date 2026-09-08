# Tab Overview branch archaeology

This ledger separates the historical `dev/migrie/f/overview-view` prototype from the active reviewed line `dev/migrie/overview-for-pr`.

## `dev/migrie/f/overview-view`

### Functional lineage

This is a March–May 2026 prototype for `microsoft/terminal#15385` (Tab Overview). Its tip, `62c47cf8fb45d70efd9dc56e4690f3d894b11513`, is a May 14 merge from then-current `origin/main`; the useful Overview work predates that tip.

Representative contracts recovered from the prototype include:

- clickable live tab previews (`6f384c59f409bb588cbbedefea75ed0f74db5f71`);
- a theme-aware overview background (`724c5197889cbeac427eb52e5f9dc6ea35c8cb73`);
- route unhandled overview keys through the global action dispatcher while keeping local grid navigation local (`451bf3700832551931c87ffcf9d0c66f326c6e45`);
- release/reparent Overview content before an external tab-selection change remounts the selected tab (`3dd5dadb38d28bf9ba073f14831600f7c7db62bf`);
- dismiss/reparent Overview content before executing a non-`ToggleOverview` action, so tab-state-mutating actions do not run while live tab content is parented into the overview (`d3c17f9d6fe2de969d56bd8ce41eb490f4c1dda5`).

The prototype also contains exploratory/debug commits with informal messages and an unrelated handoff-manifest change (`c67adbdedb9839ededaf5dd6a7ddf848a94a17dd`, `need you`). That commit enables the `windows.comInterface` block in `Package-Dev.appxmanifest`, including the `ITerminalHandoff3` proxy registration. Current repository `main` contains the same enabled block; in fact the current file blob is the same `4a3153076c8d0986611b5c04356149b5da795232` blob produced by that commit. Therefore this accidental side delta is already absorbed and does not require preservation as a branch.

### Reviewed successor — ACTIVE, not legacy

The product successor is **microsoft/terminal#20267 — Add a "Tab Overview" pane to the Terminal**, targeting issue #15385. The PR remains **open** and its head is exactly `dev/migrie/overview-for-pr` (currently upstream SHA `02336267a7e95631be5627c4084434c0afe69aab`).

Git ancestry between the prototype and PR line is divergent rather than a straight descendant relationship, so this disposition is based on contract replay, not branch-name similarity.

Inspection of the active PR head confirms that the important prototype contracts were deliberately reimplemented in the reviewed line:

- `OverviewPane` reparents live tab content and `ClearTabContent()` restores it; exit animation/destruction paths also clean it up.
- `TerminalPage::_DismissOverviewVisuals()` is called before non-`ToggleOverview` actions and in external tab-selection paths before remounting selected content.
- `TerminalPage.xaml` wires `OverviewPaneElement` with `PreviewKeyDown="_KeyDownHandler"`, preserving global keybinding dispatch while the Overview has focus.
- preview cell borders have a `PointerPressed` handler selecting/clicking the corresponding tab.
- Overview background/foreground handling is theme/Mica aware in the current implementation.

The active PR additionally replaces the prototype's rough layout with a substantially more complete adaptive grid and enter/exit animation lifecycle.

### Disposition

`dev/migrie/f/overview-view`: **ALREADY ABSORBED by the active #20267 implementation** at the contract level. No code from the prototype should be transplanted into the curated branch.

`dev/migrie/overview-for-pr`: **CONSERVE / ACTIVE WORK — DO NOT DELETE** while PR #20267 is open. This is not a legacy branch merely because it lives under `dev/migrie/**`.

### Branch retirement

**SAFE TO DELETE: `dev/migrie/f/overview-view`** after this ledger entry.

The old ref no longer carries unique Overview or handoff-manifest knowledge. The durable product implementation currently lives in the active PR branch; its eventual merge/closure should be evaluated before that active ref is ever considered for retirement.
