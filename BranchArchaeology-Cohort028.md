# Branch Archaeology — Cohort 028

This cohort closes the June 2024 preparation branches that were explicitly rebuilt and squashed into the reviewed local-snippets implementation.

## `dev/migrie/f/just-local-snippets`

- Tip `73afe480414eb84a8682122d235db907150cc144`, dated 2024-06-01, is `plumb it through. It works (cherry-picked from 38999195b)`.
- Original prototype commit `38999195b2969199dbe4936cd4bb4ab504e046bd` loaded `.wt.json` from the current working directory, exposed CWD through the control context, parsed local actions, and made them available to the snippets/tasks UI.
- PR #17388 (`Add support for local snippets in the CWD`) contains commit `b5d063b48eb3fe2a77f6b8da15195909c89ed683` with the same message and explicit cherry-pick provenance from `38999195b`.
- Disposition: **ALREADY ABSORBED / rewritten into #17388**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/local-snippets-on-action-refactor`

- Tip `aa27423485efc2bfe0c116fe6e4b29cab4aef0f4`, dated 2024-06-02, caps the cleanup/refactor sequence around ActionMap-backed local snippets.
- Its functional sequence (`smaller refactors`, `a much cleaner abstraction`, `cleanup`, `more cleanup`, `last cleanup before review`) was rewritten into the corresponding #17388 commits `5a029f0...`, `d41e973...`, `091797c...`, `3aa5b4b...`, and `f0bc7f1...`.
- The reviewed PR continued from that exact design, adding a CWD-to-actions cache, settings-reload invalidation and limiting `.wt.json` imports to `sendInput` actions.
- Disposition: **ALREADY ABSORBED / rewritten into #17388**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/local-snippets-cleaner`

- Tip `deab3fe05bfd833c7794c8ea44a09a9a89baf01c`, dated 2024-06-07, integrated the local-snippets work with contemporary `main` / action-refactor changes immediately before review.
- The first commit on PR #17388, `ae705f3ae1ef7bfe55293f5e01c62e94eb762da0`, is explicitly titled `Squashed commit of the following:` and its embedded history names `deab3fe05bfd833c7794c8ea44a09a9a89baf01c` plus its predecessor local-snippets commits.
- PR #17388 head `06c0e662e8c143628c55ef8d72444d48c568ca44` merged as `21fa303a3df9a71ca80730c8256bc2c29c63c41d` on 2024-07-26.
- Disposition: **ALREADY ABSORBED / explicit squash source of #17388**.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 028 deletion set

- `dev/migrie/f/just-local-snippets`
- `dev/migrie/f/local-snippets-on-action-refactor`
- `dev/migrie/f/local-snippets-cleaner`
