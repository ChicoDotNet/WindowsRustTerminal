# Dustin Howett branch archaeology — Cohort 006

This supplemental ledger entry continues `BranchArchaeology.md` on `dev/duhowett/main` and preserves deletion provenance for the branch below.

## `dev/duhowett/i-figured-out-why-sometimes-the-publish-build-failed`

- Sole branch-specific commit: `e512a7b1e5cd9cb0f36aabb6f05fcaf666ffae90` (2024-04-18), `Make Publish depend on both Build and Package`.
- The branch is exactly one commit ahead of its historical merge base and changes one line in `build/pipelines/templates-v2/pipeline-onebranch-full-release-build.yml`.
- Intent: prevent intermittent publish-stage failures by making `Publish` wait for the packaging stage instead of depending only on `Build`.
- Current `main` contains the maintained form of exactly that dependency: `Publish` depends on `Build` and conditionally also on `Package` whenever Terminal, ConPTY, or WPF packaging is requested.
- The current implementation is strictly more precise than the prototype because package dependency is omitted when no package-producing target is enabled.
- Classification: **ALREADY ABSORBED / refined in maintained OneBranch pipeline**.
- Disposition: **SAFE TO DELETE**.

## Cohort 006 result

Safe ref:

- `dev/duhowett/i-figured-out-why-sometimes-the-publish-build-failed`
