# Branch Archaeology — Cohort 019

This cohort closes three behavior/refactor experiments from `dev/migrie/b/**` after checking both their historical intent and the current product architecture.

## `dev/migrie/b/no-nesting-when-searching`

- Functional line: `98af9d816e7374c91a0b09e2be13d081aaa398a7`, `46f46a9377519dc24dcb3835ffe95a66afd0c74e`, `2e253c417e463e6eb556ebd7f2945e7f60da999a`, and tip `053e070f618329fce775e28fa4c81710c0957fee` (2024-06 through 2024-08).
- Historical intent: experiment with filtering snippets/suggestions by both command name and send-input payload, flatten nested commands while searching, and rank results by fuzzy-match weight.
- The branch itself records negative design feedback: `Remove all nesting. This feels... less good?` followed by `try to sort based on weighting but this feels goofy too. We're clearly not selecting the right item`.
- Modern reading: the current snippets pane intentionally preserves nesting during filtering. `FilteredTask::UpdateFilter` recursively updates children; `Visibility()` keeps a parent visible if the parent or any child matches; the UI remains a TreeView. This is a deliberate productized alternative to the branch's flatten-and-sort experiments.
- Disposition: **NO-PORT / rejected search-UX experiment**.
- Recovery value: preserve the lesson that flattening and simple weight sorting were explored and rejected; do not revive those deltas as a presumed fix.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/remove-terminaltab`

- Functional commits include `0ee00637a6987fc9d9aff249284091fba0ff7799` (`entirely remove TerminalTab, and merge into TabBase`) and `7f69ab5801b5f8390b1f1740171ea029600714c5` (header cleanup), on top of the 2024 scratchpad/non-terminal-content experiments.
- Historical intent: remove the artificial `TabBase`/`TerminalTab` split and move terminal-tab behavior into one tab type, simplifying consumers and making non-terminal pane content easier to support.
- Definitive successor: `microsoft/terminal#19136` (`Merge TabBase+TerminalTab into just Tab`) merged as `f22295ef2c230f79340763b91a57f3f6c6739a24` and closes `#17529`.
- Modern reading: current `main` uses the unified `Tab` type; references such as `_GetTerminalTabImpl` were replaced by `_GetTabImpl`, exactly matching the architectural direction of the experiment.
- Disposition: **NO-PORT / superseded by #19136; architectural intent absorbed**.
- Recovery value: provenance only.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/theme.profile`

- Functional commit: `2b5127b150b320a66b8c01ca9b7807de483d4b78` (`one yike`, 2022-09-14); later tip movement is spelling maintenance.
- Historical intent: prototype a `theme.profile` object whose appearance settings would become a low-priority parent of each profile's default appearance, allowing a theme to influence terminal-profile appearance globally.
- Prototype maturity: deliberately incomplete. The branch leaves a nullability TODO in the settings editor; `ProfileTheme::Copy()` returns null; `ProfileTheme::ToJson()` returns JSON null; the conversion trait's `FromJson()` returns null; the code manually layers appearance internals.
- No upstream PR is associated with the prototype, and current `main` contains no `ProfileTheme` type.
- Later product work solved narrower theme-sensitive appearance needs through supported mechanisms (for example dark/light color-scheme selection) rather than productizing this unfinished profile-parent object.
- Disposition: **NO-PORT / incomplete rejected prototype**.
- Recovery value: concept/provenance only — a future feature may reconsider theme-wide profile defaults, but this implementation is not a safe or complete starting point.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 019 deletion set

- `dev/migrie/b/no-nesting-when-searching`
- `dev/migrie/b/remove-terminaltab`
- `dev/migrie/b/theme.profile`
