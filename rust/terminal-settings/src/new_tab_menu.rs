//! Portable `NewTabMenu` settings semantics.
//!
//! This slice intentionally owns only the deterministic menu-model behavior
//! exercised by Microsoft's `NewTabMenuTests`. Broader settings JSON layering is
//! added by later `SettingsModel` slices.

/// New-tab menu entry kinds from `NewTabMenuEntry.idl`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewTabMenuEntryType {
    Invalid,
    Profile,
    Separator,
    Folder,
    RemainingProfiles,
    MatchProfiles,
    Action,
}

/// Portable new-tab menu entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewTabMenuEntry {
    entry_type: NewTabMenuEntryType,
}

impl NewTabMenuEntry {
    fn new(entry_type: NewTabMenuEntryType) -> Self {
        Self { entry_type }
    }

    #[must_use]
    pub const fn entry_type(&self) -> NewTabMenuEntryType {
        self.entry_type
    }
}

/// Deterministic projection of `WindowSettingsDefaults().NewTabMenu()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewTabMenuSettings {
    entries: Vec<NewTabMenuEntry>,
    warnings: Vec<String>,
}

impl NewTabMenuSettings {
    /// Parses only the `newTabMenu` fragment needed by this migration slice.
    ///
    /// Microsoft defaults an absent `newTabMenu` property to one
    /// `RemainingProfiles` entry. A present array is preserved as provided; in
    /// particular, a folder with no name or child entries remains a valid entry.
    pub fn from_user_settings_json(input: &str) -> Result<Self, NewTabMenuParseError> {
        let Some(key_offset) = input.find("\"newTabMenu\"") else {
            return Ok(Self {
                entries: vec![NewTabMenuEntry::new(
                    NewTabMenuEntryType::RemainingProfiles,
                )],
                warnings: Vec::new(),
            });
        };

        let after_key = &input[key_offset + "\"newTabMenu\"".len()..];
        let Some(array_start_rel) = after_key.find('[') else {
            return Err(NewTabMenuParseError::ExpectedArray);
        };
        let array_start = key_offset + "\"newTabMenu\"".len() + array_start_rel;
        let array_end = find_matching_delimiter(input, array_start, '[', ']')
            .ok_or(NewTabMenuParseError::UnterminatedArray)?;
        let array = &input[array_start + 1..array_end];

        let mut entries = Vec::new();
        let mut cursor = 0;
        while cursor < array.len() {
            let remainder = &array[cursor..];
            let Some(object_start_rel) = remainder.find('{') else {
                break;
            };
            let object_start = cursor + object_start_rel;
            let object_end = find_matching_delimiter(array, object_start, '{', '}')
                .ok_or(NewTabMenuParseError::UnterminatedObject)?;
            let object = &array[object_start..=object_end];
            let entry_type = parse_entry_type(object)?;
            entries.push(NewTabMenuEntry::new(entry_type));
            cursor = object_end + 1;
        }

        Ok(Self {
            entries,
            warnings: Vec::new(),
        })
    }

    #[must_use]
    pub fn entries(&self) -> &[NewTabMenuEntry] {
        &self.entries
    }

    #[must_use]
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}

/// Parse failures for the deliberately narrow new-tab-menu JSON slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewTabMenuParseError {
    ExpectedArray,
    UnterminatedArray,
    UnterminatedObject,
    MissingType,
    UnknownType,
}

fn parse_entry_type(object: &str) -> Result<NewTabMenuEntryType, NewTabMenuParseError> {
    let Some(type_key) = object.find("\"type\"") else {
        return Err(NewTabMenuParseError::MissingType);
    };
    let after_key = &object[type_key + "\"type\"".len()..];
    let Some(colon) = after_key.find(':') else {
        return Err(NewTabMenuParseError::MissingType);
    };
    let value = after_key[colon + 1..].trim_start();
    let Some(value) = value.strip_prefix('"') else {
        return Err(NewTabMenuParseError::MissingType);
    };
    let Some(end_quote) = value.find('"') else {
        return Err(NewTabMenuParseError::MissingType);
    };

    match &value[..end_quote] {
        "profile" => Ok(NewTabMenuEntryType::Profile),
        "separator" => Ok(NewTabMenuEntryType::Separator),
        "folder" => Ok(NewTabMenuEntryType::Folder),
        "remainingProfiles" => Ok(NewTabMenuEntryType::RemainingProfiles),
        "matchProfiles" => Ok(NewTabMenuEntryType::MatchProfiles),
        "action" => Ok(NewTabMenuEntryType::Action),
        _ => Err(NewTabMenuParseError::UnknownType),
    }
}

fn find_matching_delimiter(
    input: &str,
    start: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (relative, ch) in input[start..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            match ch {
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
            continue;
        }
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(start + relative);
            }
        }
    }

    None
}
