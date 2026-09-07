# Branch Archaeology — Cohort 021

This cohort begins the surviving `dev/migrie/f/**` namespace with one older CI experiment whose objective is now permanently represented by modern CI.

## `dev/migrie/f/get-localtests-in-ci`

- Functional work dates to 2020-05-20; later tip movement is spelling maintenance.
- The branch is an early attempt to make TerminalApp LocalTests runnable in Azure Pipelines. It experimented with bundling the VC runtime into the LocalTests TestHostApp, manipulating package dependencies, and then corrected the pipeline test search path from `$(Configuration)` to `$(BuildConfiguration)` in commit `774570dabaaee6cfe0b7b0ea3a592926c28a4531`.
- The experimentation history is visibly exploratory: commit `31f09fe12d916608e265a355760a0955fd4607f0` (`hey who knows, maybe this will work`) was explicitly reverted before subsequent adjustments.
- Modern contract replay: Cohort 018 already certified that current `build/pipelines/ci.yml` retains AppX dependencies specifically because LocalTests require them, while current `templates-v2/job-test-project.yml` installs those dependencies and executes `*LocalTests*.dll` for supported architectures, then converts/publishes the results.
- A later `dev/migrie/b/localtests-ci-2022` experiment independently pursued the same goal and was also classified as absorbed by the permanent modern CI sensor.
- Disposition: **NO-PORT / obsolete early CI experiment; objective fully productized**.
- Recovery value: provenance only. Do not resurrect the 2020 VC-runtime/package-dependency hacks or old Azure task layout.
- Branch retirement: **SAFE after this ledger commit**.

## Deferred sibling branches

The following were deliberately **not** included in this deletion set:

- `dev/migrie/f/disable-nesting`
- `dev/migrie/f/filter-weight-input-too`

Both touch Suggestions/snippets search behavior that is still represented in the current Suggestions UI specification. Their old implementation may no longer fit the current architecture, but the intent is not yet proven obsolete. They require explicit PORT/TRANSFORM/NO-PORT classification before branch retirement.

## Cohort 021 deletion set

- `dev/migrie/f/get-localtests-in-ci`
