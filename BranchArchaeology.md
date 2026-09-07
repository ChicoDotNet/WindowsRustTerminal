# Pankaj branch archaeology

## `dev/pabhoj/sui_action_overhaul`

**Disposition: ALREADY ABSORBED / superseded by reviewed Actions editor work.**

This large prototype is the early Settings UI action editor line exercised in Carlos's 1.24 selfhosts. Its functional surface is the same family later split into reviewed PRs:

- #18915 — ActionArgs reflection/settings-model support, merged 2025-09-03.
- #18917 — full action editing in Settings UI, merged 2025-12-09.

The production PRs contain the same core contracts: enumerate commands, reflect ActionArgs, add/edit/delete actions and keybindings, and render type-specific argument editors. They are the canonical implementation; the 67-commit prototype no longer carries unique product knowledge.

## `dev/pabhoj/sui_follow_ups`

**Disposition: NO-PORT / mechanical-only tip.**

Compared with current `main`, this ref has one exclusive commit: Dustin's `Migrate spelling-0.0.21 changes from main` (2022-07-29). Its diff is entirely spellbot configuration. No Pankaj product implementation remains uniquely on this ref.

## Recovery guidance

For Actions Settings UI history, follow #18915 and #18917 rather than resurrecting `sui_action_overhaul`. Preserve this `dev/pabhoj/main` lane for any additional Pankaj archaeology.
