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
- `dev/miniksa/input2` — retain only until its still-unique input/code-page characterization matrix is replayed against the current product and durable contracts are created for the supported behavior.

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

Status: **performance and input archaeology compressed; additional legacy refs safe to delete; two evidence branches temporarily retained**

The `miniksa` namespace contains a mixture of abandoned issue work, test prototypes, and performance/compatibility laboratories. The cleanup rule for this namespace is evidence-first: keep current product contracts and useful hypotheses, not historical branch names.

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

#### Input archaeology

| Branch | Classification | Decision |
| --- | --- | --- |
| `dev/miniksa/input` | **INCOMPLETE EXPLORATION / SUPERSEDED BY ARCHITECTURE** | Delete. Its only functional delta was a 2020 `dbcs.cpp` conversion experiment whose own commit message records that the caller still allocated an insufficient return buffer and that the problem had to be solved one layer up. The current `dbcs.cpp` no longer contains that conversion path, so the patch is neither complete nor structurally applicable. |
| `dev/miniksa/input_tests` | **REFACTOR SUPERSEDED; CONTRACT VALUE ABSORBED OR RETAINED ELSEWHERE** | Delete. The branch replaced `_handlePostCharInputLoop` and added cooked-input/alias characterization. Current cooked input has since been substantially rewritten and still owns post-loop alias handling through `Alias::s_MatchAndCopyAlias`. Current unit tests cover alias expansion semantics extensively. The branch's still-useful end-to-end input characterization is also present in the retained `dev/miniksa/input2` evidence source, so this ref is not required as a second owner. |
| `dev/miniksa/input2` | **UNMIGRATED CHARACTERIZATION / CONTRACT REPLAY SOURCE** | Retain temporarily. It adds roughly 873 lines of `API_InputTests.cpp` behavior discovery that are not present in the current test file. Do not cherry-pick the 2020 test file wholesale. Replay the matrix against the 2026 product, classify supported versus legacy-only behavior, then encode supported behavior as modern contracts before deleting the ref. |

##### Contract Replay matrix from `input2`

The following tests are the durable evidence to evaluate; the branch itself is not the desired long-term artifact:

| Historical test | Behavior under characterization | Replay decision |
| --- | --- | --- |
| `TestCookedAliasProcessing` | End-to-end `ReadConsoleA` cooked input with DOSKEY alias expansion, including `$T` multi-command expansion. | Recreate or map to a current integration contract. Alias unit semantics already exist, but the cooked-input seam is separate behavior. |
| `TestCookedTextEntry` | Baseline cooked text entry and return shape through `ReadConsoleA`. | Verify current coverage; keep only if it closes a real seam not covered by newer tests. |
| `TestCookedAlphaPermutations` | Input/output CP 437/932 permutations, cooked/raw mode interactions, and font-dependent legacy behavior. | Split portable/current compatibility expectations from Console V1-only observations. |
| `TestReadCharByChar` | Byte-at-a-time reads with DBCS lead/trail-byte carry behavior across cooked, raw, and direct reads. | Replay against supported modes; preserve observable API behavior, not old internal buffering. |
| `TestReadLeadTrailString` | Lead/trail byte stitching when the caller buffer divides a DBCS character/string. | Replay as an encoding-boundary contract if still supported. |
| `TestReadChangeCodepageInMiddle` | Code-page switch after a partial multi-byte read; historical expectation explicitly discards partial bytes before re-encoding remaining input. | High-value compatibility contract candidate; verify current behavior before canonizing it. |
| `TestReadChangeCodepageBetweenBytes` | Code-page switch between lead and trail bytes and prevention of stale-byte stitching into later results. | High-value compatibility contract candidate; verify current behavior before canonizing it. |

##### Decision-compression rules for `input2`

1. Treat the 2020 results as **observations**, not automatically as desired 2026 semantics. Several commits explicitly distinguish Console V1 from theoretical V2 behavior.
2. Prefer current supported-product behavior when V1 and V2 disagree. Preserve V1-only observations only when Windows compatibility still requires them.
3. Keep three read modes distinct during replay: cooked, raw, and direct. A passing result in one mode is not evidence for the others.
4. Exercise CP 437 and CP 932 boundaries specifically because the historical tests were designed around single-byte versus DBCS transitions.
5. Code-page changes with a pending lead/trail byte are first-class boundary cases. Record exact bytes returned, residual state, and behavior after the switch.
6. Do not restore the old `dbcs.cpp` implementation merely to satisfy an old test. If a supported contract fails, repair the current owner of the behavior.
7. Retire `dev/miniksa/input2` only when every row above is either represented by a current executable contract or explicitly classified as unsupported/legacy-only with rationale in this ledger.

The product/curation branches remain protected:

- `dev/miniksa/main`
- `dev/miniksa/issue-987-vt-adapter-test-coverage`

## Next batches

1. Run a Contract Replay pass for the seven `dev/miniksa/input2` scenarios against the current product; convert supported behavior into durable modern tests, then retire `dev/miniksa/input2`.
2. Establish a modern renderer hot-path performance contract/benchmark for the allocation/reinterpretation TODO; once captured, retire `dev/miniksa/perf_buffer_dig`.
3. Continue grouping remaining inherited branches by upstream owner/namespace. For maintainers whose historical work is strategically useful, create one `dev/<maintainer>/main` lane before removing their legacy refs. Preserve only ChicoDotNet-owned work, curated maintainer lanes, and release lineage unless a specific exception is documented here.
