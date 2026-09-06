# Branch cleanup ledger

This document tracks the retirement of inherited legacy branches from the original `microsoft/terminal` fork and the replacement of useful upstream-maintainer namespaces with a small set of curated tracking lanes.

## Objective

Reduce the repository branch surface until it contains primarily:

- ChicoDotNet-owned development branches;
- the isolated Microsoft synchronization branch `dev/ChicoDotNet`;
- curated maintainer lanes used to revive abandoned or stalled upstream work;
- active Rust migration branches (`rust/*`);
- release lineage (`release-*` and version-line branches used for releases);
- `main` as the eventual Rust product branch after R09.

Microsoft upstream changes are consumed through `dev/ChicoDotNet`. Individual historical upstream developer branches are not part of the long-term repository topology.

## Safety rules

1. Never delete `dev/ChicoDotNet`.
2. Never delete a ChicoDotNet-owned active branch.
3. Never delete the head branch of an open ChicoDotNet pull request.
4. Preserve release branches and version-line branches.
5. Prefer deleting inherited developer refs by namespace instead of evaluating hundreds of branches one by one when the namespace is clearly upstream-owned.
6. Record every cleanup batch here before or together with deletion.
7. Do not merge every historical branch into a maintainer lane. The lanes are fresh baselines for curated recovery work, not archives of every abandoned experiment.

## Current protected working set

- `dev/ChicoDotNet`
- `dev/cazamor/main`
- `dev/duhowett/main`
- `dev/lhecker/main`
- `dev/migrie/main`
- `rust/main`
- `rust/r08-product-integration`
- `rust/r09-product-integration` — head of PR #46
- `release-*`
- release/version-line branches such as `1.17`

## Maintainer consolidation lanes

The preferred canonical shape is `dev/<maintainer>/main`.

A direct branch such as `dev/cazamor` cannot coexist with existing refs such as `dev/cazamor/...`; GitHub rejects that ref shape while child refs remain. `dev/<maintainer>/main` provides the same visual grouping, can be created before legacy deletion, and leaves room for temporary issue branches under the same namespace.

The first consolidated lanes were created from the current `dev/ChicoDotNet` baseline:

| Canonical lane | Historical namespaces absorbed | Purpose |
| --- | --- | --- |
| `dev/cazamor/main` | `dev/cazamor/*` | Curated Carlos Zamora / settings-accessibility work lane |
| `dev/duhowett/main` | `dev/duhowett/*`, `dev/duhowtt/*` | Curated Dustin Howett / core-platform work lane |
| `dev/lhecker/main` | `dev/lhecker/*` | Curated Leonard Hecker / renderer-core work lane |
| `dev/migrie/main` | `dev/migrie/*`, `dev/mgirie/*` | Curated Michael Gries / product-integration work lane |

### Lane workflow

1. Refresh `dev/ChicoDotNet` from a green `microsoft/terminal:main` state.
2. Refresh the relevant `dev/<maintainer>/main` lane from that baseline before beginning recovery work.
3. Select an upstream issue that is open, valuable, and relatively abandoned or stalled.
4. Create a temporary branch such as `dev/<maintainer>/issue-<number>-<slug>` from the maintainer lane.
5. Reuse useful ideas from historical branches selectively; never merge an entire namespace wholesale.
6. If Microsoft ships an equivalent fix first, mark the local effort superseded and retire the temporary branch.
7. If ChicoDotNet completes the issue first, carry it through review/CI and retire the temporary branch after integration.
8. Keep `dev/<maintainer>/main` as the durable lane; issue branches are disposable.

This gives the repository a small set of upstream-observation lanes while `dev/ChicoDotNet` remains the authoritative intake point for Microsoft releases.

## Cleanup batches

### Batch 001 — Microsoft Terminal core developer namespaces

Status: **consolidation lanes created; legacy ref deletion pending**

These are inherited development branches from core Microsoft Terminal maintainers. Their useful future role is now represented by the consolidated lanes above.

| Namespace | Branches identified | Action |
| --- | ---: | --- |
| `dev/cazamor/*` | 84 | Delete legacy refs except `dev/cazamor/main` |
| `dev/duhowett/*` | 88 | Delete legacy refs except `dev/duhowett/main` |
| `dev/duhowtt/*` | 1 | Delete legacy typo ref; fold into `dev/duhowett/main` |
| `dev/lhecker/*` | 69 | Delete legacy refs except `dev/lhecker/main` |
| `dev/migrie/*` | 256 | Delete legacy refs except `dev/migrie/main` |
| `dev/mgirie/*` | 1 | Delete legacy typo ref; fold into `dev/migrie/main` |
| **Legacy refs identified** | **499** | **Pending deletion** |

Notes:

- The 499 count predates creation of the four canonical `/main` lanes; those four new branches are protected and are not part of the deletion count.
- `dev/ChicoDotNet` is intentionally excluded even though it shares the `dev/` prefix.
- A developer branch whose name contains the word `release` (for example `dev/migrie/release-1.12-rejuv-attempt-2`) is still a developer work branch and is not part of the preserved top-level release lineage.
- The fork currently has only one open pull request, PR #46, and it is ours; there are no open Microsoft legacy PRs in this fork to close. The cleanup target is therefore inherited branch refs.

## Next batches

After Batch 001, continue grouping remaining inherited branches by upstream owner/namespace. For maintainers whose historical work is strategically useful, create one `dev/<maintainer>/main` lane before removing their legacy refs. Preserve only ChicoDotNet-owned work, curated maintainer lanes, and release lineage unless a specific exception is documented here.
