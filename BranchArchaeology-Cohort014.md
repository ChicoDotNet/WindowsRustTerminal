# Branch Archaeology — Cohort 014

This cohort covers five later `dev/migrie/b/**` branches from 2023–2024: a ConPTY hidden-window style experiment, two input/paste fixes, a pane-duplication crash fix, and a marks-clearing TDD branch.

As in previous cohorts, an open or recently closed issue does not by itself make an old implementation authoritative. We preserve the behavioral contract and negative experimental results, then prefer the later merged architecture.

## `dev/migrie/b/15536-or-15219-idk`

- Final branch head: `9dee1280af9e6ca033764785e0a5d3f87c8d84b9`.
- The branch became PR #16014, `Experiment with some conpty windowing fixes`, and was closed without merge on 2026-06-24.
- Its title is accurate: the author explicitly did not know whether the experiment targeted #15536, #15219, or possibly #13388.
- Useful experiment:
  - change the hidden ConPTY HWND from `WS_OVERLAPPEDWINDOW | WS_POPUP` to `WS_OVERLAPPED | WS_MINIMIZEBOX | WS_SYSMENU | WS_POPUP`;
  - remove `WS_EX_TOOLWINDOW` while retaining `WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_NOACTIVATE`;
  - place those styles behind `Feature_ConPtyHwndStyles`, enabled only for Dev/Canary branding so the experiment could be backed out.
- Useful negative knowledge from preceding commit `57ff674be77721d1b55246f56b431119bca58a66` and the issue investigation:
  - `WM_GETMINMAXINFO` did not fire as hoped for the owned popup window;
  - `SetWindowPlacement` with `WPF_SETMINPOSITION` and `(-32000,-32000)` did not position the hidden window as expected.
- #15536 was eventually closed in July 2026 after the original reporter confirmed the artifacts had not appeared for a long time; no causal fix was attributed to this abandoned PR.
- #15219 remains open/Needs-Repro, so this is **not** classified as fixing that issue.
- Disposition: **NO-PORT / recovery candidate; experimental style matrix and dead ends preserved here**.
- Branch retirement: **SAFE after this ledger commit**. If #15219 becomes reproducible again, start from current ConPTY HWND architecture and the negative results above, not this 2023 feature-staging branch.

## `dev/migrie/b/15803-activate-dont-copypasta`

- Head: `0a9ad5d453605141978a94c48b352bc9bad895fb`.
- Behavioral contract: a right-click that *activates/focuses* the Terminal window must not simultaneously paste the clipboard into the newly focused application.
- This branch became PR #16987. GitHub's PR metadata currently says `merged_at: null`, but merge commit `79506b2feaa5f17e397c7ba8cf876b6fa37de7b1` exists in `microsoft/terminal` with the exact branch head `0a9ad5d...` as its second parent.
- The merged implementation carries `thisClickFocused` through `TermControl` into `ControlInteractivity` and suppresses right-click paste when that click caused focus. It also includes regression-test changes.
- Disposition: **ALREADY ABSORBED / graph-certified merge**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/15812-broadcast-paste-two`

- Functional head: `35aba0cdc1520d794687005d8da63ea0974b85e6` (`this isn't the dumbest`).
- Historical problem: keyboard/action paste could be orchestrated across broadcast panes, but context-menu/right-click paste followed a callback path through `ControlInteractivity`/`TerminalPage` that bypassed `TermControl::_pasteTextWithBroadcast`.
- The branch experimented with intercepting `PasteFromClipboard` inside `TermControl`, replacing the callback, and funneling returned clipboard text through `_pasteTextWithBroadcast`; this avoided asking every pane to fetch the clipboard independently.
- Important limitation identified immediately in the #15812 discussion: the originating control could apply bracketed-paste semantics, while rebroadcast recipients received a raw write. Correctness therefore required carrying an input/paste source through the broadcast path rather than simply hairpinning strings.
- #15812 itself was later closed after shipping with only residual follow-up work.
- Definitive behavioral fix for the practical mouse/context-menu broadcast problem arrived in PR #19879, merged 2026-02-19 as `b6375cc0280b27b5ba5079f024a18a03bbb6f551`; its validation explicitly covers mouse paste with broadcast enabled.
- The architectural insight from this branch also survives in the newer 2026 input-plumbing work: PR #20155 proposes `WriteInputString(..., Source)` and removal of the callback/hairpin paste machinery so paste encoding and broadcast can be expressed explicitly. #20155 is still open, so it is cited as current design lineage, **not** as merged evidence.
- Disposition: **TRANSFORM / historical hairpin implementation superseded; contract and source-aware input insight preserved**.
- Branch retirement: **SAFE after this ledger commit**. Do not resurrect the callback interception; use current/next-generation source-aware input APIs.

## `dev/migrie/b/17075-its-me-the-killer`

- Functional tip: `7abb15fa6dec883dda0c85d62a03de6dd501c312`, `Don't explode when duplicating a pane`.
- Root cause: pane duplication failed to guard a null `INewContentArgs`.
- The same fix was re-established on current main as `dev/migrie/b/17075-but-on-main` and merged via PR #17110 on 2024-04-23, merge `87a9f72b9a323ef2e49db9e74bd740cc9c2aab31`, closing #17075 and #17076.
- Disposition: **NO-PORT / precursor superseded by merged on-main version**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/17130-clear-marks-2`

- Historical branch demonstrates the desired TDD sequence:
  - `0076e426077be37078220bcb3acba997876e6da7`: `start with a test`;
  - `36f35912805ef5c15e46b64d13c79ea8673e2396`: `actually fix this`.
- The final PR branch `dev/migrie/b/17130-clear-marks-2-try-2` replays the same commits/intent on current main (new SHAs `9454d15e...` and `3e677cc0...`) and adds follow-up refinement.
- PR #17144 merged 2024-05-02 as `92e05f246a7cc59262c42687bbcab15ecf0f109e`, explicitly closing #17130. Its description calls out the tests as valuable.
- Disposition: **NO-PORT / TDD contract replayed and merged on successor branch**.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 014 deletion set

The following refs are safe to retire:

- `dev/migrie/b/15536-or-15219-idk`
- `dev/migrie/b/15803-activate-dont-copypasta`
- `dev/migrie/b/15812-broadcast-paste-two`
- `dev/migrie/b/17075-its-me-the-killer`
- `dev/migrie/b/17130-clear-marks-2`
