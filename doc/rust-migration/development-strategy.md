# Rust migration development strategy

This document defines the repeatable development loop for the Rust migration. It is intentionally independent of any one pull request so that changing branches or opening a continuation PR does not silently change the way the migration is developed or certified.

## Pull-request lifecycle

Active migration PRs stay **Draft while functionality is being written**.

Draft is a development mode, not a relaxation of product correctness:

- `rust-ci.yml` runs `cargo fmt --all` before the fast checks.
- the quality job runs `cargo clippy --fix --workspace --all-targets --locked --allow-dirty --allow-staged -- -D warnings` in incremental/Draft mode;
- Linux and Windows test jobs run rustfmt before `cargo check` and `cargo test`;
- the R08 `-RequireZero` functional-debt exit gate is reserved for an integration-ready PR;
- repository spelling is not paid on every synchronize event and is restored by `ready_for_review`.

These corrective CI steps operate in the runner checkout. They prevent mechanical fmt/Clippy drift from obscuring functional feedback during rapid development, but they do not rewrite the source branch. Before a PR becomes integration-ready, any remaining mechanical diff must be folded into the current functional increment or the final certification change rather than creating avoidable one-off repair history.

Syntax errors, type errors, failing tests and semantic regressions are not mechanical drift and cannot be waived by the preflight. They are corrected as soon as authoritative CI identifies them, preferably inside the next coherent functional increment when that preserves reviewability.

A migration PR is marked **Ready for review only when it is an exit candidate**. That transition intentionally restores the strict integration gates: rustfmt verification, Clippy with `-D warnings`, spelling, zero functional debt where required, boundary/stage contracts and final certification appropriate to the phase.

## Spelling policy

Spelling is a certification gate, not a per-synchronize development tax.

- cancellation, superseded runs and runner/infrastructure failures are not lexical failures;
- a genuine typo is fixed in source;
- a genuine new domain/API/product term is added to the appropriate `.github/actions/spelling/allow/*.txt` dictionary (or the equivalent check-spelling metadata) rather than repeatedly failing the build;
- `ready_for_review` must obtain a fresh successful spelling result before integration.

The goal is to keep spelling authoritative without making accepted technical vocabulary a recurring source of noise.

## Modified-sandwich strategy

Development alternates between **Missing** and **Partial** Microsoft contracts while staying on the same functional neighborhood whenever possible.

The purpose is end-to-end coverage, not score manipulation. A typical cycle is:

```text
Missing contract
    -> establish or extend the Rust product owner
    -> adjacent Partial contract
    -> harden the same owner/boundary
    -> adjacent Missing contract
    -> continue the vertical slice
```

Selection rules:

1. Prefer contracts that extend an owner or boundary touched by the previous increment.
2. Alternate Missing and functional Partial work when an adjacent candidate exists; do not exhaust an easy Missing list while leaving the same end-to-end path Partial.
3. Favor vertical behavior that crosses parsing/model/fixup/projection boundaries over isolated test-count wins.
4. Promote a contract only with a real Rust owner or a direct witness proving that the existing Rust owner already satisfies it.
5. Never reclassify metadata merely to reduce Missing or Partial totals.
6. A surviving Partial must have explicit remaining behavior. Platform/language/API-shape/upstream-ignored boundaries are recorded as such rather than disguised as functional completion.

This is the migration "sandwich": new Rust ownership grows from one side while nearby partial ownership is hardened from the other, repeatedly closing complete functional slices instead of creating a wide front of disconnected ports.

## Commit and PR discipline

- `rust/main` is the compact migration baseline (`Initial rust migration effort`).
- Each functional increment should be one self-contained reviewable commit whenever practical.
- Mechanical API-edit commits may be used while constructing an increment, but are squashed before the increment is considered complete.
- The active PR description is a living Microsoft-style review artifact: Summary, References, Detailed Description, Validation Steps and Checklist are updated with every increment.
- CI queued/pending is a writing window: continue safe adjacent increments. A real failure with authoritative logs takes priority over widening the branch.
- Do not merge, start the next migration phase or mark Ready for review merely because individual increments are green.

## R08 exit discipline

R08 is not exit-ready until known functional debt has been eliminated or explicitly proven to be a genuine non-Rust/product boundary. The final integration-ready head must pass the strict gates after development mode is turned off. Draft-mode speed is therefore a way to shorten the feedback loop, not a substitute for the final proof.