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

## `dev/cazamor/sui/proto/profile-nav-view`

- Functional period: February–March 2021. `41c6c149ef6241844dff301b14b52e631089bf6f` introduces the prototype; `3566194502e694de89fada6740cc73ede8be1db4` only adds transparency; later tip commits are spelling maintenance.
- Historical intent: replace the profile editor's `Pivot` tabs (`General`, `Appearance`, `Advanced`) with an inner top-mode `NavigationView`, manually switching three content regions by visibility.
- Historical maturity: UI experiment. It comments out the existing pivot/navigation-state code and uses string tags plus manual `Visibility` toggles rather than establishing a durable navigation/view-model contract.
- Modern reading: this inner profile `NavigationView` model was not adopted. The maintained Settings UI now presents profile/default/color-scheme navigation through Settings cards/expanders and page-level navigation, while profile editing has continued through later settings-container/rejuvenation architecture. The 2021 branch therefore represents an explored UX direction rather than product behavior that needs replay.
- Disposition: **NO-PORT / UX prototype not adopted**.
- Recovery value: provenance only; no unique behavioral contract or implementation should be carried forward.
- Branch retirement: **SAFE after this ledger commit reaches `dev/cazamor/main`**.

## `dev/cazamor/spec/tsm-actions-temp`

- Functional period: March 2021. `3905c49ca764cebced478ef310aed9f78aa842f2` introduced `Actions Addendum.md`; `230cfe785b6cc4a6cd001f3d694da07206d7d6b5` incorporated spec-review feedback. The branch tip was later polluted by spelling migrations.
- Historical intent: redesign action storage so commands and keybindings are represented by one model, make serialization/deserialization authoritative in the settings model, introduce `ActionMap` for queries/collision handling, and prepare the Settings UI to edit actions. The draft explicitly called out future Action IDs (`#6899`).
- Definitive lineage: the local draft became official spec PR `microsoft/terminal#9428` / commit `5713cd2148b1b6e471d42e66065e511effad439d`. PR `#9621` / `22fd06e19b3e564d448d60ebf34ffd63764f8080` then introduced `ActionMap`, removed `KeyMapping`, unified action deserialization, and used action IDs internally. PR `#9926` / `ff8fdbd2431f1cfd8211833815be481dfdec4420` added action serialization; PR `#9949` / `c66910b685a8fd404afb8f7c09a230d803f7cd19` connected `ActionMap` to an editable Actions Settings UI. In 2024, PR `#17162` / `ece0c04c38b6f820476fd480a96a8b103a5ca7f2` refactored ActionMap/Command around stable ActionIDs, completing the major future consideration anticipated by the draft.
- Modern reading: the durable architecture proposed here is represented by the maintained ActionMap/ActionID stack and its later evolution; the historical draft does not contain a missing contract that should be replayed independently.
- Disposition: **ALREADY ABSORBED / provenance-only**.
- Recovery value: preserve the design lineage—especially the early recognition that serialization, keybindings, command-palette commands and future stable IDs belong to one action model—not the temporary draft branch.
- Branch retirement: **SAFE after this ledger commit reaches `dev/cazamor/main`**.

## Deletion set

- `dev/cazamor/spec/settings-ui-architecture-draft`
- `dev/cazamor/adaptive-cards-prototype`
- `dev/cazamor/sui/proto/profile-nav-view`
- `dev/cazamor/spec/tsm-actions-temp`
