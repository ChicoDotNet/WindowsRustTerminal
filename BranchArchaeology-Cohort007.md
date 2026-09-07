# Mike Griese branch archaeology — Cohort 007

This cohort covers historical branches attached to issues that are still open or whose final product direction remains unsettled. The goal is not to pretend those product questions are solved. The goal is to preserve the useful engineering hypothesis, constraints, and authoritative upstream continuation so stale local refs are no longer required as knowledge storage.

## `dev/migrie/f/drag-panes`

- Historical intent: first mouse-drag pane-resize prototype for `microsoft/terminal#992`.
- Key commits:
  - `25de1a294068638337f1d4baf75fcfcf5e213b1f` — `Some basic dragging hookup`.
  - `eded53abe10ba048651b26d4a3b7a98bc88aab2a` — `oh no the DPI scaling doesn't work for this at all`.
- The upstream #992 discussion later points contributors at this branch only as an old experiment and explicitly recommends resurrecting the approach from current `main` because the original branch is years old and had XAML-island scaling problems.
- A newer authoritative implementation attempt now exists as PR `microsoft/terminal#16895`.
- Disposition: **NO-PORT / superseded as prototype by #16895**.
- Recovery value: the failure mode was DPI/XAML-island scaling. A future implementation should characterize pointer coordinate transforms and DPI before trusting drag deltas.
- Branch retirement: **SAFE**.

## `dev/migrie/f/992-redux-redux`

- Historical intent: current-generation mouse pane resizing for `microsoft/terminal#992`.
- This is not an abandoned mystery branch: its exact upstream head `4684e1880fa08667f29640ba8d6777955623c9ac` is PR `microsoft/terminal#16895`, **Add support for resizing panes with mouse**.
- PR #16895 is still open and not merged. It explicitly says `Closes #992` and preserves the current implementation, review context, and remaining TODOs upstream.
- Core design preserved by the PR:
  - `ManipulationDelta` is used for pointer dragging;
  - leaf panes own the visible border and receive drag events;
  - parent panes own split geometry;
  - a `ManipulationRequested` event bubbles the resize request from the leaf to the ancestor that owns the relevant split;
  - drag-start checks distinguish border drags from selection/content drags;
  - remaining bug-bash notes include snap-to-cell behavior and a possible crash.
- Disposition: **PRESERVED UPSTREAM / active PR**.
- The fork-local branch is merely a copy of knowledge already retained more authoritatively by the open upstream PR. Deleting the local ref does not discard the implementation or discussion.
- Branch retirement: **SAFE LOCALLY**.

## `dev/migrie/f/1860-hey-might-was-well-hack-during-a-hackathon`

- Historical intent: early hackathon exploration for `microsoft/terminal#1860`, horizontal scrolling / disabling forced wrapping.
- The branch was exploratory and self-critical (`4b5b04edbc8a6cb93a6a897eb41c99480578386f`, `Already I hate this`).
- The design problem is architectural: traditional terminal semantics and ConPTY assume a much tighter relationship between visible viewport width and terminal buffer width than the classic console model did.
- A later Mike Griese branch, `dev/migrie/f/1860-this-is-literally-what-less-is-for`, became the explicit WIP referenced from the issue and contains the more useful implementation notes.
- Disposition: **NO-PORT / superseded by later #1860 experiment**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/1860-this-is-literally-what-less-is-for`

- Historical intent: later WIP attempt at horizontal scrolling / minimum terminal buffer width for open issue `microsoft/terminal#1860`.
- The upstream issue explicitly links this branch as its WIP implementation.
- Key experimental observations:
  - `cd1ebb6e6718afc01a5255b668c56b614e4acd6c` — resizing approach works, but the author dislikes the UX/design;
  - `3f9e4559cda57ad58dfcd102cde5362b277222e1` — establishes an actual scrollbar;
  - `df2fe4fe66174cdccb27bc3acc82dae716978608` — discovers the DX renderer lacks horizontal-scrolling support and adds experimental help;
  - `fb460d7f60537295718dc5168c5b59956174c330` — adds an actual scrollbar while explicitly recommending `less` instead.
- Durable recovery design preserved from the issue discussion:
  1. an experimental `minimumBufferWidth`-style setting could keep the ConPTY/terminal buffer wider than the visible viewport;
  2. the visible viewport would then be allowed to be narrower than the VT-visible backing width;
  3. widening the window should grow the effective buffer naturally;
  4. tracking the longest printed row could avoid exposing a scrollbar across unused backing width and avoid repeated expensive `MeasureRight` scans;
  5. `Terminal`, `ControlCore`, `TermControl`, scrollbar state, cursor positioning, selection and renderer paths contain assumptions that viewport width equals terminal width and must be audited as one contract, not patched independently.
- The issue remains open, so the product question is intentionally **not** classified as solved.
- Disposition: **NO-PORT / RECOVERY CANDIDATE**. Rebuild fresh against modern TerminalCore/renderer architecture if #1860 is pursued.
- Branch retirement: **SAFE after this ledger**.

