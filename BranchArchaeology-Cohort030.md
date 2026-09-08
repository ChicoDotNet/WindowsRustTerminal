# Branch Archaeology — Cohort 030

This cohort begins the final namespace-level closure of `dev/migrie/**`. It compresses fourteen surviving refs into four already-reviewed product lineages: the 2026 desktop-notification stack, generic pane content/snippets/Markdown, default profile icons, and Suggestions descriptions/tooltips.

The governing rule is semantic rather than hash-only: a historical ref may be retired when its durable contract is either explicitly carried into a later reviewed implementation, represented by a still-live successor PR, or preserved here as negative/recovery knowledge.

## 1. Desktop notifications — 2026 prototype generation

### `dev/migrie/fhl-spring26/activity-notifications`

- Exact head of upstream PR #19935, `Add settings for "notifying" on "activity"`, closed without merge.
- Durable contract: configurable notifications for inactive output/activity and next prompt, plus shell-integration-based automatic running-command detection. Notification styles can combine tab indicator, audible bell, taskbar signal, and desktop notification.
- Definitive successor: PR #20014, `Add settings for "notifying" on "activity"`, is still OPEN and explicitly says it is heavily based on #19935. Its current contract is `notifyOnActivity`, `notifyOnActivityThreshold`, `notifyOnNextPrompt`, `notifyOnNextPromptThreshold`, and `autoDetectRunningCommand`.
- Disposition: **NO-PORT / superseded by live #20014**. Upstream #20014 is the authority while open.

### `dev/migrie/fhl-spring26/bellStyle-notification`

- Exact head of PR #19936, closed without merge; adds `notification` as a `bellStyle` flag.
- Definitive successor: PR #20011 states that it is heavily based on #19936 and MERGED on 2026-05-04.
- Disposition: **ALREADY ABSORBED / superseded by merged #20011**.

### `dev/migrie/fhl-spring26/unpackaged-notify`

- Exact head of PR #19937, closed without merge; gives unpackaged Terminal instances an AUMID so Windows notifications can be emitted.
- Definitive successor: PR #20013 states that it is heavily based on #19937, improves the AUMID choice to the current per-executable-hash pattern, and MERGED on 2026-03-25.
- Disposition: **ALREADY ABSORBED / superseded by merged #20013**.

### `dev/migrie/fhl-spring26/osc777`

- Exact head of PR #19938, the 2026 redux of historical #14425: parse `OSC 777 ; notify ; title ; body`, notify only when appropriate, and focus/summon the originating Terminal when the toast is activated.
- Definitive successor: PR #20012 says it is heavily based on #19938, MERGED on 2026-06-04 and closed #7718.
- Disposition: **ALREADY ABSORBED / superseded by merged #20012**.

### `dev/migrie/fhl-spring26/osc777-2`

- Parallel March-2026 implementation. Its functional commit `4aff331374213aa2deefc8e72afaa6f3dd15f2b8` wires `NotificationRequested` through `IPaneContent`, carries title/body, introduces output-notification style flags and routes OSC777 into the toast pipeline.
- The same durable behavior is now owned by the reviewed #20010 infrastructure plus merged #20012 OSC777 implementation.
- Disposition: **NO-PORT / parallel prototype superseded by #20010 + #20012**.

### `dev/migrie/fhl-spring26/2/notification-infrastructure`

- One functional commit, `368d4483ebde1f50f0cc15514d18e3ad33c4a2cb`, whose message states the contract directly: toast sending, shell-integration mark propagation, tab activity indicator, event pipeline.
- Definitive successor: merged PR #20010, `Add toast notification infrastructure`, explicitly says it is heavily based on #19935 and provides the reviewed `DesktopNotification` abstraction, notification event bubbling, robust tab/pane targeting after reorder/move/swap, and focused-pane suppression.
- The remaining activity/next-prompt behavior continues in live PR #20014.
- Disposition: **NO-PORT / superseded prototype; contract preserved by #20010 + #20014**.

### Notification recovery contract

If this area regresses, replay against current code rather than transplanting a prototype branch:

- notification events must retain title/body and originating pane/tab identity;
- moved/reordered tabs and swapped/closed panes must resolve robustly, with sensible fallback;
- a focused active pane should not generate ordinary activity notifications;
- packaged, unpackaged and elevated notification activation must work without spawning an unwanted Terminal window;
- OSC777 activation must return focus to the originating window/tab/pane where possible;
- activity/next-prompt notifications must respect style flags and thresholds;
- shell-integration command state and activity indicators must remain coherent.

`dev/migrie/fhl-spring26/nextTab-filter` is intentionally NOT part of this retirement set. It contains a separate March-2026 implementation for filtering next/previous-tab navigation by tab state and requires independent closure analysis.

## 2. Pane content, snippets/tasks, Markdown and notebook prototypes

### `dev/migrie/base-pane`

- Tip `864f1b53f4876e8992cdd84e5e2c0c4aa7a956fc`, 2024-05-24: `base classes for all, for fun and profit`.
- This is intermediate factoring from the non-terminal-pane generation.
- Reviewed architecture: #16170 MERGED and made `Pane` host `IPaneContent`; #17537 MERGED and deliberately moved repeated pane-content event boilerplate into a shared base class.
- Disposition: **NO-PORT / intermediate factoring superseded by #16170 + #17537**.

### `dev/migrie/fhl/tasks-pane`

- May-2024 FHL integration branch combining the tasks-pane exploration with `sui-panes` work.
- Product successor: #17330, `Add a snippets pane`, MERGED 2024-07-08. It provides the reviewed TreeView/filter/play-button experience over snippet/sendInput actions on the modern pane-content abstraction.
- Disposition: **NO-PORT / pre-review tasks/snippets prototype superseded by #17330**.

