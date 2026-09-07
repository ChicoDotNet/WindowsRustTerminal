# Mike Griese branch archaeology

This ledger preserves the intent and disposition of historical `dev/migrie/**` work while the inherited branch namespace is consolidated into `dev/migrie/main`.

The goal is not to archive every branch by merging it. The goal is to understand the work chronologically, recover only contracts or implementation ideas that are still valuable, and then retire historical refs without losing the reasoning needed to revisit them.

## Method

1. Work from the oldest functional experiments toward newer ones.
2. Do not trust branch tip dates blindly: later mechanical maintenance (for example spelling/configuration churn) can make old experiments look newer than they are.
3. Prefer the earliest functional commit, merge-base/ancestry, related issue history, and later successor branches when reconstructing chronology.
4. Classify each historical branch as one of:
   - **NO-PORT / superseded** — a later implementation solved the same problem more completely.
   - **NO-PORT / recovery candidate** — the historical patch should not be transplanted, but its still-relevant behavior is recorded for a fresh modern implementation or regression test.
   - **PORT** — a contract, test, or implementation remains uniquely valuable and should be carried into the curated lane.
   - **ALREADY ABSORBED** — the branch contributes no unique work relative to the modern baseline.
5. Once the useful intent, provenance, and recovery path are preserved here (and any required code has been integrated), the historical branch ref may be deleted.

## Cohort 001 — 2019 console behavior experiments

### `dev/migrie/b/411-init-tab-stops`

- Historical tip: `da6dc0693ca36c2f2af7cd8d67ffedc08daddc2d`
- Historical intent: implement initial/default tab stops for issue `microsoft/terminal#411`.
- Modern reading: this line of work was later superseded by the more complete tab-stop reimplementation associated with `microsoft/terminal#5173`.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond provenance; do not transplant the 2019 implementation into the modern product.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/1223-change-256-table`

- Historical tip: `608c660bfb6263dd1b896538a11cffc1d6fb2d26`.
- Historical intent: render through the full 256-color table and preserve the color index rather than collapsing it to the current palette value.
- The branch itself left an explicit unresolved concern around `TextAttribute::IsLegacy` semantics.
- Modern reading: the later fix associated with `microsoft/terminal#5834` solved `#1223` more completely by distinguishing 16-color and 256-color indexing/table behavior, addressing the design hole left by this experiment.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond provenance; the later design is the authoritative direction.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/663-paste-lf-always`

- Historical tip: `c0d7d56b00f1d7aa743e107c82a1706933a6c68f`.
- Historical intent: fix `microsoft/terminal#663` by converting pasted LF (`\n`) to CR (`\r`) even outside Virtual Terminal input mode.
- Historical patch shape: a deliberately small change in `Clipboard::TextToKeyEvents` that removed the `IsInVirtualTerminalInputMode()` gate from LF-to-CR normalization. The commit itself questioned whether that unconditional behavior was safe and what it might regress.
- Modern reading (2026): `#663` remains open and the modern implementation still gates LF-to-CR conversion on Virtual Terminal input mode. The old patch therefore represents a still-live problem, but not a sufficiently proven solution.
- Existing modern test seam: `src/host/ut_host/ClipboardTests.cpp` already exercises `Clipboard::TextToKeyEvents`, so a fresh characterization/regression test can be built without preserving the old branch.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value: retain the behavioral question — multiline LF-only paste into non-VT console input — and solve it from the modern baseline with a focused test before changing semantics.
- Branch retirement: **SAFE after this ledger commit**. The historical patch is intentionally not carried forward; its contract, provenance, and modern recovery path are preserved here.

## Cohort 001 deletion set

After the commit that introduced this ledger is present on `dev/migrie/main`, the following historical refs no longer carry unique knowledge that requires a branch ref:

- `dev/migrie/b/411-init-tab-stops`
- `dev/migrie/b/1223-change-256-table`
- `dev/migrie/b/663-paste-lf-always`

## Cohort 002 — late-2019 / early-2020 experiments resolved by later product work

### `dev/migrie/b/2011-reordered-fallthrough-strings`

- Historical tip: `6a4c3248791cb8966b4869c54ab55bc5edfdc7ef`.
- Historical intent: investigate and reshape the state-machine/output path around the VT sequence reordering reported in `microsoft/terminal#2011`.
- Modern reading: upstream later fixed `#2011` with `microsoft/terminal#4896` (`ffd8f53529fb3ea33afb3cb0a4c8d9faa45afa8a`), flushing immediately when ConPTY encounters an unknown string. That change explicitly superseded the earlier approach in `#2665` and included tests.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond provenance; the later flush-on-unknown design is the authoritative solution.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/2455-try-getting-tests-working`

- Historical tip: `37af3f00fe1245a3e6368a07edc9d46ad99ee19b`.
- Historical intent: explore tests and guards around missing profiles for `microsoft/terminal#2455`.
- The historical tip explicitly records that the attempted test approach did not work and that the investigation was stumped at that point.
- Modern reading: upstream later closed `#2455` with `microsoft/terminal#5090` (`b3fa88eaedf3ba227a64c95ebc6b20b87a6af54b`), handling nonexistent profiles and adding working settings/UI-oriented tests.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond provenance; do not resurrect the failed 2019 test scaffolding.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/3088-weird-exact-wrap-resize`

- Historical tip: `245c8edd7f08790e688d420ac7204659a96ce444` (tip date is polluted by later spelling maintenance).
- Historical intent: investigate exact-width wrapping/resizing behavior associated with `microsoft/terminal#3088`.
- Modern reading: the problem survived several generations of resize/reflow work. Upstream eventually closed `#3088` in 2023 with the reimplementation of `TextBuffer::Reflow` in `microsoft/terminal#15701` (`74748394c17c168843b511dd837268445e5dfd6c`), with unit and feature coverage and a substantially simpler Unicode-safe algorithm.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond historical context; the modern reflow implementation is materially more mature than this experiment.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/3490-resize-down`

- Historical tip: `c091471fa4928e16d50927cad167fc607f47f63e`.
- Historical intent: one step in the investigation of repeated-resize/reflow corruption for `microsoft/terminal#3490`; the branch notes that its version made the associated test work.
- Modern reading: this was an intermediate algorithm, not the final architecture.
- Disposition: **NO-PORT / superseded**.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/3490-a-simpler-resize`

- Historical tip: `2fe6d8e6e4811618aa8ce45586303117a8c860f3`.
- Historical intent: simplify the resize algorithm enough that most tests passed.
- Modern reading: it remained experimental and partial. Upstream later solved `#3490` as a side effect of proper terminal-buffer reflow in `microsoft/terminal#4741` (`93b31f6e3f19ed4f4ffb55fad15be9b889b50f04`). That final change explicitly explains that the earlier heavy ConPTY approach made the problem worse and carries the large `#3490` test forward.
- Disposition: **NO-PORT / superseded**.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/3490-try-another-resize-algo`

- Historical tip: `41eeda1b4e6e2a0bc40f8640f3fe4c2ff7c6bca1`.
- Historical intent: another resize algorithm attempted after problems seen in `#4354`.
- The branch tip explicitly concludes that the attempted solution was not actually fixing the problem and says to step back and try again.
- Modern reading: the subsequent `#4741` reflow design is the successful successor and closes `#3490` with tests.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond documenting the discarded path.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 002 deletion set

