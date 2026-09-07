# WindowsRustTerminal

> **An experimental Rust-first evolution of Windows Terminal.**
>
> **No hay problema.**

WindowsRustTerminal is an independent open-source engineering experiment based on
[Microsoft Windows Terminal](https://github.com/microsoft/terminal).

The question behind the project is deliberately simple:

**What happens if a mature, production-grade Windows terminal codebase is progressively migrated toward Rust while preserving its behavior, user experience, Windows integration, and accumulated engineering knowledge?**

I wanted to know.

So I started building it.

This repository is not an argument that another team should rewrite its software.
It is not an attempt to prescribe how Microsoft should develop Windows Terminal.
It is not a rejection of the extraordinary engineering work already present in
the upstream project.

It is a working experiment.

A place to explore Rust, software modernization, behavioral compatibility,
incremental migration, architecture, automated testing, and the economics of
rewriting mature systems through code rather than speculation.

---

## No hay problema

There is a phrase I have used since childhood:

> **No hay problema.**

Literally: **“There is no problem.”**

It does not mean problems are imaginary.

It means that when a problem appears, my first instinct is not to surrender to
its size.

Understand it.

Break it down.

Test assumptions.

Build something.

Measure the result.

Learn.

Repeat.

That philosophy became one of the reasons WindowsRustTerminal exists.

Large legacy systems are often surrounded by perfectly reasonable explanations
for why something would be difficult, expensive, risky, or impractical.

Those explanations matter.

But sometimes there is still value in asking:

**What if we simply try?**

---

## Why this project exists

Windows Terminal is a particularly interesting system for this experiment.

It is mature.

It is highly visible.

It interacts deeply with Windows.

It contains years of compatibility decisions, terminal behavior, rendering,
input handling, VT processing, platform integration, UI architecture, and
production experience.

That makes it exactly the kind of software where a rewrite should **not** be
treated casually.

And that is precisely what makes it interesting.

The goal is not to erase the engineering history of Windows Terminal.

The goal is to preserve what works while investigating whether Rust can become
an increasingly strong owner of the product's core behavior.

This project explores questions such as:

- Can a large C++ Windows application be migrated incrementally to Rust?
- Can behavioral compatibility be measured instead of assumed?
- Can Rust replace components without requiring a big-bang rewrite?
- Can existing Windows, XAML, WinRT, and product integration be preserved where
  rewriting them provides little value?
- Can automated contracts turn the original implementation into an executable
  specification?
- Can Rust improve memory-safety boundaries without sacrificing compatibility?
- What is the actual engineering cost of such a migration?
- Which parts should be copied conceptually, transformed, adapted, or left alone?
- Where does Rust provide meaningful architectural value, and where does it not?

The repository exists to produce evidence for those questions.

---

## Rust-first does not mean rewrite everything

One of the most important lessons from this experiment is that language migration
and product migration are not the same thing.

WindowsRustTerminal follows a **progressive ownership** model.

Rust should own components when there is a meaningful reason for Rust to own
them.

Existing Windows infrastructure should remain when replacing it would merely
recreate something that already works.

The target is therefore not:

> “Make every file Rust.”

The target is:

> **Move product responsibility toward Rust while preserving the best existing
> assets of the system.**

That distinction matters.

A successful modernization is not measured by the percentage of files that use
a new programming language.

It is measured by whether the resulting product is simpler to reason about,
safer to change, verifiably compatible, maintainable, and useful.

---

## Migration philosophy

The migration is being developed as a sequence of small, testable engineering
increments rather than a single rewrite event.

The core loop is:

1. **Observe the existing behavior.**
2. **Capture that behavior as a contract.**
3. **Implement the equivalent behavior in Rust.**
4. **Replay the contract against both implementations.**
5. **Investigate every difference.**
6. **Integrate only when equivalence is understood.**
7. **Let CI preserve what has already been learned.**

This makes the existing implementation more than legacy code.

It becomes an **oracle**.

Instead of trying to reproduce years of behavior from documentation or memory,
the migration asks the running system what it actually does.

That approach is especially valuable for terminal software, where edge cases,
escape sequences, parser state, input behavior, compatibility rules, and
historical details can matter as much as the obvious feature set.

---

## Contract Replay

A central technique in WindowsRustTerminal is what I call the **Contract Replay
Loop**.

For a migration or refactor:

```text
Existing implementation
        │
        ▼
Observed behavior
        │
        ▼
Executable contract
        │
        ├──────────────► Existing implementation
        │
        └──────────────► Rust implementation
                              │
                              ▼
                         Compare results
                              │
                    ┌─────────┴─────────┐
                    │                   │
                 Equivalent          Different
                    │                   │
                 Integrate          Investigate
```

This changes the question from:

> “Does the Rust version look correct?”

to:

> **“Can the Rust implementation demonstrate the same externally observable
> behavior?”**

That is a much stronger engineering question.

---

## Current migration state

The project has progressed through a series of migration rounds.

By the completion of **R08**, the functional contract suite had reached:

**968 / 968 covered contracts**

with:

**Missing = 0**

That does not mean the entire Windows Terminal product has already been replaced
by Rust.

It means something more precise:

The migrated behavioral surface covered by that contract system had reached
measured equivalence against the reference C++ implementation.

The current work moves beyond isolated behavioral equivalence and into
**product integration**: connecting Rust ownership to the actual application
architecture while retaining the parts of the Windows product stack that still
make sense to keep.

That distinction is important.

Passing tests is not the finish line.

**Shipping a coherent product is.**

---

## Why Rust?

I believe Rust is one of the strongest languages available today for building
systems software that must balance:

- performance,
- memory safety,
- explicit ownership,
- predictable resource management,
- concurrency,
- low-level control,
- strong tooling,
- testability,
- and long-term maintainability.

But this project is not based on the claim that:

> Rust automatically makes software fast.

It does not.

Nor does Rust automatically make an architecture good.

It does not.

Rust does, however, give engineers a powerful set of constraints and abstractions
for expressing ownership and safety decisions explicitly.

In a codebase that handles parsers, buffers, terminal state, Windows APIs,
concurrency, process interaction, and large amounts of stateful behavior, that
is worth exploring seriously.

WindowsRustTerminal exists to discover where those advantages survive contact
with a real production codebase.

---

## Why experiment instead of debate?

Software engineering has always advanced through people who were curious enough
to build something.

Linus Torvalds began Linux as a personal project rather than an attempt to command
the operating-system world.

Grace Hopper pushed programming toward higher-level languages because making
computers easier for humans to instruct was worth exploring.

Robert Griesemer, Rob Pike, and Ken Thompson created Go while trying to reduce
the complexity engineers experienced building large software systems.

Apple popularized a remarkably durable invitation to **Think Different**.

Microsoft's modern mission is to **empower every person and every organization
on the planet to achieve more**.

These stories are very different.

But I see a common engineering instinct in them:

**Curiosity becomes much more useful when it produces an artifact.**

WindowsRustTerminal follows that tradition in a very modest way.

I had authentic curiosity.

I enjoy Rust.

I care deeply about software architecture.

I wanted to understand this codebase.

I had some extra time.

And the source code was available.

So instead of limiting the question to:

> “Would rewriting Windows Terminal in Rust make sense?”

I chose a different question:

> **“How far can I actually take it?”**

No hay problema.

---

## This is an experiment, but it is not a toy

There is a useful difference between an experimental project and a disposable
one.

WindowsRustTerminal is experimental because its architecture is exploring a
different technical direction.

It is not intended to be careless.

The migration values:

- behavioral evidence over intuition,
- incremental change over blind rewrites,
- reproducible builds,
- automated testing,
- continuous integration,
- explicit architectural decisions,
- small verifiable increments,
- compatibility,
- measurable progress,
- and preserving valuable existing engineering.

The objective is not merely to produce Rust code.

The objective is to learn whether a **credible Rust-first Windows terminal** can
emerge from the experiment.

---

## Relationship to Microsoft Windows Terminal

WindowsRustTerminal originates from Microsoft's open-source
[Windows Terminal](https://github.com/microsoft/terminal) project.

The upstream Windows Terminal team created the application, architecture,
terminal behavior, Windows integration, rendering infrastructure, documentation,
tests, and enormous body of engineering knowledge on which this experiment is
based.

That work deserves explicit credit.

**WindowsRustTerminal is an independent fork and is not affiliated with,
endorsed by, or maintained by Microsoft.**

Microsoft Windows Terminal continues to have its own architecture, roadmap,
governance, maintainers, and engineering priorities.

This fork has a different purpose:

**to explore a Rust-first architectural path and document what happens.**

There is no requirement that upstream agree with the experiment.

There is no requirement that this experiment become upstream.

Open source allows both projects to pursue the questions their maintainers find
valuable.

That is a feature.

---

## What this project is not

WindowsRustTerminal is **not**:

- a demand that Microsoft rewrite Windows Terminal;
- a claim that C++ is obsolete;
- a claim that Rust automatically produces better software;
- an attempt to erase the work of the original Windows Terminal engineers;
- a promise that every Windows Terminal component will eventually be rewritten;
- a benchmark result disguised as a conclusion;
- or a finished product pretending to be one.

It is:

**an engineering experiment being progressively turned into software.**

---

## Engineering principles

### 1. Behavior before implementation

The original implementation tells us what the product actually does.

We capture that behavior before replacing it.

### 2. Evidence before confidence

A green contract is stronger than “this should work.”

### 3. Incremental migration

Large rewrites become safer when decomposed into independently verifiable
ownership transitions.

### 4. CI as engineering memory

Once a behavior has been discovered, CI should help prevent the project from
forgetting it.

### 5. Preserve valuable assets

Rewriting working infrastructure merely to increase the percentage of Rust is
not a goal.

### 6. Explicit boundaries

FFI, ownership, compatibility, platform integration, and unsafe code should have
clear reasons to exist.

### 7. Product over language purity

Rust serves the product.

The product does not exist to serve Rust.

---

## Architecture direction

The long-term architectural direction is a terminal in which Rust increasingly
owns product behavior while integrating pragmatically with the Windows platform.

Areas of investigation include:

- terminal parser behavior,
- VT processing,
- text and buffer behavior,
- input handling,
- state machines,
- command-line infrastructure,
- process and pseudoconsole integration,
- Windows API boundaries,
- WinRT interoperability,
- UI integration,
- configuration,
- product packaging,
- startup,
- runtime ownership,
- performance,
- memory behavior,
- and failure safety.

Some components may migrate completely.

Some may remain platform-native.

Some may become adapters.

Some may prove not worth changing.

**The experiment decides through evidence.**

---

## Migration classification

When examining existing code, changes are generally treated as one of four
categories:

### COPY

The behavior and structure are sufficiently correct that the new implementation
should preserve them directly.

### TRANSFORM

The behavior remains, but Rust allows the implementation to be expressed more
clearly or safely.

### ADAPT

The existing architecture depends on APIs, runtime assumptions, or language
boundaries that require an interoperability layer.

### PRESERVE

Rewriting the component would currently add more cost than value.

This classification helps prevent a language migration from turning into an
uncontrolled architecture rewrite.

---

## For Rust developers

If you come from Rust, this repository is an opportunity to work with a large
Windows-native application rather than an isolated systems-programming example.

Expect to encounter:

- Windows APIs,
- WinRT,
- XAML integration,
- C++ interop,
- terminal protocols,
- build-system constraints,
- platform packaging,
- behavioral compatibility,
- and migration boundaries.

The goal is to make the Rust portions of the repository feel like a serious Rust
codebase while respecting the reality of the Windows ecosystem around them.

---

## For .NET and Windows developers

If you come from .NET, C#, XAML, WinUI, or the broader Microsoft ecosystem, Rust
should not need to feel like an alien island inside the repository.

The architecture aims to keep responsibilities explicit and discoverable.

The migration is intentionally being shaped so that developers familiar with
either Rust or Microsoft application development can understand where product
responsibilities live and how the pieces interact.

---

## For software architects

WindowsRustTerminal is also a case study in a broader problem:

**How do you modernize a large system without throwing away everything the old
system already knows?**

The techniques explored here can apply beyond terminal emulators:

- C++ to Rust migration,
- legacy-system modernization,
- strangler-style architecture,
- behavioral characterization,
- contract testing,
- compatibility preservation,
- incremental rewrites,
- native interoperability,
- and evidence-driven architecture.

Windows Terminal happens to be the laboratory.

The engineering problem is much larger.

---

## Project status

WindowsRustTerminal is under active development.

Expect:

- incomplete product integration,
- architecture changes,
- temporary adapters,
- experimental branches,
- evolving build instructions,
- and migration infrastructure that may disappear once its purpose has been
  fulfilled.

The project should be evaluated by the evidence present at a particular
migration stage, not by assumptions about the final architecture.

---

## Building

The repository currently contains both the original Windows Terminal
infrastructure and the Rust migration work.

Because product integration is still evolving, build instructions may differ
depending on the migration stage and branch.

For the latest Rust development path, inspect:

```text
rust/
Cargo.toml
.github/workflows/rust-ci.yml
```

The original Windows Terminal build infrastructure remains available while the
product migration progresses.

More streamlined WindowsRustTerminal build and run instructions will replace
this section as product ownership converges.

---

## Contributing

Contributions that help answer the engineering questions of this fork are
welcome.

Particularly valuable areas include:

- behavioral compatibility,
- Rust implementations,
- Windows interoperability,
- test coverage,
- performance measurement,
- memory analysis,
- unsafe-code reduction,
- architectural simplification,
- build reproducibility,
- documentation,
- and migration tooling.

A useful contribution should ideally make one of these statements easier to
prove:

> The Rust implementation behaves correctly.

> The architecture became simpler.

> A boundary became safer.

> The product became easier to build or maintain.

> A previously unknown migration risk became measurable.

---

## A note on disagreement

Good engineers can examine the same system and choose different architectures.

That is normal.

One team may reasonably conclude that maintaining the existing C++ architecture
is the right investment.

Another engineer may reasonably be curious about what a Rust-first architecture
could become.

Both can be intellectually honest.

WindowsRustTerminal does not need another project to be wrong in order for this
experiment to be worthwhile.

The repository only needs to answer its own question well.

---

## The larger idea

Technology history contains many examples of useful things beginning with a
developer asking a question nobody assigned them to answer.

Sometimes the experiment fails.

That is useful information.

Sometimes it works only partially.

That is useful information too.

And occasionally the experiment reveals that something assumed to be
impractical was simply unexplored.

The important part is that the answer becomes inspectable.

Code can be read.

Tests can be rerun.

Benchmarks can be challenged.

Architectures can be compared.

Claims can be falsified.

That is one of the things I love most about software engineering.

You do not always have to win an argument.

Sometimes you can simply build the alternative and learn from the result.

---

# No hay problema.

A difficult problem is not an insult.

A disagreement is not an emergency.

A failed experiment is not wasted work if it produces knowledge.

And an open-source codebase is an invitation to learn.

WindowsRustTerminal began because I was curious whether this could be done.

It continues because the answer has become much more interesting than the
question.

**Build. Measure. Learn. Improve.**

No hay problema.

---

## Upstream project

WindowsRustTerminal is derived from:

**Microsoft Windows Terminal**  
https://github.com/microsoft/terminal

Please visit the upstream repository for the official Microsoft Windows Terminal
project, releases, documentation, support, roadmap, and contribution process.

---

## License

This project preserves the licensing requirements of the upstream Windows
Terminal source code.

See [LICENSE](./LICENSE) and the applicable notices in this repository for
details.

---

## Search and research topics

WindowsRustTerminal may be useful to engineers researching:

**Windows Terminal Rust rewrite, Windows Terminal Rust port, C++ to Rust
migration, large C++ codebase modernization, Rust Windows development, Rust
WinRT interoperability, Rust XAML integration, terminal emulator architecture,
VT parser Rust, ConPTY Rust, Windows console Rust, incremental rewrite strategy,
contract replay testing, characterization testing, legacy system modernization,
Rust FFI Windows, memory-safe systems programming, Rust migration case study,
behavioral compatibility testing, Windows native Rust application, terminal
emulator migration, and evidence-driven software architecture.**