### `dev/migrie/fhl/md-pane`

- Tip explicitly merges `dev/migrie/fhl/tasks-pane` into the Markdown-pane experiment, showing these were development layers rather than independent durable products.
- Reviewed successor: #17585, `Add support for markdown -> XAML parsing`, MERGED 2024-11-12. It introduces the dedicated `Microsoft.Terminal.UI.Markdown` component backed by `cmark-gfm` and a supported experimental `MarkdownPaneContent` testbed.
- Disposition: **NO-PORT / pre-review Markdown prototype superseded by #17585**.

### `dev/migrie/fhl/local-tasks-2024`

- Tip `38999195b2969199dbe4936cd4bb4ab504e046bd`, 2024-03-18, `plumb it through. It works`.
- This is an early local-task/snippet source prototype from the same generation subsequently consolidated into reviewed snippets/local-snippets behavior. The durable UI contract is represented by #17330 and the later local-snippet work already recorded in prior Migrie cohorts.
- Disposition: **NO-PORT / source prototype superseded by the reviewed snippets lineage**.

### `dev/migrie/fhl/adaptive-card-extension`

- Historical functional line ends in commit `01d5434afa8960966ee020ca9aca62568c4dd4cd`, whose message explicitly records that the DLL/app-extension packaging approach did not work, ran into mdmerge/resource packaging failures, and was being abandoned.
- This is valuable negative evidence, not an implementation to recover.
- Recovery lesson: do not revive the old Adaptive Card extension/package boundary merely because generic non-terminal pane content is now possible. Any future extension model should be designed against the current `IPaneContent`/packaging architecture.
- Disposition: **NO-PORT / explicitly abandoned failed experiment**.

### `dev/migrie/fhl/2024-inline-notebook`

- Tip `c04f089953fcde2b3f21d86c9d2d90c53ccfb85b`, 2024-03-14, explicitly says `I guess I'm stashing this for now. I've got other plans`.
- It belongs to the notebook/non-terminal-content exploration immediately before the reviewed pane-content/snippets/Markdown architecture matured.
- Recovery value is conceptual only: executable/notebook-like experiences inside Terminal were explored. Reconsider that capability against current pane-content and security/process boundaries rather than restoring this staging implementation.
- Disposition: **NO-PORT / stashed prototype superseded architecturally**.

`dev/migrie/fhl/notebook-proto-000` and `dev/migrie/s/markdown-notebooks` are intentionally deferred; their unique notebook/process-model knowledge has not yet been fully compressed.

## 3. Default profile icons

### `dev/migrie/f/default-icons`

- Functional work dates to 2022; commit `40bdbadf96b8520e95eb7959fa67269342fd6074` is an early attempt at generating sensible profile icons.
- Definitive successor: PR #15843, `When the profile icon is set to null, fall back to the icon of the commandline`, MERGED 2024-02-26. It formalizes executable-icon derivation, command-line normalization and the `"none"` sentinel.
- Disposition: **NO-PORT / early implementation superseded by merged #15843**.

## 4. Suggestions descriptions/tooltips

### `dev/migrie/fhl/sxnui-tooltips-3`

- Tip `8b19458dd5e1555fb877a347e76ce79f81ee809c`, 2024-02-16, says `I can't quite get bottom-up mode to work right`.
- Its diff contains explicit unresolved positioning code: `TODO! This is all wrong. It just jumps around randomly.` It is therefore not a final UI implementation to preserve.
- Historical PR #14939 was an earlier abandoned tooltip attempt.
- Definitive successor: PR #17376, `Add descriptions to commands (namely, snippets)`, MERGED 2024-06-12. Its description explicitly cites #14939 as the last abandoned attempt and implements action descriptions plus the Suggestions description/tooltip experience without the rejected TeachingTip approach.
- Disposition: **NO-PORT / unfinished UI experiment superseded by merged #17376**.

## Cohort 030 deletion set

**SAFE TO DELETE after this ledger commit:**

- `dev/migrie/fhl-spring26/activity-notifications`
- `dev/migrie/fhl-spring26/bellStyle-notification`
- `dev/migrie/fhl-spring26/unpackaged-notify`
- `dev/migrie/fhl-spring26/osc777`
- `dev/migrie/fhl-spring26/osc777-2`
- `dev/migrie/fhl-spring26/2/notification-infrastructure`
- `dev/migrie/base-pane`
- `dev/migrie/fhl/tasks-pane`
- `dev/migrie/fhl/md-pane`
- `dev/migrie/fhl/local-tasks-2024`
- `dev/migrie/fhl/adaptive-card-extension`
- `dev/migrie/fhl/2024-inline-notebook`
- `dev/migrie/f/default-icons`
- `dev/migrie/fhl/sxnui-tooltips-3`

## Explicit keep/defer boundary

This cohort does not authorize deletion of:

- `dev/migrie/main` — permanent curated archaeology lane;
- `dev/migrie/overview-for-pr` — current fork head of open upstream PR #20267;
- `dev/migrie/pr-15717/its-dangerous-to-go-alone` — review companion pending independent active-PR revalidation;
- `dev/migrie/fhl-spring26/nextTab-filter` — modern recovery implementation pending separate analysis;
- `dev/migrie/fhl/notebook-proto-000` and `dev/migrie/s/markdown-notebooks` — notebook lineage still under study;
- all other refs not explicitly listed in the Cohort 030 deletion set.
