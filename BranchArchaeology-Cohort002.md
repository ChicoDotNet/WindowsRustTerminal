# Lhecker Branch Archaeology — Cohort 002

## Scope and closure

This is the second namespace-wide compression pass for `dev/lhecker/**`.

Cohort 001 started from 71 refs and classified 27 lateral refs as safe to retire. Cohort 002 analyzes the remaining 37 non-`main`, non-active-PR refs as one closure unit rather than as independent branches.

Result of this pass:

- 31 additional refs are safe to retire after preserving their durable knowledge here.
- 6 non-PR refs remain intentionally preserved because their code is still a substantial recovery implementation or a live experiment.
- 6 exact upstream PR heads remain active and must not be deleted.
- `dev/lhecker/main` remains the durable knowledge lane.

Logical namespace target after both cohorts: **71 refs -> 13 justified refs** (`main` + 6 active PR heads + 6 recovery/live experiments).

## Canonical KEEP — active upstream PR heads

These branches are exact heads of upstream PRs that remain open as of this pass:

- `dev/lhecker/theme-quality` — microsoft/terminal #20508, New Ottosson Dark/Light themes.
- `dev/lhecker/generate-256-colors` — #19883, dynamic 8-bit color palettes, draft.
- `dev/lhecker/1410-large-scrollback` — #18290, large/infinite-style scrollback.
- `dev/lhecker/dcs-perf` — #19640, batched DCS processing/performance.
- `dev/lhecker/14165-conhost-font-size` — #19905, AtlasEngine conhost font sizing.
- `dev/lhecker/osc-7-wsl` — #20094, OSC 7 / WSL UNC path mangling.

Disposition: **KEEP / DO NOT DELETE while their PRs remain open.**

## KEEP — recovery implementations / live experiments

### `dev/lhecker/atlas-engine-compute-shader`

A single commit, but not a small idea. It replaces Atlas's vertex/pixel rendering path with a D3D compute-shader path, adds unordered-access swap-chain use, compute constant buffers, tile queues and sprite buffers. There is no equivalent compute-shader backend in current `main`.

Disposition: **KEEP — recovery implementation.**

### `dev/lhecker/renderer-overhaul-2nd-attempt`

The branch collapses genealogically to one semantic commit, but that commit is a large renderer architecture proposal: broad `renderer/base` simplification, separation/reworking of GDI rendering responsibilities, and interface/lifecycle changes. No unambiguous successor was found.

Disposition: **KEEP — recovery implementation.**

### `dev/lhecker/inproc-conpty`

A compact sequence of several WIP commits from March 2026, recent enough and implementation-heavy enough that reducing it to prose would discard useful executable knowledge.

Disposition: **KEEP — contemporary recovery implementation.**

### `dev/lhecker/1860-horizontal-scrollbar`

A 2025 implementation experiment tied to microsoft/terminal #1860. The issue remains open and is still carried in a current Terminal milestone.

Disposition: **KEEP / WAIT — live feature recovery.**

### `dev/lhecker/18928-wip`

Substantial tmux Control Mode prototype: dedicated `TmuxControl` / `TmuxConnection` implementation plus cross-cutting plumbing. This is not reconstructible cheaply from a short behavioral note.

Disposition: **KEEP — recovery implementation.**

### `dev/lhecker/theme-quality-test`

One experimental commit directly on the active `theme-quality` family, exploring alternative theme-quality/color-space calculations. It should remain coupled to #20508 until the active theme work settles.

Disposition: **KEEP / WAIT while `theme-quality` is active.**

---

# SAFE TO RETIRE

## Renderer, color and Unicode family

### `dev/lhecker/ColorScheme-improvements`

One mechanical cleanup, `Simplify passing color tables`. No independent product contract remains.

Disposition: **SAFE — mechanical cleanup.**

### `dev/lhecker/animated-cursor-wip`

The durable behavior is small and fully replayable:

- retain a current and target cursor rectangle;
- move each edge toward its target with bounded/easing-like steps;
- request another paint while convergence is incomplete;
- evaluate continuous-redraw demand after `Present`, so the renderer thread does not sleep between animation frames.

Disposition: **SAFE — Contract Replay captured.**

### `dev/lhecker/atlas-engine-srgb`

Single experiment to use native sRGB support for blending.

Recovery contract: when revisiting Atlas color correctness, prefer native/correct sRGB blending where supported, preserve alpha behavior, and validate fallback paths against the modern renderer.

Disposition: **SAFE — recovery contract captured.**

### `dev/lhecker/atlas-engine-stride-copy`

