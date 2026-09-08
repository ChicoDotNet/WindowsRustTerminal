# David Federman branch archaeology

## `dev/dfederm/msbuildcache-reenable`

**Disposition: ALREADY ABSORBED / superseded by later MSBuildCache integration. SAFE TO RETIRE.**

Tip: `c77ac66c3392584a564e61074ec8dffe4b35feaa` (`Re-enable MSBuildCache`).

The corresponding upstream work is PR #19084, `Re-enable MSBuildCache`, which was closed without merge in October 2025. That abandoned branch is no longer the canonical state: current upstream `main` contains `enableCaching` / `MSBuildCache` pipeline handling, and PR #20622 (`Update MSBuildCache to 0.1.340-preview`) merged on 2026-08-31.

The capability survived through later reviewed integration even though this specific PR did not. Replaying `c77ac66c` would risk restoring stale cache configuration rather than the current implementation.

## Recovery guidance

For MSBuildCache behavior, start from current `main` and the merged MSBuildCache lineage (#17393 and later updates such as #20622). Do not resurrect #19084.

## Namespace result

`dev/dfederm/msbuildcache-reenable` can be retired. `dev/dfederm/main` is the curated authority for this namespace.
