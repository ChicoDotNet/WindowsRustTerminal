# Branch Archaeology — Cohort 012

## #13388 focus / activation experiments

This cohort covers the five historical `dev/migrie/b/13388-*` branches. `microsoft/terminal#13388` remains open in 2026 and has accumulated multiple focus/foreground failure modes, so this is **not** being classified as a solved issue. The goal here is narrower: preserve the still-useful diagnostic contract and the useful hypotheses, while retiring implementations that were explicitly experimental or later superseded by more mature HWND/focus handling.

A later important product lineage is `microsoft/terminal#17828` / commit `17a55da0f9889aafd84df024299982e7b94f5482`, which handles ConPTY HWND focus transitions explicitly (`WM_ENABLE`, pseudo-window `WM_ACTIVATE`, and focus transfer to the XAML island). It does not prove every variant of #13388 is fixed, but it is a materially more mature architectural reference than the 2022–2023 spikes below.

`microsoft/terminal#19520` is **not** treated as an authoritative successor: although it proposed a simple `content.Focus(FocusState::Programmatic)` fix and claimed to close #13388, that PR was closed without merge.

### `dev/migrie/b/13388-experiments-0`

- Functional commit: `c7d730b80d32a2cf95119c4e37904f5c174dee78` (`this is the thing Evan was talking about`).
- Historical intent: instrument and interfere with activation of the hidden pseudo-console window while investigating #13388.
- Patch shape: adds handlers for `WM_ACTIVATE`, `WM_MOUSEMOVE`, and `WM_MOUSEACTIVATE`; the mouse-activation path returns `MA_NOACTIVATEANDEAT`.
- Modern reading: useful as evidence that hidden-window activation/mouse activation was suspected as a focus-stealing vector, but this was diagnostic/spike code rather than a proven product algorithm. Later #17828 handles pseudo-window activation with a much more explicit focus-transfer model.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value: preserve the behavioral question — whether the hidden ConPTY HWND is becoming active or foreground unexpectedly — not this instrumentation implementation.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/13388-experiments-1`

- Functional commit: `7da382c948265d64e600a75f9c7fd49bfe38141a` (`This was the next theory Evan had for #13388`).
- Historical intent: on pseudo-window `WM_ACTIVATE`, find `GA_ROOTOWNER` and call `SetActiveWindow(ownerHwnd)` whenever the pseudo window is being activated.
- Modern reading: this captures an important hypothesis — activation should be redirected away from the pseudo-console HWND and toward the owning Terminal window — but `SetActiveWindow(ownerHwnd)` is an early formulation. #17828 later implements the focus transfer at the HWND/XAML-island boundary more deliberately.
- Disposition: **NO-PORT / superseded experiment**.
- Recovery value: retain the invariant that pseudo-console activation must not leave keyboard focus on a non-input HWND.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/13388-attempt-002`

- Historical tip: `3bed10aed0cde470290744fe77bae9c950a4c2e6` (`fix the build`).
- Functional commit: `68d854011e7091602e3a9f6c17e0ccf8cbbf2910`.
- Historical intent: pass the owner's HWND on the ConPTY command line so the pseudo-console side has the owner before its hidden window is created.
- Modern reading: the branch represents a useful lifecycle hypothesis — correct ownership should exist at window creation time, not be repaired afterward — but it is not a demonstrated final fix for the still-open #13388 family.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value: preserve the creation-time ownership invariant for future reproduction/contract work; do not transplant this historical command-line plumbing blindly.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/13388-attempt-003`

- Functional commit/tip: `55bdee26a57876d743e51af56c9b73ed93da4068`.
- Historical intent: another attempt to use activation state (`GetActiveWindow`) to recover the Terminal owner.
- The commit itself says the author does not think the idea will work, questions whether it differs from the earlier attempt, and calls the `GetActiveWindow` check useless.
- Disposition: **NO-PORT / rejected experiment**.
- Recovery value: negative knowledge only — do not use this branch as evidence that `GetActiveWindow` identifies the owning Terminal in this scenario.
- Branch retirement: **SAFE after this ledger commit**.

### `dev/migrie/b/13388-focus-logger`

- Historical tip: `211f3160a5884ed34dc173046798f0170bd68e26` (`Or, in powershell`).
- Historical intent: create a small reusable foreground-process sensor while debugging #13388.
- Unique artifact: `src/tools/scratch/focus-logger.ps1`, which polls `GetForegroundWindow` / `GetWindowThreadProcessId` and reports the process that acquires foreground.
- Modern reading: unlike the speculative fixes above, this remains directly useful because current #13388 reports still revolve around distinguishing Terminal losing focus internally from another process taking foreground.
- Disposition: **PORT / diagnostic sensor**.
- Port: recovered to `dev/migrie/main` as `src/tools/scratch/focus-logger.ps1` in commit `a323742f75aa30e035472c2b1420d060dec3c06a`, with provenance retained in the file header.
- Branch retirement: **SAFE after this ledger commit** because the unique executable diagnostic has been preserved on the curated lane.

## Cohort 012 deletion set

Once this ledger file is present on `dev/migrie/main`, all unique knowledge from this family is either represented by the curated diagnostic sensor, this recovery record, or later upstream focus architecture. The following historical refs are therefore safe to retire:

- `dev/migrie/b/13388-attempt-002`
- `dev/migrie/b/13388-attempt-003`
- `dev/migrie/b/13388-experiments-0`
- `dev/migrie/b/13388-experiments-1`
- `dev/migrie/b/13388-focus-logger`
