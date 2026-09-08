# Keyboard selection branch archaeology

This ledger preserves the intent and disposition of `dev/migrie/gh-10824`.

## `dev/migrie/gh-10824`

### Functional history

Relative to its historical merge base `4f6f3b98b828cf75259c80fd285b7d4c15e22f19`, this branch contains six commits:

- `5b51af102149e80d44f1d1e4e8015e17b4ed1b63` — **Implement Keyboard Selection**.
- `45af92d1af3ef2155a86a7a5fd6d53fbb0fa0f3e` — apply review feedback.
- `b371eed864681fbe7ed5d8cbf38a29433318843b` — fix a bad merge.
- `0cc9bd0aadbfcafc7ca9f7cef405b61fa74f3fde` — fix schema.
- `ad9900857aa44b3ccbde148cc5ecb53d1a2e5b18` — CI-build repair for Carlos's branch.
- `516a923ac66d58807b40851216a7a43a9b337c4b` — mechanical spelling-0.0.21 migration.

The functional contract is keyboard-driven selection in the shared terminal core: Shift+arrows move by character, Ctrl+Shift+Left/Right move by word, Shift+Home/End move to line boundaries, and Ctrl+Shift+Home/End move to buffer boundaries, including wide-glyph, pivot-crossing and viewport-scroll behavior.

### Authoritative destination

The branch is a support/checkpoint line for **microsoft/terminal#10824 — Implement Keyboard Selection**. That PR documents the same contract, references #715 and spec #2840, targets the ControlCore/TerminalSelection layer, and was merged on 2021-09-23 as merge commit `c070be12d3639bbe1944658aacb56728ab5ea8c7` for Terminal v1.12.

The reviewed PR head continued independently from the same historical base; consequently the six commits on this helper branch are not the authoritative final patch. The final reviewed implementation contains the product behavior, while the helper branch's merge/schema/CI/spelling repairs are either incorporated into the final PR line or made obsolete by its successful merge and later repository evolution.

### Disposition

**ALREADY ABSORBED** by merged PR #10824.

There is no reason to replay the helper branch commits. If keyboard-selection behavior ever regresses, the durable contract should be reconstructed from PR #10824 / issues #715 and #2840 and tested against modern `TerminalSelection`, rather than resurrecting this branch.

### Branch retirement

**SAFE TO DELETE: `dev/migrie/gh-10824`**.
