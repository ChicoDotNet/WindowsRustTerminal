# OOP Early Research — supplemental archaeology

These refs predate or sit alongside the named Process Model 2 generations. They are safe to retire after this note because their useful conclusions were either productized by reviewed PRs or invalidated by the later architecture.

## `dev/migrie/oop-tear-apart-control`
## `dev/migrie/oop-terminal.control-split-control`

These branches explored splitting the monolithic `TermControl` into a lower-level core/interactivity boundary. The idea was productized by merged PR #9820 (`8910a16f...`), which formally created `ControlCore`, `ControlInteractivity`, and `TermControl`, added tests, and documented the responsibilities of each layer. Preserve #9820 as the authoritative implementation rather than either prototype branch.

Disposition: **ALREADY ABSORBED / superseded by #9820**.

## `dev/migrie/oop-mixed-elevation-1`

This branch explored the mixed-elevation portion of Process Model 2. microsoft/terminal#5000 records the final conclusion for #1032: hosting elevated and unelevated content seamlessly in one window was not technically achievable under the security model. The supported alternative became per-profile elevation/new elevated windows (#632 and its follow-on implementation), not this OOP prototype.

Disposition: **NO-PORT / hypothesis rejected by security & process-model constraints**.

## `dev/migrie/oop-rpc-000`
## `dev/migrie/oop-broker-000`

These are RPC research laboratories (`src/tools/RpcResearch`) for crossing the proposed content/window process boundary. `oop-broker-000` evolves the raw RPC/proxy-stub experiments toward a packaged/broker-oriented model. They are not product implementations; they were mechanism research for Process Model 2. Process Model v3 removed the per-TermControl process boundary and therefore removed the requirement these experiments were trying to solve.

Disposition: **NO-PORT / research tooling for an abandoned boundary**.

## `dev/migrie/oop-window-content-1`

This is another ScratchIsland/ScratchWinRT experiment around hosting window-side UI separately from terminal content. It belongs to the same early content-process investigation later summarized in #5000 and superseded first by the reviewed OOP work, then entirely by Process Model v3.

Disposition: **NO-PORT / OOP hosting prototype superseded by later architecture**.

## Recovery guidance

For the split-control architecture use merged #9820. For historical Process Model 2/OOP reasoning use #5000 and PR #12938. For the shipped multi-window architecture follow Process Model v3 (#14825, #14843, #14851, #14866, #14901, #14935).
