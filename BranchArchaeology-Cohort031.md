# Migrie namespace archaeology — Cohort 031 / closure

Date reviewed: 2026-09-07

This cohort closes the current `dev/migrie/**` namespace as a unit of work. It complements `BranchArchaeology-Cohort030.md` and the older topic-specific archaeology ledgers already stored on `dev/migrie/main`.

## Closure result

At the start of this pass the fork exposed **68 physical `dev/migrie/**` refs**.

Every one of those refs is now classified. There are **no branches left in an unresearched / unknown state**.

- Cohort 030 already authorized **14** refs for retirement.
- Cohort 031 authorizes **39 additional** refs for retirement.
- **2** refs remain WAIT because they carry knowledge tied directly to still-open upstream PRs.
- **13** refs remain KEEP, including `dev/migrie/main`, because they are contemporary recovery implementations/specifications or otherwise preserve irreducible design/code not found in a reviewed successor.

If the 53 SAFE refs from Cohorts 030+031 are physically deleted, the namespace compresses from **68 refs to 15 deliberate survivors**.

The closure question is therefore answered:

> What unique knowledge remains anywhere in the current Migrie namespace that is not either preserved in `dev/migrie/main`, represented by a reviewed upstream successor, or deliberately retained in one of the KEEP/WAIT refs below?

**None is known.**

---

## Cohort 031 — SAFE TO RETIRE (39)

### Product work with a reviewed or shipped successor

These branches no longer own their product contract. The durable behavior exists in reviewed upstream work or in a later architecture.

- `dev/migrie/f/compatibility-sui`
- `dev/migrie/f/now-with-more-compat-settings`
  - Historical compatibility/settings-UI explorations. Later Settings UI/model work is the authority; do not replay these old branches mechanically.

