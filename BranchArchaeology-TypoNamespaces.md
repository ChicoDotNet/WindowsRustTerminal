# Typo / stray namespace archaeology

This supplement records historical branches whose namespace is a typo and should not create a separate contributor lane.

## `dev/duhowtt/hax/cpal-jumplist-async`

- Namespace reading: `duhowtt` is a historical typo for Dustin Howett / `duhowett`, not a distinct contributor.
- Functional commit: `397a84e5596c9a68a915ac098386652c2ce63974` (2021-09-23), `Make the command palette async, jump list limit 13 items`; later tip `e0ba9e9...` is spelling maintenance.
- Historical intent has two independent parts:
  1. Move command-palette command expansion/keybinding-label preparation off the UI thread, expose `Loading`/`Loaded`, and show a ProgressRing while rebuilding commands.
  2. Stop adding jump-list profile tasks after an empirically observed Windows 10 limit of 13 items.
- Historical maturity: prototype only; there is no upstream PR from this branch.
- Modern reading — Command Palette: the historical `Loading`/`Loaded` and ProgressRing contract is not present in the current CommandPalette. The surrounding command/action architecture has changed materially since 2021, so the old coroutine/UI plumbing should not be transplanted. Any palette-startup performance concern should be characterized against the current implementation before designing a fix.
- Modern reading — jump list: current `Jumplist::UpdateJumplist` already performs Explorer work on a background thread and calls `ICustomDestinationList::BeginList`, which returns the system-provided slot count. The current `_updateProfiles` nevertheless iterates all profiles and does not use that `slots` value to truncate. The old hard-coded 13 is therefore obsolete as an implementation but preserves a useful compatibility observation.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value: if jump-list capacity resurfaces as a bug, use the runtime `slots` result from `BeginList` rather than reviving a Windows-10-specific constant. If command-palette loading becomes measurable again, benchmark current command expansion and introduce an async/loading contract from the modern architecture rather than replaying this patch.
- Branch retirement: **SAFE after this supplement reaches `dev/duhowett/main`**.

## Namespace retirement

- `dev/duhowtt/hax/cpal-jumplist-async`

Do not create `dev/duhowtt/main`; the canonical contributor lane is `dev/duhowett/main`.
