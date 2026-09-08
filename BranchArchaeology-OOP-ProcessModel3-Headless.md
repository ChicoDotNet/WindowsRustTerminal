# Branch archaeology — Process Model 3 headless lineage

## Scope

This note resolves the surviving high-ahead branch:

- `dev/migrie/oop/3/of-the-silmarils`

It extends the Process Model 2 archaeology in `BranchArchaeology-OOP-ProcessModel2.md`. The goal is not to revive the historical branch mechanically, but to identify its semantic base, isolate genuinely exclusive work, and preserve the recovery path.

## Decision

**Classification: DOCUMENT / TRANSFORM knowledge; source branch is safe to retire after this note is integrated.**

The apparent ~134-145 commits ahead are almost entirely inherited Process Model 3 / `main` history. The branch's unique residual work collapses to one WIP commit implementing an early `wt --headless` contract.

Most of its ancestry already shipped upstream. The remaining command-line contract is still an active product idea and has a modern upstream recovery path under `microsoft/terminal#9996` / PR `#20145`.

Do not port the 2023 WIP code literally. Preserve the behavior and constraints.

## Genealogy and semantic base

The Tolkien-named Process Model 3 branches were staging generations, not independent products. The surviving ref is the integration descendant of earlier branches such as:

- `dev/migrie/oop/3/foreword`
- `dev/migrie/oop/3/ainulindale`
- `dev/migrie/oop/3/quenta-silmarillion`
- `dev/migrie/oop/3/akallabeth`
- `dev/migrie/oop/3/feanor-and-the-unchaining-of-melkor`
- `dev/migrie/oop/3/of-the-silmarils`

Only `dev/migrie/oop/3/of-the-silmarils` still exists in this fork at the time of this archaeology.

The product sequence upstream was split into reviewed PRs:

1. `#14825` — split `AppLogic` into process-wide and per-window responsibilities.
2. `#14843` — Process Model 3 proper: all windows in one process, `WindowEmperor`, one UI thread per window at that time.
3. `#14851` — introduce `ContentManager` and content identities.
4. `#14866` — plumbing for moving panes/tabs between windows.
5. `#14901` — drag tabs between existing windows.
6. `#14935` — tear tabs out into new windows; closes the main `#5000` saga.

The branch looks very large compared with modern `main`, but that is the wrong comparison.

## Decision compression: 134 commits -> one exclusive WIP commit

The critical semantic base is PR `#14944`, **Add support for running the Terminal without any windows**.

Upstream PR `#14944` had head:

- branch: `dev/migrie/oop/3/feanor-and-the-unchaining-of-melkor`
- head commit: `8329b52da069f6c49696a15ff98a67b65aadc06d`
- merged as: `6f8ef58673c77c84a23ece4da47176321ca256cf`

It shipped `compatibility.allowHeadless`, `AppLogic::AllowHeadless()`, and the `WindowEmperor` lifetime rule that allows the process to remain alive after its last window closes while still honoring an explicit Quit.

Comparing that semantic head to `dev/migrie/oop/3/of-the-silmarils` yields:

- `ahead_by = 1`
- `behind_by = 31`
- exactly one exclusive branch commit: `c111b6c7bab9157d21b8c2d75ae2ab64afea29f2` (`WIP`)

That one commit is the only knowledge not already represented by the merged `#14944` lineage.

## What the exclusive `c111b6c` WIP tried to do

The commit prototypes an explicit top-level `wt --headless` mode.

Behavioral intent:

- `wt --headless` starts or converts the Terminal instance into a mode that can remain alive with no visible windows.
- A pure headless invocation must not receive the normal implicit `new-tab` startup action.
- Headless state participates in the `WindowEmperor` process-lifetime decision.
- The command-line mode is distinct from targeting a specific existing window.
- An explicit Quit must still terminate the process.

Implementation sketch in the WIP:

- `AppCommandlineArgs` parses `--headless` and skips default `new-tab` injection.
- `Monarch` attempts to notice `--headless` during remoting handoff and stores a `HeadlessMode` state.
- `WindowEmperor::_windowExitedHandler` considers both the persisted `compatibility.allowHeadless` setting and the transient headless mode.
- help resources describe a background instance that remains available until an explicit Quit.

