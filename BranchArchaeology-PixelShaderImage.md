# Pixel shader image branch archaeology

This ledger preserves the intent and disposition of `dev/migrie/14073-on-main`.

## `dev/migrie/14073-on-main`

### Functional history

This branch is an integration/support line around external PR **microsoft/terminal#14073 — Added experimental.pixelShaderImagePath**. Relative to modern `main` it appears five commits ahead, but its semantic history is a 2024 replay of a much older contribution onto then-current `main`, not five independent product features.

The imported contribution includes commit `ee9cc96c9e1778faaa633970723a071f2aaa07b7` (2022-10-23), **Added support for loading pixes shader texture to Atlas**. Its contract is to let a custom Atlas pixel shader consume an additional image texture configured through `experimental.pixelShaderImagePath`, plumbing that setting through appearance/settings/control into the renderer and binding the image to the shader. The historical PR documents exploratory validation for unset/set/changed/invalid paths and a shader consuming the custom texture.

Mike's 2024 integration line merged then-current `origin/main` into the pull-request work (`aa3df882a9a86cad528951ad3dedff044e635dbf`) and contains follow-up integration fixes such as `e2ff838ec315aef6db04b3675992dc6b039ed0b6` (`yes this is actually right`). These commits are branch-maintenance/reconciliation around the PR rather than a distinct durable contract.

### Authoritative destination

PR #14073 was ultimately merged upstream on **2024-03-08** as merge commit `0ba680ad532391443cb2bb54686751b788cf3a18` for the Terminal v1.21 line.

The current upstream implementation still exposes `PixelShaderImagePath` through `IControlAppearance` and `IAppearanceConfig`, carries it into `ControlCore`, stores `customPixelShaderImagePath` in the Atlas renderer state, loads/binds the texture in `BackendD3D`, and retains the `samples/PixelShaders/BackgroundImage.hlsl` demonstration. The current `ChicoDotNet/WindowsRustTerminal` `main` likewise contains `String PixelShaderImagePath { get; };` in `src/cascadia/TerminalControl/IControlAppearance.idl`.

Thus the useful behavior represented by this branch is not stranded: it is present in the reviewed modern implementation. Repository code search on the fork did not initially surface the symbol, but direct file inspection confirms it is present; that search-index miss is not evidence of missing functionality.

### Disposition

**ALREADY ABSORBED** by merged PR #14073 and its later evolved implementation.

Do not transplant the historical DirectXTK/WIC integration branch or its reconciliation commits. If this behavior regresses, replay the contract against the modern Atlas renderer: configured image media resolves successfully, changes are picked up, invalid paths fail safely, and a custom shader receives the configured image through the current texture-binding path.

### Branch retirement

**SAFE TO DELETE: `dev/migrie/14073-on-main`**.
