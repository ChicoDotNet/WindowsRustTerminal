//! Portable serialization owner for Windows Terminal actions and keybindings.
//!
//! The action map keeps the shared typed JSON tree as its serialization source
//! of truth. R08 can therefore prove Microsoft's round-trip contract without
//! prematurely reimplementing command execution or generated-ID semantics.

use crate::settings_json::{self, JsonValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionMapError {
    InvalidJson,
    ExpectedActionMap,
    ExpectedActionsArray,
    ExpectedKeybindingsArray,
    ExpectedEntryObject,
    ExpectedCommand,
    ExpectedActionName,
    ExpectedNestedCommandsArray,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionMapDocument {
    root: JsonValue,
}

impl ActionMapDocument {
    /// Parses an ActionMap serialization vector while retaining all typed JSON
    /// values for structure-identical projection.
    ///
    /// Microsoft serializes ActionMap either as a bare action array or through
    /// GlobalAppSettings as an object containing `actions` and `keybindings`.
    /// Both forms are accepted here and validated recursively for the portable
    /// structural contract used by `SerializationTests::Actions`.
    ///
    /// # Errors
    ///
    /// Returns [`ActionMapError`] for malformed JSON or an invalid action-map
    /// shape.
    pub fn from_json(input: &str) -> Result<Self, ActionMapError> {
        let root = settings_json::parse(input).map_err(|_| ActionMapError::InvalidJson)?;
        validate_root(&root)?;
        Ok(Self { root })
    }

    #[must_use]
    pub const fn to_json_value(&self) -> &JsonValue {
        &self.root
    }
}

fn validate_root(root: &JsonValue) -> Result<(), ActionMapError> {
    match root {
        JsonValue::Array(actions) => validate_actions(actions),
        JsonValue::Object(object) => {
            let mut has_surface = false;
            if let Some(actions) = object.get("actions") {
                has_surface = true;
                let JsonValue::Array(actions) = actions else {
                    return Err(ActionMapError::ExpectedActionsArray);
                };
                validate_actions(actions)?;
            }
            if let Some(keybindings) = object.get("keybindings") {
                has_surface = true;
                let JsonValue::Array(keybindings) = keybindings else {
                    return Err(ActionMapError::ExpectedKeybindingsArray);
                };
                validate_keybindings(keybindings)?;
            }
            if has_surface {
                Ok(())
            } else {
                Err(ActionMapError::ExpectedActionMap)
            }
        }
        _ => Err(ActionMapError::ExpectedActionMap),
    }
}

fn validate_actions(actions: &[JsonValue]) -> Result<(), ActionMapError> {
    for action in actions {
        validate_action_entry(action)?;
    }
    Ok(())
}

fn validate_action_entry(action: &JsonValue) -> Result<(), ActionMapError> {
    let JsonValue::Object(action) = action else {
        return Err(ActionMapError::ExpectedEntryObject);
    };

    if let Some(commands) = action.get("commands") {
        let JsonValue::Array(commands) = commands else {
            return Err(ActionMapError::ExpectedNestedCommandsArray);
        };
        for command in commands {
            validate_action_entry(command)?;
        }
        return Ok(());
    }

    let Some(command) = action.get("command") else {
        return Err(ActionMapError::ExpectedCommand);
    };
    match command {
        JsonValue::String(_) => Ok(()),
        JsonValue::Object(command) => match command.get("action") {
            Some(JsonValue::String(_)) => Ok(()),
            _ => Err(ActionMapError::ExpectedActionName),
        },
        _ => Err(ActionMapError::ExpectedCommand),
    }
}

fn validate_keybindings(keybindings: &[JsonValue]) -> Result<(), ActionMapError> {
    for keybinding in keybindings {
        if !matches!(keybinding, JsonValue::Object(_)) {
            return Err(ActionMapError::ExpectedEntryObject);
        }
    }
    Ok(())
}
