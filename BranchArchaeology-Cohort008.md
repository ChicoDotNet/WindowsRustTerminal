# Dustin Howett branch archaeology — Cohort 008

This supplemental ledger entry continues `BranchArchaeology.md` on `dev/duhowett/main`. It records a 2020 renderer/WPF stabilization snapshot, a 2021 text-buffer architecture family, and one still-live renderer contract that must remain recoverable.

## `dev/duhowett/wpf-renderer-revert`

- Tip: `06001e4acc405a282e06203dbdd7bfd37eec6116` (2020-02-28), `fix mouse events in the wpf control (#4720)`.
- The branch is four commits ahead of its historical base. It is not a coherent independent feature; it is a temporary stabilization snapshot:
  1. revert PR #4731 / merge commit `d7ea526c3c950b4342c0c204d946174c5871ffe4` (avoid splitting surrogate pairs while breaking runs for scaling),
  2. revert PR #4671 / merge commit `671110c88a13c6de2371a9ec001ae4c6f581d241` (clip text to the expected row),
  3. revert PR #4668 / merge commit `4420950337ccdd7bb5bb66d84538d2ea4195c101` (restrict run-height adjustment and fix trailing half of fullwidth glyph rendering),
  4. cherry-pick the WPF mouse-input fix from PR #4720 / merge commit `4393fefb7130c5f54bc7dd8bfed97f82afb4e98c`.