## Why the old implementation must not be copied

The WIP is visibly unfinished. Examples include:

- the remoting loop tests `args` instead of the iterated `arg` when looking for `--headless`;
- reset logic duplicates `_isHandoffListener = false` instead of clearly resetting all newly added state;
- it couples transient command-line state to `Monarch` in an early Process Model 3 remoting design;
- it does not clearly separate first-process startup behavior from single-instance handoff into an already-running process.

These are evidence of intent, not production-ready code.

## Upstream destination

PR `#14944` deliberately did **not** close `microsoft/terminal#9996`. Its description says the setting is not enough and that some form of `wt --headless` or `wt --hidden` is still needed.

Issue `#9996` remains open and tracks the headless-monarch / globalSummon / quake-mode scenario.

In April 2026, PR `#20145`, **Add --headless CLI flag to start WT without opening a window**, independently reconstructed almost the same contract:

- top-level `--headless`;
- do not inject a default `new-tab` when no startup action is requested;
- skip persisted-layout restoration for a pure headless launch;
- keep `WindowEmperor` alive for global hotkeys;
- normal explicit subcommands still create windows;
- keep `compatibility.allowHeadless` behavior intact.

PR `#20145` was closed without merge by its author with the explicit note that it was being closed to test locally before resubmission. It was not rejected as a product direction.

Its stated limitation is also useful: `wt --headless` handed off to an already-running instance still needs a deliberate semantic decision. The 2023 WIP attempted to propagate that state through `Monarch`, whereas the 2026 MVP explicitly deferred that case.

## Durable contract for future Rust / recovery work

If this capability is implemented in Windows Rust Terminal, preserve these distinctions rather than the historical class layout:

1. **Persistent policy vs launch intent**
   - `allowHeadless` is a persisted policy controlling process lifetime after windows close.
   - `--headless` is transient launch intent controlling whether an initial visible window is created.

2. **Pure headless launch**
   - no implicit `new-tab`;
   - do not restore visible persisted layouts merely because they exist;
   - remain available for global summon/hotkey behavior.

3. **Headless plus explicit action**
   - an explicit user action must not silently disappear; define whether the action wins and opens a window. The 2026 proposal chooses this behavior.

4. **Already-running instance**
   - treat single-instance handoff explicitly. Do not accidentally create a window because the receiving process interprets `--headless` as an ordinary commandline.
   - decide whether `--headless` mutates the running instance's transient lifetime mode or is only valid for initial startup.

5. **Explicit Quit wins**
   - headless policy must never make Quit ineffective.

6. **Isolated mode interaction**
   - keep isolated-instance lifetime semantics independent and explicit rather than mixing them accidentally with global headless state.

## Classification

| Branch | Classification | Reason | Disposition |
| --- | --- | --- | --- |
| `dev/migrie/oop/3/of-the-silmarils` | **DOCUMENT / TRANSFORM** | Process Model 3 ancestry and `allowHeadless` implementation shipped; only unique commit is unfinished `wt --headless` prototype. Contract and modern upstream recovery path are preserved here. | **SAFE TO DELETE after this document lands in `dev/migrie/main`** |

Earlier Tolkien-named `oop/3` refs that no longer exist locally need no resurrection merely for archival purposes: their production lineage is represented by the merged PR sequence above and the surviving integration branch genealogy documented here.

## References

- `microsoft/terminal#5000` — Process Model 3 / tear-out megathread
- `microsoft/terminal#14825`
- `microsoft/terminal#14843`
- `microsoft/terminal#14851`
- `microsoft/terminal#14866`
- `microsoft/terminal#14901`
- `microsoft/terminal#14935`
- `microsoft/terminal#14944` — merged `compatibility.allowHeadless`; historical head `dev/migrie/oop/3/feanor-and-the-unchaining-of-melkor`
- `microsoft/terminal#9996` — still-open headless/globalSummon recovery path
- `microsoft/terminal#20145` — 2026 `--headless` implementation attempt; closed for local validation, not merged
- historical exclusive commit `c111b6c7bab9157d21b8c2d75ae2ab64afea29f2`