Small strided-copy helper experiment with no independent product behavior.

Disposition: **SAFE — mechanical utility.**

### `dev/lhecker/colrv1`

Single historical COLRv1 implementation experiment.

Recovery contract: modern renderer support for COLRv1 must preserve layered color glyphs/gradients and appropriate fallback; rederive against current DirectWrite/renderer APIs rather than replaying the 2023 patch mechanically.

Disposition: **SAFE — recovery contract captured.**

### `dev/lhecker/curly-improvements`

Exact upstream PR #18092, draft, one commit / three files, closed without merge.

Recovery contract: curly underlines should remain visually smooth and correctly positioned across font sizes, DPI and decoration metrics. Reimplement against current Atlas metrics if needed.

Disposition: **SAFE — closed prototype, contract captured.**

### `dev/lhecker/dark-mode`
### `dev/lhecker/dark-mode-alt`

Two one-commit variants of the same conhost idea.

Recovery contract:

- conhost should follow system light/dark state;
- high contrast wins over ordinary dark-mode styling;
- apply the correct DWM/UXTheme dark frame behavior where supported;
- retain compatibility on older Windows versions and avoid making private/legacy theme APIs a permanent dependency.

Disposition: **SAFE — variants collapsed into one contract.**

### `dev/lhecker/grapheme-backup`

Post-#16916 experiment modifying generated grapheme-break rules, including CR/LF, Regional Indicator parity, ZWJ/Extended_Pictographic and Indic conjunct behavior. Because Unicode data/rules evolve, this generated-rule patch is not durable authority.

Recovery contract: derive grapheme behavior from the current UAX #29/UCD and run conformance tests covering at least CR × LF, controls, RI parity, Hangul, Extend/ZWJ/Extended_Pictographic and Indic linker/consonant sequences.

Disposition: **SAFE — stale rules experiment; conformance contract captured.**

### `dev/lhecker/remove-glyph-width`
### `dev/lhecker/igfw-scroll-region`

