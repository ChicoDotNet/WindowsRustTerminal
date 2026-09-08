# Branch archaeology — Process Model 3 staging / Darkening of Valinor

## Scope

This note resolves the surviving high-ahead branch:

- `dev/migrie/of-the-darkening-of-valinor`

The branch initially appears to be a large independent body of work (`~207` commits ahead of modern `main`). That count is misleading. Its genealogy places it inside the Process Model 3 staging tree, and its branch-specific tip intent was later rewritten as a small clean PR and merged upstream.

## Decision

**Classification: NO-PORT / DOCUMENT — SAFE TO DELETE once this note is integrated into `dev/migrie/main`.**

The useful information is:

1. this branch is a descendant/staging snapshot of the `quenta-silmarillion` Process Model 3 integration work;
2. Process Model 3 was subsequently split into the reviewed PR sequence tracked by `microsoft/terminal#14957`;
3. the branch-specific tip fixes a `_quake` startup regression introduced by Process Model 3;
4. that fix was cleaned up, reviewed, and merged as `microsoft/terminal#15030`.

No historical code from this ref needs to be recovered separately.

## Genealogy

The current tip is:

- `9179992d50eaa852e3a02a2a3cfeda73bf14ec5b`
- `Fix wt -w _quake by not throwing when setting the window name`

Its parent is `cf89ce36258a48de14afad20ad897284c4eee773`, whose commit message is:

> Merge remote-tracking branch 'origin/main' into dev/migrie/oop/3/quenta-silmarillion

That parent in turn follows integration merges such as:

- `Merge branch 'dev/migrie/oop/3/valaquenta' into dev/migrie/oop/3/quenta-silmarillion`

The semantic merge-base with current history is `f3a722e0e92cb4731ff5be57c4300e21b652bebe`, the merged PR `#14851` (`Introduce a ContentManager helper`). The apparent 207-ahead count therefore mostly represents an old Process Model 3 integration graph plus then-current `main`, not 207 independent changes unique to this ref.

## Upstream architectural destination

Issue `microsoft/terminal#14957` is the Process Model v3 / tab tear-out ship list. It explicitly identifies the original production PR sequence required to ship `#5000 / #1256`:

- `#14825`
- `#14843`
- `#14851`
- `#14866`
- `#14901`
- `#14935`

All six original PRs are checked complete in that tracker.

The same tracker explicitly lists the blocker:

- ``wt -w _quake` is broken?`

and later marks the Process Model 3 ship work as completed while retaining only separate follow-up/backlog items.

This means the large integration ancestry of `of-the-darkening-of-valinor` is not a missing alternative architecture. It is provenance for the staged Process Model 3 rollout whose reviewed form is already represented by those merged PRs and the existing Process Model archaeology documents.

## The branch-specific tip

Commit `9179992d...` changes `TerminalWindow.cpp` so that initializing a window name before XAML is ready does not construct/raise `PropertyChangedEventArgs` through an invalid UI path. It also contains one irrelevant blank-line change in `AppHost.cpp`.

The same functional patch was immediately rewritten onto a clean `main` base for upstream review:

- PR `microsoft/terminal#15030`
- title: `Fix wt -w _quake by not throwing when setting the window name`
- PR head: `dev/migrie/b/of-the-darkening-of-valinor`
- clean feature commit: `99beb1659f23db801955cf6cd30feb71f11ba493`
- follow-up mechanical commit: `b1f9c03d17e3c5835d4c21231a103c24cae40e8a` (`thanks VS`)
- merged commit: `b34444f40a311423bb1ac3e418d8a3e8d83f0d0f`
- merged 2023-03-24

PR #15030 explicitly says the regression came from `#14843` and is related to `#5000` / `#14957`.

The reviewed diff is effectively the branch-tip fix without the enormous integration ancestry. That clean PR is therefore the authoritative implementation and recovery path.

## Durable lesson

This branch is a textbook example of why historical `ahead_by` is only a sensor:

- apparent delta against modern `main`: ~207 commits;
- semantic family: Process Model 3 integration staging;
- branch-specific intent: one small `_quake` startup regression fix;
- authoritative successor: merged PR #15030.

When a legacy branch mixes an integration graph with a small tip fix, identify the clean PR rewrite rather than treating the full graph as provenance that must be retained.

## Classification

| Branch | Classification | Evidence | Disposition |
| --- | --- | --- | --- |
| `dev/migrie/of-the-darkening-of-valinor` | **NO-PORT / DOCUMENT** | PM3 staging ancestry is represented by #14825/#14843/#14851/#14866/#14901/#14935; tip fix was rewritten and merged in #15030 | **SAFE TO DELETE** |

## References

- `microsoft/terminal#5000` — Process Model 3 tracker / historical architecture
- `microsoft/terminal#14957` — Process Model v3 / Tab tear-out ship list
- `microsoft/terminal#15030` — clean merged `_quake` fix
- local historical tip `9179992d50eaa852e3a02a2a3cfeda73bf14ec5b`
- clean PR commit `99beb1659f23db801955cf6cd30feb71f11ba493`
- merged PR commit `b34444f40a311423bb1ac3e418d8a3e8d83f0d0f`
