//! Portable root settings-document owner for `CascadiaSettings` serialization.
//!
//! This type composes the shared [`SettingsDocument`] owner instead of creating
//! a second serializer. It validates the portable aggregate shapes that belong
//! to the settings root while retaining the complete typed JSON tree for
//! lossless round-trip projection.

use crate::{
    serialization::{SerializationError, SettingsDocument},
    settings_json::JsonValue,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CascadiaSettingsError {
    Serialization(SerializationError),
    InvalidProfilesShape,
    InvalidSchemesShape,
    InvalidActionsShape,
    InvalidKeybindingsShape,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CascadiaSettingsDocument {
    document: SettingsDocument,
}

impl CascadiaSettingsDocument {
    /// Parses a complete settings document through the shared serialization
    /// owner and validates portable aggregate members when they are present.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed JSON/root shape or for aggregate members
    /// whose JSON shape cannot represent Cascadia settings.
    pub fn from_json(input: &str) -> Result<Self, CascadiaSettingsError> {
        let document = SettingsDocument::from_json(input)
            .map_err(CascadiaSettingsError::Serialization)?;
        let root = document
            .to_json_value()
            .as_object()
            .expect("SettingsDocument guarantees an object root");

        if let Some(profiles) = root.get("profiles") {
            match profiles {
                JsonValue::Array(_) => {}
                JsonValue::Object(object)
                    if matches!(object.get("list"), Some(JsonValue::Array(_))) => {}
                _ => return Err(CascadiaSettingsError::InvalidProfilesShape),
            }
        }
        if !matches!(root.get("schemes"), None | Some(JsonValue::Array(_))) {
            return Err(CascadiaSettingsError::InvalidSchemesShape);
        }
        if !matches!(root.get("actions"), None | Some(JsonValue::Array(_))) {
            return Err(CascadiaSettingsError::InvalidActionsShape);
        }
        if !matches!(root.get("keybindings"), None | Some(JsonValue::Array(_))) {
            return Err(CascadiaSettingsError::InvalidKeybindingsShape);
        }

        Ok(Self { document })
    }

    #[must_use]
    pub const fn to_json_value(&self) -> &JsonValue {
        self.document.to_json_value()
    }
}
