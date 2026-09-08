# Workspaces branch archaeology

This ledger preserves the genealogy and disposition of the historical `dev/migrie/workspaces-real` branch.

## `dev/migrie/workspaces-real`

### Functional lineage

The branch diverged from main at `17e612659724cf82e49f803acac85ac12716ebbb` and contains seven exclusive prototype commits dated 2026-04-24 through 2026-04-27. The branch evolved `ApplicationState`, window/tab layout persistence, `WindowEmperor`, Terminal actions and the tab-row UI. Its intent was to preserve named-window state as reusable **workspaces**.

The prototype was followed immediately by the reviewed PR line:

- PR **microsoft/terminal#20162**, opened 2026-04-29: **Add support for "workspaces" based on window names**. It formalized the same contract: named windows persist their window layout and buffers independently; `openWorkspace` restores the named state; a flyout exposes saved/open workspaces. The PR closed issue #17084 and merged on 2026-06-04.
- PR **#20291** reverted #20162 later on 2026-06-04 after blocking review findings and unrelated changes were noticed.
- PR **#20314**, **Add support for "workspaces" based on window names (redux)**, explicitly rebooted #20162 with the event from #20311 and merged on 2026-06-16.

Current repository `main` contains `OpenWorkspace` and `Workspaces` in `AllShortcutActions.h`; the modern implementation also contains `OpenWorkspaceArgs`, workspace persistence/restoration and workspace UI. Therefore the contract explored by `workspaces-real` is present in the reviewed modern implementation rather than stranded on this prototype branch.

A contemporaneous `dev/cazamor/selfhost/2026-05-04-workspaces` ref also exists, but it is a separate developer namespace and is not declared disposable by this ledger. It should be evaluated independently against the merged PR genealogy before retiring that ref.

### Disposition

**NO-PORT / superseded** by the reviewed #20162 → revert #20291 → corrected #20314 lineage, with the final workspace behavior present on modern `main`.

Do not transplant the seven exploratory commits. Their useful contract and product intent were carried into the reviewed implementation, while the review/revert/redux sequence is stronger evidence than the prototype itself for the final architecture.

### Branch retirement

**SAFE TO DELETE: `dev/migrie/workspaces-real`**.

The unique historical intent and its destination are preserved here; the branch no longer contains unique knowledge requiring a live ref.
