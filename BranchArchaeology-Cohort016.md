# Branch Archaeology — Cohort 016

This cohort closes three non-numeric `dev/migrie/b/**` branches whose useful intent is now represented by later upstream architecture and tests.

## `dev/migrie/b/add-support-for-vsc-marks`

- Functional/head commit: `070b8f68ccf2ab74b9f64f83839c6390ee795222`.
- Historical intent: accept VS Code/xterm.js OSC 633 shell-integration mark sequences as aliases for the FinalTerm/OSC 133 marks Windows Terminal already understood.
- Upstream lineage: PR `microsoft/terminal#15727` uses this exact branch/head. GitHub metadata reports `merged_at: null`, but merge commit `c7edab21d85e6860c2440c304e4a7c13c0e155d3` has `070b8f68...` as its second parent, so the branch is certified as merged by the commit graph rather than by the nullable PR field.
- Disposition: **ALREADY ABSORBED**.
- Recovery value: provenance only; the useful implementation is in upstream history.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/be-better-at-hiding`

- Functional/head commit: `59e719404e57b38353e37ce9afbed8c875e35366` (2020-01-06).
- Historical intent: correct the VT renderer's cursor-hide sequence from `CSI 25 l` to the DEC private-mode form `CSI ? 25 l` (`DECTCEM`).
- The commit itself notes that this was not useful by itself until Terminal also implemented cursor hiding.
- Upstream successor: `microsoft/terminal#4902` implemented cursor visibility/blinking semantics in Terminal and added tests, closing `#3093` among related cursor issues.
- Modern contract replay: current `src/host/ut_host/VtIoTests.cpp` explicitly expects `\x1b[?25l` followed by `\x1b[?25h` when cursor visibility is toggled, while modern renderer/core code also models `?25l/h` as the cursor-visibility contract.
- Disposition: **NO-PORT / superseded with modern contract coverage**.
- Recovery value: none beyond provenance; the exact behavioral contract is now tested in the modern tree.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/moving-focus-is-hard`

- Functional commits: `aa2fe262a194c8946d2434bdae74edb7a256eaf9` and refinement `548dd0ea92c0824fb2b2b29e4214654fdd1ac6f6` (2020-05-29). Later tip movement is spelling maintenance.
- Historical intent: prevent focus from silently landing on a tab/transient tab UI after tab activation or rename dismissal; the refined design introduced a `RequestFocusActiveControl` event so `TerminalPage`, rather than `Tab`, owned the decision about which control receives focus.
- The first attempt explicitly warned that focusing on every `GotFocus` could misbehave for rename and accessibility scenarios.
- Lineage: PR `#6290` later formalized essentially the broad "focus active control whenever TabViewItem gets focus" approach and explicitly worried about accessibility. It was replaced by the narrower design in PR `microsoft/terminal#10048`.
- Definitive successor: `#10048` is merged (`8564b269c4b757e9da4a20b2613e80d287fcb76d`). It describes itself as a redo of `#6290`, uses precision focus restoration for context-menu close, renamer dismissal and tab tapping, and introduces `RequestFocusActiveControl` ownership at the tab/page boundary.
- Modern reading: current `main` still contains `Tab::RequestFocusActiveControl`, raises it from `Tab.cpp`, and handles it in `TabManagement.cpp` by calling `_FocusCurrentTab(false)` when transient UI is dismissed. The architectural idea from the 2020 experiment therefore survives in a refined, productized form.
- Disposition: **NO-PORT / superseded by #10048; architectural intent retained upstream**.
- Recovery value: provenance only; do not resurrect the broad `GotFocus` interception.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 016 deletion set

- `dev/migrie/b/add-support-for-vsc-marks`
- `dev/migrie/b/be-better-at-hiding`
- `dev/migrie/b/moving-focus-is-hard`