- PR #4548 had added UIA support to the WPF control and introduced an extra HWND layer; #4720 moved mouse handling to the native control because that indirection broke mouse-button input.
- All four authoritative upstream PRs (#4668, #4671, #4720, #4731) were merged. The local branch existed to combine the mouse fix with temporary local renderer reverts; those reverts were never the upstream product line.
- Recovery rule: for WPF input use the maintained WPF/native input path descending from #4548/#4720. For rendering correctness use current renderer/grapheme tests; do not recreate this bundle of 2020 reverts.
- Classification: **NO-PORT / temporary stabilization snapshot superseded by upstream history**.
- Disposition: **SAFE TO DELETE**.

## 2021 ROW/text-buffer family

### Genealogy

`dev/duhowett/hax/no-writable-glyphat` and `dev/duhowett/hax/rle-row` share the internal ancestor `227ce8ff205579ea72c45659ec3bf9dfc4908994` (`beef prevention`). They are therefore two experiments from the same program, not unrelated branches.

- `hax/no-writable-glyphat` stops after functional commit `17068f44602c481341b3171e6b47e547d0f34828`, `HAX: high water mark, remove Copy functions, fix reflow`, followed by a spelling migration.
- `hax/rle-row` continues on the other side of that fork through a long architecture experiment: `Rewrite Reflow`, `Kill GlyphAt (writable and const)`, `HAX: rle-based row`, `port to Leonard's RLE`, removal of legacy OCI/row sources, writer/template experiments, damage tracking, and buffer/reflow fixes. Its final functional checkpoint is explicitly unfinished (`Writing CHAR_INFO doesn't work`, `RowImage is half-baked`, duplicated algorithms in `Row.hpp`); the tip then received spelling migration noise.
- No upstream PR was found whose head is `dev/duhowett/hax/rle-row`.

### What the experiment was trying to learn

The family tested a broad simplification of the legacy text-buffer representation:

- stop exposing mutable glyph storage through `GlyphAt` / writable row internals;
- collapse `CharRow`/`CharRowCell`/`UnicodeStorage` layers into `ROW`;
- make reflow and writes operate on a more explicit row abstraction;
- use run-length encoding where repeated row state makes it profitable;
- remove legacy Output Cell Infrastructure paths that forced duplicate representations and copy operations;
- track changed/damaged ranges explicitly instead of scattering offset+length conventions.

The exact `rle-row` representation was experimental and incomplete; it should not be revived as a patch series.

### Modern successor state

Current `main` has absorbed the important architectural direction while choosing a different final representation:

- `CharRow.hpp` and `UnicodeStorage.hpp` no longer exist.
- `ROW::GlyphAt(...)` is read-only (`std::wstring_view ... const noexcept`); writable glyph access did not survive.
- `ROW` owns text and column mapping directly through compact character/offset storage rather than the prototype's RLE text representation.
- Repeated attributes are represented with `til::small_rle<TextAttribute, ...>` (`RowAttributes`).
- The modern `ROW` API exposes explicit write/copy state (`RowWriteState`, `RowCopyTextFromState`) and dirty begin/end ranges.
- Later maintained work depends on this chosen representation. PR #15858 (merged 2023-08-24) introduced `CharToColumnMapper` and mutation-aware search on top of `_charOffsets`; PR #16916 (merged 2024-06-26) extended `ROW::WriteHelper` for grapheme clusters. These are maintained successors and make the 2021 RLE-text prototype non-authoritative.

### `dev/duhowett/hax/no-writable-glyphat`

- Unique useful intent: eliminate writable low-level glyph access and make reflow/copy logic operate through explicit row APIs.
- Current `main` has that direction: only const `GlyphAt` remains and row mutation goes through explicit operations.
- Classification: **ALREADY ABSORBED / architectural direction present in modern ROW**.
- Disposition: **SAFE TO DELETE**.

### `dev/duhowett/hax/rle-row`

- Unique useful knowledge is the architectural experiment above, not its unfinished implementation.
- Modern `main` adopted the major simplifications (ROW ownership, no CharRow/UnicodeStorage, no writable GlyphAt, RLE for repeated attributes, explicit dirty/write state) while deliberately using compact chars/offsets for text. Subsequent ICU and grapheme-cluster work built on that maintained representation.
- If row storage is revisited, replay the performance/behavior contracts rather than cherry-picking this branch: reflow equivalence, wide-glyph boundaries, CHAR_INFO compatibility, row reuse/wrap correctness, memory footprint, mutation/search mapping, and grapheme-cluster correctness.
- Classification: **NO-PORT / superseded architecture experiment; knowledge preserved as recovery guidance**.
- Disposition: **SAFE TO DELETE**.

## `dev/duhowett/hax/punchout`

- Functional commit: `cc7091af64a1907d86e3c01901b008a3d34e9f23` (2020-07-27), `When rendering default bg in reverse over transparency, punch text out`; later tip work is spelling migration.
- It implements a special Direct2D geometry path for the case where transparency + default background + reverse video turns the effective foreground transparent. Instead of drawing an opaque/black glyph, the renderer punches the glyph shape out so acrylic/transparency remains visible.
- The commit explicitly says `Fixes #7014`.
- Upstream issue #7014, `Transparent background doesn't become transparent foreground when rendered in reverse video`, is still **OPEN** in 2026. Its contract remains: with default background (SGR 49) and reverse video (SGR 7), the foreground that semantically comes from the transparent default background must remain transparent; it must not become a black square.
- The old `CustomTextRenderer` implementation is not suitable for direct cherry-pick into the modern renderer, but it is valuable evidence for a still-unresolved behavior.
- Recovery: reproduce #7014 on modern rendering paths (Atlas/Direct2D as applicable), encode the VT/transparency/reverse-video behavior as a focused regression test, then implement the smallest modern fix. Use the old geometry-punchout code only as design evidence.
- Classification: **CONSERVAR / unresolved recovery reference**.
- Disposition: **DO NOT DELETE**.

## Cohort 008 result

Safe refs:

- `dev/duhowett/wpf-renderer-revert`
- `dev/duhowett/hax/no-writable-glyphat`
- `dev/duhowett/hax/rle-row`

Explicitly retained:

- `dev/duhowett/hax/punchout` — upstream #7014 remains open; unique attempted implementation for a live rendering contract.
- `dev/duhowett/main` — permanent consolidation lane.
