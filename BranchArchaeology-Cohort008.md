# Cohort 008 — issue #2988 ConPTY ownership and foreground lineage

This cohort records the 2022 prototype lineage for `microsoft/terminal#2988` so the inherited experiment refs can be retired without losing the reasoning that led to the final ConPTY ownership/focus design.

## `dev/migrie/b/2988-niksa-msgs-prototype`

- Historical tip: `de7add18ee330b01202a0697fb60e2c15aca781c`; the functional proof-of-concept is `5923cf3261cf2c77e1864cc5e8007b5489db343f` (2022-03-28).
- Historical intent: prove that GUI applications launched from a ConPTY session, notably `Out-GridView`, could receive foreground rights despite the pseudo console not being the visible Terminal window.
- Prototype behavior: the branch changed `ConsoleHandleConnectionRequest` so any process in ConPTY received foreground permission (`inConpty || hasFocus`). The commit explicitly calls this a hack and a proof of concept.
- Modern reading: this demonstrated the missing ownership/focus signal but granted too broadly. The production lineage refined the idea so foreground rights are only granted when the terminal that owns the pseudoconsole actually has focus.
- Successor lineage:
  - `microsoft/terminal#12526`, merge `26d67d9c0af0a05b0e65aa76537006862d39535b`: Terminal tells ConPTY which HWND owns the pseudo window; part 1/3 of #2988.
  - `microsoft/terminal#12799`, merge `a496af361458dcf6c185c1d7923b78f7a21017ec`: Terminal focus state is sent to ConPTY so spawned GUI windows can legitimately enter the foreground.
  - `microsoft/terminal#12899`, merge `0da5bd77269f6060a38e6e075ce97a9b0e3e6991`: security refinement verifies the pseudoconsole owner itself is actually foreground before granting foreground rights.
  - `microsoft/terminal#12900`, merge `87f5034db1eb0102358d05eeff82ebcc8032c9be`: permanent FocusIn/FocusOut VT-input plumbing; combined with the ownership work, completes #2988.
- Disposition: **NO-PORT / superseded and security-refined**.
- Recovery value: preserve the original diagnostic insight — ConPTY needed an explicit, trusted relationship between the visible terminal owner and console-process foreground permission — but never resurrect the blanket `inConpty || hasFocus` grant.
- Branch retirement: **SAFE after this ledger commit**.

## `dev/migrie/b/2988-merged-prototypes`

- Historical tip: `5a665ae533e368a18f4b9b87c890845597cb6b47`; the important functional checkpoint is `479c6c9f08cf273768e64f9f7fbc74a2c231b815` (2022-03-31), whose message records that the combined prototype worked.
- Historical intent: combine the owner/reparenting prototype with focus notification. The branch sends `FocusIn`/`FocusOut` (`ESC [ I` / `ESC [ O`) from Terminal, plumbs `FocusChanged` through VT dispatch into conhost, and updates `ProcessHandleList.ModifyConsoleProcessFocus`.
- Modern reading: this is the direct prototype ancestor of the production sequence above, not an independent unfinished feature. The final PR family decomposed it into reviewable ownership, focus, security, and VT-input pieces and merged them to upstream `main`.
- Issue state: `microsoft/terminal#2988` is closed as completed / fix committed (2022-04-28).
- Disposition: **ALREADY ABSORBED**.
- Recovery value: the architectural lesson is preserved by the final PR chain: owner HWND establishes trust/relationship; FocusIn/FocusOut communicates current focus; ConPTY validates the owner before granting foreground rights.
- Branch retirement: **SAFE after this ledger commit**.

## Cohort 008 deletion set

- `dev/migrie/b/2988-niksa-msgs-prototype`
- `dev/migrie/b/2988-merged-prototypes`