Both belong to removal of legacy `IsGlyphFullWidth` / `GlyphWidth.hpp` assumptions. Current architecture no longer carries `GlyphWidth.hpp`, and the merged grapheme-cluster work (#16916) replaced that family of width heuristics with modern grapheme measurement.

Disposition: **SAFE — absorbed by architecture.**

## Input, console, TIL and tooling family

### `dev/lhecker/cleanup`

Experimental tokenization/integer-parsing cleanup.

Recovery contract: parsing helpers should be bounds/overflow safe, locale independent where protocol/config semantics require it, and covered by boundary tests.

Disposition: **SAFE — generic cleanup knowledge captured.**

### `dev/lhecker/client-context-input-output-mode`

The unique change is primarily a stale type/name refactor around `InputBuffer` / `SCREEN_INFORMATION`, not an independent user-visible input/output-mode feature.

Disposition: **SAFE — stale mechanical refactor.**

### `dev/lhecker/conhost-oom`

Misleading branch name. Its useful line became streaming UTF-8/UTF-16 conversion work; later PR #14417 (`Rewrite Utf16Parser`) merged a newer Unicode implementation into `til::unicode` with malformed-surrogate handling and extensive tests.

Recovery contract: streaming UTF-8/UTF-16 conversion must correctly preserve partial sequences across calls, produce replacement behavior for malformed input, handle surrogate pairs correctly, expose resettable state, and be test-driven.

Disposition: **SAFE — superseded by merged Unicode architecture.**

### `dev/lhecker/fused-event`

One-file utility copied closely from C++/WinRT event internals to provide a one-shot event.

Recovery contract:

- handlers execute at most once;
- add/remove token semantics remain correct;
- move operations are safe;
- concurrent raise/remove operations are synchronized;
- delegate destruction occurs outside critical locks where possible.

Because this depends on C++/WinRT internals, a future implementation should be rederived against the current runtime rather than preserving this exact copy.

Disposition: **SAFE — utility contract captured.**

### `dev/lhecker/lock-console-guard`

Early locking experiment. Its design direction was superseded by Leonard's merged PR #13746 introducing `recursive_ticket_lock` and using it in Terminal.

Disposition: **SAFE — successor merged.**

### `dev/lhecker/propsheet-fontdlg-refactor`

Large textual diff but legacy cleanup, not a new product architecture. It removes duplicated font-size lists/caches/handle recreation and simplifies property-sheet font handling.

Recovery contract: any future conhost font-dialog cleanup must preserve DPI transitions, TrueType/raster behavior, DBCS handling, preview correctness and the distinction between desired and realized font metrics.

Disposition: **SAFE — legacy no-port cleanup; contract captured.**

### `dev/lhecker/ring-buffer-input-buffer`

Large but explicitly incomplete experiment: direct/ring-style input storage, changed read APIs, TODOs for append/prepend, temporarily empty/no-op paths, and warnings relaxed. Its code should not be treated as a recoverable finished implementation.

Recovery contract if revisited:

- preserve peek vs consume semantics;
- preserve blocking/wait behavior;
- preserve ANSI/Unicode conversion and partial-byte state;
- preserve repeat counts;
- preserve append/prepend order;
- preserve flush/readiness counts;
- introduce the storage rewrite behind current tests and keep warning-clean builds.

Disposition: **SAFE — NO-PORT experimental implementation; replay contract captured.**

### `dev/lhecker/sdk-26100`

Single SDK-upgrade commit. Current `main` already targets Windows SDK 10.0.26100.0, so the intent is absorbed even though the historical PR did not merge verbatim.

Disposition: **SAFE — absorbed by intent.**

### `dev/lhecker/til-ulong-cleanup`

Small `til::to_long` refactor/fix.

Recovery contract: preserve signed/unsigned boundary and overflow semantics in modern conversion helpers; prefer tests over carrying the old helper patch.

Disposition: **SAFE — small correctness contract captured.**

### `dev/lhecker/benchcat-fix`

One-file v145/tooling patch: correct stderr handle, simplify literal/string append, clean switch parsing and stdout use.

Recovery contract: `benchcat` should compile under the current toolset, keep stdout/stderr distinct, and parse `-s` / `-v` deterministically without fragile CRT assumptions.

Disposition: **SAFE — tooling contract captured.**

### `dev/lhecker/winconpty-cleanup`

Large 2024 cleanup of WinConPTY's build/private NT-handle/path infrastructure. The exact helper architecture no longer corresponds to current `main`.

Recovery contract:

- create ConDrv server/reference/connect handles with correct NT object attributes and inheritance/synchronization semantics;
- locate inbox vs side-by-side console host safely across architectures;
- preserve `__INSIDE_WINDOWS` / KernelBase constraints;
- minimize inappropriate STL/runtime dependencies in inbox code;
- rederive against current WinConPTY internals.

Disposition: **SAFE — stale architecture cleanup, contract captured.**

## Windowing / remoting family

### `dev/lhecker/bugbash`

A high-ahead integration scratchpad. Its top history consists of explicit merges from Cazamor feature branches and Lhecker's `dethronement` line. `dethronement` became merged PR #18215 (`Remove Monarch/Peasant & Make UI single-threaded`), followed by merged regression fixes #18325 and #18345.

The apparent hundreds of ahead commits are therefore imported family history, not hundreds of unique Lhecker decisions.

Disposition: **SAFE — integration scratchpad; successors merged.**

### `dev/lhecker/window-thread-climate-control`

Large old multi-window lifecycle experiment touching Monarch/Peasant, WindowManager, AppHost, WindowEmperor, WindowThread, XAML teardown and process termination. Its concrete architecture was later superseded by merged #18215, which removed Monarch/Peasant and made the UI single-threaded.

Durable lessons:

- register windows only after they are ready;
- avoid destructor/message-pump reentrancy races;
- make lifetime and termination ordering explicit;
- never reuse old window-thread/refrigeration mechanics mechanically after the single-threaded transition.

Disposition: **SAFE — architecture superseded, lessons captured.**

### `dev/lhecker/openconsole-async-start`

2023 Terminal handoff / COM experiment: start the IO server before handoff, move to a newer `ITerminalHandoff` contract, make handle ownership/duplication explicit and safely revoke the single-use listener. That handoff architecture is no longer present in current `main`.

Recovery contract if handoff is ever reintroduced:

- listener/IO server must be ready before handing the session to Terminal;
- ownership and duplication of process/pipe handles must be explicit;
- one-shot COM registration must be revoked on all completion/error paths;
- startup response and handle lifetimes require integration tests.

Disposition: **SAFE — superseded architecture; contract captured.**

### `dev/lhecker/wellp2-alt`

Tiny alternate close-path experiment routing close through WindowManager/WindowEmperor rather than posting `WM_CLOSE` directly. Current windowing architecture has moved on.

Recovery contract: centralize close ownership and avoid reentrancy/message-pump hazards; implement against the current single-threaded window manager rather than replaying the patch.

Disposition: **SAFE — recovery contract captured.**

### `dev/lhecker/attach-thread-input`

Single-file foreground-activation robustness patch.

Recovery contract:

- return immediately if the Terminal window is already foreground;
- if foreground belongs to another thread, verify that window is responsive before attaching queues;
- abort activation if `AttachThreadInput` fails;
- always detach after activation;
- bring/show/activate the intended window and preserve monitor-move behavior.

Disposition: **SAFE — complete behavior captured.**

## Resolved / replayable issue branches

### `dev/lhecker/12351-broken-locales`

microsoft/terminal #12351 was closed as completed and marked `Resolution-Fix-Committed` in February 2022.

Disposition: **SAFE — issue resolved upstream.**

### `dev/lhecker/7118-cursor-color`

The issue remains open, but this branch does not contain the inversion renderer implementation. Its unique change only changes built-in schemes to use `"cursorColor": "invert"`.

Recovery contract: cursor inversion should guarantee a readable cursor by deriving/swapping/inverting the cell foreground/background instead of relying on a fixed color; validate light/dark cells and selection/rendering interactions against current settings and renderer code.

Disposition: **SAFE — issue remains canonical; branch contains only replayable defaults prototype.**

### `dev/lhecker/15689-tab-drag-crash-fix`

The historical fix pumps remaining XAML messages before closing `WindowThread`, preventing an outstanding `UIElement.StartDragAsync` from reaching `DXamlCore` after teardown. The issue is still open, but this patch depends on the old multi-`WindowThread` architecture superseded by #18215.

Recovery contract:

- reproduce tab tear-out/drag while the last tab/window is closing;
- ensure asynchronous drag completion cannot race UI/XAML teardown;
- validate current single-threaded destruction ordering rather than restoring the old message-pump workaround.

Disposition: **SAFE — old architecture superseded; root cause/repro captured.**

### `dev/lhecker/17656-win32im-double-encoding`

Two-file fix prototype: plain pasted/text strings should go directly to the active input buffer, rather than being synthesized into key events and then encoded again under Win32 input mode.

Recovery contract:

- plain/pasted text must not be double-encoded into Win32-input key-event sequences;
- text pass-through uses the direct string path;
- genuine control/key events continue through the key-event path;
- cover bracketed paste / VT-input interactions in tests.

Disposition: **SAFE — issue may remain open, but the branch behavior is fully captured.**

---

# Cohort 002 SAFE refs

The following 31 refs contain no unique knowledge that still requires the ref after this ledger entry:

- `dev/lhecker/ColorScheme-improvements`
- `dev/lhecker/animated-cursor-wip`
- `dev/lhecker/atlas-engine-srgb`
- `dev/lhecker/atlas-engine-stride-copy`
- `dev/lhecker/attach-thread-input`
- `dev/lhecker/benchcat-fix`
- `dev/lhecker/bugbash`
- `dev/lhecker/cleanup`
- `dev/lhecker/client-context-input-output-mode`
- `dev/lhecker/colrv1`
- `dev/lhecker/conhost-oom`
- `dev/lhecker/curly-improvements`
- `dev/lhecker/dark-mode`
- `dev/lhecker/dark-mode-alt`
- `dev/lhecker/fused-event`
- `dev/lhecker/grapheme-backup`
- `dev/lhecker/igfw-scroll-region`
- `dev/lhecker/lock-console-guard`
- `dev/lhecker/openconsole-async-start`
- `dev/lhecker/propsheet-fontdlg-refactor`
- `dev/lhecker/remove-glyph-width`
- `dev/lhecker/ring-buffer-input-buffer`
- `dev/lhecker/sdk-26100`
- `dev/lhecker/til-ulong-cleanup`
- `dev/lhecker/wellp2-alt`
- `dev/lhecker/winconpty-cleanup`
- `dev/lhecker/window-thread-climate-control`
- `dev/lhecker/7118-cursor-color`
- `dev/lhecker/12351-broken-locales`
- `dev/lhecker/15689-tab-drag-crash-fix`
- `dev/lhecker/17656-win32im-double-encoding`

## Namespace closure statement

After Cohorts 001 and 002, every investigated `dev/lhecker/**` ref falls into one of four explicit buckets:

1. durable archaeology in `dev/lhecker/main`;
2. an exact open upstream PR head;
3. one of the six intentionally retained recovery/live implementations listed above; or
4. SAFE-to-retire history whose durable behavior, successor or recovery contract is recorded in the two cohort ledgers.

No generic unknown lateral branch remains in the logical Lhecker namespace. Any future reduction below the 13-ref target should be event-driven: an upstream PR merges/closes, a recovery implementation is explicitly replayed/ported, or a live experiment is superseded.