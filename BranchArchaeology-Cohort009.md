# Cohort 009 — legacy bug experiments and absorbed fixes

This cohort applies the archaeology rule that a historical ref is no longer required once either (a) its behavior was incorporated into a reviewed upstream fix, or (b) the branch is only an explicitly incomplete diagnostic experiment whose useful hypothesis is recorded here with a modern recovery path.

## `dev/migrie/b/4591-custom-scaling-bug`

- Issue: `microsoft/terminal#4591`, fractional/custom display scaling glyph artifacts; still open.
- Historical experiment: `50a78bb7ea43cc78239d5e786c6f14d9bd41e178` and follow-up `8bcf0a936f546a75c6ae09465c72cb95a1808623` moved more sizing/rendering calculations to floating-point math.
- Author's own result: the change made the rendering better but did **not** fully fix the problem.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value: if revisited, reproduce on the current renderer at fractional DPI and isolate current glyph-metric / pixel-rounding boundaries. Do not transplant the old float-conversion patch across multiple generations of rendering architecture.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/5033-bad-start`

- Issue: `microsoft/terminal#5033`, conhost VK_F4/cooked-read regression.
- Functional experiment: `67e0080d6d321014bbe8ed3ff9e11605c7301596`, explicitly described as "a bad non-fix ... that helped me figure it out".
- Final lineage: the modern cooked-read rewrite closed #5033 via commit `821ae3af2d311350fbfaa4c77af09858488681e9` / the same rewritten cooked-read ownership that also superseded the #1503 archaeology.
- Disposition: **NO-PORT / superseded**.
- Recovery value: preserve only the diagnostic fact that the old fix exposed cooked-read state handling; use the modern cooked-read implementation/tests as authority.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/5113-experiments`

- Issue: `microsoft/terminal#5113`, copied text incorrectly broken at visual wraps.
- Historical branch is exploratory by design. Commit `068a98c40e78be965172f5fd6517254a525debf7` says parts may be garbage and identifies delayed-EOL cursor painting as the suspected mechanism; nearby experiments were reverted while the model was being understood.
- Final fix: PR `microsoft/terminal#5181`, merge `6fabc4abb7c974c188645f768cbb36831c532ff5`, implemented corrected `ScrollFrame` behavior, added extensive tests, and closed #5113.
- Disposition: **NO-PORT / superseded by tested fix**.
- Recovery value: delayed-EOL/wrapped-line semantics are the useful diagnostic insight; PR #5181 and its tests are the implementation authority.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/5161-mingw-vim-fix`

- Issue: `microsoft/terminal#5161`, one of the wrapped-line/scroll-frame failures investigated alongside #5113.
- Historical work: `3495cad437600296d4c8733670c8d4ad756ad29a` was explicitly stashed because no minimal reproduction could be produced.
- Final fix: PR #5181 explicitly closes #5161 by ensuring `removeSpaces` is applied only to the actual bottom line, under the corrected scrolling/wrap model and tests.
- Disposition: **NO-PORT / superseded**.
- Recovery value: retain #5161 as a regression scenario associated with ScrollFrame/wrapped-line handling, not as transplantable code.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/6160-dynamic-default-warning`

