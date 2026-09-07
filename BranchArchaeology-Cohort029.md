# Branch Archaeology — Cohort 029

This cohort records the evolution from attempted source generation for action boilerplate to the X-macro design that ultimately shipped.

## `dev/migrie/f/lets-just-generate-these`

- Functional work is from 2020-07-21; later tip commits are spelling migrations.
- Commit `ce0596abcca79bba1597768d5644ab32e06cf464` states the intent directly: `This auto-generates ActionAndArgs.cpp`; follow-up `b0a838d9d663f134c459c0b07c0968164cc55768` begins extending the generator to `ActionArgs.h`.
- The branch therefore explored external/source generation as a way to remove repetitive action boilerplate.
- The durable solution went in a different direction. PR #11859 (`Use x-macros for action args too`) merged as `81d92975372a48469faac7423d8c3d74d5a5b500` and explicitly says that, although the author originally wanted to do more, X-macros struck the right balance between ease of use, native C++ support and amount of synthesized boilerplate.
- Disposition: **NO-PORT / source-generator experiment superseded by the native X-macro design in #11859**.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/xmacro`

- Functional commit `a94ac92b0339bedc052b1d3c90f74fb124ae51ac`, 2021-03-04: `Did you know about X Macros? Now you do!`.
- This is the exploratory branch for using X-macros to collapse repeated action boilerplate.
- Definitive first productization: PR #9667 (`Add X Macro for fun and for profit`) merged as `c09472347c5f92317d0d907f87421fd14066537f`, defining all ShortcutActions once and synthesizing declarations/handlers/IDL boilerplate from that list.
- Definitive ActionArgs extension: PR #11859 merged as `81d92975372a48469faac7423d8c3d74d5a5b500`, applying X-macros to action arguments and closing #3475.
- Disposition: **ALREADY ABSORBED / superseded by #9667 + #11859**.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 029 deletion set

- `dev/migrie/f/lets-just-generate-these`
- `dev/migrie/f/xmacro`
