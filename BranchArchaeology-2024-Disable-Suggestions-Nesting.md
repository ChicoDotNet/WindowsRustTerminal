# 2024 Suggestions UI: disabling nesting while searching

## Historical source

- Upstream PR: `microsoft/terminal#17418`
- Historical branch: `dev/migrie/f/disable-nesting`
- Historical commit: `941ef48cfe8eee6b85ebfc2652d02543891968d1`
- Base at the time: `523af87e34161dfb989b38ef97c2d62b58b258c2`
- Author: Mike Griese (`zadjii-msft`)
- State: **closed without merge**

## Original idea

The proposal added a `nesting` parameter to `showSuggestions`:

```jsonc
{
    "action": "showSuggestions",
    "nesting": "disabled",
    "source": "all",
    "useCommandline": true
}
```

Default behavior remained `"enabled"`; `"disabled"` flattened matching snippets into one top-level list.

The user scenario was command-line-assisted search. If the user had already typed something like `gitco`, invoking suggestions could immediately surface a matching `git commit ...` snippet without forcing navigation through snippet categories.

## Implementation knowledge in the abandoned branch

The one-commit branch touched seven files and established a complete contract chain:

1. `ActionArgs.idl` introduced `SuggestionsNesting { Disabled, Enabled }` and added it to `SuggestionsArgs`.
2. `ActionArgs.h` added the JSON-backed action argument with default `Enabled`.
3. `TerminalSettingsSerializationHelpers.h` mapped `"enabled"` / `"disabled"`.
4. `ActionMap.h` / `.idl` propagated the nesting decision into the filtering API.
5. `ActionMap.cpp` recursively filtered nested commands:
   - `Enabled`: preserve parent nodes and reconstructed hierarchy.
   - `Disabled`: append recursively matching leaves directly into the current result list.
6. `AppActionHandlers.cpp` passed the action argument into the filter.

The conceptual algorithm was therefore:

```text
filter(node):
    matching leaf -> return leaf
    nested node -> recursively filter children

    nesting enabled  -> keep matching subtree under copied parent
    nesting disabled -> flatten matching descendants into current level
```

That algorithm remains useful independently of the historical code shape.

## Why upstream closed it

The PR was not rejected because flattening was technically invalid.

Dustin Howett proposed a better UX rule: make nesting effectively **transparent when the user is searching**, instead of requiring users to configure an explicit nesting mode.

Mike agreed that this was sane and closed the PR intentionally because the better behavior depended on prerequisite work:

1. merge the snippets pane (`#17330`),
2. extract `FilteredTask` into its own file,
3. support filtering on the literal `input` as well as the command/snippet name,
4. then implement transparent de-nesting during search.

The critical product insight is:

> hierarchy is valuable for browsing; hierarchy becomes friction during targeted search.

So the desired behavior is contextual rather than necessarily a persistent user preference.

## 2026 Contract Replay / architecture translation

During branch archaeology we replayed the old contract against the newer snippets architecture.

Replay source branch:

- `dev/migrie/replay-disable-nesting-source`

Replay implementation commit:

- `ff112cda9e3ad50956414457627e5ba0e8d91b6c`
- classification: **TRANSFORM**, not COPY

The modern architecture no longer uses the old synchronous `FilterToSendInput` boundary. The recovered idea maps naturally onto the newer asynchronous snippets pipeline:

```text
historical: ActionMap::FilterToSendInput(commandline, nesting)
modern:     ActionMap::FilterToSnippets(commandline, cwd, nesting)
```

The replay demonstrated the semantic translation:

- preserve the same `SuggestionsNesting` JSON contract if an explicit mode is ever desired;
- thread the mode through the async `FilterToSnippets` boundary;
- preserve hierarchy when enabled;
- recursively flatten matching descendants when disabled;
- keep the dedicated snippets pane hierarchical while allowing `showSuggestions` to request flattened results.

The replay was intentionally stopped before full build certification. Compilation of the isolated SettingsModel project reached unrelated generated-header dependencies (`ITerminalHandoff.h`). That information has no bearing on the product/architecture knowledge being preserved here, and further CI work would not increase the value of this archaeology record.

## Durable guidance

When implementing this idea in Rust or in a future Terminal architecture:

### Preferred product behavior

Prefer **automatic transparent nesting during active search** over a permanent user-facing switch, unless a concrete use case demonstrates the need for explicit control.

### Separate browsing from searching

- browsing with no query: retain hierarchy/categories;
- targeted query / existing command-line text: return a flat relevance-oriented result set;
- the dedicated snippets browser may remain hierarchical even if the transient suggestions UI flattens search results.

### Filtering contract

Filtering should consider at least:

- display/name text,
- literal snippet input,
- current command-line prefix,
- optionally current working directory/context where relevant.

### Architecture rule

Do not couple tree traversal to UI presentation. A useful internal form is:

```text
collect matching leaves recursively
        +
presentation policy: hierarchical | flat
```

This makes the search semantics reusable across command palette, suggestions popup, snippets pane, and future Rust UI surfaces.

## Branch disposition

Knowledge from the following archaeology/replay branches is now consolidated here:

- `dev/migrie/replay-disable-nesting-source`
- `dev/migrie/ci-suggestions-nesting-contract-replay`
- `dev/migrie/ci-certify-disable-nesting-ff112`

They are archaeology infrastructure, not durable product branches, and may be deleted once branch cleanup is performed.

For the original upstream branch `dev/migrie/f/disable-nesting`, this document preserves both the implementation idea and, more importantly, the reason it was closed. It is therefore also **safe to retire for knowledge-preservation purposes**.
