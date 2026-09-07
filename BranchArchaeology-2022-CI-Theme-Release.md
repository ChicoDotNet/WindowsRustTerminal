# 2022 CI, theming, and release archaeology

This supplement records three 2022 branches whose intent is now represented by later product/infrastructure work or by the final release lineage.

## `dev/migrie/localtests-in-ci`

- Functional commits: `3c1b978908db6f0e4b02da6518579a721c56150c` and `ce22d5f47589a04af17be6b18339ff083e9ebe85` (2022-02-11); later tip is spelling maintenance.
- Historical intent: experiment with running Cascadia local-test DLLs in the Azure Pipelines `test-console-ci.yml` path by adding `Run-Tests.ps1` invocations and trying progressively broader filename patterns.
- Historical maturity: explicitly exploratory (`sure lets try it`, `try somethin else`), with ad-hoc duplicate test tasks while discovering which local-test artifacts were runnable in CI.
- Modern reading: the current pipeline architecture has a dedicated test-project job that invokes `Run-Tests.ps1` with `-MatchPattern '*LocalTests*.dll'`. The behavioral goal therefore survived, but in the newer pipeline generation rather than through this prototype.
- Disposition: **NO-PORT / superseded by modern CI architecture**.
- Recovery value: none requiring the old YAML experiment; the current CI job is the authoritative sensor.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/release-1.12-rejuv-attempt-2`

- Historical intent: revive/update the 1.12 release line around March 2022.
- Ancestry check against `release/1.12`: the branch is 42 commits behind and only 1 commit ahead. Its merge base is `e9955368eef2b694c311579e9ef211f11bd69631`, so the functional product work represented by the branch is already in the release lineage.
- The sole branch-only commit is Dustin Howett's mechanical `spelling-0.0.21` migration, not unique 1.12 functionality.
- Disposition: **ALREADY ABSORBED**.
- Recovery value: none; do not preserve a stale release resurrection branch solely for mechanical spelling metadata.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/titebar-colors`

- Functional commits include `cd0012a6b1acf17da40ce08dabd4a8b83e35013a` (2022-04-25) and `ac49459dde07e410594a35c6ec7aed81cad90515` (2022-04-26).
- Historical intent: prototype a titlebar/tab-row background brush, propagate it through `TerminalPage`/`AppLogic`/`AppHost`, and vary accent-derived colors when the window activates/deactivates.
- Historical maturity: exploratory and hard-coded around system accent resources; no upstream PR was created from this branch.
- Modern reading: upstream shipped the supported theme architecture in `microsoft/terminal#12992` (`07d58a800c69f5d39dbdca1612d9db3955fb32b2`, 2022-07-07). That implementation explicitly supports changing the tab-row/titlebar color using `ThemeColor`, including literal colors, `accent`, and `terminalBackground`, plus runtime updates and tests.
- Disposition: **NO-PORT / superseded**.
- Recovery value: provenance only; future titlebar styling belongs in the modern Theme model rather than this direct brush/event plumbing.
- Branch retirement: **SAFE after this ledger commit**.

## Deletion set

Once this file is present on `dev/migrie/main`, these refs no longer carry unique knowledge requiring preservation as branches:

- `dev/migrie/localtests-in-ci`
- `dev/migrie/release-1.12-rejuv-attempt-2`
- `dev/migrie/titebar-colors`
