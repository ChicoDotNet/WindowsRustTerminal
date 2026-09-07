# Branch Archaeology — Cohort 026

This cohort closes six historical commandline/window-management/process-model branches whose durable behavior either shipped through later reviewed PRs or was explicitly abandoned.

## `dev/migrie/f/execute-commandlines`

- Historical tip `6e1ce8de26e7bab7e50e706c94ca83b3d13b8f5d` is spelling maintenance; functional work dates to June 2020.
- Commit `10ace041c3e4d835d3557016aef38938dd1ba81d` shows the actual prototype contract: Command Palette parses a `wt` commandline, trims leading whitespace, converts it to actions, and executes those actions in the current window.
- Definitive successors: PR #6537 added the reviewed `wt`/execute-commandline action and merged as `d0ff5f6b5e7687c438e0a8b120280be307eda778`; PR #7293 then added Command Palette commandline mode and merged as `f897ce0a9f37b00efc0921fe46fe59628c0a217d`.
- Disposition: **ALREADY ABSORBED / superseded by #6537 + #7293**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/remote-commandlines`

- Historical tip `b39ba2761272bac913f4725a7d793425fdc3b0ee` is spelling maintenance; functional lineage predates the reviewed window-management implementation.
- Intent: route a `wt` commandline to another running Terminal window.
- Definitive successor: PR #8898, `Add support for running a commandline in another WT window`, implemented `--window/-w`, Monarch/Peasant routing and tests, and merged as `03ebe514e94f8d6a70b31578c5596fb9a1903174`.
- Later Process Model v3 retained the remoting contract while simplifying ownership into a single Terminal process.
- Disposition: **ALREADY ABSORBED / superseded by #8898 and later Process Model v3**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/name-windows`

- Historical tip `c000b2a00eb99c1aa0f0b308926b8c53b78a68a1` is spelling maintenance.
- Intent: allow Terminal windows to be addressed by stable user-facing names rather than only numeric IDs.
- Definitive successor: PR #9300, `Add support for naming windows with the -w parameter`, explicitly supports `wt -w foo new-tab` / `split-pane`, routes to an existing named window or creates it if absent, and merged as `43c469fc950f7b1fe1d1eac34be580fdb47119f1`.
- Named-window behavior remained part of Process Model v3 and received later fixes such as #15030.
- Disposition: **ALREADY ABSORBED / superseded by #9300**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/process-model-v3-test-0`

- Tip `f59046df90d9f800147732e6cc418e01b45641a0`, 2022-12-12, records the prototype target directly: `One app per process, one logic per window, INIT THE XAML ISLAND BEFORE any xaml`.
- The reviewed Process Model v3 series formalized precisely this ownership split. PR #14843, `One process to rule them all`, merged as `b9248fa903eaa8d187c72bf08f8aa17b25f9d6bb`: one Terminal process, one global `AppLogic`/`WindowEmperor`, and one `WindowThread`/`AppHost`/`TerminalWindow`/`TerminalPage` per window.
- Disposition: **NO-PORT / prototype superseded by merged Process Model v3**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/commandline-lib`

- Historical tip `d1e080cf631bd96355be418068ebe704d877d665` is spelling maintenance; this is the earlier exploration of extracting `AppCommandlineArgs` from `TerminalApp` into a standalone project.
- It was superseded by the review branch `dev/migrie/r/commandline-lib-002` / PR #8841.
- The extraction was never adopted. Current `microsoft/terminal` still keeps `AppCommandlineArgs.h/.cpp` under `src/cascadia/TerminalApp`, where Remoting, Command Palette and TerminalPage consume it.
- Disposition: **NO-PORT / abandoned project-boundary experiment**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/r/commandline-lib-002`

- Tip `4147ec121783159ded75c11c74f4cc9ffd738ce2`; exact head of PR #8841, `Move the commandline args to their own project`.
- The motivation was to expose parsing to Settings Editor for draft PR #8812 (`startupActions` in SUI). Neither PR merged.
- #8841 was explicitly closed by its author on 2023-07-20 with: `I honestly don't think I'm coming back to this any time soon. I'm gonna close this to clean up the queue.`
- The dependent #8812 remained unmerged and was closed in 2025; the current product still keeps the parser in TerminalApp.
- Disposition: **NO-PORT / explicitly abandoned and contradicted by current project boundary**.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 026 deletion set

- `dev/migrie/f/execute-commandlines`
- `dev/migrie/f/remote-commandlines`
- `dev/migrie/f/name-windows`
- `dev/migrie/f/process-model-v3-test-0`
- `dev/migrie/f/commandline-lib`
- `dev/migrie/r/commandline-lib-002`
