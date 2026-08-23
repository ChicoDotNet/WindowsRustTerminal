# Microsoft-to-Rust test equivalence matrix

This document is the evidence ledger for deciding when a Microsoft C++/TAEF test must remain in the per-change compatibility gate and when a proven Rust equivalent can carry the fast inner loop.

The matrix is deliberately conservative: until an individual Microsoft contract is mapped to concrete Rust evidence, its area remains **Partial** and the relevant Microsoft test stays blocking for a changed C++/FFI boundary.

## Baseline

The fully integrated R07 checkpoint (`33190ef8d43626adabc7286e2ffe09ea383300fe`) contains **418 distinct Rust tests**. The same 418 contracts run on Linux and Windows, so the CI matrix performs 836 executions but represents 418 distinct test definitions. R07 reported zero ignored tests.

R08a adds four `terminal-parser-ffi` tests, bringing the current R08 branch inventory to **422 distinct Rust tests** before subsequent R08 slices.

`tools/rust/contract-baseline.json` records the Microsoft `terminal` suite at **760 total**, with zero failed/blocked/not-run allowed and at most one skipped. The full suite remains the certification oracle; its approximately 24-minute runtime is not used as a reason to weaken a gate.

R08c adds two complementary inventories. `tools/rust/Get-MicrosoftTestInventory.ps1` derives stable source-level `TEST_METHOD` identities without a build. The TAEF harness also uses `/listProperties` to enumerate expanded runtime invocation identities, including data-driven `#metadataSet` cases, before executing the expensive suite. A baseline-count mismatch therefore fails early before spending the full contract runtime.

Source methods are contract groups, not a replacement for TAEF's expanded 760-case inventory. The runtime inventory is authoritative when finer-grained boundary selection is needed.

## Coverage classifications

| Classification | Meaning | Microsoft test in per-boundary gate? |
|---|---|---|
| Exact | Rust covers the same relevant behavior and vectors | Can leave the per-change boundary set after evidence is recorded |
| Stronger | Rust covers the Microsoft case plus additional vectors/invariants | Can leave the per-change boundary set after evidence is recorded |
| Partial | Rust covers only part of the behavior | Yes |
| Platform-only | Requires Windows/COM/WinRT/GDI/DWrite/DX or another platform surface | Yes |
| UI-managed | Responsibility correctly belongs to C#/XAML rather than Rust | Validate in the managed/UI contract appropriate to that surface |
| Missing | No adequate migrated equivalent exists | Yes |

Leaving the per-change set does **not** remove a Microsoft test from full certification. The complete Microsoft suite remains an R08 exit gate and an R09 final-validation gate.

## Current Rust inventory

| Area | Rust crate | Stage | R07 stable tests | Current R08 tests | Initial equivalence status | C# retained? | Default CI tier |
|---|---|---:|---:|---:|---|---|---|
| VT parser | `terminal-parser` | R01 | 39 | 39 | Partial; Base64 + StateMachine mapping in progress | No | Fast + affected boundary |
| Terminal input | `terminal-input` | R02 | 28 | 28 | Partial pending method mapping | No | Fast + affected boundary |
| Adapter / dispatch / Sixel | `terminal-adapter` | R03 | 77 | 77 | Partial pending method mapping | No | Fast + affected boundary |
| TextBuffer / foundational types | `terminal-buffer` | R04 | 68 | 68 | Partial pending method mapping | No | Fast + affected boundary |
| TerminalCore | `terminal-core` | R05 | 38 | 38 | Partial pending method mapping | No | Fast + affected boundary |
| Host / server / interactivity / ConPTY | `terminal-host` | R06 | 118 | 118 | Partial pending method mapping | No | Fast + affected boundary |
| Renderer | `terminal-renderer` | R07 | 50 | 50 | Partial pending method mapping | No | Fast + affected boundary |
| Product FFI foundation | `terminal-parser-ffi` | R08 | 0 | 4 | Platform/boundary evidence not yet consumed by C++ | No | Fast; Microsoft contract becomes blocking when a consumer is added |
| XAML code-behind / bindings / view models | existing managed projects | R08 | n/a | n/a | UI-managed where already owned by C# | Yes | Managed/UI contract |
| WinRT/COM/XAML native boundary | existing platform layer | R08 | n/a | n/a | Platform-only until narrowed | Where applicable | Boundary + Stage |

