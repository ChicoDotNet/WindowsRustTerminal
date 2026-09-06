# Branch cleanup ledger

This document tracks the retirement of inherited legacy branches from the original `microsoft/terminal` fork.

## Objective

Reduce the repository branch surface until it contains primarily:

- ChicoDotNet-owned development branches;
- the isolated Microsoft synchronization branch `dev/ChicoDotNet`;
- active Rust migration branches (`rust/*`);
- release lineage (`release-*` and version-line branches used for releases);
- `main` as the eventual Rust product branch after R09.

Microsoft upstream changes are consumed through `dev/ChicoDotNet`; individual upstream developer branches are not part of the long-term repository topology.

## Safety rules

1. Never delete `dev/ChicoDotNet`.
2. Never delete a ChicoDotNet-owned active branch.
3. Never delete the head branch of an open ChicoDotNet pull request.
4. Preserve release branches and version-line branches.
5. Prefer deleting inherited developer refs by namespace instead of evaluating hundreds of branches one by one when the namespace is clearly upstream-owned.
6. Record every cleanup batch here before or together with deletion.

## Current protected working set

- `dev/ChicoDotNet`
- `rust/main`
- `rust/r08-product-integration`
- `rust/r09-product-integration` — head of PR #46
- `release-*`
- release/version-line branches such as `1.17`

## Cleanup batches

### Batch 001 — Microsoft Terminal core developer namespaces

Status: **identified; ref deletion pending**

These are inherited development branches from core Microsoft Terminal maintainers. They are not needed now that upstream integration is isolated behind `dev/ChicoDotNet`.

| Namespace | Branches identified | Action |
| --- | ---: | --- |
| `dev/cazamor/*` | 84 | Delete inherited refs |
| `dev/duhowett/*` | 88 | Delete inherited refs |
| `dev/duhowtt/*` | 1 | Delete inherited ref (legacy typo namespace) |
| `dev/lhecker/*` | 69 | Delete inherited refs |
| `dev/migrie/*` | 256 | Delete inherited refs |
| `dev/mgirie/*` | 1 | Delete inherited ref (legacy typo namespace) |
| **Total** | **499** | **Pending deletion** |

Notes:

- `dev/ChicoDotNet` is intentionally excluded even though it shares the `dev/` prefix.
- A developer branch whose name contains the word `release` (for example `dev/migrie/release-1.12-rejuv-attempt-2`) is still a developer work branch and is not part of the preserved top-level release lineage.
- The fork currently has only one open pull request, PR #46, and it is ours; there are no open Microsoft legacy PRs in this fork to close. The cleanup target is therefore inherited branch refs.

## Next batches

After Batch 001, continue grouping remaining inherited branches by upstream owner/namespace. Preserve only ChicoDotNet-owned work and release lineage unless a specific exception is documented here.
