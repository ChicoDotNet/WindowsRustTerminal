# Search v2 / v3 branch archaeology

This ledger preserves the genealogy and disposition of `dev/migrie/search-v2-v3`.

## `dev/migrie/search-v2-v3`

### Why the branch looks much larger than it is

Against modern `main` the branch reports **75 commits ahead**, with merge base `c4436157c116880c320b5df278d4d886a930c0f3` (2023-08-28). That count is historical topology, not 75 independent Search features. The branch combines an older first-parent Search experiment, the `dev/migrie/fhl/search-marks` line, and a later merge of then-current `origin/main` (`444398d630c4ec48716d16b0faaf1318edc5f1dd`). Its resulting functional delta is concentrated in buffer search, `ControlCore`, `SearchBoxControl`, `TermControl`, search-result event args and scrollbar/search-marker integration.

### Functional lineage

The important merged parent is `c1d1f6371e50c2b8f4d51a0ca203d620c145a7cb`, whose message is **Merge branch 'dev/migrie/fhl/search-marks' into dev/migrie/search-v2-v3**. That merge brings together the Search-v2 work with the line that became the reviewed product change.

The branch explored the following durable Search contract:

- live search as the query changes;
- enumerate/cache all matches rather than only stepping one result at a time;
- report total match count and current match index to the search UI;
- expose search-hit locations so they can be represented on the scrollbar;
- keep match/scroll presentation in sync as buffer position changes.

The branch also contains abandoned asynchronous-search scaffolding. Commits `574f30b4249a576413087a4bc11bb7250055d694` (**dead code construction crew**) and `198decc6e09aa445d90d19c0c4c5b4736926c91b` (**dead code construction crew part 2**) explicitly remove/comment out that discarded implementation. That is useful evidence that the coroutine/throttled-search path itself is not a recovery target.

This work intersected directly with upstream PR **#15858 — Use ICU for text search**, merged 2023-08-24. ICU changed the core search model so all results could be accumulated efficiently and cached, making the UI/search-marks design substantially simpler.

### Authoritative successor

PR **microsoft/terminal#14045 — Show number of search results & positions of hits in scrollbar** is the authoritative destination. Its description explicitly says it resurrects #8588 and implements:

- search status in `SearchBox` (match count + current index),
- live search,
- match positions in the scrollbar,
- with the implementation made substantially easier after #15858.

It closes #8631 and #6319, both tracked by the Search v2 epic #3920.

Most importantly, the final PR head `bd2f2405ccb7bad6d82703abbcc9e1e27f4fca95` is a **direct descendant of the historical `search-v2-v3` tip `971e7c5c91b65d2d5739f1a97fc58ebeb499cbe2`**: comparing the branch tip to the reviewed PR head shows the PR head **8 commits ahead and 0 behind**, with the branch tip as merge base. This is stronger than similarity-by-diff: the reviewed PR literally continued from this branch state.

PR #14045 merged on 2023-09-05 as `0cbde94e4b2e530eac00acc7de1808fe2fd10273` for Terminal v1.19. Modern `main` has since evolved Search further, including regex support tracked in the same Search v2 epic.

### Disposition

**ALREADY ABSORBED** by the `search-v2-v3` → reviewed #14045 lineage, with #15858 supplying the modern ICU search substrate.

Do not transplant the 75-commit historical topology or revive the abandoned async scaffolding. If Search status/marks regress, replay the modern contract from #14045/#3920 against the current ICU-backed search implementation: live query updates, accurate `current/total` status, match markers, mutation/staleness handling and scrollbar synchronization.

### Branch retirement

**SAFE TO DELETE: `dev/migrie/search-v2-v3`**.

The branch's apparent size has been compressed into its actual genealogy and reviewed destination; no unique implementation knowledge requires preserving the ref.
