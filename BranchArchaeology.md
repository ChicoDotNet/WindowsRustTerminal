# Carlos Zamora branch archaeology

This ledger consolidates historical `dev/cazamor/**` work into `dev/cazamor/main`. Branch tip dates are not treated as authoritative when mechanical migrations obscure the original functional work.

## `dev/cazamor/spec/settings-ui-architecture-draft`

- Functional period: July–August 2020; later tip commits are mechanical spelling migrations.
- Historical artifact: `doc/specs/#885 - winrt Terminal Settings.md`, an early architecture draft for the Terminal Settings Model.
- Historical intent: move settings serialization/deserialization out of `TerminalApp`, expose settings as WinRT objects in a dedicated `Microsoft.Terminal.Settings.Model` layer, make the model consumable by TerminalApp/TerminalControl/Settings UI and future extensions, and support clone/save/layering workflows for a graphical settings editor.
- Historical maturity: design draft. It correctly identifies many of the architectural seams that later shipped, but contains provisional names/APIs (`AppSettings`, draft Clone/Save/LayerSettings flows) and future-consideration sections rather than a final contract.
- Definitive successors: `microsoft/terminal#6904` (`f0b8875770558dd0908022f8df9ec3d7d8be0817`, 2020-10-09) formalized the WinRT TerminalSettings specification. `microsoft/terminal#7667` (`2608e948224a3779be8ac55375543f4018db3081`, 2020-10-06) introduced the `TerminalSettingsModel` project, moved settings responsibilities out of TerminalApp, exposed WinRT settings objects under `Microsoft.Terminal.Settings.Model`, added the DLL/LIB testing architecture, and closed `#885`.
- Disposition: **NO-PORT / superseded**.
- Recovery value: provenance only. The durable insight—that the Settings UI and other consumers need a WinRT settings model separated from TerminalApp—is represented by the supported TSM architecture and its later evolution.
- Branch retirement: **SAFE after this ledger commit reaches `dev/cazamor/main`**.

## `dev/cazamor/adaptive-cards-prototype`

- Functional period: January 2021 (`f72cbfb926d82de264020f7c53475651cc54915d`, `Introduce AC prototype`); later tip commits are spelling maintenance.
- Historical intent: replace the plain hyperlink tooltip in `TermControl` with a rich preview supplied by an external service and rendered as an Adaptive Card. The prototype adds `AdaptiveCards.Rendering.Uwp`, injects the rendered card into the hyperlink tooltip, and sketches an HTTP-backed `_GetAdaptiveCardPreview` provider.
- Historical maturity: deliberately rough spike. The implementation contains `TODO CARLOS`, a hard-coded GitHub-card JSON response and token, commented-out HTTP code, synchronous `.get()` sketches, and no upstream PR. It is evidence of a product idea, not a safe implementation.
- Modern reading: current `main` contains no Adaptive Cards dependency or hyperlink-preview provider contract. Hyperlinks remain a maintained terminal concept, but this particular rich-preview/provider experiment was not absorbed. Repository issue search does not reveal a dedicated shipped Adaptive Cards hyperlink-preview feature; the only historical Adaptive Cards issue reference is in the separate notification/OSC777 discussion.
- Disposition: **NO-PORT / recovery candidate**.
- Recovery value: preserve the product idea, not the patch: a future hyperlink-hover experience could support pluggable/rich previews fetched asynchronously and rendered in a constrained UI surface. Any future implementation must be designed from the modern security/privacy/threading model and must not replay the hard-coded service/token or blocking HTTP sketch.
- Branch retirement: **SAFE after this ledger commit reaches `dev/cazamor/main`**.

## Deletion set

- `dev/cazamor/spec/settings-ui-architecture-draft`
- `dev/cazamor/adaptive-cards-prototype`
