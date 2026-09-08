# Branch archaeology — #9992 per-window settings generations

## Scope

This note consolidates the long-running `microsoft/terminal#9992` experiment family: named windows with window-scoped settings, quake-window defaults, and the architectural extraction needed to make those settings first-class.

The important result is that this is **one evolving product idea across multiple hackathon generations**, not a set of independent features. The local fork contains several historical snapshots that are either explicitly superseded, a documented dead end, or exact duplicates of branches that Microsoft still preserves upstream.

This note intentionally excludes `dev/migrie/fhl-spring-2026/quake-5`: its visible commits center on workspace persistence/menu state and it is being treated as a separate workspace-family investigation.

## Decision summary

After this note lands in `dev/migrie/main`, the following local refs are **SAFE TO DELETE**:

- `dev/migrie/fhl-fall-2023/9992-window-name-settings`
- `dev/migrie/fhl-fall-2023/9992-default-quake-settings`
- `dev/migrie/fhl-fall-2023/9992-quake-II`
- `dev/migrie/fhl-fall-2023/11162-quake-III-arena`
- `dev/migrie/fhl-spring-2026/quake-3.5`
- `dev/migrie/fhl-spring-2026/quake-4`
- `dev/migrie/per-window-final` **in this fork only**

The feature itself is **not dead**. `microsoft/terminal#9992` remains open and labeled `In-PR`, and Microsoft still has an active `dev/migrie/per-window-final` branch. The fork's `per-window-final` ref is safe specifically because it is an exact ancestor/snapshot of that active upstream branch and contains no unique commits.

Classification of the family: **TRANSFORM / ACTIVE-UPSTREAM**. Preserve the contract and genealogy; do not resurrect historical code generations.

## Product contract

The durable idea across the generations is:

1. A named Terminal window can resolve a set of window-level settings distinct from global defaults.
2. The ordinary/default window still has a well-defined default `WindowSettings` projection.
3. Quake/global-summon windows are a canonical consumer: `_quake` should be able to have different window behavior/appearance from ordinary windows without abusing profile settings.
4. Window-scoped settings must cover launch/window behavior and presentation concerns that actually belong to the window, not to an individual terminal profile/tab.
5. Settings resolution must remain coherent when settings hot-reload while windows already exist.
6. Startup actions should be interpreted in the context of the target window and its resolved settings, rather than prematurely as process-global state.
7. The architecture needs a clear model boundary (`WindowSettings`) before the Settings UI can safely expose global/default/per-window editing.

The UI problem is a separate concern: the #9992 thread explicitly notes that global settings and per-window settings become confusing if presented as one undifferentiated surface.

## Generation 1 — Fall 2023

Issue `microsoft/terminal#9992` acts as the architecture map for this generation and explicitly names the historical branches.

### `9992-window-name-settings`

- local and upstream SHA: `3dd735e858b3f7c835243283bad4fb8029a9aa4f`
- first named-window-settings prototype in the mapped chain.

### `9992-quake-II`

- local and upstream SHA: `b73fa0091fdcb46bf94a49834de32bf14811f4a7`
- direct descendant of `9992-window-name-settings`.
- graph comparison: **6 commits ahead, 0 behind**.
- expands serialization/deserialization and `WindowSettings` handling.

### `11162-quake-III-arena`

- local and upstream SHA: `b493b0e35494fd0d5cb3bd59942ccb76cc4cc0a7`
- direct descendant of `9992-quake-II`.
- graph comparison: **5 commits ahead, 0 behind**.
- continues the same window/quake settings architecture and also carries the related #11162/#11174 concerns.

Therefore the 2023 mainline is a real ancestry chain:

`9992-window-name-settings` -> `9992-quake-II` -> `11162-quake-III-arena`

The earlier heads do not need independent preservation once the lineage and contract are documented.

### `9992-default-quake-settings`

- local and upstream SHA: `aeb69a9cbb71f905655e0ee3b90ba26571bd3350`
- the #9992 issue body explicitly labels this attempt **“this was a dead end”**.

This is a direct NO-PORT case. Keep the architectural lesson, not the branch.

## Generation 2 — Spring 2026

The #9992 issue was updated for a new hackathon and explicitly says:

> New year, new hackathon. Updated branch is here: `dev/migrie/fhl-spring-2026/quake-4`

That statement is the semantic supersession link from the 2023 family to the 2026 rewrite.

### `quake-3.5`

- local and upstream SHA: `52ca28ca4843aa8c13a64a0094d81a19a79ddc37`
- one exclusive WIP commit (`holy man`) over `55f96bc...`.
- touches the same TerminalApp / SettingsModel / WindowEmperor / settings-editor domains.
- `quake-4` is not a direct descendant because the experiment was rewritten/rebased, but its branch generation and the #9992 update make `quake-4` the authoritative successor.

This is a superseded predecessor, not a separate recovery candidate.

### `quake-4`

- local and upstream SHA: `25e73209400130919a81d9aa41ec14da52e0331f`
- 2026 updated prototype explicitly named by #9992.
- supports named-window behavior including quake-window settings, docking-related behavior, default vs per-window settings, and substantial hot reload.
- the issue notes remaining product/design debt, especially Settings UI organization and incomplete hot reload coverage.

