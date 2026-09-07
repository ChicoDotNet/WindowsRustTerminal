# Cohort 010 — windowing, settings, scrolling and focus experiments

This cohort preserves the architectural or diagnostic value of a dozen `dev/migrie/b/**` refs whose implementation ownership either moved into reviewed upstream fixes or whose old prototype is no longer an appropriate implementation candidate.

## `dev/migrie/b/9053-part-3-the-actual-doing-of-the-thing`

- Issue: `microsoft/terminal#9053`, honoring startup show state (`start /min`, `/max`, ShellExecute/STARTUPINFO) in the multi-window process model.
- This branch became the upstream head of PR #13838.
- Final fix: PR #13838, merge `2c16e7c07b496e2bfbd399d2534d7d3a0ebb1db5`, passes the originating process `wShowCmd` from `STARTUPINFO` to the actual long-lived Terminal process and closes #9053.
- Disposition: **ALREADY ABSORBED**.
- Recovery value: startup visibility belongs to the originating invocation and must survive handoff into the long-lived process; `SW_SHOWDEFAULT` on the already-running process is insufficient.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/9053-part-4-i-guess-defterm`

- Historical intent: explore applying the same startup-show-state handling to Default Terminal/defterm initiated connections.
- Final design explicitly rejects the need for this path: PR #13838 notes that defterm refuses handoff for minimized console applications before Terminal receives them. Carrying `/min` into glomming would introduce ambiguous combinations such as minimized and maximized invocations targeting the same window.
- Disposition: **NO-PORT / explicitly rejected by final design**.
- Recovery value: keep the boundary clear — ShellExecute/wt handoff preserves `wShowCmd`; defterm filtering owns minimized-console behavior before Terminal.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/9320-interfacial-separation`

- Issue: `microsoft/terminal#9320`, Settings UI popups remaining open/detached when a XAML Islands window moves or a ScrollViewer scrolls.
- This was an earlier iteration of the work that became `dev/migrie/b/9320-interfacial-separation-2` and upstream PR #10922.
- Final fix: PR #10922, merge `29be8564f6ab4c34b46abd8099adbd02286574d1`, manually dismisses popups on window movement/SUI scrolling and centralizes ScrollViewer handling; closes #9320.
- Disposition: **NO-PORT / superseded by merged successor iteration**.
- Recovery value: XAML Islands do not automatically provide all popup-dismissal behavior of a native XAML app; the explicit `DismissAllPopups` seam is intentional compatibility glue.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/10332-less-snappy-scrolling`

- Issue: `microsoft/terminal#10332`, apparent blank-line scrolling after ConPTY-backed console resize.
- Historical experiment changed snap behavior and was later judged overkill.
- The issue was closed as answered. Mike Griese explicitly records that the branch delta is not relevant once the profile uses `"snapOnInput": false`; the remaining blank lines are inherent to how ConPTY maintains the console-buffer/terminal illusion during resize.
- Disposition: **NO-PORT / diagnostic experiment superseded by existing setting**.
- Recovery value: distinguish ConPTY-emitted resize history from terminal viewport snapping. Before changing buffer semantics, test whether `snapOnInput:false` already expresses the desired UX.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/10609-sui-leak`

- Issue: `microsoft/terminal#10609`, Settings UI memory growth and related save/reload issues; still open and currently externally blocked on WinUI3 work.
- Historical functional commit `1005e0dfc58de12d2972cddf4498384cf4346bf1` replaces strong captures of the profile page in PropertyChanged/Initialized callbacks with a `weakThis` capture. The author described the result only as "this certainly makes it a bit better".
- This is a useful leak hypothesis, not a complete fix for the multi-symptom issue.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value: when re-investigating SUI lifetime leaks, audit event/callback capture graphs first and prefer weak ownership where the source can outlive the page. Reproduce on current WinUI/Settings UI before transplanting 2022 lifetime code.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/10875-but-more-clever`

- Issue: `microsoft/terminal#10875`, scancode/vkey ActionMap layering made quake-mode/scancode bindings impossible to override normally.
- Upstream diagnostic PR #10907 deliberately added logging/tests while Leonard Hecker worked on the real correction.
- Final fix: PR #10917, merge `d465a47bc5bdd6eca2620a67144d139226d1d617`, ensures `KeyChord` has a vkey and layers bindings using the resolved vkey; tests pass and #10875 is closed.
- Disposition: **NO-PORT / superseded by tested upstream fix**.
- Recovery value: normalize physical/scancode bindings to the logical key identity used for layering rather than allowing representation details to prevent overrides.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/11092-unfocused-acrylic-settings`

- Issue: `microsoft/terminal#11092`, focused and unfocused opacity/appearance behavior.
- The old branch predates the final focused/unfocused appearance model and overlaps acrylic experiments that were later separated from opacity semantics.
- Final fix for #11092: PR #15974, merge `27e1081c8cc417fade9e053b7b6738a6c382f7b9`, adds distinct focused/unfocused opacity handling while preserving runtime opacity changes. Related acrylic ownership was separately formalized by #15923.
- Disposition: **NO-PORT / superseded by final appearance model**.
- Recovery value: runtime opacity is distinct from configured focused/unfocused opacity; focus transitions must not erase a user's runtime adjustment.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/11146-next-tab-in-cmdpal`

