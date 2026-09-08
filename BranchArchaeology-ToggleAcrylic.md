# Toggle Acrylic review-companion archaeology

This ledger records the status of `dev/migrie/pr-15717/its-dangerous-to-go-alone` without declaring it legacy-retirable while its associated upstream pull request remains active.

## `dev/migrie/pr-15717/its-dangerous-to-go-alone`

### Relationship to PR #15717

Upstream PR **microsoft/terminal#15717 — Add a key binding Toggle Acrylic** was opened by Jaswir for issue #2531 and remains **OPEN**. The current PR head is `Jaswir:jaswir/2531` at `d6987a2038d7d4a5dc8b5a1b50103db2edd7ad91`.

The historical `dev/migrie/pr-15717/its-dangerous-to-go-alone` branch is a reviewer-side companion line. It contains the contributor's PR commits through `d0d2493e12cdbc1c25e99c21fb6500ab2ccf5900`, then Mike Griese's unique review experiment:

- `ffc78620e69248533acf658f332567cf17a00731` — `take this 🗡️` (2023-08-23).

The current upstream PR head and Mike's branch **diverge at `d0d2493...`**; the `ffc78620...` experiment was never incorporated into the contributor's current head.

### Contract / review problem

The contributor introduced a separate `_acrylicToggle` boolean because `UseAcrylic` is a runtime setting that was being forced false when opacity reached 100%. The observed failure contract was:

1. start with acrylic enabled;
2. change opacity to 100% using mouse-wheel opacity adjustment or the Adjust Opacity action;
3. move opacity below 100% again;
4. acrylic should resume according to the user's chosen runtime state rather than becoming permanently false as a side effect of visiting 100% opacity.

Mike challenged the extra boolean in review and then explicitly linked `ffc78620e` as an alternative. His experiment removes `_acrylicToggle`, stops mutating the runtime acrylic preference merely because opacity changes, and instead gates actual acrylic use on `Opacity() < 1.0` at the presentation/use site. His review comment says this approach appeared to work and was a starting point for further testing.

### Modern-main reading

Current repository `main` does **not** contain a `ToggleAcrylic` action. Its modern `ControlCore::_setOpacity` still computes `_runtimeUseAcrylic` from opacity and the configured acrylic setting, and the active PR remains the place where a user-visible Toggle Acrylic action is being proposed.

Therefore `ffc78620...` is not an unmerged general fix that should be transplanted into `main` in isolation. It is an implementation hypothesis for the still-open Toggle Acrylic feature, specifically addressing interaction between a runtime acrylic preference and opacity reaching 100%.

### Disposition

**CONSERVE / WAIT FOR INTEGRATION — active review companion.**

Do not port `ffc78620...` into `dev/migrie/main` as product code today. Do not delete the branch while PR #15717 remains open and the reviewer experiment is not represented in the PR head.

If #15717 resumes, use Contract Replay rather than blindly cherry-picking the 2023 patch. The minimum replay should exercise both initial `useAcrylic=true` and `false`, Toggle Acrylic at sub-100% opacity, transitions to 100% and back via mouse-wheel opacity adjustment, transitions via Adjust Opacity actions, and preservation of the user's runtime toggle intent across those transitions. Then implement against the current `ControlCore`/appearance model.

### Retirement condition

Re-evaluate this branch when one of the following becomes true:

- PR #15717 incorporates a modern implementation satisfying the replay contract;
- another merged successor implements the same Toggle Acrylic behavior;
- the PR/feature is explicitly abandoned, in which case preserve the contract and rejection rationale as a recovery candidate before retiring the ref.

Until then, **NOT SAFE TO DELETE**.
