# FHL 2021 — command-palette `select-list` archaeology

## Branch

- `dev/migrie/fhl-2021/cmdpal-select-list`
- tip `be43a081a3c17a4c5a11b4f8dfb066c5fe34443c` (2021-07-29)
- diverged from historical base `4b45bb8df1f90e6fc463172cb861bf968e162e77`
- 6 commits ahead of that historical line and roughly 2600 behind the current product line at review time

**Disposition: NO-PORT / dormant hackathon prototype. Preserve the contract and discoveries; do not resurrect the 2021 implementation.**

## Product idea recovered

The branch prototypes microsoft/terminal#8797, **Extension: Pipe output to command palette in Terminal**. The intended workflow was conceptually:

```text
git branch | wt -w 0 select-list --prefix "git checkout "
```

The lines arriving on standard input become command-palette choices. Choosing an item generates terminal input consisting of the configured prefix plus the selected line. This was a useful exploration of treating the command palette as an interactive selector for shell-produced data rather than only as a static menu of Terminal commands.

The issue remains open and is currently parked in Icebox upstream, so the *idea* is not rejected. However, the author explicitly described this implementation as an experiment / thought experiment created during hackathon week rather than production-ready work.

## Commit trail and what it taught us

The meaningful prototype chain is:

1. `cf964754988735df63929134d84302324326e0f5` — **update `wt.exe` to pass handles through to `windowsterminal.exe`**.
   - The crucial discovery was that the GUI process could read inherited standard input; the small `wt.exe` shim simply had not been creating the child process with handle inheritance enabled.
   - The experiment changed `CreateProcessW(..., bInheritHandles, ...)` from `FALSE` to `true` and proved that redirected/piped stdin could reach the Terminal process.
2. `445b985474b45865007d07957502f44ac0ab2a62` — **collect the input from stdin and pass to `Remoting::CommandlineArgs`**.
   - Standard input was promoted into the remoting command-line envelope so an invocation could carry more than argv + cwd across the window-routing boundary.
3. `5f5f47a2ac60332e60968bd60b87cf9542cb54eb` — **plumb input all the way through. Now need to attach to command palette**.
   - Added the draft `select-list` action/parser and transformed stdin lines into selectable commands.
4. `84e95cf06abef48393ae3f4109cca1b9caea115b` / `80a4fedc16dbf63a7265d1f43275a690df0081ac` — completed the hackathon demonstration sufficiently to exercise #8797 end to end.
5. `be43a081a3c17a4c5a11b4f8dfb066c5fe34443c` — spelling migration only; no new product contract.

The most durable technical learning is **not** the old `SelectListArgs` class. It is that a launcher/shim which spawns the real Terminal process can break shell pipeline semantics simply by failing to inherit the standard handles. Any future CLI feature that consumes stdin should test the entire launcher → process/remoting path, not only `GetStdHandle` inside the final executable.

## Prototype contract

The branch demonstrates this behavioral contract:

1. receive redirected/piped stdin from the `wt.exe` invocation;
2. preserve it across whatever routing/remoting is required to reach the target Terminal window;
3. split the input into candidate lines;
4. present those candidates in an interactive picker (then the command palette);
5. on selection, synthesize a `sendInput` action from `prefix + selected item`;
6. leave execution semantics explicit rather than silently executing arbitrary piped data.

That contract is the useful recovery target if the feature is revisited.

## Why the implementation is not a current patch

The prototype contains unmistakable hackathon scaffolding and unfinished behavior: `DebugBreak()` calls, TODO descriptions, draft action plumbing, simplistic one-shot stdin reading/splitting, and UI lifecycle work still called out in the issue discussion. Follow-up ideas included restoring the normal palette after selection or dismissal, trimming CR/whitespace, ignoring empty lines, adding `--suffix`, a short `sl` form, deciding target-window defaults, and optionally sending Enter.

The current repository does not contain `SelectList` / `select-list` product code. In the years since this experiment, the command palette, action model, remoting/window routing, and launcher architecture have evolved substantially. Cherry-picking these commits would therefore restore obsolete plumbing and debug scaffolding rather than implement the still-interesting product capability cleanly.

## Recovery guidance

If microsoft/terminal#8797 or an equivalent feature is revived:

- start with a new contract test around piped stdin reaching the intended Terminal invocation/window;
- explicitly define bounded/streaming behavior, encoding, line normalization, empty-line handling, cancellation, and maximum input size;
- treat shell input as untrusted data and keep selection separate from execution unless the user explicitly opts into execution;
- integrate with the current suggestions / command-palette architecture rather than recreating `SelectListArgs` from 2021;
- test `wt.exe`/launcher handle inheritance or its modern equivalent end to end;
- design window routing/remoting semantics deliberately for `-w` and for invocations that may create a new window.

## Retirement status

The branch is **safe to delete once this archaeology commit is present on `dev/migrie/main`**. The historical implementation is non-production prototype code, the durable contracts and launcher discovery are preserved here, and the upstream product idea remains discoverable as microsoft/terminal#8797.