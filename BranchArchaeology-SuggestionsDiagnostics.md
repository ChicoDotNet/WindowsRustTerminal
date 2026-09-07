# Suggestions UI and diagnostic branch archaeology

This supplement records four historical `dev/migrie/f/**` branches whose useful disposition is clearer when treated together: three Suggestions UI experiments and one local Narrator diagnostic tool.

## `dev/migrie/f/disable-nesting`

Historical tip: `941ef48cfe8eee6b85ebfc2652d02543891968d1`.

Intent: add a `nesting` parameter to `showSuggestions` so snippets could be flattened into a single top-level list, making command-line-driven filtering easier.

The branch became microsoft/terminal PR #17418, but that PR was closed without merge. During review, Dustin Howett proposed making nesting transparent automatically while searching instead of adding a user-facing nesting switch. Mike Griese agreed and explicitly closed the PR pending prerequisites such as the snippets pane and filtering on literal input.

Modern reading: the explicit `nesting` setting from this branch was a rejected product design. Current `SuggestionsControl` still filters the current command level rather than carrying this explicit setting.

Disposition: **NO-PORT / rejected design**.

Recovery value: preserve the design conclusion, not the patch: if flattening is revisited, prefer search-driven transparent nesting over a persisted `nesting` option.

Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/filter-weight-input-too`

Historical tip: `6de80103a1263f82fa66e4d117cdef10dc330201`.

Intent: make filtering/weighting consider a snippet command's literal `sendInput.input`, not only its display name. This directly followed the #17418 review discussion that filtering literal input should precede transparent nesting.

Historical implementation: refactored the old `FilteredCommand` / `HighlightedText` weighting machinery and experimented with including command input in matching.

Modern reading: the old weighting implementation has been superseded by the `fzf` matcher. Modern `FilteredCommand` matches `IPaletteItem.Name` and `Subtitle`; `ActionPaletteItem::Subtitle` is a language-neutral display name, not `SendInputArgs.Input`. Therefore the exact product idea in this branch is not currently implemented for named snippets.

Disposition: **NO-PORT / recovery candidate**.

Recovery value: retain the behavioral question as a future modern feature/test: a named `sendInput` snippet should be discoverable by typing text that occurs only in its literal input. If implemented, do it against the modern `fzf`/`IPaletteItem` abstraction rather than transplanting this pre-fzf patch.

Branch retirement: **SAFE after this ledger commit** because the unique idea and modern recovery path are recorded here.

## `dev/migrie/f/sxnui-font-size-change`

Historical tip: `63b15994062abcf3f8025e167d1c630d242b7b09`.

Intent: make Suggestions UI list item font size follow a dynamic control font height.

Historical maturity: explicitly experimental. The patch describes itself as an “incredibly bodgy” workaround and sets `ListViewItem.FontSize` during `_choosingItemContainer` because the attempted XAML binding/style approaches were not working.

Modern reading: current `SuggestionsControl.xaml` deliberately uses a fixed 32-pixel list item and `FontSize="12"`. The runtime container mutation from this prototype is not present.

Disposition: **NO-PORT / abandoned prototype**.

Recovery value: provenance only. If dynamic font scaling becomes a product requirement, implement it from the current XAML/control architecture rather than reviving the container-preparation hack.

Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/f/narrator-buddy`

Historical tip: `98dc5a519e7e67f045a08e76b4b82ffc076a232b`.

Intent: local accessibility diagnostics. The branch modified only `src/tools/scratch/main.cpp` to listen to Narrator ETW events and print spoken output, including diagnostics for exhausted ETW trace slots.

No PR was opened from this branch, and the Narrator Buddy code is not present in current `main`.

Disposition: **NO-PORT / one-off diagnostic tooling**.

Recovery value: the technique is documented here: Narrator speech can be inspected through its ETW provider when debugging accessibility announcements. The scratch implementation itself is not a product dependency or test contract.

Branch retirement: **SAFE after this ledger commit**.

## Deletion set

Once this supplement is present on `dev/migrie/main`, the following refs may be deleted:

- `dev/migrie/f/disable-nesting`
- `dev/migrie/f/filter-weight-input-too`
- `dev/migrie/f/sxnui-font-size-change`
- `dev/migrie/f/narrator-buddy`
