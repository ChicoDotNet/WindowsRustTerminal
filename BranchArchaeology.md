# Kayla Cinnamon branch archaeology

This ledger consolidates historical `dev/cinnamon/**` work into `dev/cinnamon/main`. The historical branches are treated as design/provenance sources, not merge candidates: only knowledge that is not already obvious from the modern product is retained here.

## `dev/cinnamon/open-json`

- Functional period: October 2020; later tip commits are mechanical spelling migrations.
- Historical intent: connect the then-emerging graphical Settings UI to an “Open JSON” navigation item that launches `settings.json` (and conceptually `defaults.json`) through the shell.
- Historical maturity: prototype only. The key functional commit `d3550073fc36ab11f4425b6bed0b4018053f5853` is explicitly titled “this doesn't work, lnk2019 error”. It uses provisional shell-launch plumbing, including a machine-specific Windows SDK include path, and never became a PR.
- Definitive successor: the Settings UI itself shipped through `microsoft/terminal#8048` (`3e2b94334d5b43283e49fce71d4c4f76741a1e36`, 2020-12-11), with Kayla among the co-authors. Subsequent supported work made opening settings files a product feature: `#8670` (`990e06b445efae6ae549a5afd70751c8f18233be`) polishes the Settings UI/OpenSettings path, and `#9224` (`35e1168bfa7c0b598e8e1909ec284bb541bf7a66`) establishes the supported UI/JSON/defaults navigation model. Later accessibility work (`#18828`, `a8a47b93671361e529ff9f967d0e7c1b028eebb9`) confirms the Open JSON control as a maintained product surface.
- Disposition: **NO-PORT / superseded**.
- Recovery value: provenance only. The durable product intent—users must be able to move directly between the graphical Settings UI and the underlying JSON/default settings files—is fully represented in the modern Settings UI.
- Branch retirement: **SAFE after this ledger commit is present on `dev/cinnamon/main`**.

## `dev/cinnamon/fhl/find-contextmenu`

- Functional period: mid-2021; the branch was rebased/merged with main in August 2021 and later received spelling maintenance.
- Historical intent: expose Find from a tab’s context menu, wiring the tab/menu action into the active terminal control.
- Historical maturity: exploratory feature branch with merge-conflict cleanup; it did not become the canonical product implementation.
- Definitive successor: `microsoft/terminal#13055` (`b851b0d3f45d885484f4e0fe8a102abd022bc79e`, 2022-05-16), “Add find item to tab menu”, implements the feature and closes `#5633`. `microsoft/terminal#14673` (`f3439e201e99f7c42c11953cf4f98eef153ec366`, 2023-01-16) then hardens Find/Export context-menu behavior for unfocused tabs.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond provenance; the supported tab context-menu implementation and later bug fix supersede the historical branch.
- Branch retirement: **SAFE after this ledger commit is present on `dev/cinnamon/main`**.

## Deletion set

- `dev/cinnamon/open-json`
- `dev/cinnamon/fhl/find-contextmenu`

After those refs are removed, `dev/cinnamon/main` is the only Cinnamon contributor lane that needs to remain.
