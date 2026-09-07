# Branch Archaeology — Cohort 025

This cohort closes four surviving 2021 Quake/global-summon experiments. Their durable research notes are preserved here, while the production behavior was subsequently reviewed and merged through the Quake Mode/globalSummon PR series.

## `dev/migrie/f/quake-dropdown`

- Historical tip `1847ce66e0db50689e5a5b65e89c14cffe37ee34` is spelling maintenance; the functional line is from April 2021.
- The branch experimented with dropdown animation and records inconsistent `AnimateWindow` behavior. Commit `3edec55b65ab57c3296e3dcac59749f9c5891b96` says the desired animation worked only rarely, often popped into existence, or mixed minimize/dropdown animations.
- Definitive successor: PR `microsoft/terminal#9977`, `Add property to control dropdown speed of global summon`, merged as `6e11780ca6afdcbe216c7a06bb2d6a19cf3edd97`.
- #9977 explicitly preserves the research results from this lineage: `AnimateWindow` showed borders/was unreliable, `SetWindowRgn` interacted badly with DWM/XAML, and `SetWindowPos(...SWP_NOSENDCHANGING)` did not provide useful default-duration animation. The final implementation owns `dropdownDuration`, including Quake's 200ms default.
- Disposition: **NO-PORT / research experiments superseded by tested #9977 implementation**.
- Recovery value: the negative Win32 animation findings are preserved by #9977 and this ledger; do not restore the prototype implementation.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/quake-dropdown-2`

- Historical tip `5c0f6b289d7918b73be078402a9a4d97a94fa810` is spelling maintenance. Functional commit `de3507c7aa299cdf717380172c0b78f4d591e728` says `This is the one, this is what we'll ship`, after a sequence of explicitly rejected attempts such as `already I hate attempt 7`.
- This is a later experimental iteration of the same dropdown-animation research, not the reviewed product head.
- Final reviewed branch was `dev/migrie/f/quake-dropdown-final`, head `f4c99936262c654557c3f9e63ad1fac1f66b8c00`, merged through PR #9977.
- The historical branch and final head diverged, so this is not classified as direct ancestry; the final PR nevertheless documents and replaces the same animation-decision space and includes tests.
- Disposition: **NO-PORT / superseded experimental iteration of #9977**.
- Recovery value: provenance only; use the merged `dropdownDuration` behavior and #9977's recorded Win32 research.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/quake-toCurrent-experiments`

- Historical tip `404348f2e3da3b28c45bff443de5e718f074a49f` is spelling maintenance.
- Historical intent: determine how `globalSummon`/Quake should locate or move a window to the currently active virtual desktop.
- Important diagnostic commit `4f2de1977ace3a2c8c9327a9776a183769f9480d` records that determining the current desktop was unreliable: the registry value used by `VirtualDesktopUtils` could be absent, while asking for the foreground window's desktop failed when that window was pinned/show-on-all-desktops.
- The branch also explored summon visibility/desktop behaviors while the final action contract was still being designed.
- Definitive successor: PR `microsoft/terminal#9954`, `Add desktop param to globalSummon; set _quake = toCurrent`, merged as `65b22b9abba26fa94150505cee4444dcb4e42182`.
- #9954 formalized `desktop: toCurrent | any | onCurrent`, made Quake use `toCurrent`, reused the PowerToys `VirtualDesktopUtils` lineage, and added tests.
- Disposition: **NO-PORT / exploratory desktop-detection lineage superseded by #9954**.
- Recovery value: preserve the caveat that virtual-desktop identity is not trivially equivalent to foreground-window identity, especially with pinned windows.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/quake-toCurrent-experiments-2`

- Historical tip `8c46f39d47ec6085dcc47ea205059a3c43a7b4ac` is spelling maintenance.
- This branch is explicitly a dead-end continuation of the `toCurrent` investigation. Functional commits include `f3c7d9c47149af185a91eb2d9c6dc742fa0e99a` (`This obviously works but is somehow just as dumb as a Sleep(1)`) and `30a3621ec6fc35a81caf0df91813d04d5c55278e` (`Well this certainly wasn't it`).
- The product contract was subsequently implemented in PR #9954 on `dev/migrie/f/quake-toCurrent-desktop`, rather than by carrying these timing/workaround experiments forward.
- Disposition: **NO-PORT / rejected experiments superseded by merged #9954**.
- Recovery value: negative knowledge only — do not solve virtual-desktop synchronization with arbitrary sleeps/timing hacks.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 025 deletion set

- `dev/migrie/f/quake-dropdown`
- `dev/migrie/f/quake-dropdown-2`
- `dev/migrie/f/quake-toCurrent-experiments`
- `dev/migrie/f/quake-toCurrent-experiments-2`
