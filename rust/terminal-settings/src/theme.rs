//! Portable theme-model semantics from `SettingsModel`.
//!
//! This module owns deterministic theme parsing and selection behavior while
//! XAML/WinRT projection remains at the platform boundary.

use std::collections::BTreeMap;

/// RGBA color value used by theme settings.
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

    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Portable equivalent of XAML `ElementTheme`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ElementTheme {
    #[default]
    Default,
    Light,
    Dark,
}

/// Theme-color source kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeColorType {
    Color,
    Accent,
    TerminalBackground,
}

/// Theme color preserving its semantic source kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeColor {
    color_type: ThemeColorType,
    color: Option<Color>,
}

impl ThemeColor {
    #[must_use]
    pub const fn color_type(&self) -> ThemeColorType {
        self.color_type
    }

    #[must_use]
    pub const fn color(&self) -> Option<Color> {
        self.color
    }
}

/// Theme settings applied to the tab row.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TabRowTheme {
    background: Option<ThemeColor>,
    unfocused_background: Option<ThemeColor>,
}

impl TabRowTheme {
    #[must_use]
    pub const fn background(&self) -> Option<ThemeColor> {
        self.background
    }

    #[must_use]
    pub const fn unfocused_background(&self) -> Option<ThemeColor> {
        self.unfocused_background
    }
}

/// Theme settings applied to the window surface.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WindowTheme {
    requested_theme: ElementTheme,
    use_mica: bool,
}

impl WindowTheme {
    #[must_use]
    pub const fn requested_theme(&self) -> ElementTheme {
        self.requested_theme
    }

    #[must_use]
    pub const fn use_mica(&self) -> bool {
        self.use_mica
    }
}

/// Portable theme owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    name: String,
    tab_row: Option<TabRowTheme>,
    window: Option<WindowTheme>,
}

impl Theme {
    /// Parses a serialized theme object.
    ///
    /// Missing or explicit-null sub-objects remain absent, matching Microsoft's
    /// `Theme::FromJson` behavior.
    ///
    /// # Errors
    ///
    /// Returns [`ThemeParseError`] when the required theme name is absent, a
    /// present sub-object is malformed, or a supported theme value is invalid.
    pub fn from_json(input: &str) -> Result<Self, ThemeParseError> {
        let name = string_member(input, "name")?.ok_or(ThemeParseError::MissingName)?;

        let tab_row = match object_member(input, "tabRow")? {
            None => None,
            Some(tab_row) => Some(TabRowTheme {
                background: theme_color_member(tab_row, "background")?,
                unfocused_background: theme_color_member(tab_row, "unfocusedBackground")?,
            }),
        };

        let window = match object_member(input, "window")? {
            None => None,
            Some(window) => Some(WindowTheme {
                requested_theme: match string_member(window, "applicationTheme")? {
                    None => ElementTheme::Default,
                    Some(value) if value.eq_ignore_ascii_case("light") => ElementTheme::Light,
                    Some(value) if value.eq_ignore_ascii_case("dark") => ElementTheme::Dark,
                    Some(value) if value.eq_ignore_ascii_case("system") => ElementTheme::Default,
                    Some(_) => return Err(ThemeParseError::InvalidElementTheme),
                },
                use_mica: bool_member(window, "useMica")?.unwrap_or(false),
            }),
        };

        Ok(Self {
            name,
            tab_row,
            window,
        })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn tab_row(&self) -> Option<&TabRowTheme> {
        self.tab_row.as_ref()
    }

    #[must_use]
    pub const fn window(&self) -> Option<&WindowTheme> {
        self.window.as_ref()
    }

    #[must_use]
    pub fn requested_theme(&self) -> ElementTheme {
        self.window
            .as_ref()
            .map_or(ElementTheme::Default, WindowTheme::requested_theme)
    }

    fn system() -> Self {
        Self {
            name: "system".to_owned(),
            tab_row: None,
            window: None,
        }
    }
}

/// Settings warning produced by deterministic theme selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsLoadWarning {
    UnknownTheme,
}

/// Theme collection plus current-theme selection semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeSettings {
    themes: BTreeMap<String, Theme>,
    current_theme_name: String,
    warnings: Vec<SettingsLoadWarning>,
}

impl ThemeSettings {
    /// Parses the theme-related subset of a user settings object.
    ///
    /// # Errors
    ///
    /// Returns [`ThemeParseError`] when the themes array or any contained theme
    /// object is malformed.
    pub fn from_user_settings_json(input: &str) -> Result<Self, ThemeParseError> {
        let mut themes = BTreeMap::new();
        if let Some(array) = array_member(input, "themes")? {
            for object in top_level_objects(array)? {
                let theme = Theme::from_json(object)?;
                themes.insert(theme.name.clone(), theme);
            }
        }

        let requested = string_member(input, "theme")?.unwrap_or_else(|| "system".to_owned());
        let known_builtin = matches!(requested.as_str(), "system" | "light" | "dark");
        let valid = known_builtin || themes.contains_key(&requested);
        let warnings = if valid {
            Vec::new()
        } else {
            vec![SettingsLoadWarning::UnknownTheme]
        };

        Ok(Self {
            themes,
            current_theme_name: if valid {
                requested
            } else {
                "system".to_owned()
            },
            warnings,
        })
    }