- Issue: `microsoft/terminal#11146`, Next/Previous Tab from the command palette could reset selection to the first item when filter text changed.
- Final fix: PR #16858, merge `806d5e2d05a31bf042b6d98aeacd33d24880d189`, ignores the relevant `_filterTextChanged` transition while in `TabSwitchMode`; includes validation/tests and closes #11146.
- Disposition: **ALREADY ABSORBED / later tested fix**.
- Recovery value: tab-switch mode owns selection independently from normal command-palette filter reset behavior.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/11522-dumb-idea`

- Issue: `microsoft/terminal#11522`, switching Windows input method resets Terminal session colors and zoom. **The issue remains open.**
- Historical branch is explicitly an exploratory idea, not a certified fix.
- Later lineage refined the diagnosis substantially:
  - #14498 explored avoiding a full UI reload on keyboard-layout changes and was closed stale.
  - #19933 (2026) proposed targeted re-resolution of `sc(nnn)` bindings instead of `ReloadSettingsThrottled()`, specifically to preserve runtime colors/zoom; it was closed without merge.
  - #20230, merge `8fe6c21ef88a73a7985b5968ee18936928ccac69`, now preserves runtime font-size delta across settings reloads, addressing one symptom but only references #11522 rather than closing it.
- Disposition: **NO-PORT / recovery candidate; product bug remains open**.
- Recovery value: the durable design direction is to treat keyboard-layout change as an input-binding invalidation event, not as a reason to reload all settings/session appearance. Re-resolve only layout-dependent bindings and preserve runtime state; also account separately for OEM punctuation and OS-registered global hotkeys.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/11561-dead-ends`

- Issue: `microsoft/terminal#11561`, visible blank/transparent frame during Terminal initialization.
- The branch records abandoned startup/window-creation paths. The first broad fix (#12979) was reverted because it introduced resize problems on external displays.
- Final fix: PR #13811, merge `083fc647bb1c59bacf2906504780fe7993aa6cd1`, keeps window creation in `_HandleCreateWindow`, creates it hidden, and only shows it from `_AppInitializedHandler` after initialization. It deliberately separates #9053 startup-state handling into its own reviewable fix.
- Disposition: **NO-PORT / dead ends superseded by corrected window lifecycle**.
- Recovery value: create the HWND at the normal lifecycle point but defer visibility; do not move window creation itself into late initialization merely to avoid flashing incomplete UI.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/11668-i-think`

- Issue: `microsoft/terminal#11668`, drag/drop into elevated Terminal with UAC behavior; closed as duplicate and marked as tracking an external/platform issue.
- Historical functional commit `56fe974f43f8d6c645abc1d499fa985cf48af637` is explicitly tentative: "I _think_ this is what @eryksun was talking about in #11668".
- The old ref is therefore a platform-behavior investigation, not owned product functionality awaiting integration.
- Disposition: **NO-PORT / external-platform recovery note**.
- Recovery value: elevated drag/drop crosses Windows integrity/UIPI and shell/OLE boundaries; resolve current platform constraints and the canonical duplicate/external tracker before attempting Terminal-local code.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/11743-win10-opacity-is-hard`

- Issue: `microsoft/terminal#11743`, Windows 10 opacity/acrylic behavior after the settings refactor.
- This branch is an earlier experiment; the reviewed successor was `dev/migrie/b/11743-win10-opacity-is-hard-02`.
- Final fix: PR #12255, merge `15a047512962548efb9ffe9bbf86bdb4abd93da8`, restores the Windows-10-specific behavior where setting opacity also enables the acrylic brush; manually validated on a physical Windows 10 machine and closes #11743.
- Disposition: **NO-PORT / superseded by merged successor iteration**.
- Recovery value: Windows 10 acrylic behavior has platform-specific activation semantics that cannot be inferred solely from the newer settings model; validate on real Win10 hardware when touching this path.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 010 deletion set

- `dev/migrie/b/9053-part-3-the-actual-doing-of-the-thing`
- `dev/migrie/b/9053-part-4-i-guess-defterm`
- `dev/migrie/b/9320-interfacial-separation`
- `dev/migrie/b/10332-less-snappy-scrolling`
- `dev/migrie/b/10609-sui-leak`
- `dev/migrie/b/10875-but-more-clever`
- `dev/migrie/b/11092-unfocused-acrylic-settings`
- `dev/migrie/b/11146-next-tab-in-cmdpal`
- `dev/migrie/b/11522-dumb-idea`
- `dev/migrie/b/11561-dead-ends`
- `dev/migrie/b/11668-i-think`
- `dev/migrie/b/11743-win10-opacity-is-hard`
