# Javier de la Garza branch archaeology

This ledger preserves the useful intent recovered from historical `dev/jadelaga/**` refs while consolidating the namespace into `dev/jadelaga/main`.

## Cohort 001 — Visual Studio / WPF dependency restore

### `dev/jadelaga/VS-Pty.netFixes`

- Historical tip / functional commit: `69720d082f33bb21459dcb65acd0f2c8192d09b7` (2024-12-17).
- Semantic base: the branch is a servicing line over the 1.21 release family. Compared with `release/1.21`, it contains one functional commit (`ahead_by=1`), despite much larger distances to modern `main`.
- Intent: avoid NuGet restore failures when `src/cascadia/WpfTerminalControl/WpfTerminalControl.csproj` is consumed as a dependency, by setting `<ManagePackageVersionsCentrally>false</ManagePackageVersionsCentrally>` in that project.
- Successor: the exact same functional patch was replayed onto the 1.22 servicing line as commit `633a2f4faab6faf804d65e6657804d2de8c73c95` in `dev/jadelaga/VS-Pty.Net-1.22`.
- Disposition: **ALREADY ABSORBED by the 1.22 successor branch**.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/jadelaga/VS-Pty.Net-1.22`

- Historical tip: `ffb0bf7f1898e27fa61f188766e4eb434ddfb046`, a merge of `release-1.22` into the servicing branch.
- Functional commit: `633a2f4faab6faf804d65e6657804d2de8c73c95` (same five-line project-file delta as `69720d0`).
- Semantic base: compared with current `release/1.22`, the branch is only `ahead_by=2`; the only changed file is `WpfTerminalControl.csproj`. One of those commits is the servicing merge itself, so the durable product idea is the single package-management property above.
- Modern reading: current `main` still contains `WpfTerminalControl.csproj`, now targeting `net472;net8.0-windows`, and does **not** contain `ManagePackageVersionsCentrally`. Repository search also finds no current use of that property and no upstream Terminal PR matching it. Therefore the restore concern is not proven superseded.
- Why the old patch should not be transplanted blindly: the WPF target framework, package infrastructure, build graph and Visual Studio integration context have all moved on since the 1.21/1.22 servicing branches. A project-local opt-out may still be the correct fix, but applying it without reproducing the modern consumer failure would turn a historical workaround into an unverified package-management policy.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery guidance: reproduce the Visual Studio / external-consumer scenario against modern `main` with the Terminal WPF project consumed from a build that enables NuGet Central Package Management. Treat restore success/failure as the contract. If the failure still reproduces, apply the smallest modern scoped mitigation and certify it in CI; only then decide whether `ManagePackageVersionsCentrally=false` remains the correct implementation.
- Branch retirement: **SAFE after this ledger commit** because the unique behavior, provenance, non-port rationale and modern replay path are preserved here.

## Cohort 001 deletion set

Once this ledger commit is present on `dev/jadelaga/main`, the following historical refs no longer carry unique knowledge that requires a branch ref:

- `dev/jadelaga/VS-Pty.netFixes`
- `dev/jadelaga/VS-Pty.Net-1.22`

## Durable lane

Always retain `dev/jadelaga/main`.