**Current total:** 418 stable R07 tests; 422 on the R08 branch before later R08 semantic slices.

## Evidence rows

### R01 Base64

`src/terminal/parser/ut_parser/Base64Test.cpp` contains exactly two Microsoft `TEST_METHOD` contracts, and the source-inventory self-test locks that identity set.

| Microsoft suite/test | Area | Behavior | Rust equivalent | Vector evidence | Coverage | Windows dependency | FFI dependency | CI tier | Stage | Notes |
|---|---|---|---|---|---|---|---|---|---|---|
| `terminal / Base64Test.DecodeUTF8` | Parser/Base64 | Decode multilingual UTF-8 and emoji/skin-tone payloads | `base64::tests::matches_windows_terminal_unicode_vectors` | Rust uses the same two Base64 inputs and the same expected Unicode strings as Microsoft | Exact | No | No | Fast + Full certification | R01 | Direct vector-for-vector equivalence |
| `terminal / Base64Test.DecodeFuzz` | Parser/Base64 | ASCII round-trip across varying lengths, padded/unpadded input, including empty input | `base64::tests::deterministic_ascii_round_trips_match_reference_encoding`; `decodes_rfc_4648_vectors_with_and_without_padding` | Microsoft samples 8 random lengths/content choices; Rust deterministically covers every length 0..128 and both padded and unpadded forms, plus canonical RFC vectors | Stronger | No | No | Fast + Full certification | R01 | Rust trades nondeterministic sampling for broader reproducible length/padding coverage |

### R01 StateMachine

`StateMachineTest.cpp` defines seven source methods. Six have direct semantic counterparts with matching ordering/vectors. `PassThroughUnhandled` deliberately remains Partial because its Microsoft observation orders the unknown CSI before following printable text, while the closest Rust test proves the inverse ordering. The data-driven DCS method expands to four Microsoft runtime invocations and Rust covers the same four terminators.

| Microsoft suite/test | Area | Behavior | Rust equivalent | Vector evidence | Coverage | Windows dependency | FFI dependency | CI tier | Stage | Notes |
|---|---|---|---|---|---|---|---|---|---|---|
| `terminal / StateMachineTest.TwoStateMachinesDoNotInterfereWithEachOther` | Parser/state machine | Parser instance isolation across interleaved partial/full CSI sequences | `state_machine::tests::two_state_machines_do_not_interfere` | Same partial `ESC[12`, independent `ESC[3C`, then completion `;34m`; same parameter observations | Exact | No | No | Fast + Full certification | R01 | Direct scenario equivalence |
| `terminal / StateMachineTest.PassThroughUnhandled` | Parser/state machine | Unknown CSI is flushed intact while following printable text remains printable | nearest: `state_machine::tests::unhandled_csi_is_passed_through_without_losing_prior_text` | Microsoft sends `ESC[?999h` first and printable text afterward; Rust currently proves printable text first and unknown CSI afterward | Partial | No | No | Fast + Boundary + Full certification | R01 | Keep Microsoft boundary coverage until the sequence-first ordering is explicit in Rust |
| `terminal / StateMachineTest.RunStorageBeforeEscape` | Parser/state machine | Buffered printable run is emitted before transition into an escape sequence | `state_machine::tests::unhandled_csi_is_passed_through_without_losing_prior_text` | Both send `12345 Hello World` followed by `ESC[?999h` and observe the complete text plus passthrough sequence | Exact | No | No | Fast + Full certification | R01 | Direct ordering/vector match |
| `terminal / StateMachineTest.BulkTextPrint` | Parser/state machine | Plain text is emitted as a single bulk print run | `state_machine::tests::bulk_text_is_printed_as_one_run` | Same `12345 Hello World` payload and expected single run | Exact | No | No | Fast + Full certification | R01 | Direct scenario equivalence |
| `terminal / StateMachineTest.PassThroughUnhandledSplitAcrossWrites` | Parser/state machine | Unknown CSI/OSC sequences survive two- and three-part write boundaries | `state_machine::tests::unhandled_sequences_survive_split_writes` | Rust covers the same split CSI cases and the OSC split at ESC/ST used by Microsoft | Exact | No | No | Fast + Full certification | R01 | Direct split-write equivalence |
| `terminal / StateMachineTest.DcsDataStringsReceivedByHandler` | Parser/state machine | DCS id/params/data delivery and termination by ST, CSI, CAN, or SUB | `state_machine::tests::dcs_data_is_delivered_and_st_can_terminate_it`; `dcs_can_be_terminated_by_csi_can_or_sub` | Microsoft data source has terminatorType `{0,1,2,3}`; Rust explicitly covers ST plus CSI/CAN/SUB and validates id, params, data, execution/CSI side effects, and following text | Exact | No | No | Fast + Full certification | R01 | Four expanded TAEF cases map to two Rust tests; runtime inventory supplies canonical invocation identities |
| `terminal / StateMachineTest.VtParameterSubspanTest` | Parser/parameters | Parameter subspan at 0, 2, end, and past-end | `state_machine::tests::parameter_subspan_matches_terminal_semantics` | Same values `[12,34,56,78]`, offsets `0,2,4,6`, sizes/default omitted value semantics | Exact | No | No | Fast + Full certification | R01 | Direct vector-for-vector equivalence |

