# OSC777 / desktop notification branch archaeology

This ledger separates the 2022–2024 `#7718` prototype lineage from the later 2026 notification work.

## Historical linear family

The following refs are not independent experiments. They form a strict lineage from the same historical base:

1. `dev/migrie/fhl/7718-notifications-reboot` — 25 commits ahead of its historical base.
2. `dev/migrie/fhl/7718-notifications` — direct descendant, 5 commits ahead of `...-reboot`.
3. `dev/migrie/7718-notifications-experiments` — direct descendant, exactly 1 commit ahead of `...-notifications`.

### `dev/migrie/fhl/7718-notifications`

This branch is the exact head of upstream PR **microsoft/terminal#14425 — Add support for OSC777 - send notification**, at `70d905b6588ef33f7d01b7edc2f624518e67947c`.

The durable contract was:

- parse `OSC 777 ; notify ; title ; body ST`;
- send a desktop toast only when the relevant tab/window is inactive;
- when the toast is clicked, bring/summon the originating Terminal window;
- safely carry title/body into the toast;
- account for packaged/elevated/unpackaged activation constraints.

PR #14425 was opened in 2022 and ultimately closed without merge in 2025. Its implementation used `WindowManager::SummonForNotification` and additional cross-process activation machinery because clicking a toast could launch another `windowsterminal.exe`.

### `dev/migrie/fhl/7718-notifications-reboot`

This is an ancestor/checkpoint of the #14425 line. Because `...-notifications` is 5 commits ahead and 0 behind it, the later #14425 head already contains the useful knowledge from this reboot generation.

Disposition: **ALREADY ABSORBED by the later `...-notifications` generation**, and ultimately superseded by the 2026 implementation below.

### `dev/migrie/7718-notifications-experiments`

This branch adds one commit on top of the #14425 head:

- `d28c0be8c1b5b6806b925a7141f66da597996431` (2024-02-15) — **Trying like 80 different things for chrswan but none of these actually worked**.

The commit experiments with activation paths (`App::OnActivated`, toast `Activated` handlers, `IslandWindow`/Emperor routing and related plumbing), includes diagnostic scaffolding such as `DoTheThing() { DebugBreak(); }`, and explicitly records that none of the attempted approaches worked. It is failed-design evidence, not a patch to recover.

Disposition: **NO-PORT / superseded failed experiment**.

## Authoritative 2026 successor

The old #14425 line was explicitly resurrected in upstream PR **#19938 — Add support for OSC777 - send notification (redux)**. Its description says **“Redux of #14425, for 2026”** and retains the same user contract. Crucially, it replaced the old complicated activation/IPC approach with listening for the toast's `Activated` event, which the PR notes works for elevated use; unpackaged support was handled through the newer notification infrastructure.

#19938 itself closed without merge, but upstream PR **#20012 — Add support for OSC777 (Send Notification)** states that it is **“Heavily based on #19938”**. #20012 merged on **2026-06-04** as merge commit `93bdbfaa3d62304f4b50b4ca4484da4dd08e4a1f` and closed issue #7718.

Current repository `main` contains the modern parser path in `AdaptDispatch::DoUrxvtAction`: it recognizes `OSC 777;notify;title;body` and gates it on `OptionalFeature::DesktopNotification`. Therefore the protocol contract represented by the old branches is present in modern code.

## Disposition

- `dev/migrie/fhl/7718-notifications-reboot`: **ALREADY ABSORBED / superseded**.
- `dev/migrie/fhl/7718-notifications`: **NO-PORT / superseded** by #19938 → merged #20012.
- `dev/migrie/7718-notifications-experiments`: **NO-PORT / superseded failed experiment**; do not recover the failed activation scaffolding.

If OSC777 notification behavior regresses, replay the modern contract against the current notification implementation rather than transplanting #14425-era IPC: parsing, inactive-window/tab policy, title/body fidelity, toast delivery, click-to-focus/summon, packaged/unpackaged/elevated behavior, and graceful disablement when `DesktopNotification` is unavailable.

## Branch retirement

**SAFE TO DELETE:**

- `dev/migrie/fhl/7718-notifications-reboot`
- `dev/migrie/fhl/7718-notifications`
- `dev/migrie/7718-notifications-experiments`

## Explicit non-authorization for 2026 branches

This ledger does **not** automatically authorize deletion of later branches such as:

- `dev/migrie/fhl-spring26/activity-notifications`
- `dev/migrie/fhl-spring26/bellStyle-notification`
- `dev/migrie/fhl-spring26/2/notification-infrastructure`

Those represent the broader 2026 notification/settings infrastructure and must be evaluated separately against PRs such as #19935, #20014 and related successors.
