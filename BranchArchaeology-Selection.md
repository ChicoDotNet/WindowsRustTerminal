# Dustin Howett — selection archaeology

This file extends `BranchArchaeology.md` for early selection experiments. Completed classifications here supersede older provisional retain notes.

## `dev/duhowett/hax-selection-exclusive`

- Functional commit: `d99293dccf70072b20c59dd93c36cacb3ab19ebe` (2020-03-25), `HAX: try to make selection exclusive`; the commit message explicitly notes unresolved issues around block selection and word pivoting.
- Historical intent: represent selection as an inclusive start plus exclusive end, allow degenerate selections, make mouse selection round to cell boundaries, and update word/line expansion and rendering to respect exclusive endpoints.
- Historical maturity: incomplete exploratory patch. It contains disabled `#if 0` endpoint-conversion code and explicitly records unresolved block/word behavior, so it should not be treated as a shippable contract.
- Definitive successor: `microsoft/terminal#18106` / `64d4fbab17f181c19b502d3170f36f8e16c594b1` (2025-01-28), `Make selection an exclusive range`, performs the same migration comprehensively. It makes selection end-exclusive, enables degenerate selections, rounds mouse selection to the nearest cell boundary, introduces exclusive-bound navigation helpers, modernizes word boundaries, updates renderer/copy/session restore/mark mode/hyperlinks/UIA, and validates block/mouse/multiclick/accessibility scenarios with tests.
- Follow-on consolidation: `microsoft/terminal#19882` / `040c730a444aaf0c2ee5d6e8c2bedf184f700cac` (2026-03-13) simplifies and unifies the word expansion/navigation functions around exclusive end positions, removing the duplicate selection/accessibility word helpers that the 2020 experiment struggled with.
- Classification: **ALREADY ABSORBED / superseded by complete implementation**.
- Recovery value: provenance only. The early branch correctly identified the exclusive-range model and mouse-boundary behavior years before the production migration, but its implementation is intentionally incomplete and should not be replayed.
- This supersedes the provisional `CONSERVAR / selection lineage still requires reconstruction` note in `BranchArchaeology.md`.
- Disposition: **SAFE TO DELETE**.

## Deletion set

- `dev/duhowett/hax-selection-exclusive`
