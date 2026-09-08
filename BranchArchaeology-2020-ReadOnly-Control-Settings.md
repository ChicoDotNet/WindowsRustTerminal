# 2020 Read-only Core/Control settings experiment

## Historical source

- Branch: `dev/migrie/f/settings-getters-only`
- Feature commit: `ffa27cda7e10015579f44512186f27011721ab79`
- Parent: `1c6aa4d109de63e8d660b7ab65044346bd0b3e4e` (`#7167`, which moved `ICoreSettings` / `IControlSettings` into TerminalCore / TerminalControl)
- Commit message: **“Force everything to be getters only. Doesn't work yet, but it's close”**

The branch later accumulated spelling-sync commits; those are incidental. The knowledge-bearing change is the feature commit above.

## What the experiment changed

The experiment changed the projected WinRT settings interfaces from mutable properties to read-only consumer contracts.

Conceptually:

```text
before: settings interface = get + set surface
trial:  control/core interface = getters only
```

Examples included:

- `HistorySize`, `InitialRows`, `InitialCols`
- cursor settings
- title/application-title behavior
- font and padding settings
- acrylic/rendering settings
- command line / starting directory
- background-image settings
- selection, antialiasing and control behavior

The experiment changed the interface declaration, not the underlying settings ownership model. The intent was architectural: **TerminalCore and TerminalControl should consume settings, not own mutation of settings.**

## Why it matters

Separating mutation from consumption creates a useful boundary:

```text
settings model / builder / loader
        |
        | produces a resolved snapshot/view
        v
read-only ICoreSettings / IControlSettings
        |
        v
TerminalCore / TerminalControl
```

That prevents low-level terminal runtime code from casually mutating configuration that is actually owned by a higher-level settings model.

It also makes several design properties clearer:

- configuration flows downward;
- mutation happens at an explicit owner;
- runtime consumers can be tested against a stable settings contract;
- settings reload can replace/reapply a snapshot rather than expose arbitrary setters everywhere;
- the control/core boundary becomes easier to port because it resembles a data interface rather than a shared mutable object.

## Historical outcome

The 2020 commit explicitly says the experiment did not yet work end-to-end. It should therefore **not** be treated as a patch to resurrect verbatim.

However, the architectural direction survived. Current `main` exposes `ICoreSettings` and `IControlSettings` overwhelmingly as `{ get; }` properties. In other words, the experiment captured a design direction that later became normal architecture even though this exact branch was unfinished.

## Rust migration guidance

For Windows RusTerminal, preserve the principle rather than the old MIDL mechanics.

Prefer something equivalent to:

```rust
pub trait CoreSettings {
    fn history_size(&self) -> i32;
    fn initial_rows(&self) -> i32;
    fn initial_cols(&self) -> i32;
    // ...
}
```

or immutable/resolved settings structs passed by shared reference.

Avoid placing mutation methods on the runtime-facing settings trait merely because the source settings model is mutable elsewhere.

Useful rule:

> **Mutable at the configuration boundary; read-only at the rendering/control boundary.**

If runtime behavior genuinely needs to change a setting, route that request back to the settings owner rather than mutating the consumed snapshot in place.

## Branch disposition

The knowledge-bearing experiment is fully preserved here, and the surviving design direction is visible in current `main`.

`dev/migrie/f/settings-getters-only` is therefore **safe to retire for knowledge-preservation purposes**.
