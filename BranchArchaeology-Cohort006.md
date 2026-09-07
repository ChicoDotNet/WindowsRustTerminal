# Mike Griese branch archaeology — Cohort 006

This cohort covers later numbered experiments whose upstream issues now have enough history to distinguish shipped successors from still-useful prototype ideas.

## `dev/migrie/f/10509-mica-and-transparent-titlebars`

- Historical intent: explore Mica and transparent titlebars for `microsoft/terminal#10509`.
- The branch records the exploratory nature directly. Commit `a5b3063e620ba3c93e10685683d677411cd57bf8` says the approach appeared to work but broke Snap Flyouts; later notes continued investigating the window/titlebar constraints.
- The theming work was subsequently formalized in the Theme model and Mica draft specification.
- Authoritative successor: PR `#13935`, commit `031271f8244687d900888d2f34206729c29b1d93`, re-enabled supported Mica and transparent titlebars, synchronized titlebar/control opacity, fixed the Snap Flyout interaction, and explicitly closed `#10509`.
- Disposition: **NO-PORT / superseded**.
- Recovery value: provenance only; the final titlebar/Mica implementation solved the exact flaw recorded by the prototype.
- Branch retirement: **SAFE**.

## `dev/migrie/f/12336-let-it-mellow`

- Historical intent: investigate the continuous-output buffering/performance behavior from `microsoft/terminal#12336`.
- The branch is explicitly experimental performance archaeology. Commit `1a9612775e49256d462f9fae722b510e9146b7b3` records a one-line flush change improving a local benchmark by roughly 30%; commit `74181014cfc97a4357b416b2622f73558756357b` then tried flushing only during `Present()` and recorded that it did not meaningfully move the needle; the next commit `4e82c9accfb5109f26d48c7e84e3412a48d31e34` immediately reverted that experiment.
- Authoritative successor: PR `#17510`, commit `450eec48de252a3a8d270bade847755ecbeb5a75`, removed the old VtEngine architecture in favor of direct/synchronous API-to-VT translation and overlapped IO. The final change reports a much larger ConPTY performance improvement and explicitly closes `#12336` among the old output-path defects.
- Disposition: **NO-PORT / superseded**.
- Recovery value: retain the historical observation that flush placement affected throughput, but do not transplant the old micro-optimization onto an architecture that no longer has the same VtEngine ownership model.
- Branch retirement: **SAFE**.

## `dev/migrie/f/12861-preview-input`

- Historical intent: prototype previewing `sendInput`/snippet text for `microsoft/terminal#12861`.
- The prototype reached details such as trimming backspaces from preview text (`8636cc04644c906ffcc19356b3bdee77b8693397`), but it was built against the older text-services/input architecture.
- Authoritative successor: PR `#17386`, commit `86ba98607f7d0360f41410e5a5b0a0fef0ef02b3`, explicitly **re-implements** previewing with the new TSF composition model, handles shell-integration command lines and PowerShell ghost text, visualizes control codes, and closes `#12861`.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none requiring preservation of the old implementation; the final TSF implementation preserves and extends the product behavior.
- Branch retirement: **SAFE**.

## `dev/migrie/f/16005-a11y-pane`

- Historical intent: explore the accessibility output-pane idea from `microsoft/terminal#16005` for screen-reader users who want navigable command output.
- The issue discussion explicitly links this branch as the prototype.
- Prototype implementation (`bb283002473a010f7c9e093edb099705a0b46801`, `Add an a11y pane, PoC`):
  - adds an experimental `openAccessibilityPane` action behind `Feature_A11yPane`;
  - snapshots the active control with `ReadEntireBuffer()`;
  - creates a split pane containing an `AccessibilityContent` `TextBox`;
  - writes the captured buffer into the text box so standard text navigation can be used;
  - a follow-up copies presentation details such as font size.
- Product-history result: this PoC was not merged as the solution. During the issue discussion, the reporter verified that NVDA can already navigate Terminal output after enabling `Follow system focus` and entering object-review mode. The reporter then personally closed the issue on 2024-08-22; there is no closing product commit.
- The reporter still observed that ordinary arrow-key navigation in a dedicated output pane would be more intuitive, so the **idea remains useful even though the issue is closed**.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value preserved here: if this UX is revisited, start from the modern pane/content architecture and characterize the accessibility contract fresh. The valuable design is a read-only/navigable textual view of terminal output, ideally decoupled from CLI key handling; the old 2023 PoC should not be transplanted verbatim.
- Branch retirement: **SAFE after this ledger**. The development idea, prototype shape, provenance and key implementation seam are now preserved without requiring the historical ref.

## Cohort 006 deletion set

The following refs no longer require preservation as branches:

- `dev/migrie/f/10509-mica-and-transparent-titlebars`
- `dev/migrie/f/12336-let-it-mellow`
- `dev/migrie/f/12861-preview-input`
- `dev/migrie/f/16005-a11y-pane`
