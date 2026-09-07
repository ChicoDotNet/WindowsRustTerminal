# Mike Griese branch archaeology — Cohort 005

This cohort applies the fast archaeology rule: when a historical branch name contains an upstream issue or PR number, first reconstruct the authoritative upstream resolution. Only preserve the branch when it still contains unique durable knowledge not represented by the final product lineage.

## `dev/migrie/f/1504-final`

- Historical intent: prototype support for EXE/DLL resource paths in the profile `icon` setting (`microsoft/terminal#1504`).
- Functional experiment: `1b362864a528626713437ecd6412b565187efc10`, whose own message is `this is all a disaster`.
- Authoritative successor: upstream PR `#14107`, commit `aa625098ed2be1d5fbdbe639c80a0dc551a0de92`, explicitly closes `#1504` and supports EXE/DLL icons in the tab, Settings UI, new-tab flyout, and command palette.
- Disposition: **NO-PORT / superseded**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/1504-prototype`

- The branch name is misleading. Commit `874a15b89bf88ac071531bf41b4e6c94a09f80d2` explicitly says the startup/handoff plumbing experiment is **for `#9458`**, not `#1504`.
- Historical intent: pass richer startup information through the default-terminal handoff so Terminal can reconstruct LNK/EXE context.
- Authoritative successor: upstream PR `#13570`, commit `7e47f6aab96b0f2d01309bc05cebe1c57b44a127`, wires LNK/EXE data from OpenCon into `ITerminalHandoff` and explicitly closes `#9458`.
- Disposition: **NO-PORT / superseded by the completed #9458 implementation**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/9458-startupInfoToTerminal`

- Historical intent: continued `#9458` startup-info / handoff plumbing.
- Upstream issue `#9458` is closed as completed.
- Authoritative successor: PR `#13570`, commit `7e47f6aab96b0f2d01309bc05cebe1c57b44a127`.
- The historical branch diverges from the final integrated lineage and contains old scratch/project churn; there is no reason to transplant it over the completed upstream implementation.
- Disposition: **NO-PORT / superseded**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/1897-less-duplicated-work`

- Historical intent: reduce duplicate work in `NonClientIslandWindow` resize/drag-region calculations for `microsoft/terminal#1897`.
- Upstream issue `#1897` is closed as completed.
- Authoritative successor: PR `#3394`, commit `5dfc021d8ed24de749504713a2ad8a4bd0903a79`, which refactors the non-client island window, removes obsolete complexity, caches/centralizes relevant geometry/DPI work, and explicitly closes `#1897` among the related window-frame defects.
- Disposition: **NO-PORT / superseded**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/2046-command-palette`

- Historical intent: early Command Palette prototype for `microsoft/terminal#2046`.
- The prototype directly motivated several prerequisite refactors, including separating active terminal state from UI focus and extracting action dispatch.
- The design was then formalized in the Command Palette specifications and implemented upstream.
- Authoritative product successor: PR `#6635`, commit `aa1ed0a19c479ce6d7c0087ab4bca95f87e83584`, which adds the Command Palette with tests and explicitly closes `#2046`.
- Disposition: **NO-PORT / superseded**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/2046-Command-Palette-v2`

- Historical intent: later iteration of the same `#2046` Command Palette prototype lineage.
- The durable design was captured by the finalized spec (`#5674`) and subsequent unified action/command work, then shipped by PR `#6635` (`aa1ed0a19c479ce6d7c0087ab4bca95f87e83584`).
- Disposition: **NO-PORT / superseded**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/2171-openterm.cmd`

- Historical intent: command-line build/run tooling around `microsoft/terminal#2171`.
- `#2171` was immediately identified by the maintainers as a duplicate of `#926`.
- `#926` was later closed by PR `#11811`, commit `6f3464de892e8b506ad6dc90d282f96be3d7ccc8`, which documents the supported command-line package build/deploy reality and explicitly notes the limitations of the resources inner loop.
- The historical branch consists of exploratory 2019 tooling commits such as `This _should_ work`, case-sensitivity fixes, and miscellaneous local tooling; it is not an authoritative modern build path.
- Disposition: **NO-PORT / superseded by documented modern build guidance**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/3726-restartConnection`

- Historical intent: implement a restart/reload action for a tab/connection (`microsoft/terminal#3726`).
- Upstream issue `#3726` is closed as completed.
- Authoritative successor: PR `#15241`, commit `70e44c791543c33000cd52b6aadd7b47150587b1`, which adds the explicit restart-connection action and closes `#3726`.
- Disposition: **NO-PORT / superseded**.
- Branch retirement: **SAFE**.

## `dev/migrie/f/6856-let-terminalpage-expandcommands`

- Here `6856` is a **PR number**, not the issue being solved.
- PR `microsoft/terminal#6856`, commit `dcc2799457c569ce6a7666113ce043db61c6cbac`, shipped iterable and nested Command Palette commands, with recursive parsing and tests, and closed the underlying feature issue `#3994`.
- The historical branch name describes an implementation staging idea around where expansion should occur; the merged PR is the authoritative product lineage.
- Disposition: **ALREADY ABSORBED / superseded by merged PR #6856**.
- Branch retirement: **SAFE**.

## Cohort 005 deletion set

The following refs no longer contain unique knowledge requiring preservation as branches:

- `dev/migrie/f/1504-final`
- `dev/migrie/f/1504-prototype`
- `dev/migrie/f/9458-startupInfoToTerminal`
- `dev/migrie/f/1897-less-duplicated-work`
- `dev/migrie/f/2046-command-palette`
- `dev/migrie/f/2046-Command-Palette-v2`
- `dev/migrie/f/2171-openterm.cmd`
- `dev/migrie/f/3726-restartConnection`
- `dev/migrie/f/6856-let-terminalpage-expandcommands`
