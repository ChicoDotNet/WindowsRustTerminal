# Pankaj branch archaeology

## `dev/pabhoj/sui_action_overhaul`

**Disposition: ALREADY ABSORBED / superseded by reviewed Actions editor work.**

This large prototype is the early Settings UI action editor line exercised in Carlos's 1.24 selfhosts. Its functional surface is the same family later split into reviewed PRs:

- #18915 — ActionArgs reflection/settings-model support, merged 2025-09-03.
- #18917 — full action editing in Settings UI, merged 2025-12-09.

The production PRs contain the same core contracts: enumerate commands, reflect ActionArgs, add/edit/delete actions and keybindings, and render type-specific argument editors. They are the canonical implementation; the 67-commit prototype no longer carries unique product knowledge.

## `dev/pabhoj/sui_follow_ups`

**Disposition: NO-PORT / mechanical-only tip.**

Compared with current `main`, this ref has one exclusive commit: Dustin's `Migrate spelling-0.0.21 changes from main` (2022-07-29). Its diff is entirely spellbot configuration. No Pankaj product implementation remains uniquely on this ref.

## `dev/pabhoj/preview_string`

**Disposition: NO-PORT / superseded Settings UI prototype. SAFE TO RETIRE.**

This branch is the head lineage of PR #13293, `Allow preview connection to take in a preview string to display`, closed without merge in June 2022. It was intended as a prerequisite for #13269 (`Rejuvenate the color schemes page`), which merged in August 2022 and introduced the reviewed color-scheme preview experience. The exact #13293 commit was not an ancestor of the final #13269 head, and modern `PreviewConnection` has evolved back to a parameterless constructor. Therefore do not resurrect the old patch.

Recovery contract if this behavior is needed again: a Settings page must be able to provide purpose-specific preview content without hard-coding one global preview string. Re-design that contract against the current Settings Editor/PreviewConnection architecture.

## `dev/pabhoj/cursor_light`

**Disposition: NO-PORT / recovery candidate. SAFE TO RETIRE.**

Historical destination: draft PR #10821, `Add support for a HighlightCursor keybinding`, closed without merge on 2025-12-02. The product contract was: a toggle action darkens the terminal except around the cursor, the highlight follows cursor movement/output/scrolling, and toggling again restores the normal view.

There is no reason to carry the 2021 implementation forward mechanically. If revived, Contract Replay should start from the action contract and implement the effect using the current renderer/control visual stack.

## Terminal Chat / `feature/llm` family

The historical branches below target the long-lived protected upstream `feature/llm` branch, not ordinary `main`. That semantic base is why some of them look hundreds of commits ahead when compared directly with this fork's `main`. Treat only their branch-exclusive product contract as archaeology.

### `dev/pabhoj/featurellm_improve_parsing`

**Disposition: NO-PORT / recovery candidate. SAFE TO RETIRE.**

Head lineage of PR #18201, `Improve parsing of responses from the AI`, closed without merge. Contract: parse structured/Markdown LM responses with a real Markdown parser (`cmark` in the prototype) instead of manual code-block extraction. If Terminal Chat parsing is revisited, replay the parsing behavior against the current provider/UI response model rather than restoring this branch.

### `dev/pabhoj/featurellm_timeout`

**Disposition: NO-PORT / recovery candidate. SAFE TO RETIRE.**

Head of PR #18234, `Add a timeout for getting suggestions from the LMProvider`, closed without merge on 2026-03-31. Contract: every LM provider must bound potentially long HTTP/auth operations, and the orchestration/palette layer must have a longer outer timeout so a misbehaving provider cannot hang the UI indefinitely; timeout must surface a user-facing error.

### `dev/pabhoj/featurellm_fix_paste`

**Disposition: NO-PORT / recovery candidate. SAFE TO RETIRE.**

Head of PR #18412, `Fix copy/paste when Terminal Chat is open`, closed without merge in June 2026. Contract: when Terminal Chat owns focus, copy/paste commands must reach the chat/palette instead of being consumed by the active terminal control. The associated issue #16589 was ultimately closed `not_planned`, so preserve the behavior as recovery knowledge rather than porting the patch.

### `dev/pabhoj/page_control_input_cleanup`

**Disposition: NO-PORT / recovery candidate. SAFE TO RETIRE.**

Head of draft PR #19238, `Remove action handling from term control's preview key down`, also targeting `feature/llm`. Compared against its semantic PR base, it contains one exclusive commit, not 278 independent product commits. Contract: application-level `TerminalPage::PreviewKeyDown` should resolve keybindings and dispatch control-level actions to the focused control, avoiding the hairpin `TermControl receives keybinding -> sends action to app -> app executes it back on control` flow. This was another attempt to address #16589; that issue is now `not_planned`.

If input routing is revisited, replay the ownership rule (focused surface handles its commands; app-level routing dispatches control actions explicitly) against the current input architecture instead of cherry-picking the historical diff.

## `dev/pabhoj/wtcli`

**Disposition: ACTIVE / DO NOT DELETE.**

This namespace corresponds to open upstream PR #20461, `Implement wtcli, an application to query details about an existing Terminal instance`. The upstream PR has continued beyond the snapshot currently mirrored in this fork. Keep this ref while #20461 is active; upstream PR history/head is the authority for ongoing implementation.

Core active contract: `wtcli` exposes a local COM Windows Terminal Protocol for inspecting and driving a running Terminal instance, with inspect/read/mutate commands and machine-readable JSON output.

## Namespace compression result

After this archaeology, the durable Pankaj state is:

- `dev/pabhoj/main` — curated archaeology authority; keep.
- `dev/pabhoj/wtcli` — active PR #20461; keep until integration/closure.

The following historical refs are safe to retire because their unique knowledge is captured above:

- `dev/pabhoj/preview_string`
- `dev/pabhoj/cursor_light`
- `dev/pabhoj/featurellm_improve_parsing`
- `dev/pabhoj/featurellm_timeout`
- `dev/pabhoj/featurellm_fix_paste`
- `dev/pabhoj/page_control_input_cleanup`

Previously resolved refs `sui_action_overhaul` and `sui_follow_ups` remain covered by the earlier sections.

## Recovery guidance

For Actions Settings UI history, follow #18915 and #18917 rather than resurrecting `sui_action_overhaul`. For Terminal Chat history, use the protected upstream `feature/llm` line plus the PR contracts recorded here. Preserve this `dev/pabhoj/main` lane for any future Pankaj archaeology.