- Issue: `microsoft/terminal#6160`, misleading settings/default-profile failure when WSL discovery was slow.
- Historical experiment: `d486ccf24424174e1507e165fc38be8a9511dcf6` prototyped a dynamic-default warning but explicitly deferred it because #9997/profile-layering work would alter the required model.
- Later architecture: #10967 reduced the frequency; profile persistence/layering changed the fallback model; on 2026-08-12 Dustin Howett closed #6160 noting the original failure had effectively disappeared around v1.2 when Terminal stopped shelling out to `wsl.exe` to discover distributions.
- Disposition: **NO-PORT / obsoleted by architecture**.
- Recovery value: if a dynamic source is temporarily unavailable, distinguish "configured profile source unavailable" from malformed user settings and provide a sane fallback. The old warning implementation is not authoritative.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/6421-passthrough-alt`

- Issue: `microsoft/terminal#6421`, Alt/VK_MENU not passed through system XAML.
- Final fix: PR `microsoft/terminal#6461`, merge `e8ece1645c50ea05ac4ec191b71c885cbcec709a`, generalized the direct-key-event handler and manually tunneled VK_MENU; it explicitly closes #6421. A subsequent regression (#6513) received its own focused correction.
- Disposition: **ALREADY ABSORBED**.
- Recovery value: XAML/system input may consume keys before TermControl; the intentional direct-event tunnel is the architectural seam, while later regressions must be tested independently.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/6523-endpaint-outside-lock`

- Issue: `microsoft/terminal#6523`, move expensive drawing/EndPaint work outside the I/O lock.
- Historical experiment was revisited in 2022; maintainers noted D2D can flush internally before EndDraw, the change did not measurably improve Terminal performance, and RDP safety still needed validation.
- Final architecture: on 2024-10-10 Luciano Heckler closed the issue with "we do this now in AtlasEngine".
- Disposition: **NO-PORT / absorbed by AtlasEngine**.
- Recovery value: keep lock duration independent from queued GPU work where renderer architecture permits, but benchmark the actual renderer rather than assuming EndPaint placement is a performance win.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/7422-1px-top-border`

- Issue: `microsoft/terminal#7422`, top row of pixels unclickable while maximized.
- Final fix: PR `microsoft/terminal#10746`, merge `b1bcc59230311a915f7da50c754991e870b7c026`, intentionally shifts the XAML island upward one pixel to satisfy Fitts' Law; the PR explicitly closes #7422.
- Disposition: **ALREADY ABSORBED**.
- Recovery value: the accepted tradeoff and rationale live in PR #10746; the historical ref adds no independent contract.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/8480-keybindings-in-tabs`

- Issue: `microsoft/terminal#8480`, keyboard shortcuts unavailable when accessibility/keyboard focus lives in the tab row.
- Final fix: PR `microsoft/terminal#12260`, merge `6f69487829bf1e0d243e797404bae941e5572978`, adds a shortcut handler to `TabRowControl`; validated with Narrator and tab focus scenarios.
- Disposition: **ALREADY ABSORBED**.
- Recovery value: global shortcuts need a handler at the tab-row focus boundary, not only in TermControl/TerminalPage.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/8663-input-to-oem-crash`

- Issue: `microsoft/terminal#8663`, UTF-8/emoji input could expand to more narrow bytes/input records than the old width heuristic expected.
- This branch became the head of PR #12342, which added tests and demonstrated the concrete crash fix but was later closed unmerged.
- Structural successor: PR `microsoft/terminal#14745`, merge `599b55081762af1594cd8419320e79b9be533944`, removed `TranslateUnicodeToOem` and its invalid 1/2-byte assumptions, supports longer UTF-8 `char` sequences, includes tests for #8663, and explicitly fixes it.
- Disposition: **NO-PORT / superseded by structural Unicode fix**.
- Recovery value: never infer encoded-byte count from glyph display width; size/advance byte spans using actual conversion results.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/8698-YOURE-OUT-OF-ORDER`

- Issue: `microsoft/terminal#8698`, ConPTY pass-through sequences could overtake asynchronously rendered frame changes and arrive out of order.
- Historical attempts explored forcing paints/flushes around pass-through, with multiple acknowledged failure modes.
- Final architecture: commit `450eec48de252a3a8d270bade847755ecbeb5a75` / PR #17510 removed the VtEngine renderer-based translation path and translates Console API calls synchronously to VT; application VT output is passed through unmodified. It explicitly closes #8698.
- Disposition: **NO-PORT / superseded by synchronous ConPTY translation architecture**.
- Recovery value: ordering is an ownership/serialization contract; synchronous API-to-VT translation removes the old split between dispatch-time passthrough and renderer-time output rather than patching individual flush points.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 009 deletion set

- `dev/migrie/b/4591-custom-scaling-bug`
- `dev/migrie/b/5033-bad-start`
- `dev/migrie/b/5113-experiments`
- `dev/migrie/b/5161-mingw-vim-fix`
- `dev/migrie/b/6160-dynamic-default-warning`
- `dev/migrie/b/6421-passthrough-alt`
- `dev/migrie/b/6523-endpaint-outside-lock`
- `dev/migrie/b/7422-1px-top-border`
- `dev/migrie/b/8480-keybindings-in-tabs`
- `dev/migrie/b/8663-input-to-oem-crash`
- `dev/migrie/b/8698-YOURE-OUT-OF-ORDER`
