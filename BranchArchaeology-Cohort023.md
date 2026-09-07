# Branch Archaeology — Cohort 023

This cohort closes four surviving elevation/warning experiments whose durable behavior either entered reviewed upstream product work or whose product direction was explicitly rejected. A related branch, `dev/migrie/f/non-terminal-content-elevation-warning`, is intentionally deferred because it also contains generic non-terminal pane/UserControl groundwork that needs separate archaeology before the ref can be retired.

## `dev/migrie/f/--elevate`

- Historical fork tip: `729b349a7f546f43592437007a2ba7bb236d83b0`; this is later spelling maintenance. The functional head is `46c6403ac1dc95db533a9e998a3feeb331e63abc`.
- That functional head is the exact upstream head of draft PR `microsoft/terminal#12142`, `Add a --elevate flag`.
- Historical design: attach `--elevate` / `--no-elevate` to individual `new-tab` commands.
- The PR investigation exposed fundamental lifecycle/composition problems: persisted-window restoration could race monarch/window-manager lifetime, and command lines containing multiple elevated tab actions could spawn multiple unrelated elevated windows.
- The author closed #12142 and opened issue `#12191` to track a top-level `wt --elevate new-tab` design instead.
- Final product direction: `#12191` was closed on 2026-03-26. The maintainer explicitly recorded that the right solution is Windows `sudo`, so the Terminal-specific command-line elevation feature is no longer the intended product direction.
- Disposition: **NO-PORT / explicitly rejected product direction**.
- Recovery value: provenance and the negative design lesson that elevation is a process/window-level concern, not safely composable as an arbitrary per-`new-tab` flag.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/ctrl-click-elevate`

- Historical fork tip: `ec03c17e76686d15b7d3a4d4683315f4b6b923c7`; this is spelling maintenance. Functional commit: `26c1e684de9aac85b077f9100cce7117c761b526`, `the whole thing`.
- Historical intent: Ctrl+Click should launch a profile elevated; the branch also added the corresponding tooltip text and included elevation state in generated action names.
- Definitive successor: PR `microsoft/terminal#12209`, `Add support for ctrl+click on the dropdown to launch elevated`, merged as `e520779bce36acb575a750c112cdbb0a2e029536`.
- Commit-graph certification: the final PR head `a584b101e5a9e5689a497c49fc31e68e84134be9` has `26c1e684de9aac85b077f9100cce7117c761b526` as its exact merge base and is three commits ahead with zero commits behind. The historical functional branch is therefore a direct ancestor of the reviewed, merged implementation.
- Disposition: **ALREADY ABSORBED / direct ancestor of merged PR #12209**.
- Recovery value: provenance only; the product behavior lives in the merged lineage.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/just-elevated-state`

- Historical fork tip: `595721658a7f9fa549dc72b3a8a24ed501254200`; this is spelling maintenance. The useful 2021 line includes `306ad30753a204e936bd2152a2b09dee7cd23137` and earlier exploratory state-file work.
- Historical intent: establish an application-state abstraction and an elevated-only state store that could be read unelevated but written only from elevated Terminal instances.
- Definitive successor: PR `microsoft/terminal#11222`, `Add a file for storing elevated-only state`, merged as `c79334ffbbcbb0d0483579035328aebd27f517f4` from the rewritten sibling `dev/migrie/f/just-elevated-state-2`.
- The merged PR formalized `elevated-state.json`, its security/permission behavior, and tests. The old branch is not direct ancestry of the final head; it is an earlier prototype superseded by the reviewed rewrite.
- Disposition: **NO-PORT / superseded by merged rewritten successor #11222**.
- Recovery value: provenance only; use the merged state-file security model rather than the exploratory branch.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/warning-dlg-automation`

- Historical fork tip: `d0a21a91f6ed8a099a875ca81c7133c93ddc36ab`; this is spelling maintenance. Functional commit: `3b8e7236ae26d577e92043fe58ff448a15b4fd81`, whose message is `bunch of dead ends in this`.
- Historical intent: experiment with accessibility/automation behavior for the `AdminWarningPlaceholder` elevation warning, including automation notifications, initial focus, `AutomationProperties.Name`, and button event signaling.
- The underlying warning UI was developed in PR `microsoft/terminal#11308`, `Warn before the user runs a new commandline elevated`.
- Definitive product decision: merged PR `#12137`, `Profile auto-elevation, version 3` (`bc97af701e4061a18da111dbd00f5a766c6dfb13`), explicitly states that the team decided not to do #11308 and shipped profile elevation **without the warning**.
- The automation branch therefore targets a UI/product path that was explicitly abandoned, and its own functional commit identifies the work as dead ends.
- Disposition: **NO-PORT / dead-end automation for rejected warning design**.
- Recovery value: none requiring preservation as executable code; general accessibility contracts should be implemented against whatever current UI owns a security prompt, not against the abandoned `AdminWarningPlaceholder`.
- Branch retirement: **SAFE after this ledger commit**.

## Deferred sibling

`dev/migrie/f/non-terminal-content-elevation-warning` is **not** included in this deletion set. Its elevation-warning behavior was rejected by #12137, but PR #11308 also used the branch to make `Pane` host generic `UserControl` content. That non-terminal-content architecture needs its own successor/contract analysis before the ref is retired.

## Cohort 023 deletion set

- `dev/migrie/f/--elevate`
- `dev/migrie/f/ctrl-click-elevate`
- `dev/migrie/f/just-elevated-state`
- `dev/migrie/f/warning-dlg-automation`
