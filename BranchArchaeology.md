# Gordon Lam branch archaeology

## `dev/yeelam/f/BuildTest`

**Disposition: NO-PORT / recovery-contract source for Rust CI. SAFE TO RETIRE after this ledger.**

This branch is a seven-commit ADO smoke-test/prototype line for the older `src/tools/wtr` Rust build path. It should not be merged or cherry-picked into the current fork because the fork's active Rust architecture now uses the repository Cargo workspace plus `.github/workflows/rust-ci.yml` and the curated `rust/*` integration lanes.

The branch nevertheless contains reusable CI knowledge that should survive branch retirement.

### Contracts worth preserving

1. **Exercise Cargo manifests in CI.** The prototype explicitly added Cargo.toml testing so Rust build failures become first-class pipeline failures.
2. **Keep experimental enablement scoped.** One commit temporarily default-enabled the Rust build for an ADO smoke test and explicitly said to revert that switch before merging to main. Treat forced enablement as test scaffolding, not product configuration.
3. **Validate Rust before unrelated C++ environment setup when possible.** The Rust install/build step was moved earlier so Rust pipeline validation did not depend on vcpkg/NuGet/VC environment setup.
4. **Detect partial VC tool installations robustly.** The prototype learned that package metadata can advertise a VC tools version whose directory is empty or partial. Recovery logic should verify the selected tool directory actually has usable `bin` contents and fall back to valid siblings; fail hard when none exists.
5. **Authenticate Azure Cargo feeds through configuration that the auth task can discover.** `CargoAuthenticate@0` was added to issue the Bearer token. The later fix removed `cratesIoFeedOverride` because that injected an environment registry that CargoAuthenticate could not discover; source replacement should live in `.cargo/config.toml` so the auth task can tokenize it and normal Cargo config resolution can use it.
6. **Map every build architecture explicitly.** The final branch includes `x86 -> i686-pc-windows-msvc` in addition to `x64 -> x86_64-pc-windows-msvc` and `arm64 -> aarch64-pc-windows-msvc`, and installs all required targets.
7. **Avoid fragile PowerShell argument splatting around cargo.** The prototype replaced an array-splat release argument after Cargo received a malformed `-` argument. Use explicit Debug/Release invocation branches when portability is more important than terseness.
8. **Propagate native-process failure explicitly.** After `cargo build`, check `$LASTEXITCODE` and fail the PowerShell step when Cargo fails.

### Current-fork interpretation

Current `main` has a dedicated GitHub Actions Rust workflow that runs formatting, Clippy, `cargo check`, tests, and contract harnesses against the repository workspace, rather than building the obsolete `src/tools/wtr` ADO prototype. Therefore the old branch implementation is not a PORT candidate.

If the project later restores Azure DevOps Rust builds, private Azure Artifacts Cargo feeds, or Windows x86 Rust matrix coverage, replay the contracts above against the then-current CI architecture rather than reviving this branch.

## Namespace result

`dev/yeelam/f/BuildTest` can be retired. `dev/yeelam/main` is the curated authority for this namespace.
