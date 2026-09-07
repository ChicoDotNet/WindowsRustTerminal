# FHL 2021 — shader experiment archaeology

## Branches

- `dev/migrie/fhl-2021/more-shader-variables` — tip `1dc2c4172ef71a0cbae0921642d80cdbc8eb3937` (2021-07-27), 9 commits ahead of its historical base.
- `dev/migrie/fhl-2021/differential-pixel-shading` — tip `0597cda21175fe7ce268c0cd49fdd5aa3ea46fd7` (2021-07-28), 28 commits ahead of the same historical line and explicitly branched from `more-shader-variables`.

**Disposition: NO-PORT / superseded by the Atlas renderer implementation.**

These branches are useful engineering archaeology, but they are not a product patch to replay onto the current tree.

## What the experiments learned

`more-shader-variables` explored the contract between the terminal renderer and user-provided pixel shaders. The branch experimented with:

- extending shader inputs beyond time/scale/resolution, including cursor/glyph-related state;
- computing shader settings once per rendered frame rather than from several unrelated invalidation paths;
- reloading the configured shader when shader effects are re-enabled;
- richer sample HLSL, including a rainbow/cursor-oriented effect.

`differential-pixel-shading` then pursued issue microsoft/terminal#7147, **Support renderer fast presents**. Its goal was to stop treating full-screen shader effects as a reason to abandon efficient presentation. The prototype evolved through backing-texture / differential-presentation experiments and eventually reached a working state; the issue discussion records that the branch worked well and was intended to be untied from its experimental ancestor before shipping.

The important architectural constraint recovered from that discussion is that the normal high-performance path must preserve partial-present information (`Present1`) where useful (including Remote Desktop scenarios), while shader-heavy local rendering may legitimately take a different presentation path.

## Why the code itself should not be resurrected

Both branches modify the old `src/renderer/dx/DxRenderer.*` implementation. The current repository no longer has a `src/renderer/dx` directory; the rendering architecture has moved on to Atlas.

More importantly, microsoft/terminal#7147 was closed as completed after merged PR microsoft/terminal#13885, **AtlasEngine: Implement support for custom shaders**. That reviewed implementation brought `experimental.retroTerminalEffect` and `experimental.pixelShaderPath` to AtlasEngine. The issue closure explicitly notes that Atlas can simply redraw the full screen every frame with the shader enabled at high refresh rates, making the old differential-DxRenderer prototype unnecessary.

Therefore the current source of truth is the Atlas renderer plus the reviewed shader implementation and its tests, not either FHL-2021 WIP branch.

## Recovery guidance

If shader behavior regresses, recover the *contracts*, not these implementations:

1. custom pixel shaders and the retro effect must continue to work in the current renderer;
2. shader state supplied to HLSL is an externally visible compatibility surface and should be changed deliberately;
3. the non-shader rendering path should preserve efficient/partial presentation semantics where the current architecture supports them;
4. performance claims should be validated against the current renderer in optimized builds rather than inferred from these 2021 prototypes.

For implementation history use microsoft/terminal#7147 and merged PR microsoft/terminal#13885. Do not resurrect `DxRenderer` or cherry-pick these branches into the current tree.

## Retirement status

Both legacy refs are **safe to delete after this archaeology commit is present on `dev/migrie/main`**. Their unique implementation is obsolete, their intent is preserved here, and the shipped successor is identifiable in upstream history.