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
8. Do not select recovery work by age alone. Prefer issues that combine stale ownership, reproducible behavior, architectural value, tractable scope, and useful overlap with the Rust migration.

## Current protected working set

- `dev/ChicoDotNet`
- `dev/cazamor/main`
- `dev/duhowett/main`
- `dev/lhecker/main`
- `dev/migrie/main`
- `dev/miniksa/main`
- `dev/miniksa/issue-987-vt-adapter-test-coverage` — active recovery pilot
- `rust/main`
- `rust/r08-product-integration`
- `rust/r09-product-integration` — head of PR #46
- `release-*`
- release/version-line branches such as `1.17`

## Maintainer consolidation lanes

The preferred canonical shape is `dev/<maintainer>/main`.

A direct branch such as `dev/cazamor` cannot coexist with existing refs such as `dev/cazamor/...`; GitHub rejects that ref shape while child refs remain. `dev/<maintainer>/main` provides the same visual grouping, can be created before legacy deletion, and leaves room for temporary issue branches under the same namespace.

The consolidated lanes are created from the current `dev/ChicoDotNet` baseline:

| Canonical lane | Historical namespaces absorbed | Purpose |
| --- | --- | --- |
| `dev/cazamor/main` | `dev/cazamor/*` | Curated Carlos Zamora / settings-accessibility work lane |
| `dev/duhowett/main` | `dev/duhowett/*`, `dev/duhowtt/*` | Curated Dustin Howett / core-platform work lane |
| `dev/lhecker/main` | `dev/lhecker/*` | Curated Leonard Hecker / renderer-core work lane |
| `dev/migrie/main` | `dev/migrie/*`, `dev/mgirie/*` | Curated Michael Gries / product-integration work lane |
| `dev/miniksa/main` | `dev/miniksa/*` | Curated Console/VT/test-infrastructure work lane associated with the original parser/console architecture |

### Lane workflow

1. Refresh `dev/ChicoDotNet` from a green `microsoft/terminal:main` state.
2. Refresh the relevant `dev/<maintainer>/main` lane from that baseline before beginning recovery work.
3. Select an upstream issue that is open, valuable, and relatively abandoned or stalled.
4. Create a temporary branch such as `dev/<maintainer>/issue-<number>-<slug>` from the maintainer lane.
5. Reuse useful ideas from historical branches selectively; never merge an entire namespace wholesale.
6. Start with a contract, characterization, or regression test whenever the behavior can be made deterministic.
7. If Microsoft ships an equivalent fix first, mark the local effort superseded and retire the temporary branch.
8. If ChicoDotNet completes the issue first, carry it through review/CI and retire the temporary branch after integration.
9. Keep `dev/<maintainer>/main` as the durable lane; issue branches are disposable.

This gives the repository a small set of upstream-observation lanes while `dev/ChicoDotNet` remains the authoritative intake point for Microsoft releases.

## Upstream issue recovery queue

### Selection criteria

Recovery candidates are ranked using five practical signals rather than a vanity score:

- **Staleness** — old open work with little recent movement is more attractive than actively owned work.
- **Contract quality** — deterministic repros, standards, existing tests, or explicit API behavior reduce ambiguity.
- **R09 affinity** — parser, VT, ConPTY, input, buffer, and other migrated or migration-adjacent behavior can produce reusable contracts for the Rust product.
- **Tractability** — prefer a narrow proof before a broad redesign.
- **Coordination risk** — assigned issues or externally tracked work are observed before being duplicated.

### Current shortlist

| Priority | Upstream issue | Area | Why it is interesting | Current disposition |
| ---: | --- | --- | --- | --- |
| **1** | `microsoft/terminal#987` — Validate that VT adapter tests cover all mock possibilities | VT / tests / code health | Help Wanted; open since 2019; no assignee or comments; improves contract evidence without changing product semantics; directly useful to R09 | **Active pilot** on `dev/miniksa/issue-987-vt-adapter-test-coverage` |
| **2** | `microsoft/terminal#4037` — split escape sequence writes | parser / ConPTY input | Repro is described as 100% deterministic; write-boundary semantics are highly relevant to parser ownership | Queue after pilot; revalidate against current `main` first |
| **3** | `microsoft/terminal#17737` — incorrect DECRQM for Win32InputMode | VT / ConPTY input | Small explicit state-machine bug with a concrete repro and a suspected stale flag | Queue; strong regression-test candidate |
| **4** | `microsoft/terminal#3082` — DEC private 47/1047 alt-buffer modes | VT compatibility | Help Wanted; old and well-scoped protocol behavior; likely testable against explicit VT semantics | Queue; semantic implementation after lower-risk contracts |
| **5** | `microsoft/terminal#2985` — propagate palette changes through ConPTY with OSC 4 | VT / ConPTY | Help Wanted and old; useful end-to-end VT propagation behavior | Queue; broader than pilot |
| Watch | `microsoft/terminal#17862` — UTF-8 codepoint across I/O boundary | decoding / input boundary | Extremely high Rust/parser affinity and deterministic boundary case | **Watch only** while `Tracking-External` / assigned ownership remains |
| Watch | `microsoft/terminal#17336` — parser performance | parser performance | Existing benchmark evidence and large potential upside | **Watch/benchmark**; assigned and materially broader scope |
| Watch | `microsoft/terminal#11794` — large single write blocks rendering | parser/render performance | High user impact and useful parser/render boundary evidence | **Watch**; reopened and more recently active |

