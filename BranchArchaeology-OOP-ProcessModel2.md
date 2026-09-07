# Branch Archaeology — Process Model 2 / OOP generations

## Decision

The first and second OOP generations are **NO-PORT / superseded by Process Model v3** and their refs are safe to retire after this ledger is integrated.

The decisive upstream record is microsoft/terminal#5000. Its historical notes map the original Process Model 2 work across `dev/migrie/oop/**`: `the-whole-thing` → `infinity-war` → `oop/2/infinity-war` / PR #12938 for the content-process design, `wandavision` for cross-window pane movement, and `oop/2/loki` for tab reattachment. The same issue records the 2023 architectural pivot: Process Model 2, where each TermControl had its own content process, was made obsolete by Process Model v3, where all Terminal windows live in one process.

PR #12938 (`dev/migrie/oop/2/infinity-war`) was the reviewed WIP culmination of the content-process approach and closed without merge. Its remaining safety/lifetime TODOs therefore belong to an architecture that was intentionally abandoned rather than unfinished production work.

Process Model v3 then shipped through the reviewed chain beginning with #14825 and #14843. #14843 introduced the single-process `WindowEmperor` / per-window-thread model. #14851 introduced the production `ContentManager`, and #14866 implemented moving panes and tabs between windows using that architecture. The Process Model v3 ship tracker (#14957) records the original PR sequence as completed.

## Generation 1 refs — safe to retire

- `dev/migrie/oop/the-whole-thing`
  - Early integration/proof of the OOP TermControl/content-process design.
  - #5000 explicitly records this as an earlier home of work later moved to `oop/infinity-war`.
- `dev/migrie/oop/infinity-war`
  - Later Process Model 2 content-process integration line; #5000 records it as the successor to `the-whole-thing`.
- `dev/migrie/oop/connection-factory`
  - Pre-1.10 work to move connection creation across the projected/OOP core boundary; #5000 preserves its purpose and follow-on PR lineage (#10023/#10051/#10067 and related work).
- `dev/migrie/oop/endgame`
  - Process Model 2 integration/staging line. Its architecture was replaced by the 2023 v3 pivot.
- `dev/migrie/oop/wandavision`
  - Prototype for moving panes/tabs between Terminal windows. The behavior was subsequently implemented in the v3 architecture by merged PR #14866.

## Generation 2 refs — safe to retire

- `dev/migrie/oop/2/infinity-war`
  - Exact head of PR #12938 (`e52e720e...`), closed without merge.
  - Canonical reviewed snapshot of the abandoned content-process approach.
- `dev/migrie/oop/2/endgame`
  - Later Process Model 2 staging/integration work. Still built around the abandoned `ContentProcess` architecture.
- `dev/migrie/oop/2/wandavision`
  - Iteration of cross-window content movement under Process Model 2; superseded by #14866.
- `dev/migrie/oop/2/loki`
  - Tab reattachment iteration explicitly tracked in #5000; superseded by the v3 move/attach plumbing.
- `dev/migrie/oop/2/COM-ISwapChainProvider-attempt-1`
  - Technical experiment supporting cross-process rendering/swap-chain ownership. Process Model v3 removed the architectural need for a TermControl-per-content-process boundary; retain the experiment only as historical provenance through this ledger/#5000.

## Retained / investigate separately

- `dev/migrie/oop/3/of-the-silmarils`
  - **Do not delete yet.** It descends directly from merged Process Model v3 heads #14825 and #14843, but diverges from the #14866 head and still carries its own post-v3 staging commits. Those commits need a separate replay against #14851/#14866/#14901/#14935 before retirement.
- Other `dev/migrie/oop-*` scratch/RPC/broker/mixed-elevation branches are not covered automatically by this disposition. Some predate the named generations or represent separate technical hypotheses and will be handled independently.

## Recovery guidance

If historical Process Model 2 reasoning is ever needed, use microsoft/terminal#5000 and PR #12938 as the authoritative narrative rather than resurrecting these stale branches. For the shipped architecture, follow #14825 → #14843 → #14851 → #14866 and the #14957 ship tracker.
