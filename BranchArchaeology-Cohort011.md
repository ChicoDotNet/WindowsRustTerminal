# Cohort 011 — WSL paths, test bootstrap, XAML teardown and ConPTY windowing

This cohort records the durable intent behind ten historical `dev/migrie/b/**` branches so their refs can be retired. Open upstream issues remain open where appropriate; retiring a branch does not imply the product issue is solved.

## `dev/migrie/b/11994-wsl-mangline`

- Issue: `microsoft/terminal#11994`, `//wsl$` paths failed through `MangleStartingDirectoryForWSL`.
- This branch was an earlier mangling attempt. The reviewed successor was `dev/migrie/b/11994-wsl-mangling-but-for-real`.
- Final fix: PR #12102, merge `b87b809fa0768eded5596a852016489b7941d4df`, moved the mangler into a testable seam, added tests, and converts `//wsl$` back to a Windows UNC form such as `\\wsl$\foo\bar`; explicitly closes #11994.
- Disposition: **NO-PORT / superseded by tested successor**.
- Recovery value: WSL starting-directory normalization must distinguish Linux-looking syntax from the special Windows WSL UNC namespace; keep this behavior under focused tests.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/12196-shim-localization`

- Issue: `microsoft/terminal#12196`, elevated profile UAC reported an Unknown Publisher.
- Historical experiment: commit `49f31554c14c1b969f2bbc390321275792ec59ad`, message `might fix this?`, tried resolving the packaged `WindowsTerminal.exe` through `GetWtExePath()` instead of assuming the executable adjacent to `elevate-shim.exe`.
- The issue was ultimately closed `Resolution-External`; on 2022-04-28 Mike Griese reported that he could no longer reproduce it and believed the OS side had fixed it.
- Disposition: **NO-PORT / external platform resolution**.
- Recovery value: publisher identity/UAC behavior depends on the actual packaged executable and OS identity chain. If it regresses, first reproduce with signed packaged builds and current OS behavior before changing shim path resolution.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/12353-with-null`

- Issue: `microsoft/terminal#12353`, profiles from some WSL fragments could retain `startingDirectory: "~"` even when the fragment replaced the command line with a distro launcher such as `ubuntu.exe`.
- Final fix: PR #12437, merge `1870feeca3db191355aa45dd8cc910ede1c9e222`, never lets literal `~` reach a Windows process starting directory. If it cannot be mangled into the WSL command line, it falls back to `%USERPROFILE%`; includes tests and closes #12353.
- Disposition: **NO-PORT / superseded by tested final fix**.
- Recovery value: `~` is shell syntax, not a valid Windows process path; resolve it into the WSL launch contract or choose a valid Windows fallback.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/12356-attempt-2`

- Issue: `microsoft/terminal#12356`, TabView background failed to follow the requested Terminal theme after a resource refactor.
- `attempt-2` was not the final resource model. The reviewed successor was `dev/migrie/b/12356-attempt-3`.
- Final fix: PR #12460, merge `5ba0d618f54bf7b7588019052bf84071ad914ede`, manually sets `TabViewBackground` resources because `App.xaml` resources cannot safely depend on runtime element theme resources; closes #12356.
- Disposition: **NO-PORT / superseded by merged successor iteration**.
- Recovery value: application-level XAML resources and runtime element theme resources have different lifetimes/theme evaluation semantics; do not assume app resources will re-evaluate with per-element theme changes.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/12387-trim-spaces`

- Issue: `microsoft/terminal#12387`, trimming trailing whitespace from pasted text broke multiline pastes, including whether the final command executed.
- Final fix: PR #13698, merge `1e18ab9442e92657aea8437de6acdf30ae692c59`, skips whitespace removal for multiline paste, adds tests, and closes #12387.
- Disposition: **NO-PORT / superseded by tested fix**.
- Recovery value: paste normalization that is reasonable for a single line is not semantics-preserving for multiline command input; multiline paste must retain line-ending/trailing-whitespace intent.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/12413-OnUnhandledException`

- Issue: `microsoft/terminal#12413`, crash during XAML `NavigationViewItemRevokers` teardown.
- The issue was tracked as an external XAML/MUX problem and is closed `Resolution-Fix-Committed` / `Tracking-External`.
- This exact branch later became PR #13744. That PR describes the catch-all `UnhandledException` handler as explicitly experimental, says it would only swallow XAML teardown exceptions (not access violations), states that it was not needed for the #12413 follow-up, and was closed without merge.
- A MUX 2.8 validation branch (#13657) likewise treated #12413 as an external UI-framework work item rather than Terminal-owned exception policy.
- Disposition: **NO-PORT / rejected workaround for externally owned teardown defect**.
- Recovery value: do not mask framework teardown faults with a global exception eater as a substitute for fixing ownership/lifetime. Revalidate current WinUI/MUX behavior and only add narrowly scoped recovery if a Terminal-owned invariant is demonstrated.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/12788-did-it-work`

- Issue: `microsoft/terminal#12788`, `runut` on a clean contributor machine cannot install/run local packaged tests because `Microsoft.VCLibs.140.00.Debug` is missing. **The issue remains open.**
- Historical commits are explicitly exploratory: `maybe maybe maybe`, `this aint it`, and `This cannot possibly work, right?`.
- The most developed attempt (`3c54c07f30fbb7e7d9bdf64272e9a804b4a67a3f`) does not provision the missing local dependency. It moves `*LocalTest*.dll` execution into a separate Azure Pipelines stage/job using a hosted Windows image and publishes its logs separately.
- Disposition: **NO-PORT / recovery candidate; contributor-bootstrap issue remains open**.
- Recovery value: distinguish two contracts: CI must run LocalTests in an environment where packaged-test prerequisites exist, and a clean contributor machine needs documented/automated installation of those prerequisites. Moving tests between CI jobs does not solve local bootstrap.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/12911-wpf-focus-fg`

- Issue: `microsoft/terminal#12911`, port the #2988/#12526/#12799/#12899/#12900 owner/focus/foreground sequence to the WPF TermControl used by Visual Studio. **The issue remains open.**
- Historical functional checkpoint `d6a02c86fdacb5f2e384af323d618aaaea34447d` is explicitly a stash, not a solution. Mike Griese wrote that the attempt "isn't it", that he did not know enough about the WPF ownership path, and critically that Terminal does not own the WPF ConPTY connection — Visual Studio does.
- Disposition: **NO-PORT / ownership-bound recovery candidate**.
- Recovery value: the WinUI owner/focus security contract from Cohort 008 still applies conceptually, but the integration seam must be implemented by/with the owner of the WPF ConPTY connection. Do not transplant the abandoned Terminal-side HWND stash.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/13066-sw_flash_repeatedly`

- Issue: `microsoft/terminal#13066`, `ShowWindow(GetConsoleWindow())` and ConPTY owner setup could cause repeated minimize/restore flashing and focus theft.
- This branch is an earlier attempt. The reviewed successor was `dev/migrie/b/13066-SW_FLASH-2`, upstream PR #13118.
- Final core fix: PR #13118, merge `77215d9d77b99b48d1ee8302736178f2ec9f3a77`, creates the pseudo window with its owner already established, uses `SetWindowLongPtr` rather than post-creation `SetParent`, preserves `SW_SHOWNOACTIVATE`, and documents `GA_ROOTOWNER`/`WS_POPUP` semantics.
- Disposition: **NO-PORT / superseded by reviewed P0 fix**.
- Recovery value: changing HWND ownership after a visible window exists can trigger OS show/activation side effects; initialize ownership before visibility and keep the pseudo HWND from stealing foreground.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/13066-for-defterm`

- This exact upstream branch became PR #13129.
- Final DefTerm completion: PR #13129, merge `d82af9367fe0e3339884027db10bce490f163389`, ensures initial visibility is also set for ConPTY windows created for Default Terminal connections and says it closes #13066 "for real".
- Disposition: **ALREADY ABSORBED**.
- Recovery value: the initial pseudo-window visibility/owner contract must be applied consistently to both normal and Default Terminal connection creation paths.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 011 deletion set

- `dev/migrie/b/11994-wsl-mangline`
- `dev/migrie/b/12196-shim-localization`
- `dev/migrie/b/12353-with-null`
- `dev/migrie/b/12356-attempt-2`
- `dev/migrie/b/12387-trim-spaces`
- `dev/migrie/b/12413-OnUnhandledException`
- `dev/migrie/b/12788-did-it-work`
- `dev/migrie/b/12911-wpf-focus-fg`
- `dev/migrie/b/13066-sw_flash_repeatedly`
- `dev/migrie/b/13066-for-defterm`
