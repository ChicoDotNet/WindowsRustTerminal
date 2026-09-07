# Branch Archaeology — Cohort 024

This cohort closes three surviving non-terminal-content experiments by separating the durable abstraction from historical implementations. The modern product architecture now has an explicit `IPaneContent` contract and reviewed non-terminal pane implementations; preserving the old branches is no longer necessary for either the architectural idea or the Markdown behavior.

## `dev/migrie/f/non-terminal-content-elevation-warning`

- Historical branch was the exact head of PR `microsoft/terminal#11308`, `Warn before the user runs a new commandline elevated`.
- The elevation-warning product direction itself was explicitly abandoned: later merged PR `#12137` (`bc97af701e4061a18da111dbd00f5a766c6dfb13`) says the team decided not to do #11308 and shipped profile auto-elevation without that warning.
- The branch nevertheless carried one broader architectural experiment: `Pane` was changed to host arbitrary `UserControl` content instead of assuming every pane contained a `TermControl`.
- That architectural intent was later reimplemented deliberately and reviewed in PR `#16170`, `Refactor Pane to be able to host non-terminal content`, merged as `08dc34612058355b4e07b8e59ba33d09d43ee656` on 2024-03-26.
- #16170 introduces `IPaneContent` as the explicit abstraction between `Pane` and terminal/non-terminal content, and was followed by the modern non-terminal-pane/session-restoration lineage.
- Disposition: **TRANSFORM / warning rejected; generic pane-content abstraction superseded by merged #16170**.
- Recovery value: preserve the design lesson that pane ownership must be expressed through a content abstraction rather than by embedding ad-hoc `UserControl` exceptions into security-warning code.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/non-terminal-panes`

- Functional work dates to July 2019; branch head `ff8eb619ea3d3280e2d1ae9b101f47c856b9ab74` records `Get the thumbnail working`.
- Historical intent: prove that Terminal tabs/panes could host arbitrary XAML experiences rather than terminal sessions. The experiment cycled among prototype hosts such as rich text, media controls, WebView and settings-like content.
- A representative functional commit, `1feb2bddcaceef5b60912d7dba6b56119a5ba931`, switches the prototype to `MediaControlHost` and starts reading the current system media session. Follow-up commits wire play/pause, title/artist updates, buttons and thumbnail behavior.
- Historical maturity: direct `App` wiring and bespoke host types; this predates any stable abstraction for non-terminal content and is a proof of possibility, not a product contract.
- Definitive architectural successor: merged PR `#16170` (`08dc34612058355b4e07b8e59ba33d09d43ee656`) makes `Pane` host `IPaneContent` and explicitly supports arbitrary non-terminal implementations through a reviewed ownership boundary.
- Disposition: **NO-PORT / proof-of-concept superseded by modern `IPaneContent` architecture**.
- Recovery value: provenance only — the 2019 experiment proved arbitrary XAML content was possible. Do not transplant the media/WebView host hacks into current Terminal.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/md-pane-official`

- Functional branch head: `7a84f5354a3787b077e5cb66c667ca34c0ba0bab` (2024-05-24), with experimental commits such as `hey this works great`, `base classes for all, for fun and profit`, `can I get a whoop, it builds`, and `wire it back up`.
- Historical intent: develop reusable pane-content plumbing and an actual Markdown pane on top of the new non-terminal-content architecture.
- The branch includes intermediate factoring such as `BasicPaneEvents` shared by non-terminal pane implementations; it was still development-stage work rather than the reviewed final parser/content design.
- Definitive rewritten successor: PR `microsoft/terminal#17585`, `Add support for markdown -> XAML parsing`, used sibling branch `dev/migrie/f/md-pane-official-2` and merged as `772f546ac43f5c3bd8ecc2929ef2da8a5d0e886e` on 2024-11-12.
- The merged implementation introduces a dedicated `Microsoft.Terminal.UI.Markdown` component backed by `cmark-gfm`, renders GitHub-flavored Markdown to a `RichTextBlock`, and provides `MarkdownPaneContent`/`x-markdown` as the supported experimental pane testbed. It is materially more complete and reviewable than the May prototype.
- Disposition: **NO-PORT / superseded by merged rewritten successor #17585**.
- Recovery value: provenance only; the durable Markdown contract and parser ownership live in the merged implementation.
- Branch retirement: **SAFE after this ledger commit**.

## Deferred nearby work

- `dev/migrie/f/overview-view` is not part of this cleanup. Its branch has active functional history through May 2026 and must be treated as recent work, not inferred legacy merely because it also uses non-terminal content.

## Cohort 024 deletion set

- `dev/migrie/f/non-terminal-content-elevation-warning`
- `dev/migrie/f/non-terminal-panes`
- `dev/migrie/f/md-pane-official`