## `dev/migrie/f/3121-wE-dOnT-hAvE-dEv-DaYs`

- Historical intent: one of the early graphical shell-completion / suggestions protocol experiments for `microsoft/terminal#3121`.
- The branch iterated on compact menu UI, typing behavior, and the wire protocol (`b0fa972...`, `985fcdb...`, `48e7348...`).
- The durable UI direction subsequently became PR `microsoft/terminal#14938`, which was merged as `a0c88bb5117b8062cee4c4cad934a60e72768eb1`.
- PR #14938 deliberately separates two concerns:
  - Suggestions UI: product implementation worth shipping;
  - shell-completion transport/protocol: explicitly experimental and subject to change.
- Issue #3121 remains open because the protocol, not the existence of the Suggestions UI, is still unsettled.
- Disposition: **UI ALREADY ABSORBED / protocol prototype superseded as design**.
- Recovery value: preserve the separation between terminal-owned suggestion presentation and shell/app-owned completion production; do not couple a future protocol to PowerShell-specific `TabExpansion2` serialization.
- Branch retirement: **SAFE**.

## `dev/migrie/f/3121-tooltips`

- Historical intent: continue #3121 toward richer suggestions/tooltips after the Suggestions UI prototype.
- This branch merged the exact `dev/migrie/fhl-2023/pwsh-autocomplete-demo` lineage that became PR #14938 and later ended with `a1043bac4be104bc532552da99e89bcca5c29ab6`, `a bunch of notes before I abandon this`.
- Current issue history still treats the unresolved portion as protocol design. Later ecosystem feedback warns that feeding completions through the PTY can stall the terminal and points toward generic Fig-style definitions and/or LSP-based completion sources.
- Disposition: **NO-PORT / RECOVERY CANDIDATE**.
- Recovery value:
  - tooltips/completions should be shell-agnostic where possible;
  - terminal UI should be reusable by shells and TUIs/editors;
  - avoid verbose PowerShell-specific JSON-over-OSC as a permanent contract;
  - investigate transport that does not contend with the only PTY I/O path;
  - treat generic completion databases/LSP sources as first-class inputs.
- Branch retirement: **SAFE after this ledger**.

## `dev/migrie/fhl/vscode-autocomplete-prototype`

- Historical intent: tiny predecessor prototype for the same #3121 family.
- The upstream issue explicitly identifies this branch as a tiny prototype and records its own shortcomings:
  - current VT sequence design was slow;
  - the whole menu should be set in a single sequence;
  - icon spacing/alphabetical ordering were inappropriate for completion mode;
  - a modal mode might let normal character input continue while arrows/tab/enter are routed to suggestions.
- Functional history includes `01a64b1b3017d633d1cffa0d757eceaa340ac6bc` (`this is a fun hack`) and later iteration on throttling.
- Its product UI descendant was merged through PR #14938; the remaining protocol design is already captured above and in the live issue.
- Disposition: **NO-PORT / superseded prototype**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/4768-taskbar-icons`

- Historical intent: WIP for `microsoft/terminal#4768`, allowing Terminal windows/profile launches to present distinct taskbar icons/identity similarly to legacy shortcut-driven console launches.
- The upstream issue explicitly links this branch and records what the WIP proved:
  - taskbar overlay can be set;
  - taskbar grouping/glomming can be influenced using a runtime AUMID;
  - the HWND icon can be changed;
  - changing the HWND icon does **not** by itself change the taskbar icon.
- Branch commits show the exploratory API probing directly: `ba9c3eb...` (`Okay this at least sometimes works`), `32bd9d34...` (`this api is useless`), `ad2f6a76...` (runtime AUMID surprisingly works well).
- Historical platform constraint: conhost receives the spawning `.lnk` through `STARTUPINFO`, opens the link, and resolves its icon resource; packaged Windows Terminal/App Execution Alias identity changes that model.
- The maintainer later recorded that the combination of upstream issues `#6556` + `#14372` looked like a better product direction for the common scenario than this branch's taskbar surgery.
- Issue #4768 remains open; this ledger does not claim the UX is solved.
- Disposition: **NO-PORT / RECOVERY CANDIDATE**.
- Recovery value: if revisited, start from modern app/window identity APIs and the #6556/#14372 direction; use the old prototype only as evidence about AUMID grouping, overlays, and HWND-vs-taskbar icon behavior.
- Branch retirement: **SAFE after this ledger**.

## Cohort 007 deletion set

The following fork-local refs no longer need to serve as knowledge storage:

- `dev/migrie/f/drag-panes`
- `dev/migrie/f/992-redux-redux`
- `dev/migrie/f/1860-hey-might-was-well-hack-during-a-hackathon`
- `dev/migrie/f/1860-this-is-literally-what-less-is-for`
- `dev/migrie/f/3121-wE-dOnT-hAvE-dEv-DaYs`
- `dev/migrie/f/3121-tooltips`
- `dev/migrie/fhl/vscode-autocomplete-prototype`
- `dev/migrie/f/4768-taskbar-icons`

Keep `dev/migrie/main` as the durable archaeology lane.