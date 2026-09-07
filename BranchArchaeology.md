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

Continue with the next chronological cohort, using later branches to explain and collapse earlier experiments wherever possible.
