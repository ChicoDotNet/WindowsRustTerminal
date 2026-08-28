//! Portable SettingsModel fragment loading owner.
//!
//! Fragments may update generated/builtin profiles and contribute actions, but
//! fragment key bindings are deliberately ignored and fragment actions are not
//! persisted into the user's settings document. Iterable nested commands are
//! expanded against the current color-scheme collection before materialization.

use std::collections::BTreeMap;

use crate::settings_json::{self, JsonObject, JsonValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FragmentError {
    InvalidJson,
    ExpectedRootObject,
    ExpectedProfilesArray,
    ExpectedProfileObject,
    ExpectedActionsArray,
    ExpectedActionObject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FragmentAction {
    nested_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FragmentSettings {
    profiles: BTreeMap<String, JsonObject>,
    profile_order: Vec<String>,
    schemes: Vec<String>,
    actions: BTreeMap<String, FragmentAction>,
    duplicate_profile: bool,
}

impl FragmentSettings {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn seed_profile(&mut self, guid: &str, name: &str) {
        let mut profile = JsonObject::new();
        profile.insert("guid".into(), JsonValue::String(guid.into()));
        profile.insert("name".into(), JsonValue::String(name.into()));
        if !self.profiles.contains_key(guid) {
            self.profile_order.push(guid.into());
        }
        self.profiles.insert(guid.into(), profile);
    }

    pub fn add_scheme(&mut self, name: &str) {
        if !self.schemes.iter().any(|scheme| scheme == name) {
            self.schemes.push(name.into());
        }
    }

    /// Merges one fragment document into the portable aggregate.
    ///
    /// # Errors
    /// Returns [`FragmentError`] for malformed fragment container shapes.
    pub fn merge_fragment(&mut self, _source: &str, input: &str) -> Result<(), FragmentError> {
        let root = settings_json::parse(input).map_err(|_| FragmentError::InvalidJson)?;
        let JsonValue::Object(root) = root else {
            return Err(FragmentError::ExpectedRootObject);
        };

        if let Some(profiles) = root.get("profiles") {
            let JsonValue::Array(profiles) = profiles else {
                return Err(FragmentError::ExpectedProfilesArray);
            };
            for profile in profiles {
                let JsonValue::Object(profile) = profile else {
                    return Err(FragmentError::ExpectedProfileObject);
                };
                self.layer_profile(profile);
            }
        }

        if let Some(actions) = root.get("actions") {
            let JsonValue::Array(actions) = actions else {
                return Err(FragmentError::ExpectedActionsArray);
            };
            for action in actions {
                let JsonValue::Object(action) = action else {
                    return Err(FragmentError::ExpectedActionObject);
                };
                self.layer_fragment_action(action);
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn profile_count(&self) -> usize {
        self.profile_order.len()
    }

    #[must_use]
    pub fn duplicate_profile(&self) -> bool {
        self.duplicate_profile
    }

    #[must_use]
    pub fn profile_name(&self, guid: &str) -> Option<&str> {
        self.profiles
            .get(guid)?
            .get("name")
            .and_then(JsonValue::as_str)
    }

    #[must_use]
    pub fn action_name_exists(&self, name: &str) -> bool {
        self.actions.contains_key(name)
    }

    /// Fragment action `keys` are ignored by SettingsModel.
    #[must_use]
    pub const fn key_is_bound(&self, _key: &str) -> bool {
        false
    }

    #[must_use]
    pub fn nested_command_count(&self, name: &str) -> Option<usize> {
        self.actions.get(name).map(|action| action.nested_count)
    }

    /// Fragment-contributed actions are runtime additions and never become
    /// serialized user actions.
    #[must_use]
    pub fn persists_fragment_action(&self, name: &str) -> bool {
        !self.actions.contains_key(name)
    }

    fn layer_profile(&mut self, profile: &JsonObject) {
        if let Some(updates) = profile.get("updates").and_then(JsonValue::as_str) {
            if let Some(existing) = self.profiles.get_mut(updates) {
                for (key, value) in profile {
                    if key != "updates" {
                        existing.insert(key.clone(), value.clone());
                    }
                }
            }
            return;
        }

        let Some(guid) = profile.get("guid").and_then(JsonValue::as_str) else {
            return;
        };
        if self.profiles.contains_key(guid) {
            self.duplicate_profile = true;
            return;
        }
        self.profile_order.push(guid.into());
        self.profiles.insert(guid.into(), profile.clone());
    }

    fn layer_fragment_action(&mut self, action: &JsonObject) {
        let Some(name) = action.get("name").and_then(JsonValue::as_str) else {
            // Nested commands cannot synthesize a stable parent name.
            return;
        };

        if action.get("command").is_some() {
            self.actions
                .insert(name.into(), FragmentAction { nested_count: 0 });
            return;
        }

        let Some(JsonValue::Array(commands)) = action.get("commands") else {
            return;
        };
        let nested_count = commands
            .iter()
            .map(|command| match command {
                JsonValue::Object(command)
                    if command.get("iterateOn").and_then(JsonValue::as_str) == Some("schemes") =>
                {
                    self.schemes.len()
                }
                JsonValue::Object(command)
                    if command.get("name").and_then(JsonValue::as_str).is_some()
                        && (command.get("command").is_some() || command.get("commands").is_some()) =>
                {
                    1
                }
                _ => 0,
            })
            .sum();
        self.actions.insert(name.into(), FragmentAction { nested_count });
    }
}