The queue is intentionally reorderable whenever a fresh `microsoft/terminal:main` sync closes, supersedes, reassigns, or materially changes a candidate.

## Recovery pilot 001 — microsoft/terminal#987

Status: **branch created; contract reconnaissance in progress**

- Maintainer lane: `dev/miniksa/main`
- Work branch: `dev/miniksa/issue-987-vt-adapter-test-coverage`
- Upstream issue: `microsoft/terminal#987`
- Scope: test evidence only unless the tests reveal a real adapter defect.
- Governing rule: do not add tests merely to raise coverage; each added test must prove that an adapter operation reaches the expected `ITerminalApi` contract with the expected arguments or observable result.

### First uncovered contract candidate

Reconnaissance identified a narrow first slice:

- `ITerminalApi::ReturnAnswerback()` is part of the adapter API contract.
- `TestGetSet::ReturnAnswerback()` exists in `ut_adapter/adapterTest.cpp`, but currently only logs that the mock was called.
- `AdaptDispatch::EnquireAnswerback()` delegates directly to `_api.ReturnAnswerback()`.
- Repository search finds the mock definition but no test assertion in `adapterTest.cpp` proving that `EnquireAnswerback()` reaches that mock.

Proposed TDD increment:

1. Make the mock call observable with the smallest possible state/counter.
2. Add one focused adapter unit test for `EnquireAnswerback()`.
3. Verify the existing adapter test project plus the new test in Windows CI.
4. Only after that green proof, continue mechanically to the next unverified mock contract.

This is deliberately a contract-by-contract replay loop, not a bulk coverage exercise.

## Cleanup batches

### Batch 001 — Microsoft Terminal core developer namespaces

Status: **consolidation lanes created; legacy ref deletion pending**

These are inherited development branches from core Microsoft Terminal maintainers. Their useful future role is now represented by the consolidated lanes above.

| Namespace | Legacy branches identified | Action |
| --- | ---: | --- |
| `dev/cazamor/*` | 84 | Delete legacy refs except `dev/cazamor/main` and any active recovery branches |
| `dev/duhowett/*` | 88 | Delete legacy refs except `dev/duhowett/main` and any active recovery branches |
| `dev/duhowtt/*` | 1 | Delete legacy typo ref; fold into `dev/duhowett/main` |
| `dev/lhecker/*` | 69 | Delete legacy refs except `dev/lhecker/main` and any active recovery branches |
| `dev/migrie/*` | 256 | Delete legacy refs except `dev/migrie/main` and any active recovery branches |
| `dev/mgirie/*` | 1 | Delete legacy typo ref; fold into `dev/migrie/main` |
| `dev/miniksa/*` | 26 | Delete legacy refs except `dev/miniksa/main` and active recovery branches |
| **Legacy refs identified** | **525** | **Pending deletion** |

Notes:

- The 525 count covers inherited legacy refs only. Newly created canonical `/main` lanes and recovery issue branches are protected and are not part of the deletion count.
- `dev/ChicoDotNet` is intentionally excluded even though it shares the `dev/` prefix.
- A developer branch whose name contains the word `release` (for example `dev/migrie/release-1.12-rejuv-attempt-2`) is still a developer work branch and is not part of the preserved top-level release lineage.
- The fork currently has only one open pull request, PR #46, and it is ours; there are no open Microsoft legacy PRs in this fork to close. The cleanup target is therefore inherited branch refs.

## Next batches

After Batch 001, continue grouping remaining inherited branches by upstream owner/namespace. For maintainers whose historical work is strategically useful, create one `dev/<maintainer>/main` lane before removing their legacy refs. Preserve only ChicoDotNet-owned work, curated maintainer lanes, active recovery branches, and release lineage unless a specific exception is documented here.
