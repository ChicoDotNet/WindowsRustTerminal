# Branch Archaeology — Cohort 013

This cohort covers seven historical `dev/migrie/b/**` branches spanning Explorer shell integration, ActionMap reverse lookup, copy-on-select clipboard semantics, ConPTY startup timing, settings-folder deserialization, and relative-process launch CWD handling.

The guiding rule remains: retire the branch only after its useful behavior, negative knowledge, diagnostic value, or final upstream lineage is preserved elsewhere.

## `dev/migrie/b/13523-context-menu`

- Functional commit: `719d2ecf3d779ba61470c81fc1b3559b5a21a0f9` (`notes for #13523`).
- Historical intent: investigate why `Open in Terminal` appeared inconsistently in Explorer context menus.
- The patch is explicitly exploratory: it records that Desktop background and `This PC` can both arrive with `psiItemArray == nullptr`, comments out several proposed branches, and temporarily enables the verb in that state.
- Definitive architectural lineage: PR #14048 / merge `5027c8031d009ff5d26032134ab1e70b0c91f587` replaces the fragile top-level Explorer-window/path lookup with `IObjectWithSite` site-chain lookup. It explicitly references #13523 and was validated on Windows 10/11 for Desktop, folder background, folder selection, Quick Access and This PC.
- Disposition: **NO-PORT / superseded architectural research**.
- Recovery value: preserve the negative finding that `psiItemArray == nullptr` is not sufficient to distinguish Desktop background from non-filesystem shell locations.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/13943-a-test-for-this`

- Functional commit: `b3c491bfe2486359f4daa6d3e2f8dcad0c41e46e` (`This is a test for #13943`).
- Historical contract: after a user layer unbinds a key (`"command": null`), reverse lookup for the previously bound action must return no key binding; the UI must therefore stop advertising that key.
- The branch encoded this twice: directly against `ActionMap` and through layered `CascadiaSettings` defaults/user settings.
- Definitive architecture: PR #17215 / merge `d6b6aacb4ffd520719a99b4fc6137f3c035fa64c` closes #13943 by removing key ownership from `Command`; callers query `ActionMap` by Action ID instead.
- Current-main contract replay: the modern `ActionMap::GetKeyBindingForAction(cmdID)` contract explicitly returns `nullptr` when an action is not bound, and current UnitTests cover unbinding JSON (`"command": null`) plus a null `GetKeyBindingForAction(...)` result in the modern settings model.
- Disposition: **NO-PORT / contract replay already absorbed by modern ActionMap tests and architecture**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/14464-copyOnSelect-moving-text`

- Historical branch became PR #14635, whose head is this exact branch. That PR was closed without merge in March 2026, so it is **not** treated as successful integration evidence.
- Useful historical work included:
  - `2ec30ef270788e26e71a504e685f69f397addd18`: basic `copyOnSelect` test scaffolding;
  - `45b3e8d4f8649390e43e703d91abd54de7753caf`: intentionally failing regression tests, including main-buffer and alt-buffer cases;
  - `ca658ba331ef641830dabe57f99198e2857197b1`: historical product fix.
- The tests captured two durable contracts:
  1. once `copyOnSelect` has copied on mouse release, a subsequent right-click paste must **not re-copy** the still-visible selection before pasting;
  2. if buffer contents move/change after selection (including alt-buffer scenarios), right-click paste must not overwrite the clipboard from stale/reinterpreted selection coordinates.
- Definitive modern lineage: PR #19943 merged 2026-03-19 as `ad7b34e55f76e8326ca79f6aa148da38a3ddc7ec` and explicitly closes #14464/#19942. The current `ControlInteractivity` implementation references `GH#19942, GH#14464` and uses `_selectionNeedsToBeCopied` to distinguish already-copied selection from selection that still needs copying.
- The exact 2023 test names no longer exist on current main, so this is not classified as "the tests were merged". Instead, their behavioral contract is preserved here and the production behavior is owned by the newer #19943 implementation.
- Disposition: **TRANSFORM / contract retained; historical implementation and test scaffolding superseded by #19943**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/14512-test-research`

- Functional tip: `6cf678b517bd5291b73026e96dbf2acd0021e702`.
- Commit message is decisive negative knowledge: `I thought this test would be relevant, but alas, it was not`.
- Root issue #14512 concerned malformed/missing early output when a new default-terminal/ConPTY session was launched by Visual Studio or similar callers.
- PR #15298 / merge `6ad8cd0a630ab927629841a14d433c3bc19a1509` makes conhost enter VtIo/ConPTY mode earlier during startup. The author initially called it only a probable #14512 fix, but later the original reporter tested the 1.18 Preview containing it, confirmed the issue appeared fixed, and agreed the issue could be closed. #14512 was subsequently closed.
- Disposition: **NO-PORT / rejected test; behavior superseded by confirmed #15298 startup fix**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/14557-empty-folder-dropdown`

- Historical branch was opened as WIP PR #14646 and closed without merge.
- Definitive implementation: PR #14629 / merge `90485e4c79b34de6abbc203fd6cc07857947413b` fixes the crash caused by `_Entries` being default-constructed to `nullptr`, adds a regression test, and closes #14557.
- Disposition: **NO-PORT / WIP superseded by clean merged fix plus test**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/15487-relative-paths-are-hard`

- Functional commit: `2e069b2358e037bf0a77410cdfef6699096a3b7b` (`this might fix #15487. Dunno how I feel about it`), followed by `62d21f0be01d17327c08734dbc1115e4cd3e0cfb` (`whoops`).
- Historical intent: recover launching executables specified relative to the working directory from which Terminal itself was launched.
- This was an explicitly tentative implementation and was superseded by the later `push-cwd` solution.
- Disposition: **NO-PORT / superseded experiment**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/15487-push-cwd`

- Final branch head: `a0faa008b2578ef328701a4383ad8f1099a2af42`.
- PR #16028 closes #15487 by carrying the virtual working directory into `ConptyConnection`, temporarily switching process CWD before `CreateProcess`, and restoring the original CWD immediately afterward.
- GitHub's PR metadata currently reports `merged_at: null`, but the repository contains merge commit `5ebb3fb544c1d78e8622be57dbb16cd22001fde6`, whose parents include the exact branch head `a0faa008...`. That commit contains the CWD push/pop implementation. The commit graph therefore certifies absorption even though the PR metadata is inconsistent.
- Disposition: **ALREADY ABSORBED / final implementation merged**.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 013 deletion set

All useful knowledge from these branches is now represented by later merged architecture/fixes, current tests, or the contracts/negative knowledge recorded above. The following refs are safe to retire:

- `dev/migrie/b/13523-context-menu`
- `dev/migrie/b/13943-a-test-for-this`
- `dev/migrie/b/14464-copyOnSelect-moving-text`
- `dev/migrie/b/14512-test-research`
- `dev/migrie/b/14557-empty-folder-dropdown`
- `dev/migrie/b/15487-push-cwd`
- `dev/migrie/b/15487-relative-paths-are-hard`
