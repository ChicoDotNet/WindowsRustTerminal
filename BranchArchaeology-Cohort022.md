# Branch Archaeology — Cohort 022

This cohort closes four surviving 2020 `dev/migrie/f/**` branches from the wrapped-line and resize/reflow lineage. They are valuable as historical stepping stones, but their durable behavior and tests were superseded by reviewed upstream implementations.

## `dev/migrie/f/conpty-wrapped-lines-2`

- Historical intent: experiment with preserving wrapped-line state end to end through ConPTY and Terminal, including early regression coverage such as `WriteWrappedLine` and buffer-side verification.
- The branch predates the reviewed implementation and contains intermediate test/renderer changes rather than the final ownership model.
- Definitive successor: upstream PR `microsoft/terminal#4415`, `Make Conpty emit wrapped lines as actually wrapped lines`, merged 2020-02-27 as `e5182fb3e8852bd54f62c2f9b8d3fea903f17807` from `dev/migrie/f/conpty-wrapping-003`.
- PR #4415 explicitly changes ConPTY output to preserve line-wrap state, adds Terminal support for wrapped lines, adds/passes tests, and closes #405/#3367.
- Disposition: **NO-PORT / superseded by #4415**.
- Recovery value: provenance only; use the reviewed wrapped-line contract and later reflow architecture as authority.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/just-wrapping`

- Historical tip: `c21f74029d1186bec69846258ca7bdcb82346964` (2020-01-27), `I think this is all I need to support wrapped lines in the Terminal`.
- Historical intent: narrow the Terminal-side implementation needed to consume and represent wrapped lines while the ConPTY wrapping work was still being assembled.
- The branch is a staging/prototype ancestor of the reviewed wrapping lineage. The final design was not frozen here; the work continued through `dev/migrie/f/conpty-wrapping-003` and PR #4415.
- Definitive successor: PR #4415 / merge `e5182fb3e8852bd54f62c2f9b8d3fea903f17807`.
- Disposition: **NO-PORT / superseded prototype**.
- Recovery value: provenance only; do not resurrect the pre-review staging implementation.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/reflow-buffer-on-resize-002`

- Historical tip: `1470abe1feb1f1398a68a44857ff57183b8c60c1` (2020-03-04), `Switch back to conpty...`.
- The branch records an explicitly unsatisfactory intermediate design: duplicated lines remained and maximize/restore behavior was still poor, with the author planning to step back and try a different approach.
- Historical intent: make terminal-buffer resize preserve/reflow logical wrapped content instead of treating each visible row as independent fixed-width history.
- Definitive successor: upstream PR `microsoft/terminal#4741`, `Add support for 'reflow'ing the Terminal buffer`, merged 2020-03-13 as `93b31f6e3f19ed4f4ffb55fad15be9b889b50f04` from `dev/migrie/f/reflow-buffer-on-resize`.
- PR #4741 explicitly explains that the earlier heavy ConPTY resize approach made the problem worse, carries the large #3490 regression test forward, implements the successful terminal-buffer reflow model, and closes #1465/#3490/#4771.
- Disposition: **NO-PORT / superseded by #4741**.
- Recovery value: negative knowledge only — do not return to the earlier ConPTY-heavy resize strategy.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/resize-quirk`

- Historical tip: `b2463a131f2ec6f5bb02ba74b2bd85a25ef5c940` (2020-03-05), `fix the tests too`; earlier commits document the `method.10`/quirky-resize experiment.
- Historical intent: refine resize semantics after the earlier reflow attempts, especially the interaction between viewport size, buffer size and repeated resize behavior.
- Definitive successor: PR #4741. Its description explicitly documents the March 5 redesign introducing the successful "quirky resize" behavior inside the final reflow architecture and preserving the #3490 stress/regression test.
- This branch is therefore an implementation iteration of the final reviewed reflow lineage rather than unique unfinished product work.
- Disposition: **ALREADY ABSORBED / superseded by final #4741 implementation**.
- Recovery value: provenance only; the reviewed reflow code/tests are authoritative.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 022 deletion set

- `dev/migrie/f/conpty-wrapped-lines-2`
- `dev/migrie/f/just-wrapping`
- `dev/migrie/f/reflow-buffer-on-resize-002`
- `dev/migrie/f/resize-quirk`

Keep `dev/migrie/main` as the durable archaeology lane.