    #[must_use]
    pub fn theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    #[must_use]
    pub fn warnings(&self) -> &[SettingsLoadWarning] {
        &self.warnings
    }

    #[must_use]
    pub fn current_theme(&self) -> Theme {
        self.themes
            .get(&self.current_theme_name)
            .cloned()
            .unwrap_or_else(Theme::system)
    }
}

/// Parse failures for the portable theme slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeParseError {
    MissingName,
    InvalidString,
    InvalidObject,
    InvalidArray,
    InvalidBoolean,
    InvalidColor,
    InvalidElementTheme,
}

fn theme_color_member(input: &str, key: &str) -> Result<Option<ThemeColor>, ThemeParseError> {
    let Some(rest) = value_after_key(input, key) else {
        return Ok(None);
    };
    let rest = rest.trim_start();
    if rest.starts_with("null") {
        return Ok(None);
    }
    let value = parse_quoted(rest).ok_or(ThemeParseError::InvalidColor)?;
    let color = match value.as_str() {
        "accent" => ThemeColor {
            color_type: ThemeColorType::Accent,
            color: None,
        },
        "terminalBackground" => ThemeColor {
            color_type: ThemeColorType::TerminalBackground,
            color: None,
        },
        _ => ThemeColor {
            color_type: ThemeColorType::Color,
            color: Some(parse_hex_color(&value)?),
        },
    };
    Ok(Some(color))
}

fn parse_hex_color(value: &str) -> Result<Color, ThemeParseError> {
    let digits = value
        .strip_prefix('#')
        .ok_or(ThemeParseError::InvalidColor)?;
    match digits.len() {
        6 => Ok(Color::rgb(
            parse_hex_byte(&digits[0..2])?,
            parse_hex_byte(&digits[2..4])?,
            parse_hex_byte(&digits[4..6])?,
        )),
        8 => Ok(Color::rgba(
            parse_hex_byte(&digits[0..2])?,
            parse_hex_byte(&digits[2..4])?,
            parse_hex_byte(&digits[4..6])?,
            parse_hex_byte(&digits[6..8])?,
        )),
        _ => Err(ThemeParseError::InvalidColor),
    }
}

fn parse_hex_byte(value: &str) -> Result<u8, ThemeParseError> {
    u8::from_str_radix(value, 16).map_err(|_| ThemeParseError::InvalidColor)
}

fn string_member(input: &str, key: &str) -> Result<Option<String>, ThemeParseError> {
    let Some(rest) = value_after_key(input, key) else {
        return Ok(None);
    };
    let rest = rest.trim_start();
    if rest.starts_with("null") {
        return Ok(None);
    }
    parse_quoted(rest)
        .map(Some)
        .ok_or(ThemeParseError::InvalidString)
}

fn bool_member(input: &str, key: &str) -> Result<Option<bool>, ThemeParseError> {
    let Some(rest) = value_after_key(input, key) else {
        return Ok(None);
    };
    let rest = rest.trim_start();
    if rest.starts_with("true") {
        Ok(Some(true))
    } else if rest.starts_with("false") {
        Ok(Some(false))
    } else if rest.starts_with("null") {
        Ok(None)
    } else {
        Err(ThemeParseError::InvalidBoolean)
    }
}

fn object_member<'a>(input: &'a str, key: &str) -> Result<Option<&'a str>, ThemeParseError> {
    let Some(rest) = value_after_key(input, key) else {
        return Ok(None);
    };
    let rest = rest.trim_start();
    if rest.starts_with("null") {
        return Ok(None);
    }
    let end = find_matching_delimiter(rest, 0, '{', '}').ok_or(ThemeParseError::InvalidObject)?;
    Ok(Some(&rest[..=end]))
}

fn array_member<'a>(input: &'a str, key: &str) -> Result<Option<&'a str>, ThemeParseError> {
    let Some(rest) = value_after_key(input, key) else {
        return Ok(None);
    };
    let rest = rest.trim_start();
    if rest.starts_with("null") {
        return Ok(None);
    }
    if !rest.starts_with('[') {
        return Err(ThemeParseError::InvalidArray);
    }
    let end = find_matching_delimiter(rest, 0, '[', ']').ok_or(ThemeParseError::InvalidArray)?;
    Ok(Some(&rest[1..end]))
}

fn top_level_objects(input: &str) -> Result<Vec<&str>, ThemeParseError> {
    let mut result = Vec::new();
    let mut cursor = 0usize;
    while cursor < input.len() {
        let remainder = &input[cursor..];
        let Some(start_rel) = remainder.find('{') else {
            break;
        };
        let start = cursor + start_rel;
        let end_rel = find_matching_delimiter(input, start, '{', '}')
            .ok_or(ThemeParseError::InvalidObject)?;
        result.push(&input[start..=end_rel]);
        cursor = end_rel + 1;
    }
    Ok(result)
}

fn value_after_key<'a>(input: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{key}\"");
    let offset = input.find(&needle)?;
    let after = &input[offset + needle.len()..];
    let colon = after.find(':')?;
    Some(&after[colon + 1..])
}

fn parse_quoted(input: &str) -> Option<String> {
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

fn find_matching_delimiter(input: &str, start: usize, open: char, close: char) -> Option<usize> {
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
