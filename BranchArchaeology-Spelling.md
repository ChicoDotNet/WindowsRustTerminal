# Spelling workflow branch archaeology

This ledger preserves the intent and disposition of the historical `dev/migrie/spellbot-cve` ref.

## `dev/migrie/spellbot-cve`

The branch diverged from historical main at `83aff8d6f0cf0d71e6d92e4f9ea8657ca531d875` and contains two exclusive commits relative to modern `main`.

`ad7ab2ff1a1c924eda6ef399c2e4404bac15b883` (2022-08-31), **Disable this temporarily**, comments out `.github/workflows/spelling2.yml`. The historical workflow identifies the reason as check-spelling advisory `GHSA-g86g-chm8-7r2p`, and the commit explicitly says the workflow can be turned back on for version 0.0.21.

`7586bb3e97243ddb9492b18461aa4c8cf9f66ed1` (2022-08-31), **Migrate spelling-0.0.21 changes from main**, synchronizes the spelling configuration after that temporary mitigation; it is not an independent product contract.

Current `main` has spell checking enabled and pins `check-spelling/check-spelling` to commit `cfb6f7e75bbfc89c71eaa30366d0c166f1bd9c8c`, documented by the workflow as **v0.0.26**. The modern workflow also uses the later restricted-permissions architecture and separate reporting/update jobs.

The durable intent was simply: do not run the affected action revision; resume on a corrected revision. Modern `main` satisfies that intent with a substantially later pinned revision and evolved workflow design.

Disposition: **NO-PORT / superseded**.

Do not restore the commented-out 2022 workflow or transplant either historical commit.

Branch retirement: **SAFE TO DELETE `dev/migrie/spellbot-cve`**.