At this checkpoint **eight of the nine Base64/StateMachine source methods are Exact or Stronger**. Those eight may leave the per-change semantic boundary set for Rust-only implementation changes. `PassThroughUnhandled` remains boundary-blocking. All nine remain in complete Microsoft certification and become boundary-relevant whenever their C ABI representation or C++ consumer changes.

## Per-test row schema

The area-level inventory above is only the bootstrap. The matrix becomes authoritative for CI reduction only when the Microsoft suite is expanded into one row per source method or independently meaningful runtime case using this schema:

| Field | Description |
|---|---|
| Microsoft suite/test | Canonical source/runtime identity |
| Area | Parser, input, adapter, buffer, core, host, renderer, control, settings, UI, platform |
| Behavior | Contract protected by the test |
| Current owner | C++, Rust, C#, XAML, or platform boundary |
| Rust equivalent | Concrete Rust test function(s), if any |
| Vector evidence | Important cases/parameters covered on each side |
| Coverage | Exact, Stronger, Partial, Platform-only, UI-managed, Missing |
| Windows dependency | Whether the contract requires Windows runtime behavior |
| FFI dependency | Whether the contract crosses the product ABI |
| C# retained | Whether healthy managed UI ownership intentionally remains C# |
| CI tier | Fast, Boundary, Stage, Full certification |
| Stage | R01 through R09 |
| Notes | Differences, known gaps, or evidence references |

## CI selection rule

1. **Fast** runs on every change: Rust fmt, Clippy with `-D warnings`, Linux/Windows workspace check+test, repository quality/spelling, TAEF harness self-test, and the Microsoft source-inventory self-test.
2. **Boundary** is added when C++/FFI/platform code changes. Run every affected Microsoft row still classified Partial, Platform-only, or Missing, plus any Exact/Stronger row whose boundary representation itself changed.
3. **Stage** runs before R08 merge for all R08 contracts that have not been proven sufficiently equivalent.
4. **Full certification** runs the complete Microsoft Terminal Suite at R08 exit and again in R09.

A contract run captures the authoritative TAEF runtime inventory before executing the suite. If that inventory differs from the recorded baseline total, the run stops before spending the cost of the full suite. A successful full run additionally requires inventory count and result total to agree.

No Microsoft test is removed from a blocking tier merely because it is slow. It leaves the per-change boundary tier only when the matrix contains concrete equivalence evidence.

## R08 managed-UI rule

The migration is **C++ to Rust**, not C# to Rust. Existing C# that naturally drives XAML remains managed code. Such rows are classified **UI-managed**, not Missing, provided the C# layer is genuinely UI orchestration rather than a wrapper around business logic that still resides in removable C++.

The desired direction is therefore:

```text
XAML -> existing C# managed UI -> narrow interop -> safe Rust semantics
```

where that ownership already exists, while native WinRT/COM/Win32 boundaries remain explicit and narrow.

## Next matrix increment

Continue R01 mapping through `InputEngineTest` and `OutputEngineTest`, then R02-R07. Runtime-expanded TAEF identities are captured automatically by the contract harness when a certification run is required. Until a method has concrete evidence, it remains Partial and does not justify relaxing the Microsoft boundary gate.
