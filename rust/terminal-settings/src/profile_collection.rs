//! Portable profile-collection layering semantics from `SettingsModel`.
//!
//! This owner keeps legacy top-level `profiles` arrays as full JSON objects,
//! layers user objects over inbox objects by strict profile GUID identity, and
//! preserves source order for profiles that are not replaced.

use crate::{
    profile::{ProfileGuid, ProfileParseError},
    settings_json::{self, JsonMember, JsonObject, JsonValue},
};

/// One layered profile object together with the identity fields needed by the
/// settings loader to reconcile inbox and user entries.
#[derive(Debug, Clone, PartialEq)]
pub struct LayeredProfile {
    object: JsonObject,
    name: Option<String>,
    guid: Option<ProfileGuid>,
}

impl LayeredProfile {
    fn from_object(object: JsonObject) -> Result<Self, ProfileParseError> {
        let mut profile = Self {
            object,
            name: None,
            guid: None,
        };
        profile.refresh_identity()?;
        Ok(profile)
    }

    fn layer_object(&mut self, overlay: JsonObject) -> Result<(), ProfileParseError> {
        for (key, value) in overlay {
            self.object.insert(key, value);
        }
        self.refresh_identity()
    }

    fn refresh_identity(&mut self) -> Result<(), ProfileParseError> {
        self.name = match JsonMember::from_object(&self.object, "name") {
            JsonMember::Missing | JsonMember::Null => None,
            JsonMember::Value(JsonValue::String(value)) => Some(value.clone()),
            JsonMember::Value(_) => return Err(ProfileParseError::InvalidString),
        };
        self.guid = match JsonMember::from_object(&self.object, "guid") {
            JsonMember::Missing | JsonMember::Null => None,
            JsonMember::Value(JsonValue::String(value)) => Some(ProfileGuid::parse(value)?),
            JsonMember::Value(_) => return Err(ProfileParseError::InvalidGuid),
        };
        Ok(())
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[must_use]
    pub const fn guid(&self) -> Option<ProfileGuid> {
        self.guid
    }

    /// Returns the fully layered JSON object, including properties that are not
    /// yet projected into the portable `Profile` owner.
    #[must_use]
    pub const fn object(&self) -> &JsonObject {
        &self.object
    }
}

/// Safe Rust owner for legacy inbox/user profile-array reconciliation.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProfileCollection {
    profiles: Vec<LayeredProfile>,
}

impl ProfileCollection {
    /// Layers user `profiles` entries over inbox entries with the same GUID.
    /// Matching user entries retain the inbox position, unmatched inbox entries
    /// remain present, and unmatched user entries append in user order.
    ///
    /// # Errors
    ///
    /// Returns [`ProfileParseError`] when either settings document is malformed,
    /// `profiles` is not an array, a profile is not an object, or an identity
    /// field has an invalid type/value.
    pub fn from_layered_legacy_arrays(
        user_json: &str,
        inbox_json: &str,
    ) -> Result<Self, ProfileParseError> {
        let mut profiles = parse_profile_objects(inbox_json)?
            .into_iter()
            .map(LayeredProfile::from_object)
            .collect::<Result<Vec<_>, _>>()?;

        for object in parse_profile_objects(user_json)? {
            let incoming = LayeredProfile::from_object(object.clone())?;
            let matching_index = incoming.guid().and_then(|guid| {
                profiles
                    .iter()
                    .position(|profile| profile.guid() == Some(guid))
            });

            if let Some(index) = matching_index {
                profiles[index].layer_object(object)?;
            } else {
                profiles.push(incoming);
            }
        }

        Ok(Self { profiles })
    }

    #[must_use]
    pub fn profiles(&self) -> &[LayeredProfile] {
        &self.profiles
    }
}

fn parse_profile_objects(input: &str) -> Result<Vec<JsonObject>, ProfileParseError> {
    let value = settings_json::parse(input).map_err(|_| ProfileParseError::InvalidJson)?;
    let root = value
        .as_object()
        .ok_or(ProfileParseError::ExpectedObject)?;
    let values = match JsonMember::from_object(root, "profiles") {
        JsonMember::Missing | JsonMember::Null => return Ok(Vec::new()),
        JsonMember::Value(JsonValue::Array(values)) => values,
        JsonMember::Value(_) => return Err(ProfileParseError::ExpectedArray),
    };

    values
        .iter()
        .map(|value| {
            value
                .as_object()
                .cloned()
                .ok_or(ProfileParseError::ExpectedObject)
        })
        .collect()
}
