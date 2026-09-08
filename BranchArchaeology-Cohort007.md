# Dustin Howett branch archaeology — Cohort 007

This supplemental ledger entry continues `BranchArchaeology.md` and `BranchArchaeology-Cohort006.md` on the permanent `dev/duhowett/main` consolidation lane. It records the next oldest unresolved families. Historical refs may be deleted only after the useful behavior, provenance, or recovery contract is captured here.

## Result

Safe to retire after this ledger commit:

- `dev/duhowett/compiler-compliance`
- `dev/duhowett/applicableactions`
- `dev/duhowett/copylink`
- `dev/duhowett/conpty_first_frame_blug`
- `dev/duhowett/wprp`
- `dev/duhowett/hax/tap_upgrade`

Keep:

- `dev/duhowett/no-private-registry-keys` — rejected upstream experiment with an unresolved behavioral distinction; do not port blindly.
- `dev/duhowett/font-64` — still protected by open upstream issue #3123 as recorded in the primary ledger.

## `dev/duhowett/compiler-compliance`

**Classification: ALREADY ABSORBED / toolchain drift.**

The branch is eight small compiler-correctness commits from February 2023, not a hidden product feature. Its contracts are mechanical correctness under stricter compilers/linkers:

- uppercase `PRIVATE` in the module-definition file for lld-link;
- place `[[nodiscard]]` before `virtual` / `static`;
- quote local `precomp.h` includes consistently;
- use `winrt::resume_foreground(Dispatcher())` instead of awaiting a dispatcher directly;
- fix dependent-name `typename` and declaration ordering in settings templates;
- make conversions such as `til::color` -> WinRT color explicit when overload resolution is ambiguous;
- add the explicit TerminalCore -> MidiAudio project dependency;
- avoid directly constructing the historical JsonCpp `CharReaderBuilder` in the affected path.

Current `main` already carries the maintained forms of these rules. Representative checks are exact or stronger: `src/propsheet/console.def` already uses uppercase `PRIVATE`; TerminalCore already explicitly references `src/audio/midi/lib/midi.vcxproj`; current renderer interfaces use `[[nodiscard]] virtual`; current UI code uses maintained coroutine helpers; and the old `CharReaderBuilder` use that motivated the tip no longer exists in the current tree.

There is no runtime behavior to replay and no unique compatibility contract left on this ref.

**Disposition: SAFE TO DELETE.**

## `dev/duhowett/applicableactions` -> `dev/duhowett/copylink`

These are one genealogy, not two independent features.

### `applicableactions`

Exact upstream PR: microsoft/terminal #15481, **“Switch context menu generation to use a flags enum of actions”**. The PR remained a draft and was closed without merge on 2026-07-31. Its final head is the branch tip `740f9f4452ebba9b10c2091d97d80b9007552c84`.

Intent:

- stop scaling the context menu through one bespoke WinRT predicate per item;
- compute a flags enum of actions applicable at the context-click position;
- let `TermControl` translate those flags into menu visibility;
- keep ownership of the interaction position in the UI/interactivity layer rather than making Core remember UI state.

Current `main` has evolved its context menu and now includes paste, copy, select-command, select-output and search surfaces, but it did not adopt the exact `MenuAction` experiment. The three-year-old draft is therefore not a patch to cherry-pick.

**Classification: NO-PORT / recovery contract.**

Recovery contract if context-menu scalability becomes a problem again:

1. Compute applicability once for the terminal position where the menu was invoked.
2. Represent applicability as data/flags rather than a growing family of one-off UI predicates.
3. Keep transient menu-position state out of the terminal core unless the core truly owns that interaction.
4. Render menu entries from that applicability result in the current menu architecture.

### `copylink`

`copylink` is a strict descendant of `applicableactions`: it is two commits ahead and zero behind that branch. Its functional commit is `e775b8807a918005e50c26a8aeafcb78389f249c`, **“Add a 'copy link' button”**.

The prototype adds `MenuAction::CopyLink`, exposes hyperlink lookup at a terminal position, shows **Copy link** only when the context-click is over a hyperlink, and copies that URI through the existing clipboard event surface. It adds the action to both the selection and non-selection context menus.

Current `main` does not expose a Copy Link menu item, and there is no active upstream PR for this branch. The old implementation depends on the unmerged `applicableactions` architecture, so it should not be cherry-picked.

**Classification: NO-PORT / recovery candidate.**

Behavioral recovery contract:

1. When opening the terminal context menu over a hyperlink, expose a **Copy link** action.
2. The action must use the hyperlink at the invocation position, not stale hover state and not arbitrary selected text.
3. Hide/omit the action when no hyperlink exists at that position.
4. Preserve the behavior in both selection and non-selection context-menu modes.
5. Route the URI through the maintained clipboard abstraction rather than adding an independent clipboard path.

The genealogy and contract are now preserved here, so both old refs are disposable.

**Disposition: SAFE TO DELETE `applicableactions` and `copylink`.**

## `dev/duhowett/conpty_first_frame_blug`

Functional commit: `c9e206997106eeecbaa287d9f41e4ccdd6f835e1`, **“HAX: Try to fix the conpty first frame boog”** (2022-02-17). The later branch tip is only spelling migration.

The branch adds a focused ConPTY output test and changes the old VT renderer optimization that stripped spaces on a full first-frame clear. The bug contract is subtle: a first frame containing cells with non-default background attributes must not discard trailing spaces, because those spaces are carrying visible background color runs.

