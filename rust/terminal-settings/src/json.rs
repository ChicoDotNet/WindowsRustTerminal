//! Shared JSON navigation for portable `SettingsModel` owners.
//!
//! This deliberately small core preserves the distinctions that matter to
//! Windows Terminal settings semantics: missing members, explicit `null`, typed
//! scalar values, and balanced object/array fragments. It is not a general JSON
//! replacement; it is the common deterministic substrate used by incremental
//! settings owners until the broader serialization slice lands.

/// Failures while navigating a settings JSON fragment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonError {
    InvalidString,
    InvalidBoolean,
    InvalidObject,
    InvalidArray,
}

/// Presence state for a JSON member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Member<'a> {
    Missing,
    Null,
    Value(&'a str),
}

/// Returns the raw value tail for `key`, preserving missing vs explicit null.
#[must_use]
pub fn member<'a>(input: &'a str, key: &str) -> Member<'a> {
    let needle = format!("\"{key}\"");
    let Some(offset) = input.find(&needle) else {
        return Member::Missing;
    };
    let after = &input[offset + needle.len()..];
    let Some(colon) = after.find(':') else {
        return Member::Missing;
    };
    let value = after[colon + 1..].trim_start();
    if value.starts_with("null") {
        Member::Null
    } else {
        Member::Value(value)
    }
}

/// Reads an optional string member. Missing and null both produce `None`.
pub fn string_member(input: &str, key: &str) -> Result<Option<String>, JsonError> {
    match member(input, key) {
        Member::Missing | Member::Null => Ok(None),
        Member::Value(value) => parse_quoted(value)
            .map(Some)
            .ok_or(JsonError::InvalidString),
    }
}

/// Reads an optional boolean member. Missing and null both produce `None`.
pub fn bool_member(input: &str, key: &str) -> Result<Option<bool>, JsonError> {
    match member(input, key) {
        Member::Missing | Member::Null => Ok(None),
        Member::Value(value) if value.starts_with("true") => Ok(Some(true)),
        Member::Value(value) if value.starts_with("false") => Ok(Some(false)),
        Member::Value(_) => Err(JsonError::InvalidBoolean),
    }
}

/// Returns a balanced object member. Missing and null both produce `None`.
pub fn object_member<'a>(input: &'a str, key: &str) -> Result<Option<&'a str>, JsonError> {
    match member(input, key) {
        Member::Missing | Member::Null => Ok(None),
        Member::Value(value) => {
            if !value.starts_with('{') {
                return Err(JsonError::InvalidObject);
            }
            let end = find_matching_delimiter(value, 0, '{', '}')
                .ok_or(JsonError::InvalidObject)?;
            Ok(Some(&value[..=end]))
        }
    }
}

/// Returns the interior of a balanced array member.
pub fn array_member<'a>(input: &'a str, key: &str) -> Result<Option<&'a str>, JsonError> {
    match member(input, key) {
        Member::Missing | Member::Null => Ok(None),
        Member::Value(value) => {
            if !value.starts_with('[') {
                return Err(JsonError::InvalidArray);
            }
            let end = find_matching_delimiter(value, 0, '[', ']')
                .ok_or(JsonError::InvalidArray)?;
            Ok(Some(&value[1..end]))
        }
    }
}

/// Splits an array interior into its top-level object fragments.
pub fn top_level_objects(input: &str) -> Result<Vec<&str>, JsonError> {
    let mut result = Vec::new();
    let mut cursor = 0usize;
    while cursor < input.len() {
        let remainder = &input[cursor..];
        let Some(start_rel) = remainder.find('{') else {
            break;
        };
        let start = cursor + start_rel;
        let end = find_matching_delimiter(input, start, '{', '}')
            .ok_or(JsonError::InvalidObject)?;
        result.push(&input[start..=end]);
        cursor = end + 1;
    }
    Ok(result)
}

/// Parses one JSON string token, handling the escapes needed by settings keys.
#[must_use]
pub fn parse_quoted(input: &str) -> Option<String> {
    let input = input.trim_start();
    let mut chars = input.chars();
    if chars.next()? != '"' {
        return None;
    }

    let mut result = String::new();
    let mut escaped = false;
    for ch in chars {
        if escaped {
            result.push(match ch {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                other => other,
            });
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Some(result);
        } else {
            result.push(ch);
        }
    }
    None
}

/// Finds a matching delimiter while ignoring delimiters embedded in strings.
#[must_use]
pub fn find_matching_delimiter(
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
        } else if ch == open {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinguishes_missing_null_and_value() {
        let json = r#"{ "nullValue": null, "value": "ok" }"#;
        assert_eq!(member(json, "missing"), Member::Missing);
        assert_eq!(member(json, "nullValue"), Member::Null);
        assert!(matches!(member(json, "value"), Member::Value(_)));
    }

    #[test]
    fn balanced_fragments_ignore_delimiters_in_strings() {
        let json = r#"{ "items": [{ "text": "} ]" }, { "text": "ok" }] }"#;
        let array = array_member(json, "items")
            .expect("array should parse")
            .expect("array should exist");
        let objects = top_level_objects(array).expect("objects should parse");
        assert_eq!(objects.len(), 2);
    }

    #[test]
    fn typed_members_preserve_null_as_absent() {
        let json = r#"{ "name": "terminal", "enabled": true, "other": null }"#;
        assert_eq!(string_member(json, "name"), Ok(Some("terminal".to_owned())));
        assert_eq!(bool_member(json, "enabled"), Ok(Some(true)));
        assert_eq!(string_member(json, "other"), Ok(None));
    }
}
