# OOP research/refactor archaeology

## Productized refactor prototypes

### `dev/migrie/oop-tear-apart-control`
### `dev/migrie/oop-terminal.control-split-control`

Both explored separating the monolithic `TermControl` into a lower-level terminal core and UI/interactivity layers. The reviewed implementation shipped in PR #9820 (`8910a16f...`): `ControlCore`, `ControlInteractivity`, and `TermControl` became explicit components, with tests and documented ownership boundaries.

Disposition: **ALREADY ABSORBED / superseded by #9820**.

## Rejected mixed-elevation research

### `dev/migrie/oop-mixed-elevation-1`

This explored the mixed-elevation goal from Process Model 2. The canonical tracker #5000 records the final resolution of #1032: securely hosting High-IL and Medium-IL terminal content seamlessly in one window was not technically achievable. Supported elevation evolved instead around elevated profiles/new windows.

Disposition: **NO-PORT / rejected by security & process constraints**.

## RPC / broker mechanism research

### `dev/migrie/oop-rpc-000`
### `dev/migrie/oop-broker-000`

These branches are research tools under `src/tools/RpcResearch`, moving from low-level RPC/proxy-stub experiments toward a broker/package model. Their purpose was to find a mechanism for a window/content-process boundary. They are not product implementations. Process Model v3 eliminated the per-TermControl content-process boundary, making this mechanism research obsolete for the shipped architecture.

Disposition: **NO-PORT / research tooling for abandoned Process Model 2 boundary**.

## Window/content hosting experiment

### `dev/migrie/oop-window-content-1`

ScratchIsland/ScratchWinRT experiment separating window-side UI from terminal content. It belongs to the early OOP hosting investigation later formalized in Process Model 2 and ultimately replaced by Process Model v3's single-process, multi-window model.

Disposition: **NO-PORT / hosting prototype superseded by later architecture**.

## Recovery references

- TermControl layering: merged PR #9820.
- Process Model 2 history and security conclusions: issue #5000.
- Canonical abandoned content-process implementation: PR #12938.
- Shipped multi-window architecture: #14825 → #14843 → #14851 → #14866 → #14901 → #14935.
