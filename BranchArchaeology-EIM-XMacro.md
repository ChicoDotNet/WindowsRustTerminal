# Engineering Improvements 2021 — Action X-Macro archaeology

## `dev/migrie/eim/3475-action-xmacros`

**Disposition: NO-PORT / superseded by merged PR #11859.**

This branch is Mike Griese's 2021-12-01 proof-of-concept for issue #3475, converting ActionArgs boilerplate to X-macros. Its commits progress from `proof of concept` through converting the remaining args, local tests passing, comma-mitigation cleanup, and documentation.

Issue #3475 (`Automatically generate ShortcutAction code from ~JSON Schema~ X-Macros`) was closed as completed on 2021-12-06. The reviewed implementation was PR #11859, `Use x-macros for action args too`, opened the same day as this PoC's final functional work and merged on 2021-12-06. #11859 explicitly closes #3475 and records the architectural choice: X-macros provided the preferred balance between reducing boilerplate and keeping the implementation native C++.

The PoC and PR heads diverge from the same historical base because the reviewed work was reconstructed/rebased rather than merged from this exact ref. That does not represent a distinct product contract: both implement the same #3475 objective, while #11859 is the reviewed and shipped source of truth.

## Recovery guidance

For ActionArgs X-macro behavior and design intent, use merged PR #11859 and its tests. Do not resurrect this PoC branch.
