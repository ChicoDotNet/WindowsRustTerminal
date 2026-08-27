//! Portable typed settings-document mutation used by serialization contracts.
//!
//! This owner deliberately keeps the parsed JSON tree as the serialization
//! source of truth so unrelated settings survive a targeted mutation. That
//! mirrors `CascadiaSettings::ToJson` for the portable portion of the model
//! without reimplementing WinRT projection.

use crate::{
    profile::Profile,
    settings_json::{self, JsonObject, JsonValue},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    InvalidJson,
    ExpectedRootObject,
    InvalidProfile,
    ExpectedSchemesArray,
    ExpectedSchemeObject,
    SchemeNotFound,
    ExpectedProfilesArray,
    ExpectedProfilesDefaultsObject,
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

    /// Parses one profile serialization vector through the safe Rust `Profile`
    /// owner while retaining the complete typed tree for lossless projection.
    ///
    /// The profile owner validates the migrated semantic surface (GUID,
    /// inheritance-backed values, nullable colors/icon, directory and
    /// environment), while the shared JSON tree preserves settings that have
    /// not yet moved into that owner. Legacy top-level font aliases are
    /// canonicalized into the modern `font` object, matching Profile::ToJson.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError::InvalidProfile`] when the migrated profile
    /// semantics reject the vector, or another serialization error when the
    /// JSON itself cannot be retained as a root object.
    pub fn from_profile_json(input: &str) -> Result<Self, SerializationError> {
        Profile::from_json(input).map_err(|_| SerializationError::InvalidProfile)?;
        let mut document = Self::from_json(input)?;
        document.canonicalize_legacy_profile_font()?;
        Ok(document)
    }

    /// Canonicalizes the legacy root `profiles: []` shape to the modern
    /// `profiles: { "list": [] }` shape used by CascadiaSettings serialization.
    /// Existing modern profile objects are preserved unchanged. When legacy
    /// `compatibility.reloadEnvironmentVariables` is present at the root, it is
    /// moved into `profiles.defaults`, matching the SettingsLoader fixup.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when the settings root is not an object,
    /// when `profiles` is present with a shape other than an array/object, or
    /// when an existing `profiles.defaults` value is not an object.
    pub fn canonicalize_legacy_profiles(&mut self) -> Result<(), SerializationError> {
        let root = self.root_object_mut()?;
        if let Some(profiles) = root.remove("profiles") {
            match profiles {
                JsonValue::Array(list) => {
                    let mut modern = JsonObject::new();
                    modern.insert("list".to_owned(), JsonValue::Array(list));
                    root.insert("profiles".to_owned(), JsonValue::Object(modern));
                }
                JsonValue::Object(object) => {
                    root.insert("profiles".to_owned(), JsonValue::Object(object));
                }
                other => {
                    root.insert("profiles".to_owned(), other);
                    return Err(SerializationError::ExpectedProfilesArray);
                }
            }
        }

        let Some(reload_environment_variables) =
            root.remove("compatibility.reloadEnvironmentVariables")
        else {
            return Ok(());
        };

        let Some(JsonValue::Object(profiles)) = root.get_mut("profiles") else {
            root.insert(
                "compatibility.reloadEnvironmentVariables".to_owned(),
                reload_environment_variables,
            );
            return Ok(());
        };

        let defaults = profiles
            .entry("defaults".to_owned())
            .or_insert_with(|| JsonValue::Object(JsonObject::new()));
        let JsonValue::Object(defaults) = defaults else {
            return Err(SerializationError::ExpectedProfilesDefaultsObject);
        };
        defaults
            .entry("compatibility.reloadEnvironmentVariables".to_owned())
            .or_insert(reload_environment_variables);
        Ok(())
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

    /// Sets an integer member on the settings root while preserving all
    /// unrelated serialized members.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when the settings root is not an object.
    pub fn set_global_i32(
        &mut self,
        member: &str,
        value: i32,
    ) -> Result<(), SerializationError> {
        self.root_object_mut()?
            .insert(member.to_owned(), JsonValue::Number(f64::from(value)));
        Ok(())
    }

    /// Sets a boolean member on the settings root while preserving all
    /// unrelated serialized members.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when the settings root is not an object.
    pub fn set_global_bool(
        &mut self,
        member: &str,
        value: bool,
    ) -> Result<(), SerializationError> {
        self.root_object_mut()?
            .insert(member.to_owned(), JsonValue::Bool(value));
        Ok(())
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

    fn canonicalize_legacy_profile_font(&mut self) -> Result<(), SerializationError> {
        let root = self.root_object_mut()?;
        let face = root.remove("fontFace");
        let size = root.remove("fontSize");
        let weight = root.remove("fontWeight");

        if face.is_none() && size.is_none() && weight.is_none() {
            return Ok(());
        }

        let font = root
            .entry("font".to_owned())
            .or_insert_with(|| JsonValue::Object(JsonObject::new()));
        let JsonValue::Object(font) = font else {
            return Err(SerializationError::InvalidProfile);
        };

        if let Some(value) = face {
            font.entry("face".to_owned()).or_insert(value);
        }
        if let Some(value) = size {
            font.entry("size".to_owned()).or_insert(value);
        }
        if let Some(value) = weight {
            font.entry("weight".to_owned()).or_insert(value);
        }
        Ok(())
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
