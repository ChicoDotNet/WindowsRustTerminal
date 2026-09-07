# Branch Archaeology — Cohort 015

## `dev/migrie/b/1.12-crash-on-exit`

This branch was missed by the issue-number-oriented passes because its name refers to the Terminal 1.12 era rather than a GitHub issue.

- Functional commit: `48b20de4f4279cdffea710fcd1b456cf6c0766af`.
- Commit message: `Add some logging. Can't seem to get the crash to repro?`
- Tip is only the later spelling migration `76d76b1fce57e7e492496acb60afd865af674f64`.
- The branch does not contain a demonstrated crash fix. It adds diagnostic instrumentation around:
  - `ApplicationState::~ApplicationState()` / `_throttler.flush()`;
  - collecting and writing persisted window layouts;
  - throttled layout-save requests;
  - a catch/log path around layout collection/save.
- There is no reliable evidence tying this 2021 diagnostic probe to later, separately diagnosed crash-on-exit fixes such as the XAML/Application lifetime work. Those later incidents therefore are **not** used as false successor evidence here.
- Durable knowledge: the historical hypothesis was that shutdown might be failing while flushing application state or persisting window layouts. The author could not reproduce the crash on this branch.
- Disposition: **NO-PORT / diagnostic probe only**.
- Recovery value: if a future shutdown crash points at persisted-layout or `ApplicationState` teardown, the exact old instrumentation remains identified by commit SHA above; do not carry this logging wholesale into current code without a current reproduction.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 015 deletion set

- `dev/migrie/b/1.12-crash-on-exit`
