# Branch Archaeology — Cohort 017

This cohort closes three non-numeric `dev/migrie/b/**` branches whose implementation or diagnostic value has been superseded by later upstream work.

## `dev/migrie/b/broken-globalsummon-overloading`

- Functional commits: `64f5b1ae7d8d2e53b4f7456ce2e2c4585c3405f3`, `3d1db046cc21f0464ccd202d66dd331914bc2828`, and `96ed95869adfe226dd0c620c5bfa68884d0324c6`; the later tip is spelling maintenance.
- Historical intent: make explicit VKEY and scan-code representations of the same physical key behave as the same key for `ActionMap` hashing/equality while preserving the original serialized representation (`vk(...)` versus `sc(...)`). This was motivated by globalSummon/Quake bindings across keyboard layouts.
- Historical contract: two `KeyChord`s that resolve to the same physical key should hash/compare equal, but each should round-trip in its original serialized form.
- Definitive upstream lineage: `microsoft/terminal#10666` (`Introduce vk() and sc() key chord specifiers`) merged as `10b12ac90ce81b1a1bbb0e7918d1046e27ab5445`, closed `#7539` and `#10203`, and added tests/US+DE validation.
- Modern reading: ownership moved out of `ActionMap` and into `KeyChord`. Current `KeyChord` construction fills a missing VKEY from a scan code; `KeyChord::Hash()` and `KeyChord::Equals()` explicitly document the layering equivalence of forms such as `win+sc(41)` and `win+\``. Current `UnitTests_SettingsModel/KeyBindingsTests.cpp` still covers explicit `vk(...)`/`sc(...)` serialization and scan-code-to-VKEY reconstruction.
- Disposition: **NO-PORT / superseded with modern contract coverage**.
- Recovery value: provenance only; the durable contract survives in a better ownership boundary and current tests.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/bump-nuget-in-c`

- Functional commits: `f465c07ff509a65f4aa0752c62d80c85d5eea3b5` and branch head `8265574e06c0103e57b9cc3b9bb82eb78cab28bd`.
- Historical intent: repair CI after the pinned NuGet version became unavailable on an Azure Pipelines agent. The first attempt targeted `6.10.1`; the branch was then changed to `6.6.2` after the agent itself reported that `6.10.1` was unavailable.
- Exact upstream PR: `microsoft/terminal#17496`, whose head is this branch and exact SHA `8265574e...`, was closed without merge.
- Modern reading: the current pipeline template installs NuGet `7.0.1`, so neither the original `6.10.1` attempt nor the later `6.6.2` fallback is a useful product or CI delta today.
- Disposition: **NO-PORT / obsolete CI probe**.
- Recovery value: diagnostic provenance only — specifically, the failed assumption that `6.10.1` existed on the hosted agent.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/cxn-restarting-attempt-1-backport`

- Historical/head commit: `2aad178aae5c82cf1a5c5d524b30b7dcb95bf83d`.
- Exact upstream PR: `microsoft/terminal#14548` (`Refactor how connection restarting is handled`) uses this exact branch/head and was closed without merge.
- Historical design: avoid transitioning a connection from `Closed` back to `Start` by stashing connection creation information in `ControlCore` and allowing the core to recreate the connection itself.
- Definitive successor: `microsoft/terminal#15240` (`(A better) Refactoring of how connection restarting is handled`) merged as `0d6642ac6d40b434ac51883e0d1f8a7da20e30ff`, explicitly calls itself a different take on `#14548`, explicitly says `Obsoletes #14548`, and closes `#14327`.
- Successor architecture: the control/core raises a restart request and the app/pane layer owns recreating the connection. This preserves a cleaner ownership boundary than storing connection-construction state inside `ControlCore`.
- Disposition: **NO-PORT / explicitly superseded by #15240**.
- Recovery value: provenance only; do not resurrect the attempt-1 ownership model.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 017 deletion set

- `dev/migrie/b/broken-globalsummon-overloading`
- `dev/migrie/b/bump-nuget-in-c`
- `dev/migrie/b/cxn-restarting-attempt-1-backport`