- `dev/migrie/f/cwd-hijinks-5506-15173`
  - Dirty CWD/process-model exploration. The reusable diagnostic/product idea matured elsewhere (including reviewed current-directory work such as #15282). Recover the behavior, not this scratch implementation.

- `dev/migrie/f/pane-exit-animation`
  - Prototype/cleanup line preceding reviewed pane-exit animation work; the reviewed feature is the authority.

- `dev/migrie/f/til-winrt.h`
  - Superseded by merged PR #16837, `Replace usages of TYPED_EVENT with til::event`, which performed the repository-wide reviewed migration.

- `dev/migrie/fhl/9583-colorSelection`
  - Proof of concept for #9583. The issue is closed `completed` / fix committed. The branch no longer owns the feature.

- `dev/migrie/fhl/10175-web-search-for-text`
  - Historical web-search/selection exploration. #10175 is closed completed and the extensibility/action architecture has moved on.

- `dev/migrie/fhl-spring-2026/confirmCloseOn`
  - **ALREADY ABSORBED.** The branch evolved into Carlos Zamora's reviewed implementation, merged as #20055 on 2026-05-04. #20055 preserves the important contract: `warning.confirmOnClose` (`never` / `automatic` / `always`), aggregate confirmation for close-other-tabs / close-tabs-after / close-other-panes / quit, and the `don't ask again` behavior. Do not preserve this branch merely because the earlier #19944 attempt closed unmerged.

- `dev/migrie/s/action-ids`
  - Superseded by the reviewed Action ID architecture, notably merged #17162 and its prerequisites.

- `dev/migrie/s/thema-schema-for-1.16`
- `dev/migrie/s/theme-pair-schema`
  - Historical theme/schema staging lines. Theme schema/support subsequently shipped and remains represented in current settings/schema code. Preserve only compatibility expectations, not these historical schema branches.

- `dev/migrie/save-input-patches`
  - Stash/review-support line around the Save Input action. The tip explicitly says it is being stashed to avoid losing work and primarily polishes key-chord error handling and success/failure toast binding. The product feature was reviewed and merged as #16513 on 2024-07-02 (`Add ability to save input action from command line`), including save-to-disk, selection fallback and toast UX. If that behavior regresses, replay those contracts against current action/settings code rather than resurrecting this stash.

### Shell integration / marks / action experiments whose contract is now sufficient

- `dev/migrie/fhl/more-shell-integration`
- `dev/migrie/fhl/scroll-marks-prototype`
  - Old shell-integration/mark plumbing. Recovery contract: prompt/command/output metadata must remain coherent; marks may drive navigation/scrollbar/status experiences; shell metadata must not regress high-performance rendering/input paths. Later shell-integration/marks work is the architectural authority.

- `dev/migrie/fhl/save-command`
  - Early experiment that turns a command line into a dynamically registered `SendInput` action. Later snippets/tasks/actions work subsumes the useful concept.

- `dev/migrie/fhl/clickSendInput`
- `dev/migrie/fhl-clickable-send-input`
  - Early clickable-input/suggestion experiments. The histories themselves call out abandoned or dirty approaches (`...but meh`, and the later proof-of-concept notes that a private OSC 9001 was the wrong abstraction and should probably relate to OSC 8). The mature unreleased idea is deliberately retained in the `5916` implementation/spec pair below.

- `dev/migrie/fhl-fall-2023/1620-automatic-tab-progress`
  - POC for #3991. The issue remains conceptually valid, but this implementation says its logic is `definitely wack`. Recovery contract is small enough to preserve here: while a newly-created tab is connecting, it may display indeterminate progress until the first byte/connected state; a future implementation must be rebuilt against current tab/status APIs.

### Explicit prototypes, failed experiments, backups and negative evidence

- `dev/migrie/f/apples-to-oranges`
  - Early theming experiment from before the reviewed theme architecture.

- `dev/migrie/f/branch-2-backup`
  - Backup/project-layout branch; not a product authority.

- `dev/migrie/f/fix-intellisense-i-guess-backup`
  - 2020 IntelliSense workaround (`lib/pch.h`-style repair); obsolete tooling/build-system archaeology.

- `dev/migrie/f/com.fabrikam.toaster`
  - TeachingTip/UX exploration; useful as negative UX evidence, not as code to recover.

- `dev/migrie/f/more-vt-renderer-tracing`
  - Temporary renderer tracing instrumentation from 2020.

- `dev/migrie/f/no-custom-caption-btns`
  - Failed caption-button experiment; its own history records that the changes did not solve the visual problem.

- `dev/migrie/f/roast-mutton`
  - Scratch/integration line inside the much larger process-model/OOP genealogy already preserved in dedicated archaeology ledgers. It is not an independent product destination.

- `dev/migrie/fhl/dyndep`
- `dev/migrie/fhl/dyndep-csharp`
  - Extension/dynamic-dependency experiments. The C# line explicitly ends with `this was kinda a dead end`. Preserve that result as negative evidence rather than preserving the refs.

- `dev/migrie/fhl/menu-complete-prototype`
  - Demo prototype; history explicitly labels later work as `bugfixes for the demo`.

- `dev/migrie/fhl/rgb-rainbow-window-frame`
  - Visual experiment; central commit literally says `I don't know what this is but I like it`. Later theme/window-frame work is the right surface for any intentional rainbow-frame feature.

- `dev/migrie/fhl/upside-down-mode`
  - Deliberately extreme rendering/UI experiment (`very bad, no good hack`, `totally batshit` in branch history). Useful idea/negative evidence, not production code.

- `dev/migrie/fhl-fall-2023/oceans`
  - HWND/XAML layering extension experiment. The branch says the approach `will most certainly break with winui 3`; it is implementation archaeology, not a durable extension architecture.

- `dev/migrie/dol/messing-with-shaders-take-1`
  - Early 2021 shader/differential-present investigation. History explicitly records `I did _not_ figure this out on DoL` and partial attempts whose buffers accumulated bad state. The old `DxRenderer` architecture is obsolete; the later Atlas renderer plus merged custom-shader work (see the dedicated FHL2021 shader archaeology ledger and #13885) is authoritative.

- `dev/migrie/eim/incremental-build-000`
  - 2021 WAP/VS incremental-build exploration. History repeatedly calls the approach `BODGY` / `Maximum bodgy`, includes experiments that `don't` help, reverts, manually-created copy-complete files and preview-VS assumptions. The current build stack is materially different. Keep only the lesson: build-performance fixes must be replayed against the modern graph/toolchain, not copied from this branch.

### Integration / presentation / historical-document snapshots

- `dev/migrie/bump-scratch`
  - Short-lived 2023 sample-maintenance scratch (`get the sample building again`).

- `dev/migrie/demo-for-presentation`
  - Presentation/demo snapshot built largely from already-reviewed product commits. It does not define a separate product contract.

- `dev/migrie/fhl/2024-spring-merge-base`
  - Integration branch combining then-current pane/scratchpad/`til::event` lines. Its children/successors, not the merge-base, are the semantic authorities.

- `dev/migrie/oct-21-roadmap-update`
  - October 2021 roadmap snapshot. Historical planning state is not executable branch authority.

- `dev/migrie/s/north-star`
  - Tasks vision/spec polished for review. Its durable direction subsequently materialized through snippets/actions/pane-content. Keep the architectural ideas in the curated archaeology, not this staging branch.

- `dev/migrie/s/5000-presentation`
  - 2022 presentation branch whose history explicitly consists of PowerPoint edits (`update the pptx`, `not totally sure what I touched in the powerpoint but whatever`). Not a product/source branch.

---

## WAIT / DO NOT DELETE (2)

### `dev/migrie/overview-for-pr`

**WAIT — active upstream PR #20267, `Add an "Tab Overview" pane to the Terminal`.**

This fork ref is the current local representation of still-open upstream work. Do not retire it until the PR merges, is superseded, or is explicitly abandoned and its remaining contract is captured.

### `dev/migrie/pr-15717/its-dangerous-to-go-alone`

**WAIT — active review companion for upstream PR #15717, `Add a key binding Toggle Acrylic`.**

The dedicated `BranchArchaeology-ToggleAcrylic.md` ledger remains authoritative. Mike's unique `ffc78620e...` reviewer experiment is not represented in the current contributor PR head. Its contract is specifically about preserving runtime acrylic-toggle intent when opacity visits 100% and then returns below 100%.

Retire only after #15717 (or a successor) resolves that behavior, or after explicit abandonment with the contract retained as recovery guidance.

---

## KEEP / DELIBERATE SURVIVORS (13)

### Permanent curated lane

- `dev/migrie/main`
  - Permanent curated knowledge lane. Never delete.

### Contemporary recovery implementations (2026)

- `dev/migrie/f/new-profile-subcommand`
  - June 2026 implementation of a command/subcommand that creates a profile from the current working directory. No reviewed successor was found.

- `dev/migrie/fhl-spring26/nextTab-filter`
  - March 2026 complete implementation adding `TabStatusFilter` to `nextTab` / `prevTab`. It filters by bell/activity/progress state (`set`, `error`, `indeterminate`, `paused`), supports direct and switcher paths, and includes a re-entrant test/demo script. No reviewed successor was found. This is real recoverable code, not a notification scratch branch.

- `dev/migrie/fhl-spring-2026/quake-5`
  - March 2026 workspace lifecycle implementation: persist on close, distinguish live vs saved workspaces, and hide already-open workspaces from the saved list. No reviewed successor found.

- `dev/migrie/fhl-spring-2026/side-tabs`
  - March 2026 substantive left/right/top/bottom tab-strip implementation. It restructures `TerminalPage` layout, preserves deferred-load overlays/dialogs, adds splitter/resource handling and vertical-tab UX. No reviewed successor found.

- `dev/migrie/fhl-spring-2026/x-open`
  - March 2026 recovery implementation on the workspace/pane line, adding rudimentary `x-open` behavior with path resolution relative to the active CWD. No reviewed successor found.

- `dev/migrie/s/snippet-params`
  - May/June 2026 snippet-parameter work, including active-parameter selection behavior. Contemporary and not represented by a reviewed successor.

### Recovery implementation/specification pairs

- `dev/migrie/fhl/5916-triggers`
- `dev/migrie/s/5916-draft`
  - Preserve together. They contain the mature unreleased triggers/custom-clickable-link design that supersedes the much dirtier clickable-input POCs. The scope is large enough that a one-paragraph recovery note would throw away useful design/code.

- `dev/migrie/fhl/notebook-proto-000`
- `dev/migrie/s/markdown-notebooks`
  - Preserve together. The implementation explores process/output/threading behavior across notebook controls; the spec predates and informs later Markdown/pane-content work but is not proven to be fully absorbed by it. Branch history records both difficult threading experiments and a later spec `breakthrough`. This remains an irreducible executable-notebook lineage.

### Recovery specifications

- `dev/migrie/s/ai-providers`
  - Preserve as a broad AI-extension/provider architecture and policy document. Later `feature/llm` work did ship an `ILMProvider` abstraction (for example merged #17394), but that does not prove absorption of the whole permissions/privacy/provider-boundary design captured here.

- `dev/migrie/s/1553-mouse-bindings`
  - Preserve as a substantial mouse-interaction design. It consolidates multiple requests and proposes concrete semantics for double/triple/quad-click selection, modifiers, delimiters/regex and mouse actions. No reviewed successor covering the whole design was found.

---

## Active upstream work not requiring a fork-side survivor

Two open upstream PRs under Migrie's namespace have canonical heads that are **not** present among the fork's 68 physical refs, so no substitute fork branch is required merely to remember them:

- #20329 — per-window-name settings — canonical upstream head `dev/migrie/per-window-final`.
- #16895 — pane resizing with mouse — canonical upstream head `dev/migrie/f/992-redux-redux`.

Upstream is the authority for those live heads.

---

## Cohort 030 retirement set (already authorized)

For completeness, the previous cohort already made these 14 refs SAFE:

- `dev/migrie/fhl-spring26/activity-notifications`
- `dev/migrie/fhl-spring26/bellStyle-notification`
- `dev/migrie/fhl-spring26/unpackaged-notify`
- `dev/migrie/fhl-spring26/osc777`
- `dev/migrie/fhl-spring26/osc777-2`
- `dev/migrie/fhl-spring26/2/notification-infrastructure`
- `dev/migrie/base-pane`
- `dev/migrie/fhl/tasks-pane`
- `dev/migrie/fhl/md-pane`
- `dev/migrie/fhl/local-tasks-2024`
- `dev/migrie/fhl/adaptive-card-extension`
- `dev/migrie/fhl/2024-inline-notebook`
- `dev/migrie/f/default-icons`
- `dev/migrie/fhl/sxnui-tooltips-3`

Do not reinterpret those as new Cohort 031 work; they are repeated only so this closure ledger can account for all 68 refs.

---

## Namespace retirement manifest

### SAFE NOW — 53 total across Cohorts 030 + 031

Delete the 14 Cohort 030 refs plus the 39 Cohort 031 refs listed above.

### WAIT — 2

- `dev/migrie/overview-for-pr`
- `dev/migrie/pr-15717/its-dangerous-to-go-alone`

### KEEP — 13

- `dev/migrie/main`
- `dev/migrie/f/new-profile-subcommand`
- `dev/migrie/fhl/notebook-proto-000`
- `dev/migrie/fhl/5916-triggers`
- `dev/migrie/fhl-spring26/nextTab-filter`
- `dev/migrie/fhl-spring-2026/quake-5`
- `dev/migrie/fhl-spring-2026/side-tabs`
- `dev/migrie/fhl-spring-2026/x-open`
- `dev/migrie/s/ai-providers`
- `dev/migrie/s/markdown-notebooks`
- `dev/migrie/s/snippet-params`
- `dev/migrie/s/1553-mouse-bindings`
- `dev/migrie/s/5916-draft`

**68 = 53 SAFE + 2 WAIT + 13 KEEP.**

No product code was transplanted into `dev/migrie/main` during this cohort. This pass is archaeology/decision compression only; future recovery work should use Contract Replay against modern `main` rather than blind cherry-picks.