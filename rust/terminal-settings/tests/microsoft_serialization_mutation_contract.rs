use terminal_settings::{
    serialization::SettingsDocument,
    settings_json::JsonValue,
};

#[test]
fn microsoft_modify_color_scheme_and_roundtrip_contract() {
    // Microsoft: SerializationTests::ModifyColorSchemeAndRoundtrip.
    let input = r##"
    {
        "defaultProfile": "{6239a42c-0000-49a3-80bd-e8fdd045185c}",
        "profiles": [
            {
                "name": "profile0",
                "guid": "{6239a42c-0000-49a3-80bd-e8fdd045185c}"
            }
        ],
        "schemes": [
            {
                "name": "MyScheme",
                "foreground": "#CCCCCC",
                "background": "#0C0C0C",
                "cursorColor": "#FFFFFF",
                "black": "#0C0C0C",
                "red": "#C50F1F",
                "green": "#13A10E",
                "yellow": "#C19C00",
                "blue": "#0037DA",
                "purple": "#881798",
                "cyan": "#3A96DD",
                "white": "#CCCCCC",
                "brightBlack": "#767676",
                "brightRed": "#E74856",
                "brightGreen": "#16C60C",
                "brightYellow": "#F9F1A5",
                "brightBlue": "#3B78FF",
                "brightPurple": "#B4009E",
                "brightCyan": "#61D6D6",
                "brightWhite": "#F2F2F2"
            }
        ]
    }
    "##;

    let mut settings = SettingsDocument::from_json(input).expect("Microsoft settings JSON parses");
    let before = settings.to_json_value().clone();
    settings
        .set_color_scheme_foreground("MyScheme", "#AABBCC")
        .expect("MyScheme exists");

    let root = settings
        .to_json_value()
        .as_object()
        .expect("settings root remains an object");
    let schemes = root
        .get("schemes")
        .and_then(JsonValue::as_array)
        .expect("schemes remains an array");
    let scheme = schemes[0].as_object().expect("scheme remains an object");

    assert_eq!(scheme.get("name").and_then(JsonValue::as_str), Some("MyScheme"));
    assert_eq!(
        scheme.get("foreground").and_then(JsonValue::as_str),
        Some("#AABBCC")
    );
    assert_eq!(
        scheme.get("background").and_then(JsonValue::as_str),
        Some("#0C0C0C")
    );

    let before_root = before.as_object().expect("original root object");
    assert_eq!(root.get("defaultProfile"), before_root.get("defaultProfile"));
    assert_eq!(root.get("profiles"), before_root.get("profiles"));
}
