# 1.14 `CompareInBounds` backport archaeology

## Branch

- `dev/cazamor/1.14/replace-compareInBounds`
- tip date: 2022-06-17
- historical merge base: `a657cb0192d32dfb2b3e660bb6eddefd5a7445ec`
- at archaeology time: 24 commits ahead / 2005 commits behind current `main`

**Disposition: NO-PORT / rejected backport. Preserve the regression lesson and use the later reviewed successor.**

## What this branch was trying to do

This branch is the Windows Terminal 1.14 backport line for the UI Automation cleanup that replaced `Viewport::CompareInBounds()` calls with ordinary `til::point` ordering/comparators.

The relevant upstream history is:

1. **PR microsoft/terminal#13244** — the original mainline cleanup. It broadly replaced `CompareInBounds()` usage with point comparisons and also simplified related bounds checks.
2. **PR microsoft/terminal#13313** — the 1.14 backport corresponding to this branch. It was **closed without merge** after regressions were found in the original mainline change.
3. **PR microsoft/terminal#13907** — reverted the original mainline change while the regression was being addressed.
4. **PR microsoft/terminal#14551**, *Replace UIA CompareInBounds with til::point comparators* — the corrected successor, merged into `main` as `7ab0e982c74f69c4c136dea7cad4e75d435e8249`.

The 1.14 branch therefore represents a patch that upstream intentionally chose **not** to ship in that servicing release.

## Regression lesson recovered

The important failure was not the idea of comparing `til::point` values directly. Carlos Zamora's explanation on the corrected PR identifies the accidental semantic change that made the original patch unsafe:

```cpp
if (!bufferSize.IsInBounds(_start, true) || !bufferSize.IsInBounds(_end, true))
```

was changed to:

```cpp
THROW_HR_IF(E_FAIL, !bufferSize.IsInBounds(_start) || !bufferSize.IsInBounds(_end));
```

Removing the explicit `true` changed the accepted boundary semantics and caused regression #13866. The release was approaching, so reverting the broad change was safer than trying to salvage the backport in place.

The later PR #14551 deliberately kept the required boundary behavior while reintroducing only the safe comparator cleanup. Its review discussion explicitly calls out that distinction.

## What shipped later

Current `main` demonstrates the intended successor semantics. For example, `UiaTextRangeBase` now orders range endpoints with ordinary point operations:

```cpp
_start = std::min(start, end);
_end = std::max(start, end);
```

and endpoint crossing is handled with `std::min` / `std::max` rather than `CompareInBounds()`.

This is the durable direction from #14551: **point ordering can use the natural `til::point` comparator, but buffer-boundary checks must retain their own explicit inclusive/exclusive contract.** Those are separate concepts and must not be collapsed during refactoring.

## Recovery guidance

If similar UIA/bounds cleanup is revisited:

1. classify every old `CompareInBounds` use before replacing it:
   - pure point ordering -> natural `til::point` comparisons are appropriate;
   - buffer membership / endpoint validity -> preserve the exact inclusive/exclusive semantics;
2. add or replay tests at the document-end / buffer-end boundary before changing helper calls;
3. do not infer `IsInBounds(point)` and `IsInBounds(point, true)` are interchangeable;
4. prefer the reviewed implementation lineage from #14551 over the abandoned 1.14 backport;
5. for servicing branches, treat a reverted parent change as a hard stop unless a narrowly corrected backport is independently certified.

## Retirement status

`dev/cazamor/1.14/replace-compareInBounds` is **safe to delete after this archaeology commit is present on `dev/cazamor/main`**.

Its unique implementation was an intentionally rejected servicing backport, the regression mechanism is preserved here, and the corrected implementation is traceable to merged PR microsoft/terminal#14551.