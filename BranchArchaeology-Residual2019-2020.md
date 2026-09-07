# Residual 2019–2020 branch archaeology

This supplement records early historical branches discovered after the initial chronological cohorts had already been retired. The same rules from `BranchArchaeology.md` apply: preserve intent and recovery value, do not transplant obsolete implementations merely to save a branch ref.

## `dev/migrie/PR#3181-comments`

- Historical tip: `e83b69616da8a190adf8fd74af7bed5e3e6f7e24` (2019-11-05).
- Historical intent: Mike Griese's local review/understanding sandbox over mcpiroman's `microsoft/terminal#3181`, “Snap to character grid when resizing window”.
- Historical maturity: exploratory. The branch reorganizes the pane-snapping machinery, adds explanatory `MG:` comments, and the tip explicitly describes the work as changes that helped Mike understand the code. Some comments record uncertainty rather than a concluded design.
- Modern reading: upstream PR `#3181` was ultimately merged on 2020-01-08 as `d4c527607a734f37ea93bd57e27355d54ed51dde`, closing the relevant snapping issues. The merged PR is the authoritative product lineage; this local sandbox is neither the canonical PR head nor a later independent contract.
- Disposition: **NO-PORT / superseded**.
- Recovery value: provenance only; do not carry the exploratory refactor/comments forward as product architecture.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/s/1203-cursorTextColor`

- Functional spec head: `e48d01f5c2ec4b25b4d6d18c303a9a09da6c1c82`; later branch-tip commits are spelling maintenance.
- Historical intent: specify a configurable `cursorTextColor` setting allowing `null`, `textForeground`, `textBackground`, or an explicit color for the glyph beneath/above the cursor.
- Modern reading: the visible core problem in `microsoft/terminal#1203` was solved by `microsoft/terminal#6337` (`1fcd95704d6a52cf47e35445f8b557753b37ef4f`), which draws the filled-box cursor beneath text in the DX renderer. The broader setting design was submitted as `microsoft/terminal#6151` and closed without merge on 2021-01-21. The modern product does not expose the proposed `cursorTextColor` setting.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value: preserve the product idea—not the 2020 implementation/spec—as a possible future contrast/customization requirement. Any revival should be redesigned against the modern renderer and settings model.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/move-lib-up-and-dll-down`

- Functional tip: `ebf75edab1b5272bd5341cb78716d19b1d507e47` (2020-08-20); later tip commits are spelling maintenance.
- Historical intent: rearrange the TerminalControl library/DLL project structure to improve separation and build organization.
- Modern reading: upstream later implemented the definitive architecture in `microsoft/terminal#9472` (`d749df70ed2b9decd123248713c3a60a736da57a`), formally splitting TerminalControl into separate lib/DLL projects, renaming the namespace to `Microsoft.Terminal.Control`, enabling the testing architecture, and preparing the later `ControlCore` split.
- Disposition: **NO-PORT / superseded**.
- Recovery value: none beyond provenance; `#9472` is the materially more complete successor.
- Branch retirement: **SAFE after this ledger commit**.

## Deletion set

Once this file is present on `dev/migrie/main`, the following refs no longer carry unique knowledge requiring preservation as branches:

- `dev/migrie/PR#3181-comments`
- `dev/migrie/s/1203-cursorTextColor`
- `dev/migrie/move-lib-up-and-dll-down`
