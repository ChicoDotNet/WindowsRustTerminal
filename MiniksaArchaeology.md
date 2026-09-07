# Mike Niksa branch archaeology

This ledger preserves the useful development intent recovered from historical `dev/miniksa/*` refs so the old refs can be retired without losing engineering knowledge.

## Completed recovery cluster

### `input2` — PORT AS CONTRACT REPLAY

Historical branch: `dev/miniksa/input2`

The historical implementation explored cooked and ANSI input behavior across text entry, alias expansion, code-page matrices, DBCS lead/trail handling, and code-page changes during reads. The old implementation is intentionally not transplanted onto the modern codebase.

Recovered durable knowledge is preserved by PR #50, **Replay complete input2 ANSI input contract matrix**, merged into `dev/miniksa/main` as merge commit `17f233f26adb5f454840cad06110a935788935b2`.

The modern replay preserves seven coherent contracts:

1. cooked text entry;
2. cooked alias processing;
3. ANSI input/output code-page and line-mode permutations;
4. byte-by-byte DBCS reads;
5. DBCS lead-byte / remaining-string stitching;
6. code-page change while a multibyte character is partial;
7. code-page change between complete multibyte characters.

Important classification discovered during replay:

- The 2020 alias expectation used CR-only endings. Current behavior consistently returns CRLF; current alias implementation corroborates CRLF. The CR-only behavior is classified as superseded legacy behavior.
- Historical CHv1 font-dependent corruption behavior was explicitly non-durable and is not resurrected as a modern contract.
- The disposable CI replay branches successfully exercised all seven real TAEF contracts. A later aggregate/certifier workflow step remained red because of workflow bookkeeping, not because any of the seven contracts failed. The workflow helper is not part of the recovered product contract.

Disposition: **PORT COMPLETE / historical implementation superseded**.

Branches made disposable by this recovery:

- `dev/miniksa/input2`
- `dev/miniksa/input2-alias-contract-replay`
- `dev/miniksa/ci-input2-alias-contract-replay`
- `dev/miniksa/ci-input2-alias-contract-replay-v2`
- `dev/miniksa/ci-input2-full-contract-replay`

### `perf_buffer_dig` — NO-PORT / SUPERSEDED

Historical branch: `dev/miniksa/perf_buffer_dig`

This branch is a renderer/buffer performance experiment touching `AttrRow`, renderer base interfaces, DirectX, GDI, UIA, VT rendering and related paint paths. Its functional history predates the modern renderer architecture by thousands of commits and is entangled with obsolete repository/spelling infrastructure.

The useful idea is retained as provenance: investigate renderer performance by reducing repeated buffer/attribute digging and unnecessary work across render engines. The concrete historical patch is not a safe or useful transplant target because the modern renderer and text-buffer ownership model have evolved substantially beyond the experiment.

Disposition: **NO-PORT / superseded by newer renderer architecture; retain the optimization intent only, not the patch**.

Branch made disposable by this classification:

- `dev/miniksa/perf_buffer_dig`

## Durable lane

Always retain `dev/miniksa/main`. Historical helper/replay branches are disposable once their disposition is recorded here and any useful contract has been integrated.
