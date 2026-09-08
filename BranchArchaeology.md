# Console Service branch archaeology

## `dev/consvc/loc-update`

**Disposition: NO-PORT / generated localization state. SAFE TO RETIRE.**

Tip: `b51ee0ecd331dbc0aabe270864b9381a79c80237` (`Localization Updates - 05/19/2023 23:23:26`).

This lane is an old generated localization snapshot, not a hand-authored product implementation. The upstream localization process continued producing newer PRs for years afterward; examples in 2026 include merged localization PRs #20196, #20259, #20277, and #20330, with #20582 opened for the 2026-08-25 localization batch.

The old branch therefore does not carry a unique product contract worth replaying. If localization state needs recovery, use the current localization pipeline and latest localization PR rather than resurrecting this 2023 snapshot.

## Namespace result

After this decision is preserved here, `dev/consvc/loc-update` can be retired and `dev/consvc/main` becomes the curated authority for this namespace.
