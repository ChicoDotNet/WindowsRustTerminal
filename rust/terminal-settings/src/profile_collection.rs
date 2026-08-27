//! Portable profile-collection layering and fixup semantics from `SettingsModel`.
//!
//! This owner keeps legacy top-level `profiles` arrays as full JSON objects,
//! layers user objects over inbox objects by strict profile GUID identity,
//! preserves source order for profiles that are not replaced, and owns the
//! deterministic legacy cmd/PowerShell commandline fixups applied by the
//! settings loader.

use crate::{
    profile::{ProfileGuid, ProfileParseError},
    settings_json::{self, JsonMember, JsonObject, JsonValue},
};

const DEFAULT_WINDOWS_POWERSHELL_GUID: &str = "{61c54bbd-c2c6-5271-96e7-009a87ff44bf}";
const DEFAULT_COMMAND_PROMPT_GUID: &str = "{0caa0dad-35be-5f56-a8ff-afceeeaa6101}";
const LEGACY_POWERSHELL_COMMANDLINE: &str = "powershell.exe";
const CANONICAL_POWERSHELL_COMMANDLINE: &str =
    "%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe";
const LEGACY_COMMAND_PROMPT_COMMANDLINE: &str = "cmd.exe";
const CANONICAL_COMMAND_PROMPT_COMMANDLINE: &str = "%SystemRoot%\\System32\\cmd.exe";

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

    fn apply_legacy_shell_commandline_fixup(
        &mut self,
        powershell_guid: ProfileGuid,
        command_prompt_guid: ProfileGuid,
    ) {
        let Some(guid) = self.guid else {
            return;
        };
        let commandline = match JsonMember::from_object(&self.object, "commandline") {
            JsonMember::Value(JsonValue::String(value)) => value.as_str(),
            _ => return,
        };

        let replacement = if guid == powershell_guid
            && commandline.eq_ignore_ascii_case(LEGACY_POWERSHELL_COMMANDLINE)
        {
            Some(CANONICAL_POWERSHELL_COMMANDLINE)
        } else if guid == command_prompt_guid
            && commandline.eq_ignore_ascii_case(LEGACY_COMMAND_PROMPT_COMMANDLINE)
        {
            Some(CANONICAL_COMMAND_PROMPT_COMMANDLINE)
        } else {
            None
        };

        if let Some(replacement) = replacement {
            self.object.insert(
                "commandline".to_owned(),
                JsonValue::String(replacement.to_owned()),
            );
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[must_use]
    pub const fn guid(&self) -> Option<ProfileGuid> {
        self.guid
    }

    #[must_use]
    pub fn commandline(&self) -> Option<&str> {
        match JsonMember::from_object(&self.object, "commandline") {
            JsonMember::Value(JsonValue::String(value)) => Some(value.as_str()),
            _ => None,
        }
    }

    /// Returns the fully layered JSON object, including properties that are not
    /// yet projected into the portable `Profile` owner.
    #[must_use]
    pub const fn object(&self) -> &JsonObject {
        &self.object
    }
}

/// Safe Rust owner for profile collection reconciliation and deterministic
/// profile fixups.
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
        let mut profiles = parse_legacy_profile_objects(inbox_json)?
            .into_iter()
            .map(LayeredProfile::from_object)
            .collect::<Result<Vec<_>, _>>()?;

        for object in parse_legacy_profile_objects(user_json)? {
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

    /// Parses a modern `profiles.list` user layer and applies the deterministic
    /// commandline compatibility patches used by Microsoft's `FixupUserSettings`.
    /// Only the canonical Windows PowerShell and Command Prompt GUIDs are
    /// eligible, and the old executable names are matched ASCII-case-insensitively.
    ///
    /// # Errors
    ///
    /// Returns [`ProfileParseError`] when the settings/profile structure or a
    /// profile identity is invalid.
    pub fn from_user_json_with_legacy_shell_path_fixups(
        user_json: &str,
    ) -> Result<Self, ProfileParseError> {
        let powershell_guid = ProfileGuid::parse(DEFAULT_WINDOWS_POWERSHELL_GUID)?;
        let command_prompt_guid = ProfileGuid::parse(DEFAULT_COMMAND_PROMPT_GUID)?;
        let mut profiles = parse_modern_profile_objects(user_json)?
            .into_iter()
            .map(LayeredProfile::from_object)
            .collect::<Result<Vec<_>, _>>()?;

        for profile in &mut profiles {
            profile.apply_legacy_shell_commandline_fixup(powershell_guid, command_prompt_guid);
        }

        Ok(Self { profiles })
    }

    #[must_use]
    pub fn profiles(&self) -> &[LayeredProfile] {
        &self.profiles
    }
}

fn parse_legacy_profile_objects(input: &str) -> Result<Vec<JsonObject>, ProfileParseError> {
    let value = settings_json::parse(input).map_err(|_| ProfileParseError::InvalidJson)?;
    let root = value
        .as_object()
        .ok_or(ProfileParseError::ExpectedObject)?;
    let values = match JsonMember::from_object(root, "profiles") {
        JsonMember::Missing | JsonMember::Null => return Ok(Vec::new()),
        JsonMember::Value(JsonValue::Array(values)) => values,
        JsonMember::Value(_) => return Err(ProfileParseError::ExpectedArray),
    };

    clone_profile_objects(values)
}

fn parse_modern_profile_objects(input: &str) -> Result<Vec<JsonObject>, ProfileParseError> {
    let value = settings_json::parse(input).map_err(|_| ProfileParseError::InvalidJson)?;
    let root = value
        .as_object()
        .ok_or(ProfileParseError::ExpectedObject)?;
    let profiles = match JsonMember::from_object(root, "profiles") {
        JsonMember::Missing | JsonMember::Null => return Ok(Vec::new()),
        JsonMember::Value(JsonValue::Object(profiles)) => profiles,
        JsonMember::Value(_) => return Err(ProfileParseError::ExpectedObject),
    };
    let values = match JsonMember::from_object(profiles, "list") {
        JsonMember::Missing | JsonMember::Null => return Ok(Vec::new()),
        JsonMember::Value(JsonValue::Array(values)) => values,
        JsonMember::Value(_) => return Err(ProfileParseError::ExpectedArray),
    };

    clone_profile_objects(values)
}

fn clone_profile_objects(values: &[JsonValue]) -> Result<Vec<JsonObject>, ProfileParseError> {
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
