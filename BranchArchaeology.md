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

## Deletion set

- `dev/cazamor/spec/settings-ui-architecture-draft`
