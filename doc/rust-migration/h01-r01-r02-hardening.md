# H01 — R01/R02 test-parity hardening

H01 starts from `rust/r08-product-integration@78cb6a0facfb87943eaa3cc9be5bb220b5805bce` and hardens the already-merged Microsoft-to-Rust equivalence ledger without changing product code.

## Goals

1. Reconcile R02 with the newer functional-lane TerminalInput audit, including downgrades where Windows keyboard-layout behavior is part of the Microsoft observation.
2. Replace the coarse R01 `OutputEngineTest.cpp = Partial` assumption with method-level `Exact` evidence wherever a dedicated Microsoft-vector Rust contract already exists.
3. Keep residual methods Partial until they receive the same row-level audit; do not mass-promote a source family.

## R02 correction

The later functional audit showed two historical classifications were too optimistic:

| Microsoft contract | Before | H01 | Reason |
|---|---:|---:|---|
| `inputTest.cpp::TerminalInputModifierKeyTests` | Partial | Platform-only | The complete Microsoft method builds Windows keyboard state and observes active-layout `ToUnicodeEx` translation. Rust covers the deterministic VT-key subset, but that does not make the whole source method portable. |
| `inputTest.cpp::DifferentModifiersTest` | Exact | Partial | Rust covers the deterministic Backspace/Delete/Tab and slash/question outputs, but Microsoft obtains part of the key identity through Windows keyboard-layout translation. |

The hardened adapter distribution is therefore:

```text
adapter=72; runtime=411
Exact=19
Partial=52
Platform-only=1
Missing=0
```

This is intentionally a stricter result than the earlier `Exact=20, Partial=52` ledger.

## R01 OutputEngine hardening

`OutputEngineTest.cpp` contains 64 source methods. Delivery 11 conservatively represented all 64 through a source-family `Partial` rule even though six dedicated Rust contract files already carried direct Microsoft vectors.

H01 promotes **35 methods to Exact** individually:

- 14 CSI/escape/state contracts from `output_engine_microsoft_state_contract.rs`;
- 17 OSC/DCS/SOS/PM/APC contracts from `output_engine_microsoft_string_contract.rs`;
- `TestC1ParserMode` from `output_engine_microsoft_remaining_contract.rs`;
- the three XParse color contracts already documented as Exact: default foreground, default background, and color-table assignment.

Every promoted method names its concrete Rust witness in `microsoft-rust-equivalence-r01.json`. The existing `OutputEngineTest.cpp` source rule remains Partial only for the **29 residual methods** not yet promoted one by one.

The hardened R01 distribution becomes:

```text
terminal=98; runtime=760
Exact=54
Stronger=11
Partial=30
Platform-only=3
Missing=0
```

Compared with the pre-H01 ledger, this is **35 Partial → Exact promotions** with no product implementation change.

## Global effect

```text
Before H01                 After H01
Exact          64          Exact          98
Stronger       11          Stronger       11
Partial       447          Partial       412
Platform-only  62          Platform-only  63
UI-managed     22          UI-managed     22
Missing       492          Missing       492
Total        1098          Total        1098
```

The one Exact reduction in R02 is deliberate evidence hardening, not regression. Net global movement is still 34 contracts into Exact while one contract is correctly recognized as platform-owned.

## Safety

- Product Rust changed: **0**
- Product C++ changed: **0**
- FFI changed: **0**
- Managed/XAML changed: **0**
- Microsoft tests removed or weakened: **0**
- Certification baselines relaxed: **0**

H01 changes only machine-readable classifications and migration documentation. The normal Rust/global witness gates must prove that every promoted semantic contract still references real Rust evidence.

## Next hardening increment

H02 should attack the first half of `adapterTest.cpp` (R03-A), promoting only contracts whose downstream observation is now materially owned in Rust rather than merely preserved as a deferred `OutputAction`.
