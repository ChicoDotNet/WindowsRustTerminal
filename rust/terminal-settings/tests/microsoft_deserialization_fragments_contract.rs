use terminal_settings::deserialization_fragments::FragmentSettings;

const POWERSHELL: &str = "{61c54bbd-c2c6-5271-96e7-009a87ff44bf}";
const CMD: &str = "{0caa0dad-35be-5f56-a8ff-afceeeaa6101}";

#[test]
fn microsoft_load_fragments_with_multiple_updates_contract() {
    let mut settings = FragmentSettings::new();
    settings.seed_profile(POWERSHELL, "Windows PowerShell");
    settings.seed_profile(CMD, "Command Prompt");
    settings
        .merge_fragment(
            "fragment",
            r#"{
                "profiles": [
                    { "updates": "{61c54bbd-c2c6-5271-96e7-009a87ff44bf}", "name": "NewName" },
                    { "updates": "{0caa0dad-35be-5f56-a8ff-afceeeaa6101}", "cursorShape": "filledBox" },
                    { "guid": "{6239a42c-0000-49a3-80bd-e8fdd045185c}", "commandline": "cmd.exe" }
                ]
            }"#,
        )
        .unwrap();

    assert!(!settings.duplicate_profile());
    assert_eq!(settings.profile_count(), 3);
    assert_eq!(settings.profile_name(POWERSHELL), Some("NewName"));
}

#[test]
fn microsoft_fragment_action_simple_contract() {
    let mut settings = FragmentSettings::new();
    settings
        .merge_fragment(
            "fragment",
            r#"{
                "actions": [{
                    "command": { "action": "addMark" },
                    "name": "Test Action",
                    "id": "Test.FragmentAction"
                }]
            }"#,
        )
        .unwrap();
    assert!(settings.action_name_exists("Test Action"));
}

#[test]
fn microsoft_fragment_action_no_keys_contract() {
    let mut settings = FragmentSettings::new();
    settings
        .merge_fragment(
            "fragment",
            r#"{
                "actions": [{
                    "command": { "action": "addMark" },
                    "keys": "ctrl+f",
                    "id": "Test.FragmentAction",
                    "name": "Test Action"
                }]
            }"#,
        )
        .unwrap();
    assert!(settings.action_name_exists("Test Action"));
    assert!(!settings.key_is_bound("ctrl+f"));
}

#[test]
fn microsoft_fragment_action_nested_contract() {
    let mut settings = FragmentSettings::new();
    settings
        .merge_fragment(
            "fragment",
            r#"{
                "actions": [{
                    "name": "nested command",
                    "commands": [
                        { "name": "child1", "command": { "action": "newTab", "commandline": "ssh me@first.com" } },
                        { "name": "child2", "command": { "action": "newTab", "commandline": "ssh me@second.com" } }
                    ]
                }]
            }"#,
        )
        .unwrap();
    assert!(settings.action_name_exists("nested command"));
    assert_eq!(settings.nested_command_count("nested command"), Some(2));
}

#[test]
fn microsoft_fragment_action_nested_no_name_contract() {
    let mut settings = FragmentSettings::new();
    settings
        .merge_fragment(
            "fragment",
            r#"{
                "actions": [{
                    "commands": [{
                        "name": "child1",
                        "command": { "action": "newTab", "commandline": "ssh me@first.com" }
                    }]
                }]
            }"#,
        )
        .unwrap();
    assert_eq!(settings.nested_command_count("child1"), None);
}

#[test]
fn microsoft_fragment_action_iterable_contract() {
    let mut settings = FragmentSettings::new();
    settings.add_scheme("Campbell");
    settings.add_scheme("Campbell Powershell");
    settings.add_scheme("One Half Dark");
    settings
        .merge_fragment(
            "fragment",
            r#"{
                "actions": [{
                    "name": "nested",
                    "commands": [{
                        "iterateOn": "schemes",
                        "name": "${scheme.name}",
                        "command": { "action": "setColorScheme", "colorScheme": "${scheme.name}" }
                    }]
                }]
            }"#,
        )
        .unwrap();
    assert_eq!(settings.nested_command_count("nested"), Some(3));
}

#[test]
fn microsoft_fragment_action_roundtrip_contract() {
    let mut settings = FragmentSettings::new();
    settings
        .merge_fragment(
            "fragment",
            r#"{
                "actions": [{
                    "command": { "action": "addMark" },
                    "name": "Test Action",
                    "id": "Test.FragmentAction"
                }]
            }"#,
        )
        .unwrap();
    assert!(settings.action_name_exists("Test Action"));
    assert!(!settings.persists_fragment_action("Test Action"));
}
