# Dustin Howett — ConPTY archaeology

This file extends `BranchArchaeology.md` for early ConPTY experiments. Where an older ledger entry provisionally says to retain a branch pending lineage reconstruction, the completed analysis here is authoritative.

## `dev/duhowett/conpty-flags`

- Functional commit: `b74666180962dabd58ebd0f1764dc9ad60049921` (2019-11-30), `HAX: conpty flags`.
- Historical intent: replace a one-off `inheritCursor` boolean with a generic ConPTY/VT behavior flag set (`VtOption`) that could flow from command-line parsing through `VtIo` into the VT render engines. The prototype included `InheritCursor`, `ForceAsciiOnly`, and `AmbiguousCharactersNarrow`, and introduced `--narrow` for the last behavior.
- `InheritCursor` lineage: the durable behavior survived as the supported `PSEUDOCONSOLE_INHERIT_CURSOR` flag. Current ConPTY plumbing translates that flag to the conhost `--inheritcursor` behavior, and `ConptyConnection` sets it through its connection flags. Later work such as `microsoft/terminal#17334` (`bdc7c4fdbc8385bbf156fbac55f097117e1a1a3b`, 2024) actively uses the flag for buffer restore, and `#17574` (`1ac221a7a5fa2f5a24b9fa8955367fe89469a67b`) hardened cursor inheritance with a timeout.
- Ambiguous-width lineage: the prototype's `AmbiguousCharactersNarrow` bit is not the retained contract. `microsoft/terminal#2928` / `1925173b02568b0d4369bd35a429a8d059adc586` (2019-10-15, before this HAX branch) established narrow as the terminal default for ambiguous-width glyphs. In 2026, `#19864` / `3b8c5606ee1866e7bf5c4e84e962798492282097` introduced the global `compatibility.ambiguousWidth` narrow/wide policy and propagates the wide override through `PSEUDOCONSOLE_AMBIGUOUS_IS_WIDE`. This is a more complete and supported expression of the same compatibility concern.
- `ForceAsciiOnly` lineage: this flag merely generalized the historical `XTERM_ASCII` renderer-mode boolean. That renderer path is not part of the current architecture and carries no independent product contract that should be recovered.
- Architectural reading: the important insight was correct—ConPTY behavior switches should travel as an explicit option/flag set rather than proliferating unrelated booleans. Modern ConPTY exposes exactly this kind of flag surface (`PSEUDOCONSOLE_INHERIT_CURSOR`, glyph-width flags, ambiguous-width policy), but through supported APIs and modern text-measurement architecture rather than the 2019 `VtOption` patch.
- Classification: **NO-PORT / ALREADY ABSORBED as an architectural ancestor**.
- Recovery value: provenance only. Preserve the mapping from the early experiment to the supported pseudo-console flags; do not replay the historical `VtOption`, `--narrow`, or `XTERM_ASCII` implementation.
- This supersedes the provisional `CONSERVAR / unresolved mixed ConPTY behavior prototype` note in `BranchArchaeology.md`.
- Disposition: **SAFE TO DELETE**.

## Deletion set

- `dev/duhowett/conpty-flags`
