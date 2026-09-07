# Late-2020 / 2021 branch archaeology

This supplement records residual historical branches from late 2020 and 2021 discovered after the earlier chronological cohorts were retired.

## `dev/migrie/fix-pr-7015`

- Functional commit: `9a77620899452ad89c1c2d594ab598af37d849f2` (2020-11-20), “This fixes the build for #7015”; later tip commits are spelling maintenance.
- Historical intent: make caption controls visually dim when the Terminal window loses non-client focus, building on external PR `microsoft/terminal#7015`.
- Historical maturity: repair/integration branch for a PR that was eventually closed without merge.
- Modern reading: the required focus signal exists in the modern `NonClientIslandWindow` path via `WM_NCACTIVATE` -> titlebar `Focused(activated)`. The user-visible inactive-caption behavior was later shipped definitively in `microsoft/terminal#19668` (`8d94edd7b0445aa2221b1d1c7576b8b9dfd4797c`, 2026-01-20), “Dim the caption buttons for unfocused windows”, including high-contrast handling and validation.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none requiring this build-fix branch; the modern implementation is the authoritative behavior.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/titlebar-shenannigans`

- Functional commit: `1644a22bd3b36a30eb33cc4134f63223625d3944` (2021-07-27); later tip is spelling maintenance.
- Historical intent: investigate `microsoft/terminal#9443`, requiring `WM_NCHITTEST` to expose caption-button hit targets such as `HTMAXBUTTON` for accessibility and Windows snap layouts.
- Historical maturity: explicitly exploratory; the commit says the problem was much larger than expected.
- Modern reading: upstream solved the scenario in `microsoft/terminal#11680` (`f2ebb21bd13b20db38305136d34fa0778baf7920`, 2021-11-29), “Add snap-layouts support to the Terminal”. That implementation deliberately reworked caption-button hit-testing/input behavior, returned `HTMAXBUTTON` where needed, restored hover/press/tooltips, validated the result, and closes `#9443`.
- Disposition: **NO-PORT / superseded**.
- Recovery value: provenance only; `#11680` is the successful architectural successor.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/5ff9a24-and-75e2b5f`

- Functional integration commit: `2596890212922518916f75bd079e4c64a3b9fc69` (2021-09-28); later tip is spelling maintenance.
- Historical intent: try to combine the window-layout persistence lineage (`microsoft/terminal#11083`) with the elevated-state lineage (`microsoft/terminal#11222`).
- Historical maturity: explicitly discarded. The commit is titled “Failed attempt to merge #11083 into #11222” and records that the attempted combination was effectively impossible/unwanted, that the locking approach should be discarded, and that a different state architecture should be used instead.
- Modern reading: `#11083` merged independently as `75e2b5fae7e2f7caef3c634710cf7aced14ffd98`; `#11222` also merged independently, as `c79334ffbbcbb0d0483579035328aebd27f517f4`. The synthetic conflict-resolution experiment was never intended to become a third canonical product lineage.
- Disposition: **NO-PORT / abandoned integration experiment**.
- Recovery value: none beyond the recorded architectural lesson: do not revive the attempted locking/conflict-resolution combination.
- Branch retirement: **SAFE after this ledger commit**.

## Deletion set

Once this file is present on `dev/migrie/main`, the following refs no longer carry unique knowledge requiring preservation as branches:

- `dev/migrie/fix-pr-7015`
- `dev/migrie/titlebar-shenannigans`
- `dev/migrie/5ff9a24-and-75e2b5f`
