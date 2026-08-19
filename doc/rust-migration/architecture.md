# Rust migration architecture

This document defines the migration track in `ChicoDotNet/terminal`. The goal is not a big-bang rewrite. The goal is a verifiable, incremental Rust implementation that preserves Windows Terminal behavior while making each migrated component independently testable.

## Principles

1. **Microsoft C++ remains the oracle until a Rust component proves equivalence.**
2. **Fast Rust feedback is the normal development loop.** Microsoft/TAEF tests are the contractual compatibility gate.
3. **Migrate vertical slices.** Do not port an entire foundational C++ library merely because the current build groups unrelated responsibilities together.
4. **Keep unsafe code at explicit FFI boundaries.** Safe implementation crates should forbid unsafe code.
5. **No product C++ is removed in R00.** Infrastructure and evidence come first.
6. **Known upstream/baseline failures are recorded rather than silently normalized.** New failures are regressions.
7. **Performance and memory are measured, not assumed.**

## Migration order

| Increment | Scope | Exit condition |
|---|---|---|
| R00 | Workspace, CI, TAEF contract harness, baseline, scorecard | Fast Rust CI works and TAEF output is evaluated independently of the legacy wrapper exit status |
| R01 | VT parser: Base64, state machine, output/input engines | Differential corpus agrees and Microsoft `terminal` suite does not regress |
| R02 | Terminal input plus required pure types | Input contracts and differential tests agree |
| R03 | Adapter/dispatch/Sixel | Adapter contract agrees |
| R04 | TextBuffer/TIL/pure foundational types | Foundational suites agree |
| R05 | TerminalCore | Core suite agrees |
| R06 | Host/server/interactivity/ConPTY | Host and ConPTY contracts agree |
| R07 | Renderer | Rendering acceptance/performance evidence agrees |
| R08 | WinRT/COM/XAML/settings/control/UI | Product-level acceptance agrees |
| R09 | Compatibility façade removal and C++ cleanup | Remaining C++ is intentional platform boundary or removed |

## FFI shape

```text
Existing C++ code/tests
        |
        v
C++ compatibility façade
        |
        | C ABI
        v
terminal-*-ffi
        |
        | safe Rust API
        v
terminal-*
```

Rust ABI is not exposed directly to C++. The C ABI should use narrow, explicit ownership rules, opaque handles, and byte/slice-oriented buffers where practical.

## R01 target

The first production slice is `src/terminal/parser`. The existing project already isolates parser behavior into a static library and has a strong TAEF contract. R01 begins with Base64 because it is small and deterministic, then moves through the state machine and engines before connecting the C++ façade to Rust.

The intended proof is stronger than "the Rust tests pass":

```text
same VT corpus
   +--> C++ parser --> observation A
   |
   +--> Rust parser -> observation B

A == B
```

Only after differential equality is established does the Microsoft `terminal` suite become the final compatibility gate for the slice.
