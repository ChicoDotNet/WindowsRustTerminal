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
8. A unique historical commit is not sufficient reason to preserve a branch. Preserve the contract, evidence, benchmark hypothesis, or decision; retire the ref when the knowledge has another durable home.

## Current protected working set

- `dev/ChicoDotNet`
- `dev/cazamor/main`
- `dev/duhowett/main`
- `dev/lhecker/main`
- `dev/migrie/main`
- `dev/miniksa/main`
- `dev/miniksa/issue-987-vt-adapter-test-coverage`
- `rust/main`
- `rust/r08-product-integration`
- `rust/r09-product-integration` — head of PR #46
- `release-*`
- release/version-line branches such as `1.17`

Temporary evidence retention:

- `dev/miniksa/perf_buffer_dig` — retain only until the renderer hot-path allocation hypothesis is represented by a modern performance contract/benchmark.

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
| `dev/miniksa/main` | selected `dev/miniksa/*` archaeology | Curated Michael Niksa / console-core and performance recovery lane |

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

### Batch 002 — `dev/miniksa/*` decision compression

Status: **performance archaeology compressed; additional legacy refs safe to delete; one performance experiment temporarily retained**

The `miniksa` namespace contains a mixture of abandoned issue work, test prototypes, and a July 2020 performance laboratory. The cleanup rule for this namespace is evidence-first: keep current product contracts and useful hypotheses, not historical branch names.

#### Already classified safe to delete

| Branch | Decision | Evidence |
| --- | --- | --- |
| `dev/miniksa/input_tests_2` | Delete | Exact commit alias of retained `dev/miniksa/input2`; no unique history is lost. |
| `dev/miniksa/4254` | Delete | Historical fix is superseded by the broader upstream parameter-limit fix already present in the current lineage. |
| `dev/miniksa/4309` | Delete | Work was merged upstream and its merge is already in the current lineage. |
| `dev/miniksa/ci-987-adapter-contract` | Delete | Disposable CI sensor. Its job was completed after VS2026 compilation and a direct TAEF GREEN for the adapter answerback contract. |

#### Performance archaeology

| Branch | Classification | Decision |
| --- | --- | --- |
| `dev/miniksa/gotta_go_fast_spsc` | **ABSORBED / EVOLVED** | Delete. The current tree contains an evolved `til::spsc` implementation plus durable `SPSCTests` covering API smoke, drop behavior, and integration. The branch is no longer the owner of the idea or its contract. |
| `dev/miniksa/perf_skip_checks` | **SUPERSEDED BY ARCHITECTURE** | Delete. The 2020 patches removed checked lookups from `_storage.at()`, `AttrRow`, and `CharRow`. Current `TextBuffer` no longer uses that storage model: it uses explicit virtual-memory row storage and `_getRowByOffsetDirect()`, with current performance decisions documented in code. Replaying unchecked access patches against the new architecture would be cargo-cult optimization. |
| `dev/miniksa/gotta_go_fast` | **LAB / DECISION SOURCE** | Delete after this ledger commit. It contains mutually exclusive experiments, including changes explicitly measured as not faster and reverted, asynchronous renderer/conpty queue experiments later returned to synchronous operation, cache/invalidation experiments, memory-order experiments, direct-buffer experiments, and unchecked-access experiments. Its durable value is the decision record below, not the branch. |
| `dev/miniksa/perf_buffer_dig` | **OPEN PERFORMANCE HYPOTHESIS** | Retain temporarily. The prototype attempted direct `ROW` access and replacing `Cluster` reconstruction with text + cluster-map data. The current renderer still uses `TextBufferCellIterator` and contains a TODO identifying the reconstruction/allocation path as a performance issue and suggesting an iterator/view adapter. Retire this ref only after a modern benchmark/performance contract captures the hypothesis. |

#### Durable conclusions from `gotta_go_fast*`

1. **Do not preserve rejected experiments as architecture.** The bitmap-size experiment was explicitly marked "NOT FASTER" and reverted; asynchronous renderer/conpty queues were later nerfed back to synchronous behavior.
2. **SPSC synchronization became product knowledge.** The surviving idea is represented by current `til::spsc` code and tests, not by the 2020 experiment branch.
3. **Unchecked hot-path access is not a portable recipe.** The old `at()`-to-`[]` changes targeted a storage model that has since been replaced. Any new bounds-check removal requires a fresh profile and an invariant/contract proving the access safe.
4. **Renderer reinterpretation/allocation remains a legitimate performance question.** The current renderer itself still documents this debt. Preserve it as a benchmark target, not as an instruction to cherry-pick the old prototype.
5. **Performance archaeology must end in a measurable contract.** Before changing the current renderer hot path, capture a workload representative of massive output and compare allocations/throughput/frame cost. Functional rendering parity remains a hard constraint.

#### Pending `miniksa` archaeology

The remaining input prototypes are intentionally retained for the next decision-compression pass:

- `dev/miniksa/input`
- `dev/miniksa/input_tests`
- `dev/miniksa/input2`

The product/curation branches remain protected:

- `dev/miniksa/main`
- `dev/miniksa/issue-987-vt-adapter-test-coverage`

## Next batches

1. Finish decision compression for `dev/miniksa/input`, `dev/miniksa/input_tests`, and `dev/miniksa/input2` by extracting any still-useful input contracts into current tests or documentation.
2. Establish a modern renderer hot-path performance contract/benchmark for the allocation/reinterpretation TODO; once captured, retire `dev/miniksa/perf_buffer_dig`.
3. Continue grouping remaining inherited branches by upstream owner/namespace. For maintainers whose historical work is strategically useful, create one `dev/<maintainer>/main` lane before removing their legacy refs. Preserve only ChicoDotNet-owned work, curated maintainer lanes, and release lineage unless a specific exception is documented here.
