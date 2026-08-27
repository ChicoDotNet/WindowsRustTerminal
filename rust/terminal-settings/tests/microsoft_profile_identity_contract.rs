use terminal_settings::profile_identity::ProfileIdentitySettings;

#[test]
fn microsoft_test_gen_guids_for_profiles_distinguishes_source_identity() {
    let inbox = r#"{
        "profiles": [
            {
                "name": "profile0",
                "source": "Terminal.App.UnitTest.0"
            },
            {
                "name": "profile1"
            }
        ]
    }"#;
    let user = r#"{
        "profiles": [
            {
                "name": "profile0",
                "source": "Terminal.App.UnitTest.0"
            },
            {
                "name": "profile0"
            }
        ]
    }"#;

    let settings = ProfileIdentitySettings::from_layered_legacy_arrays(user, inbox).unwrap();
    let profiles = settings.profiles();

    assert_eq!(profiles.len(), 3);
    assert_eq!(profiles[0].name(), Some("profile0"));
    assert!(!profiles[0].guid().is_zero());
    assert_eq!(profiles[0].source(), Some("Terminal.App.UnitTest.0"));

    assert_eq!(profiles[1].name(), Some("profile1"));
    assert!(!profiles[1].guid().is_zero());
    assert_eq!(profiles[1].source(), None);

    assert_eq!(profiles[2].name(), Some("profile0"));
    assert!(!profiles[2].guid().is_zero());
    assert_eq!(profiles[2].source(), None);
    assert_ne!(profiles[0].guid(), profiles[2].guid());
}

#[test]
fn microsoft_profile_defaults_prohibited_settings_do_not_inherit_identity_or_commandline() {
    let user = r#"{
        "profiles": {
            "defaults": {
                "guid": "{00000000-0000-0000-0000-000000000000}",
                "name": "Default Profile Name",
                "source": "Default Profile Source",
                "commandline": "foo.exe"
            },
            "list": [
                {
                    "name": "PowerShell",
                    "commandline": "powershell.exe",
                    "guid": "{61c54bbd-c2c6-5271-96e7-009a87ff44bf}"
                },
                {
                    "name": "Profile with just a name"
                },
                {
                    "guid": "{a0776706-1fa6-4439-b46c-287a65c084d5}"
                }
            ]
        }
    }"#;

    let settings = ProfileIdentitySettings::from_modern_json_with_prohibited_defaults(user).unwrap();
    assert!(!settings.defaults_has_guid());
    assert!(!settings.defaults_has_name());
    assert!(!settings.defaults_has_source());
    assert!(!settings.defaults_has_commandline());

    let profiles = settings.profiles();
    assert_eq!(profiles.len(), 3);

    assert_eq!(profiles[0].name(), Some("PowerShell"));
    assert_eq!(
        profiles[0].commandline(),
        Some("%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe")
    );
    assert_eq!(profiles[0].source(), None);
    assert!(!profiles[0].guid().is_zero());

    assert_eq!(profiles[1].name(), Some("Profile with just a name"));
    assert!(!profiles[1].guid().is_zero());
    assert_eq!(profiles[1].source(), None);
    assert_ne!(profiles[1].commandline(), Some("foo.exe"));

    assert_ne!(profiles[2].name(), Some("Default Profile Name"));
    assert!(!profiles[2].guid().is_zero());
    assert_eq!(profiles[2].source(), None);
    assert_ne!(profiles[2].commandline(), Some("foo.exe"));
}
