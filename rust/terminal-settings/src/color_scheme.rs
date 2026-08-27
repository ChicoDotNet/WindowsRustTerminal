//! Portable color-scheme semantics from `SettingsModel`.
//!
//! This owner covers deterministic parsing/round-tripping and the non-colliding
//! inbox+user scheme-array layering exercised by Microsoft's `ColorSchemeTests`.
//! User-owned collision/retargeting semantics remain an explicit later slice.

use std::collections::BTreeMap;

use crate::settings_json::{self, JsonMember, JsonObject, JsonValue};

/// RGBA color used by the portable settings model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

const TABLE_KEYS: [&str; 16] = [
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "purple",
    "cyan",
    "white",
    "brightBlack",
    "brightRed",
    "brightGreen",
    "brightYellow",
    "brightBlue",
    "brightPurple",
    "brightCyan",
    "brightWhite",
];

/// Canonical safe Rust owner for a color scheme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorScheme {
    name: String,
    foreground: Color,
    background: Color,
    selection_background: Color,
    cursor_color: Color,
    table: [Color; 16],
}

impl ColorScheme {
    /// Parses one serialized color-scheme object.
    ///
    /// # Errors
    ///
    /// Returns [`ColorSchemeParseError`] if the JSON is malformed, the root is
    /// not an object, a required member is missing/wrongly typed, or a color is
    /// not a six-digit `#RRGGBB` value.
    pub fn from_json(input: &str) -> Result<Self, ColorSchemeParseError> {
        let value = settings_json::parse(input).map_err(|_| ColorSchemeParseError::InvalidJson)?;
        let object = value
            .as_object()
            .ok_or(ColorSchemeParseError::ExpectedObject)?;
        Self::from_object(object)
    }

    fn from_object(object: &JsonObject) -> Result<Self, ColorSchemeParseError> {
        let name = required_string(object, "name")?.to_owned();
        let foreground = required_color(object, "foreground")?;
        let background = required_color(object, "background")?;
        let selection_background = required_color(object, "selectionBackground")?;
        let cursor_color = required_color(object, "cursorColor")?;

        let mut table = [Color::rgb(0, 0, 0); 16];
        for (index, key) in TABLE_KEYS.iter().enumerate() {
            table[index] = required_color(object, key)?;
        }

        Ok(Self {
            name,
            foreground,
            background,
            selection_background,
            cursor_color,
            table,
        })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn foreground(&self) -> Color {
        self.foreground
    }

    #[must_use]
    pub const fn background(&self) -> Color {
        self.background
    }

    #[must_use]
    pub const fn selection_background(&self) -> Color {
        self.selection_background
    }

    #[must_use]
    pub const fn cursor_color(&self) -> Color {
        self.cursor_color
    }

    #[must_use]
    pub const fn table(&self) -> &[Color; 16] {
        &self.table
    }

    /// Projects the owner back to the same typed JSON object shape consumed by
    /// Microsoft's `ColorScheme::ToJson` round-trip contract.
    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        let mut object = JsonObject::new();
        object.insert("name".to_owned(), JsonValue::String(self.name.clone()));
        object.insert(
            "foreground".to_owned(),
            JsonValue::String(format_color(self.foreground)),
        );
        object.insert(
            "background".to_owned(),
            JsonValue::String(format_color(self.background)),
        );
        object.insert(
            "selectionBackground".to_owned(),
            JsonValue::String(format_color(self.selection_background)),
        );
        object.insert(
            "cursorColor".to_owned(),
            JsonValue::String(format_color(self.cursor_color)),
        );
        for (key, color) in TABLE_KEYS.iter().zip(self.table) {
            object.insert((*key).to_owned(), JsonValue::String(format_color(color)));
        }
        JsonValue::Object(object)
    }
}

/// Layered color-scheme collection from inbox defaults followed by user settings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColorSchemeCollection {
    schemes: BTreeMap<String, ColorScheme>,
}

impl ColorSchemeCollection {
    /// Layers non-colliding `schemes` arrays from inbox and user settings.
    ///
    /// # Errors
    ///
    /// Returns [`ColorSchemeParseError`] for malformed settings/schemes or when
    /// two layers currently collide by name. Collision retargeting has different
    /// Microsoft semantics and deliberately remains deferred instead of being
    /// approximated here.
    pub fn from_inbox_and_user_json(
        inbox: &str,
        user: &str,
    ) -> Result<Self, ColorSchemeParseError> {
        let mut result = Self::default();
        result.layer_document(inbox)?;
        result.layer_document(user)?;
        Ok(result)
    }

    fn layer_document(&mut self, input: &str) -> Result<(), ColorSchemeParseError> {
        let document =
            settings_json::parse(input).map_err(|_| ColorSchemeParseError::InvalidJson)?;
        let object = document
            .as_object()
            .ok_or(ColorSchemeParseError::ExpectedObject)?;
        match JsonMember::from_object(object, "schemes") {
            JsonMember::Missing | JsonMember::Null => Ok(()),
            JsonMember::Value(JsonValue::Array(values)) => {
                for value in values {
                    let scheme = ColorScheme::from_object(
                        value
                            .as_object()
                            .ok_or(ColorSchemeParseError::ExpectedObject)?,
                    )?;
                    if self.schemes.contains_key(scheme.name()) {
                        return Err(ColorSchemeParseError::CollisionRequiresRetargeting);
                    }
                    self.schemes.insert(scheme.name.clone(), scheme);
                }
                Ok(())
            }
            JsonMember::Value(_) => Err(ColorSchemeParseError::ExpectedArray),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.schemes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.schemes.is_empty()
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ColorScheme> {
        self.schemes.get(name)
    }
}

/// Parse failures for the portable color-scheme slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSchemeParseError {
    InvalidJson,
    ExpectedObject,
    ExpectedArray,
    MissingMember,
    InvalidString,
    InvalidColor,
    CollisionRequiresRetargeting,
}

fn required_string<'a>(
    object: &'a JsonObject,
    key: &str,
) -> Result<&'a str, ColorSchemeParseError> {
    match JsonMember::from_object(object, key) {
        JsonMember::Missing | JsonMember::Null => Err(ColorSchemeParseError::MissingMember),
        JsonMember::Value(JsonValue::String(value)) => Ok(value),
        JsonMember::Value(_) => Err(ColorSchemeParseError::InvalidString),
    }
}

fn required_color(object: &JsonObject, key: &str) -> Result<Color, ColorSchemeParseError> {
    parse_color(required_string(object, key)?)
}

fn parse_color(value: &str) -> Result<Color, ColorSchemeParseError> {
    let digits = value
        .strip_prefix('#')
        .ok_or(ColorSchemeParseError::InvalidColor)?;
    if digits.len() != 6 || !digits.is_ascii() {
        return Err(ColorSchemeParseError::InvalidColor);
    }
    Ok(Color::rgb(
        parse_hex_byte(&digits[0..2])?,
        parse_hex_byte(&digits[2..4])?,
        parse_hex_byte(&digits[4..6])?,
    ))
}

fn parse_hex_byte(value: &str) -> Result<u8, ColorSchemeParseError> {
    u8::from_str_radix(value, 16).map_err(|_| ColorSchemeParseError::InvalidColor)
}

fn format_color(color: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)
}
