//! Portable typed settings-document mutation used by serialization contracts.
//!
//! This owner deliberately keeps the parsed JSON tree as the serialization
//! source of truth so unrelated settings survive a targeted mutation. That
//! mirrors `CascadiaSettings::ToJson` for the portable portion of the model
//! without reimplementing WinRT projection.

use crate::settings_json::{self, JsonValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    InvalidJson,
    ExpectedRootObject,
    ExpectedSchemesArray,
    ExpectedSchemeObject,
    SchemeNotFound,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsDocument {
    root: JsonValue,
}

impl SettingsDocument {
    /// Parses one settings JSON/JSONC document while retaining all typed values.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when the document is malformed or does
    /// not have the settings root-object shape.
    pub fn from_json(input: &str) -> Result<Self, SerializationError> {
        let root = settings_json::parse(input).map_err(|_| SerializationError::InvalidJson)?;
        if root.as_object().is_none() {
            return Err(SerializationError::ExpectedRootObject);
        }
        Ok(Self { root })
    }

    /// Changes the foreground of one named user color scheme in-place.
    /// Unrelated members remain exactly represented by the shared typed tree.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] for an invalid schemes shape or if the
    /// requested scheme is absent.
    pub fn set_color_scheme_foreground(
        &mut self,
        name: &str,
        foreground: &str,
    ) -> Result<(), SerializationError> {
        let root = match &mut self.root {
            JsonValue::Object(root) => root,
            _ => return Err(SerializationError::ExpectedRootObject),
        };
        let schemes = root
            .get_mut("schemes")
            .ok_or(SerializationError::ExpectedSchemesArray)?;
        let schemes = match schemes {
            JsonValue::Array(schemes) => schemes,
            _ => return Err(SerializationError::ExpectedSchemesArray),
        };

        for scheme in schemes {
            let scheme = match scheme {
                JsonValue::Object(scheme) => scheme,
                _ => return Err(SerializationError::ExpectedSchemeObject),
            };
            if scheme.get("name").and_then(JsonValue::as_str) == Some(name) {
                scheme.insert(
                    "foreground".to_owned(),
                    JsonValue::String(foreground.to_owned()),
                );
                return Ok(());
            }
        }

        Err(SerializationError::SchemeNotFound)
    }

    #[must_use]
    pub const fn to_json_value(&self) -> &JsonValue {
        &self.root
    }
}
