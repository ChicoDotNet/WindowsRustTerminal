//! Portable typed settings-document mutation used by serialization contracts.
//!
//! This owner deliberately keeps the parsed JSON tree as the serialization
//! source of truth so unrelated settings survive a targeted mutation. That
//! mirrors `CascadiaSettings::ToJson` for the portable portion of the model
//! without reimplementing WinRT projection.

use crate::settings_json::{self, JsonObject, JsonValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    InvalidJson,
    ExpectedRootObject,
    ExpectedSchemesArray,
    ExpectedSchemeObject,
    SchemeNotFound,
    ExpectedProfilesArray,
    ExpectedProfileObject,
    ProfileNotFound,
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
        let root = self.root_object_mut()?;
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

    /// Sets an integer member on the indexed profile while preserving all
    /// unrelated serialized members.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when the profiles shape is invalid or the
    /// requested profile index is absent.
    pub fn set_profile_i32(
        &mut self,
        index: usize,
        member: &str,
        value: i32,
    ) -> Result<(), SerializationError> {
        self.profile_object_mut(index)?
            .insert(member.to_owned(), JsonValue::Number(f64::from(value)));
        Ok(())
    }

    /// Sets a string member on the indexed profile while preserving all
    /// unrelated serialized members.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when the profiles shape is invalid or the
    /// requested profile index is absent.
    pub fn set_profile_string(
        &mut self,
        index: usize,
        member: &str,
        value: &str,
    ) -> Result<(), SerializationError> {
        self.profile_object_mut(index)?.insert(
            member.to_owned(),
            JsonValue::String(value.to_owned()),
        );
        Ok(())
    }

    #[must_use]
    pub const fn to_json_value(&self) -> &JsonValue {
        &self.root
    }

    fn root_object_mut(&mut self) -> Result<&mut JsonObject, SerializationError> {
        match &mut self.root {
            JsonValue::Object(root) => Ok(root),
            _ => Err(SerializationError::ExpectedRootObject),
        }
    }

    fn profile_object_mut(&mut self, index: usize) -> Result<&mut JsonObject, SerializationError> {
        let root = self.root_object_mut()?;
        let profiles = root
            .get_mut("profiles")
            .ok_or(SerializationError::ExpectedProfilesArray)?;
        let profiles = match profiles {
            JsonValue::Array(profiles) => profiles,
            JsonValue::Object(profiles) => match profiles.get_mut("list") {
                Some(JsonValue::Array(profiles)) => profiles,
                _ => return Err(SerializationError::ExpectedProfilesArray),
            },
            _ => return Err(SerializationError::ExpectedProfilesArray),
        };
        let profile = profiles
            .get_mut(index)
            .ok_or(SerializationError::ProfileNotFound)?;
        match profile {
            JsonValue::Object(profile) => Ok(profile),
            _ => Err(SerializationError::ExpectedProfileObject),
        }
    }
}