`quake-4` and the later `per-window-final` branch **diverge in Git history**; this is an architectural rewrite, not a missing merge. Comparing the heads gives a large divergence because both carry different mainline eras. The semantic evidence is stronger:

- #9992 identifies `quake-4` as the updated experiment.
- the later 2026 `per-window-final` commits reconstruct the same concepts with a cleaner architecture: window-owned startup actions, per-window-name theme/settings movement, `_quake` validation, and an explicit `WindowSettings` boundary.
- PR #20328 then lands the first architectural slice upstream.

Therefore `quake-4` is TRANSFORM provenance and can be retired locally after this contract is recorded.

## Generation 3 — production-oriented 2026 rewrite

### PR #20328 — architectural prequel

Merged upstream commit:

- `b4229d0f6af7215d079869986063af223ae88a0c`
- title: **Prepare for per-window settings (#20328)**

The PR explicitly says it is **part one of a two-part pair**. It introduces the `WindowSettings` concept and moves settings toward the correct architectural boundary while intentionally doing little product behavior by itself.

Historical commits on the active development line show the design direction clearly, including:

- moving startup actions to the window rather than parsing them process-globally in `AppLogic`;
- moving theme behavior to be per-window-name;
- separating tab settings from window defaults;
- validating `_quake` as a named-window scenario;
- repeated integration of a `user/migrie/per-window-prequel` branch before the production-oriented final branch.

This is the modern recovery path for the old hackathon code.

### Local fork `dev/migrie/per-window-final`

Local SHA:

- `1fefd922a4f507302fea15174ca1124031c86a2d`
- snapshot date: 2026-08-18

Microsoft still has the branch today:

- `microsoft/terminal:dev/migrie/per-window-final`
- observed upstream head during this archaeology: `fb997ce75fda41dc2996b80a0d762b6468171dda`
- upstream head date: 2026-09-02

Critical provenance comparison:

- base: local snapshot `1fefd922...`
- head: current upstream `fb997ce7...`
- **upstream ahead by 19**
- **local behind by 0 / no local-exclusive commits**
- merge-base is exactly `1fefd922...`

So the local fork branch contains **zero knowledge that is not still present upstream**. It is simply a stale snapshot of active Microsoft work.

This makes the local ref safe to delete even though the feature is not yet complete.

## Why deleting these local refs is safe

There are two independent preservation paths:

1. **Durable local knowledge**: this document in `dev/migrie/main` records the contract, rejected path, genealogy, and recovery direction.
2. **Code provenance upstream**: every historical branch listed here still exists in `microsoft/terminal` at the same SHA observed in the fork, and the active final branch has moved forward beyond the fork snapshot.

The fork therefore does not need to duplicate these source refs merely to retain archaeology.

## Branch classification

| Local branch | Classification | Evidence | Disposition |
| --- | --- | --- | --- |
| `dev/migrie/fhl-fall-2023/9992-window-name-settings` | **TRANSFORM / superseded generation** | ancestor of `9992-quake-II`; #9992 maps the evolution | **SAFE TO DELETE** |
| `dev/migrie/fhl-fall-2023/9992-default-quake-settings` | **NO-PORT** | #9992 explicitly calls it a dead end | **SAFE TO DELETE** |
| `dev/migrie/fhl-fall-2023/9992-quake-II` | **TRANSFORM / superseded generation** | descendant of window-name-settings, ancestor of quake-III | **SAFE TO DELETE** |
| `dev/migrie/fhl-fall-2023/11162-quake-III-arena` | **TRANSFORM / superseded generation** | latest mapped 2023 generation; #9992 later names quake-4 as updated branch | **SAFE TO DELETE** |
| `dev/migrie/fhl-spring-2026/quake-3.5` | **TRANSFORM / superseded WIP** | single-commit predecessor; quake-4 is documented updated generation | **SAFE TO DELETE** |
| `dev/migrie/fhl-spring-2026/quake-4` | **TRANSFORM / superseded architecture** | #9992 names it updated prototype; per-window-final/#20328 are modern rewrite path | **SAFE TO DELETE** |
| `dev/migrie/per-window-final` (fork copy) | **ACTIVE-UPSTREAM SNAPSHOT / NO LOCAL PORT** | exact ancestor of current upstream branch; 0 local-exclusive commits | **SAFE TO DELETE locally** |
| `dev/migrie/fhl-spring-2026/quake-5` | **NOT CLASSIFIED HERE** | visible work centers on workspace persistence; investigate with `workspaces-real` | **DO NOT DELETE from this decision** |

## Recovery guidance for Windows Rust Terminal

Do not port old JSON/model code literally. If the Rust product needs this behavior, treat it as an ADAPT/TRANSFORM contract:

- resolve window-level configuration from a stable window identity/name;
- make default/global window settings an explicit base layer;
- overlay named-window settings deterministically;
- separate window settings from profile/tab settings;
- evaluate startup actions in the target window context;
- support hot reload with a clear rule for which settings can update live and which require window recreation;
- keep `_quake`/global summon as a contract test for named-window specialization;
- design the settings editor so “global default” and “named-window override” are visibly distinct scopes.

## References

- `microsoft/terminal#9992` — canonical genealogy / active issue
- `microsoft/terminal#20328` — merged architectural prequel (`WindowSettings`)
- upstream active branch `dev/migrie/per-window-final`
- 2023 upstream refs at SHAs listed above
- 2026 upstream refs at SHAs listed above