Once this cohort is recorded on `dev/migrie/main`, these refs no longer carry unique implementation or contract knowledge that requires preservation as branches:

- `dev/migrie/b/2011-reordered-fallthrough-strings`
- `dev/migrie/b/2455-try-getting-tests-working`
- `dev/migrie/b/3088-weird-exact-wrap-resize`
- `dev/migrie/b/3490-resize-down`
- `dev/migrie/b/3490-a-simpler-resize`
- `dev/migrie/b/3490-try-another-resize-algo`

## Cohort 003 — early feature prototypes with definitive upstream successors

### `dev/migrie/b/1503-try-messing-with-cooked-read`

- Historical tip: `bb89a7c3ad784744cab9c397ad1a281de5452f5a`; its apparent 2020 date is contaminated by the later `spelling-0.0.21` migration.
- Historical intent: investigate `microsoft/terminal#1503`, where `COOKED_READ` could not correctly edit/display complex Unicode input such as emoji.
- Modern reading: upstream closed `#1503` in `microsoft/terminal#15783` (`821ae3af2d311350fbfaa4c77af09858488681e9`) by rewriting `COOKED_READ_DATA`. The successor explicitly targets going beyond UCS-2, validates surrogate-pair input, reduces the implementation substantially, and closes `#1503` along with several related cooked-read defects.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none that requires preserving this exploratory branch; the modern cooked-read rewrite is the authoritative implementation and the historical problem remains documented by the issue and successor commit.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/f/603-vintage-opacity`

- Historical tip: `b1eb406a4ec7efac800f0a25b7af76bec21f7679`, whose message calls the implementation a rudimentary and unusual test of `microsoft/terminal#603`.
- Historical intent: experiment with traditional-console-style non-acrylic opacity in Windows Terminal.
- Modern reading: upstream later implemented the product feature in `microsoft/terminal#11180` (`74f11b8203a3f297372630e164b7b4d51f82d83e`), adding supported vintage opacity, integrating it with appearance settings and mouse-wheel adjustment, adding/passing tests, and explicitly closing `#603`.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond provenance; the shipped settings/appearance implementation supersedes the rudimentary prototype.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/f/632-elevated-profiles`

- Historical tip: `c9b54dd21e5be153a4b00c6d33db30e487d92831`; its tip is mechanical spelling maintenance rather than the age of the functional experiment.
- Historical intent: explore per-profile elevation for `microsoft/terminal#632` instead of requiring the whole Terminal application to start elevated.
- Modern reading: after multiple design iterations and the elevation-QOL specification, upstream closed `#632` with `microsoft/terminal#12137` (`bc97af701e4061a18da111dbd00f5a766c6dfb13`). The final implementation introduced profile `elevate` semantics, action-level elevation through `NewTerminalArgs`, the required elevation shim, and tests.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond documenting the early exploration; the final elevation architecture is materially more complete and security-aware.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/f/632-on-warning-dialog`

- Historical tip: `10b97a621f862cb9591e5bda0dd8506ec68f49f8`; again, the tip is polluted by later spelling maintenance.
- Historical intent: refine the elevation flow around `#632`, specifically the warning/dialog behavior that evolved during the elevation design work.
- Modern reading: the definitive upstream `#12137` commit explicitly records that the final work was manually assembled using the diff ending at `dev/migrie/f/632-on-warning-dialog`. The useful delta from this branch therefore entered the final product lineage rather than remaining stranded here.
- Disposition: **ALREADY ABSORBED**.
- Recovery value: provenance only; the branch's useful work is represented by the final upstream implementation.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 003 deletion set

Once this cohort is recorded on `dev/migrie/main`, these refs no longer carry unique implementation or contract knowledge that requires preservation as branches:

- `dev/migrie/b/1503-try-messing-with-cooked-read`
- `dev/migrie/f/603-vintage-opacity`
- `dev/migrie/f/632-elevated-profiles`
- `dev/migrie/f/632-on-warning-dialog`

Continue chronologically. Treat branch-name issue numbers as hints, not dates; later spelling migrations can obscure the actual age of the functional experiment.
