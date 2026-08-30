//! Portable deterministic helpers owned by the Windows Terminal `types` layer.

#[must_use]
pub fn clamp_to_short_max(value: i32, minimum: i16) -> i16 {
    value.clamp(i32::from(minimum), i32::from(i16::MAX)) as i16
}

#[must_use]
pub fn split_string(input: &str, delimiter: char) -> Vec<&str> {
    if input.is_empty() {
        Vec::new()
    } else {
        input.split(delimiter).collect()
    }
}

#[must_use]
pub fn string_to_uint(input: &str) -> Option<u32> {
    if input.is_empty() {
        return None;
    }

    let mut value = 0u32;
    for byte in input.bytes() {
        if !byte.is_ascii_digit() {
            return None;
        }
        value = value
            .wrapping_mul(10)
            .wrapping_add(u32::from(byte - b'0'));
    }
    Some(value)
}

#[must_use]
pub fn filter_string_for_paste(
    input: &str,
    carriage_return_newline: bool,
    control_codes: bool,
) -> String {
    let mut filtered = String::with_capacity(input.len());

    for ch in input.chars() {
        if carriage_return_newline && ch == '\n' {
            if !filtered.ends_with('\r') {
                filtered.push('\r');
            }
            continue;
        }

        let code = u32::from(ch);
        let removable_control = control_codes
            && ((code < 0x20 || (0x7f..=0x9f).contains(&code))
                && !matches!(ch, '\t' | '\n' | '\r'));
        if !removable_control {
            filtered.push(ch);
        }
    }

    filtered
}

#[must_use]
pub fn trim_paste(input: &str) -> &str {
    let is_trim_whitespace =
        |ch: char| matches!(ch, '\t' | '\n' | '\u{000b}' | '\u{000c}' | '\r' | ' ');
    let is_newline = |ch: char| matches!(ch, '\n' | '\u{000b}' | '\u{000c}' | '\r');

    let Some((last_non_space, last_char)) = input
        .char_indices()
        .rev()
        .find(|(_, ch)| !is_trim_whitespace(*ch))
    else {
        return "";
    };
    let end = last_non_space + last_char.len_utf8();

    if input[..end].find(is_newline).is_some() {
        input
    } else {
        &input[..end]
    }
}

#[must_use]
pub fn evaluate_starting_directory(current_directory: &str, starting_directory: &str) -> String {
    let bytes = starting_directory.as_bytes();
    let absolute_windows = bytes.len() >= 3
        && bytes[1] == b':'
        && matches!(bytes[2], b'\\' | b'/');
    if absolute_windows
        || starting_directory.starts_with('~')
        || starting_directory.starts_with('/')
    {
        starting_directory.to_owned()
    } else {
        format!("{current_directory}\\{starting_directory}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn microsoft_types_clamp_to_short_max_contract() {
        assert_eq!(clamp_to_short_max(0, 1), 1);
        assert_eq!(clamp_to_short_max(-1, 1), 1);
        assert_eq!(clamp_to_short_max(50_000, 1), i16::MAX);
        assert_eq!(clamp_to_short_max(100, 1), 100);
    }

    #[test]
    fn microsoft_types_split_string_contract() {
        assert!(split_string("", ';').is_empty());
        assert_eq!(split_string("1", ';'), ["1"]);
        assert_eq!(split_string(";", ';'), ["", ""]);
        assert_eq!(split_string("123", ';'), ["123"]);
        assert_eq!(split_string(";123", ';'), ["", "123"]);
        assert_eq!(split_string("123;", ';'), ["123", ""]);
        assert_eq!(split_string("123;456", ';'), ["123", "456"]);
        assert_eq!(split_string("123;456;789", ';'), ["123", "456", "789"]);
    }

    #[test]
    fn microsoft_types_filter_string_for_paste_contract() {
        for (input, expected) in [
            ("Hello World", "Hello World"),
            ("Hello World\r", "Hello World\r"),
            ("Hello World\n", "Hello World\r"),
            ("Hello World\r\n", "Hello World\r"),
            ("Hello\rWorld\r", "Hello\rWorld\r"),
            ("Hello\nWorld\n", "Hello\rWorld\r"),
            ("Hello\r\nWorld\r\n", "Hello\rWorld\r"),
            ("Hello\nWorld\n123", "Hello\rWorld\r123"),
        ] {
            assert_eq!(filter_string_for_paste(input, true, false), expected);
        }

        let c0 = format!("Hello{}{}{} 123", char::from(1), char::from(2), char::from(3));
        assert_eq!(filter_string_for_paste(&c0, false, true), "Hello 123");
        let c1 = format!("echo{}", char::from_u32(0x9c).expect("valid C1 scalar"));
        assert_eq!(filter_string_for_paste(&c1, true, true), "echo");
        let unicode = format!("你好\r\n{}世界{}\r\n123", char::from(1), char::from(2));
        assert_eq!(filter_string_for_paste(&unicode, true, true), "你好\r世界\r123");
    }

    #[test]
    fn microsoft_types_string_to_uint_contract() {
        assert_eq!(string_to_uint(""), None);
        assert_eq!(string_to_uint("xyz"), None);
        assert_eq!(string_to_uint(";"), None);
        assert_eq!(string_to_uint("1"), Some(1));
        assert_eq!(string_to_uint("123"), Some(123));
        assert_eq!(string_to_uint("123456789"), Some(123_456_789));
    }

    #[test]
    fn microsoft_types_trim_trailing_whitespace_contract() {
        for (input, expected) in [
            ("Foo   ", "Foo"),
            ("Foo\n", "Foo"),
            ("Foo\n\n", "Foo"),
            ("Foo\r\n", "Foo"),
            ("Foo Bar\n", "Foo Bar"),
            ("Foo\tBar\n", "Foo\tBar"),
            ("Foo Bar\t", "Foo Bar"),
            ("Foo Bar\t\t", "Foo Bar"),
            ("Foo Bar\t\n", "Foo Bar"),
            ("Foo\tBar\n\t", "Foo\tBar"),
        ] {
            assert_eq!(trim_paste(input), expected);
        }
    }

    #[test]
    fn microsoft_types_dont_trim_multiline_whitespace_contract() {
        for input in [
            "Foo\tBar",
            "Foo\nBar\n",
            "Foo  Baz\nBar\n",
            "Foo\tBaz\nBar\n",
            "Foo\tBaz\nBar\t\n",
        ] {
            assert_eq!(trim_paste(input), input);
        }
    }

    #[test]
    fn microsoft_types_evaluate_starting_directory_contract() {
        for cwd in ["C:\\Windows\\System32", "C:/Users/migrie"] {
            assert_eq!(evaluate_starting_directory(cwd, ""), format!("{cwd}\\"));
            assert_eq!(evaluate_starting_directory(cwd, "C:\\Windows"), "C:\\Windows");
            assert_eq!(evaluate_starting_directory(cwd, "C:/Users/migrie"), "C:/Users/migrie");
            assert_eq!(evaluate_starting_directory(cwd, "."), format!("{cwd}\\."));
            assert_eq!(
                evaluate_starting_directory(cwd, ".\\System32"),
                format!("{cwd}\\.\\System32")
            );
            assert_eq!(evaluate_starting_directory(cwd, "./dev"), format!("{cwd}\\./dev"));
            assert_eq!(evaluate_starting_directory(cwd, "~"), "~");
            assert_eq!(evaluate_starting_directory(cwd, "~/dev"), "~/dev");
            assert_eq!(evaluate_starting_directory(cwd, "/"), "/");
            assert_eq!(evaluate_starting_directory(cwd, "/dev"), "/dev");
        }
    }
}