Historical test shape:

- write several fixed-width runs with red/green/yellow background SGR;
- preserve the spaces that paint those backgrounds in the first emitted frame;
- only apply the old first-frame space removal optimization when the relevant text attributes are default.

Current renderer architecture no longer contains the branch's `_clearedAllThisFrame` / `removeSpaces` mechanism, and the historical test name is absent. There is no active PR on the old branch. Cherry-picking the patch would therefore target dead architecture.

**Classification: NO-PORT / recovery contract.**

Recovery contract if a first-frame ConPTY regression appears:

1. Reproduce with background-colored fixed-width runs whose visual extent depends on spaces.
2. Capture the first emitted ConPTY/VT frame, not only steady-state rendering.
3. Assert that optimizations may elide visually inert default spaces but must preserve spaces carrying non-default background/attributes.
4. Implement against the current renderer/serialization layer rather than resurrecting the 2022 paint-path condition.

**Disposition: SAFE TO DELETE.**

## `dev/duhowett/wprp`

Sole branch commit: `2188e570f2943f1b0ab1ebfea3ffc383250b4031` (2020-03-10), adding `src/TerminalPerf.wprp`.

This is not product code; it is a useful ETW/WPR profiling recipe. The literal 2020 profile should not be treated as a maintained artifact because provider sets and performance questions evolve, but its instrumentation intent is worth retaining.

**Classification: NO-PORT / diagnostic recipe preserved as contract.**

Reusable profiling recipe:

- kernel/system signals: context switches, memory information, working set, virtual allocation;
- environment/census collection through the WinPerf providers;
- Console/Terminal event providers covering launcher, host, server, VT parser, VT renderer, Terminal app, control, connection and Win32 host;
- file-mode verbose profile suitable for region-of-interest performance investigation;
- keep environment census collection separable from the product-specific verbose profile.

When performance investigation needs this again, build a fresh WPR profile from the current ETW provider inventory rather than copying the 2020 XML verbatim.

**Disposition: SAFE TO DELETE.**

## `dev/duhowett/hax/tap_upgrade`

Functional commit: `dca05f7709cdb601ebd59ba013f0a392b023ab4e`, **“HAX: make OpenSettings upgrade a conn with a tap”** (2020-08-28). The other branch-specific commits are spelling infrastructure migrations.

The experiment deliberately hijacks the Settings command: instead of opening settings, it wraps/replaces the active terminal connection with a debug tap connection, creates another `TermControl`, and splits a pane so traffic can be inspected. The real settings-opening path is placed under `#if 0`.

It also prototypes making a `TermControl` connection replaceable safely by revoking output/state subscriptions before swapping the connection.

No `DebugTapConnection` / `OpenDebugTapConnection` surface survives in current `main`, and no upstream PR exists for this branch. This is clearly a developer diagnostic experiment, not a user-facing contract.

**Classification: NO-PORT / diagnostic pattern only.**

Reusable diagnostic contract:

- if a future connection tap/proxy is needed, wrap a live connection without duplicating stale event subscriptions;
- revoke subscriptions from the old connection before replacing it;
- provide a second diagnostic consumer/pane only as explicit tooling, never by hijacking a production command such as Settings;
- keep transport inspection separable from normal `TermControl` lifecycle semantics.

**Disposition: SAFE TO DELETE.**

## `dev/duhowett/no-private-registry-keys` — revised decision

**Disposition: CONSERVAR / NO BORRAR. Do not port blindly.**

The primary ledger previously marked this as an ADAPT candidate because current `main` still reads `HKCU\\Software\\Microsoft\\Windows\\DWM\\AccentColor` for titlebar accent behavior while the branch uses public `Windows.UI.ViewManagement::UISettings`.

Further archaeology found the exact upstream PR: microsoft/terminal #14188, **“Use the Windows.UI.ViewManagement APIs to get the accent color”**. It was reviewed and initially approved, but real-color validation raised the key behavioral concern: the public accent API was not proven to return the exact color used by DWM titlebars for all accent choices. A maintainer ultimately submitted **CHANGES_REQUESTED**, and the PR closed without merge on 2022-11-01.

Therefore, the current private-registry read is not merely forgotten cleanup. It is evidence of an unresolved semantic distinction between “system accent color” and “actual titlebar accent color.”

Future recovery/port gate:

1. Define a test matrix of Windows accent/titlebar configurations, including colors known to expose divergence.
2. Compare the maintained public API result with the actual titlebar color behavior expected by Terminal.
3. Only remove the private DWM read if parity is demonstrated across that matrix or a newer supported API exposes the exact DWM titlebar value.
4. Preserve the distinction between generic system accent and titlebar-specific accent in the contract.

Until that replay passes on modern Windows, keep the branch as historical evidence.

## Closure state after Cohort 007

This cohort removes six old refs while preserving three kinds of knowledge in `dev/duhowett/main`:

- compiler/toolchain correctness already absorbed by the maintained tree;
- recoverable product behavior contracts (context-menu applicability, Copy Link, ConPTY first-frame backgrounds);
- diagnostic recipes (WPR/ETW profiling and connection tapping).

The namespace as a whole is **not yet closed**. `dev/duhowett/main` remains the permanent lane; `font-64` and `no-private-registry-keys` remain explicitly protected, and newer branch families still require genealogy/PR analysis.