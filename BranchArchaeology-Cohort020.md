# Branch Archaeology — Cohort 020

This cohort closes the last known numbered `32-*` reflow experiments in `dev/migrie/b/**` by tracing them to the definitive fix for `#32` and `#12567`.

## Root contracts

- `microsoft/terminal#32`: colored regions must retain the correct extent/attributes when the console buffer is reflowed during resize; colored trailing blank cells must not be flattened into the wrong color.
- `microsoft/terminal#12567`: after a full-screen `color 2f`-style attribute change, resizing must not reset apparently empty rows/regions back to the default black attributes.
- Definitive upstream fix: PR `microsoft/terminal#12637`, `Manually copy trailing attributes on a resize`, merged as `855e1360c0ff810decf862f1d90e15b5f49e7bbd`, closes both `#32` and `#12567` and adds resize/reflow validation around trailing attributes and full-buffer color state.

## `dev/migrie/b/32-attempt-3`

- Functional line includes `200491671b24ef97393379e4da77f47ffa1601f6` (`try using iterators. Performance seems very bad`), `fd1eda2dbdeb801227897ff679495a936e3a4059` (`this seems like it fixes it but the core UTs are broken...`), and `998916d9a9c37a012c8cb46f4fcec1b0c8dafafd` (`guess what, this did work`); later tip movement is spelling maintenance.
- Technical direction: retain the existing `MeasureRight` text boundary, then continue copying the attribute iterator across trailing cells after printable text. The final experiment also corrected the expected post-resize attributes in the ConPTY roundtrip test.
- Provenance certification: GitHub associates commit `998916d9...` directly with PR `#12637`. The PR ultimately lived on the sibling branch `dev/migrie/b/32-attempt-2`, but this attempt was part of the same successful implementation lineage rather than an independent unmerged product fix.
- Definitive design in `#12637`: explicitly walk/copy trailing attributes after printable text and copy attribute rows below the last printable character; also fix current attributes being applied to the new buffer rather than the old buffer.
- Disposition: **ALREADY ABSORBED / implementation iteration of #12637**.
- Recovery value: provenance only; the durable behavior and tests entered upstream through #12637.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/32-but-im-here-for-12567`

- Functional commits: `444f7df0daf2f1518800bab3e0baba6330dd0747` (`Tests, before we try anything too serious`) and `a8d336416a2be63e25c21fbb4ba51785fd7894fd`; later tip movement is spelling maintenance.
- Experimental direction: introduce `ROW::MeasureRightIsh()` that treated visually non-default blank-space attributes as part of the meaningful row extent, then use that extended boundary during reflow in an attempt to solve both #32 and #12567 with one generalized definition of the row's right edge.
- The branch itself is decisive negative evidence. The implementation commit says the idea did not work, produced strange wrapping because too much of the row was copied, and its test did not pass.
- No PR is associated with the failed commit `a8d336...`.
- The eventual `#12637` solution deliberately chose a different separation of concerns: keep the printable-text boundary and copy trailing attributes separately, rather than redefining text extent around colored blank cells.
- Disposition: **NO-PORT / rejected alternative; contracts solved by #12637**.
- Recovery value: preserve the negative design lesson — colored blank cells are attribute state to preserve during reflow, not evidence that the printable text extent should be extended.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 020 deletion set

- `dev/migrie/b/32-attempt-3`
- `dev/migrie/b/32-but-im-here-for-12567`
