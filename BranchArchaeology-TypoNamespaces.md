# Typo / stray namespace archaeology

This supplement records historical branches whose namespace does not represent a distinct contributor lane and therefore should not cause creation of a new `dev/<user>/main`.

## `dev/mgirie/b/more-nchhittest-ideas`

- Namespace reading: `mgirie` is a historical typo for Mike Griese / `migrie`, not a distinct contributor.
- Functional period: August 2021. Later tip maintenance includes the usual spelling migration.
- Historical intent: continue experimenting with non-client hit testing and the drag-bar window around `microsoft/terminal#9443`, including attempts to move/re-parent the drag window and observe whether XAML stopped stealing mouse messages.
- Historical maturity: explicitly exploratory. The commit history contains attempt/revert pairs; for example `dfcdefd...` says making the drag rectangle not a child "did nothing", and `9714bf5...` immediately reverts that experiment.
- Existing curated lineage: `dev/migrie/main` already records the related `dev/migrie/titlebar-shenannigans` investigation and its successful successor.
- Modern reading: `microsoft/terminal#11680` (`f2ebb21bd13b20db38305136d34fa0778baf7920`) is the definitive implementation. It reworks caption-button hit testing/input, returns `HTMAXBUTTON` for Snap Layouts, handles the drag-bar/XAML-input interaction deliberately, restores hover/press/tooltips, and closes `#9443`.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond provenance; the failed alternatives are useful only as evidence of paths already disproven.
- Branch retirement: **SAFE after this ledger commit reaches `dev/migrie/main`**.

## Namespace retirement

After this supplement is integrated, the entire stray namespace can be removed:

- `dev/mgirie/b/more-nchhittest-ideas`

Do not create `dev/mgirie/main`; the canonical contributor lane is `dev/migrie/main`.
