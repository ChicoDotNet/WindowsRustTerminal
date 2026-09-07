# Carlos selfhost 1.24 archaeology

## Decision

The five `dev/cazamor/selfhost/1.24*` refs are integration/bugbash snapshots, not canonical feature branches. They combine `main`/release state with explicitly labeled `PRE-MERGE #NNNNN` feature branches. Their useful payload is preserved more authoritatively by the corresponding PRs/source branches, so the composite selfhost refs are safe to retire.

## Refs

- `dev/cazamor/selfhost/1.24`
- `dev/cazamor/selfhost/1.24-2025-04-29`
- `dev/cazamor/selfhost/1.24-2025-05-06`
- `dev/cazamor/selfhost/1.24-2025-05-15`
- `dev/cazamor/selfhost/1.24-2025-06-10`

## Payload provenance

The snapshots explicitly merge/test feature branches using commit messages such as `PRE-MERGE #18559`, `#18814`, `#18639`, `#18915`, `#18917`, `#18928`, and `#18953`.

- **#18559 — Extensions page**: merged 2025-05-28. Canonical implementation is the reviewed PR (`dev/cazamor/sui/extensions-page` upstream head).
- **#18814 — SSH Generator feature flag/UI polish**: merged 2025-05-31.
- **#18915 — ActionArgs reflection**: merged 2025-09-03.
- **#18917 — edit actions in Settings UI**: merged 2025-12-09; this is the production successor to the action-editor work exercised in the selfhosts.
- **#18639 — PowerShell installer stub**: closed without merge in 2026. Its dedicated recovery branch remains `dev/cazamor/sui/ext-page/powershell-stub`, and the PR retains review/discussion. The composite selfhost is not needed to preserve it.
- **#18928 — tmux control mode**: still open upstream in 2026. The PR/fork head is the canonical live implementation; the selfhost copy is only a pre-merge integration snapshot.
- **#18953 — configurable pane highlight colors**: closed without merge; the upstream PR retains the code/design discussion. Again, the selfhost copy is not canonical.

The dated selfhosts also include local merge/build fixes required to make combinations of these pre-merge features run together. Those fixes are specific to transient combinations and are not independent product contracts.

## Disposition

**SAFE TO RETIRE / integration snapshots superseded by canonical source PRs and later product merges.**

When investigating any feature formerly exercised by these selfhosts, go to its PR/source branch above rather than resurrecting a composite `selfhost/1.24*` ref.
