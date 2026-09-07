# Leon Liang branch archaeology

This ledger consolidates historical `dev/lelian/**` work into `dev/lelian/main` while preserving useful design provenance without carrying obsolete implementation code.

## `dev/lelian/actionid/1`

- Functional period: September 2021; the later tip also contains mechanical spelling migration.
- Historical intent: prototype stable external IDs for Terminal actions/commands so multiple declarations could refer to the same logical action, combine fields such as command/name/keys, survive settings layering, and let keybindings refer to an action identity rather than only a hash of action+args.
- Representative work: `a6b8d3568474231e04e66fc5107dbcb65d879309` introduces `ExternalID` bookkeeping and staging/combining of incomplete action declarations. `d493f6e5511ddd933c6e525a2658201ee640bbf7` continues the model and records that it "mostly works" but nested/iterable commands were not tested. The branch never became a PR.
- Historical maturity: promising prototype, but incomplete and tightly coupled to the 2021 `ActionMap` implementation.
- Definitive successor: `microsoft/terminal#16904` (`06ab6f3e1f2143a1b2681dd954df2a67041373eb`, 2024-04-18) formally adds IDs to Commands, including built-in IDs and generated IDs for user-created commands. `microsoft/terminal#17162` (`ece0c04c38b6f820476fd480a96a8b103a5ca7f2`, 2024-06-04) then refactors `ActionMap`/`Command` to actually use ActionIDs and validates layering, overwriting earlier-layer actions by ID, keybindings referring to IDs, generated IDs, legacy parsing, schema updates, and settings rewrite behavior.
- Disposition: **NO-PORT / superseded**.
- Recovery value: historical design provenance only. The durable insight is that actions need stable identity independent of their action+args hash; that insight is now represented by the supported ActionID architecture and its tests.
- Branch retirement: **SAFE after this ledger commit is present on `dev/lelian/main`**.

## Deletion set

- `dev/lelian/actionid/1`

After that ref is removed, `dev/lelian/main` is the only contributor branch that needs to remain.
