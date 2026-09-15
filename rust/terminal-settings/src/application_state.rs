//! Portable workspace-persistence semantics from `SettingsModel` `ApplicationState`.
//!
//! File scheduling and elevation-specific path selection remain boundary work;
//! this owner captures the deterministic map mutations consumed by those I/O
//! seams and by startup workspace restoration.

use std::collections::BTreeMap;

/// Portable subset of the persisted window layout needed by the workspace map.
/// Additional layout fields are added as their `SettingsModel` contracts migrate.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WindowLayout {
    tab_layout: Vec<String>,
}

impl WindowLayout {
    #[must_use]
    pub fn with_tab_layout(tab_layout: Vec<String>) -> Self {
        Self { tab_layout }
    }

    #[must_use]
    pub fn tab_layout(&self) -> &[String] {
        &self.tab_layout
    }
}

/// Portable action selected for a persisted-workspace rename request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceRenamePlan {
    Noop,
    Remove,
    Rename,
}

/// Resolves Microsoft's workspace-rename rules from storage-independent facts.
#[must_use]
pub const fn workspace_rename_plan(
    old_name_empty: bool,
    names_equal: bool,
    old_exists: bool,
    new_name_empty: bool,
) -> WorkspaceRenamePlan {
    if old_name_empty || names_equal || !old_exists {
        WorkspaceRenamePlan::Noop
    } else if new_name_empty {
        WorkspaceRenamePlan::Remove
    } else {
        WorkspaceRenamePlan::Rename
    }
}

/// Canonical safe Rust owner for persisted workspace entries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApplicationState {
    persisted_workspaces: BTreeMap<String, WindowLayout>,
}

impl ApplicationState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces a persisted workspace under `name`.
    pub fn save_workspace(&mut self, name: impl Into<String>, layout: WindowLayout) {
        self.persisted_workspaces.insert(name.into(), layout);
    }

    /// Removes `name`, returning whether the map was modified.
    pub fn remove_workspace(&mut self, name: &str) -> bool {
        self.persisted_workspaces.remove(name).is_some()
    }

    /// Renames an entry with Microsoft's exact no-op/removal rules.
    ///
    /// Equal names and an empty old name are no-ops. An empty new name removes
    /// the old entry. A non-empty new name replaces any existing target entry.
    pub fn rename_workspace(&mut self, old_name: &str, new_name: &str) -> bool {
        match workspace_rename_plan(
            old_name.is_empty(),
            old_name == new_name,
            self.persisted_workspaces.contains_key(old_name),
            new_name.is_empty(),
        ) {
            WorkspaceRenamePlan::Noop => false,
            WorkspaceRenamePlan::Remove => self.persisted_workspaces.remove(old_name).is_some(),
            WorkspaceRenamePlan::Rename => {
                let Some(layout) = self.persisted_workspaces.remove(old_name) else {
                    return false;
                };
                self.persisted_workspaces
                    .insert(new_name.to_owned(), layout);
                true
            }
        }
    }

    /// Atomically removes and returns the requested workspace.
    pub fn take_workspace(&mut self, name: &str) -> Option<WindowLayout> {
        self.persisted_workspaces.remove(name)
    }

    #[must_use]
    pub const fn all_persisted_workspaces(&self) -> &BTreeMap<String, WindowLayout> {
        &self.persisted_workspaces
    }
}

#[cfg(test)]
mod tests {
    use super::{workspace_rename_plan, ApplicationState, WindowLayout, WorkspaceRenamePlan};

    #[test]
    fn workspace_rename_plan_replays_settings_model_contract() {
        assert_eq!(
            workspace_rename_plan(true, false, true, false),
            WorkspaceRenamePlan::Noop
        );
        assert_eq!(
            workspace_rename_plan(false, true, true, false),
            WorkspaceRenamePlan::Noop
        );
        assert_eq!(
            workspace_rename_plan(false, false, false, false),
            WorkspaceRenamePlan::Noop
        );
        assert_eq!(
            workspace_rename_plan(false, false, true, true),
            WorkspaceRenamePlan::Remove
        );
        assert_eq!(
            workspace_rename_plan(false, false, true, false),
            WorkspaceRenamePlan::Rename
        );
    }

    #[test]
    fn rename_workspace_uses_the_shared_plan() {
        let original = WindowLayout::with_tab_layout(vec!["original".to_owned()]);
        let replacement = WindowLayout::with_tab_layout(vec!["replacement".to_owned()]);
        let mut state = ApplicationState::new();
        state.save_workspace("old", original.clone());
        state.save_workspace("new", replacement);

        assert!(state.rename_workspace("old", "new"));
        assert_eq!(state.all_persisted_workspaces().get("new"), Some(&original));
        assert!(!state.all_persisted_workspaces().contains_key("old"));

        assert!(state.rename_workspace("new", ""));
        assert!(state.all_persisted_workspaces().is_empty());
        assert!(!state.rename_workspace("", "ignored"));
        assert!(!state.rename_workspace("missing", "target"));
    }
}
