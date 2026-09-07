# Branch namespace and operational archaeology

This supplement records branch-namespace normalization and the disposition of historical operational/integration branches that do not fit the chronological `dev/migrie/**` feature ledger in `BranchArchaeology.md`.

## Namespace normalization rule

Historical personal namespaces are normalized to `dev/<user>/**`. Root branches are moved only when functional provenance identifies a clear owner; automated or integration-only refs are not assigned to a human merely because a human touched their tip.

### Explicit namespace aliases created

The following destination refs preserve the exact source tip and make the old names redundant:

- `user/lhecker/atlas-engine-srgb` -> `dev/lhecker/atlas-engine-srgb`
- `cinnamon/fhl/find-contextmenu` -> `dev/cinnamon/fhl/find-contextmenu`
- `cinnamon/open-json` -> `dev/cinnamon/open-json`
- `niels9001/fontweight-fixes` -> `dev/niels9001/fontweight-fixes`
- `niels9001/inactive-tab-foreground` -> `dev/niels9001/inactive-tab-foreground`
- `niels9001/page-transitions` -> `dev/niels9001/page-transitions`
- `users/GitHubPolicyService/d8615b98-f5fa-4462-bf7f-c44a1cc12096` -> `dev/GitHubPolicyService/d8615b98-f5fa-4462-bf7f-c44a1cc12096`
- `users/merlinbot/1es-pt-auto-baselining-pr` -> `dev/merlinbot/1es-pt-auto-baselining-pr`

Disposition: **ALIAS REPLACED / SAFE TO DELETE OLD REF**.

### Root branches with clear provenance

- `extendAISpec` -> `dev/nguyen-dows/extendAISpec`. The earliest functional work is Christopher Nguyen's initial commit; later DHowett commits are spelling migration. **SAFE TO DELETE OLD REF**.
- `msbuildcache-reenable` -> `dev/dfederm/msbuildcache-reenable`. Functional owner: David Federman. **SAFE TO DELETE OLD REF**.
- `of-the-darkening-of-valinor` -> `dev/migrie/of-the-darkening-of-valinor`. Functional owner: Mike Griese. **SAFE TO DELETE OLD REF**.
- `wpf-renderer-revert` -> `dev/duhowett/wpf-renderer-revert`. The branch was composed by Dustin Howett around renderer reverts; a cherry-picked Zoey Riordan commit at the tip does not change branch ownership. **SAFE TO DELETE OLD REF**.
- `loc-update` -> `dev/consvc/loc-update`. This is Console Service Bot localization output rather than a human development branch. **SAFE TO DELETE OLD REF**.

## Root operational branches that should not be renamed to a person

### `inbox`

`inbox` is an ancestor of `release/1.19` (0 commits ahead, 25 behind). It carries no unique payload and is not a personal development line.

Disposition: **ALREADY ABSORBED / SAFE TO DELETE**.

### `fabricbot-configuration-migration`

This branch was the automated source for microsoft/terminal PR #13397, “Migrate FabricBot Tasks to Config-as-Code”. The PR was merged. The branch subsequently received a spellbot exclusion adjustment and later spelling maintenance, but no unique product line remains outside the merged history.

Disposition: **ALREADY ABSORBED / SAFE TO DELETE**. Do not invent a human `dev/<user>` owner for an automated migration branch.

## Selfhost integration branches

### `selfhost-1.20`

Intent: pre-release integration/self-hosting of session persistence changes for the 1.20 line.

The meaningful branch-only payload is Leonard Hecker's `Backport session persistence improvements`, assembled by Dustin Howett as `Pre-merge #17049`. The final `release/1.20` contains the definitive `[1.20] Backport session persistence improvements (#17049)` commit with the same functional patch lineage.

Disposition: **ALREADY ABSORBED / SAFE TO DELETE**. The branch is a pre-merge integration snapshot, not a canonical feature source.

### `selfhost/1.22-bugbash-2024-06-04` and `selfhost-1.22-bugbash-2024-06-04`

The slash branch is the first bugbash integration generation. The hyphen branch is a later generation five commits ahead of it. Both are integration playgrounds composed from explicit `PRE-MERGE` feature branches/PRs.

The first generation includes pre-merge integration of:

- #16848 — CommandNotFound / Quick Fix UI — merged.
- #16895 — resize panes with mouse — **still open / not merged**.
- #16916 — grapheme clusters — merged.
- #17330 — snippets pane — merged.
- #17143 — Viewport / WalkInBounds improvements — merged.
- #16513 — save input action from command line — merged.
- #16479 — settings theme — merged.
- #17281 — actions in new-tab dropdown — merged.
- #17215 — remove Command's knowledge of keys — merged.
- #17162 — ActionMap/Command ActionIDs refactor — merged.

The later generation additionally pre-integrates #17352 and #17358; both were merged, and their merge commits are ancestors of `release/1.22`.

Important exception: #16895 is not in `release/1.22`, but its canonical source is still preserved as the live PR head `dev/migrie/f/992-redux-redux`. The selfhost branches therefore are not the sole carrier of that unmerged contract.

Disposition for both selfhost refs: **NO-PORT / integration snapshot superseded / SAFE TO DELETE**. Preserve #16895 through its canonical feature branch until that feature is separately resolved.

## Deletion set after this ledger is on `dev/migrie/main`

The following refs no longer carry unique knowledge that requires the old branch name or integration snapshot:

- `user/lhecker/atlas-engine-srgb`
- `cinnamon/fhl/find-contextmenu`
- `cinnamon/open-json`
- `niels9001/fontweight-fixes`
- `niels9001/inactive-tab-foreground`
- `niels9001/page-transitions`
- `users/GitHubPolicyService/d8615b98-f5fa-4462-bf7f-c44a1cc12096`
- `users/merlinbot/1es-pt-auto-baselining-pr`
- `extendAISpec`
- `msbuildcache-reenable`
- `of-the-darkening-of-valinor`
- `wpf-renderer-revert`
- `loc-update`
- `inbox`
- `fabricbot-configuration-migration`
- `selfhost-1.20`
- `selfhost/1.22-bugbash-2024-06-04`
- `selfhost-1.22-bugbash-2024-06-04`

`1.17` is intentionally excluded pending provenance reconstruction; it diverges materially from `release/1.17` and must not be assigned from its tip author alone.
