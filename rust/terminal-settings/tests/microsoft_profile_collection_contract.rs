use terminal_settings::profile_collection::ProfileCollection;

#[test]
fn microsoft_layer_profiles_on_array_merges_by_guid_and_preserves_inbox_order() {
    let inbox = r#"{
        "profiles": [
            {
                "name": "profile0",
                "guid": "{6239a42c-0000-49a3-80bd-e8fdd045185c}"
            },
            {
                "name": "profile1",
                "guid": "{6239a42c-1111-49a3-80bd-e8fdd045185c}"
            },
            {
                "name": "profile2",
                "guid": "{6239a42c-2222-49a3-80bd-e8fdd045185c}"
            }
        ]
    }"#;
    let user = r#"{
        "profiles": [
            {
                "name": "profile3",
                "guid": "{6239a42c-0000-49a3-80bd-e8fdd045185c}"
            },
            {
                "name": "profile4",
                "guid": "{6239a42c-1111-49a3-80bd-e8fdd045185c}"
            }
        ]
    }"#;

    let collection = ProfileCollection::from_layered_legacy_arrays(user, inbox).unwrap();
    let profiles = collection.profiles();

    assert_eq!(profiles.len(), 3);
    assert_eq!(profiles[0].name(), Some("profile3"));
    assert_eq!(profiles[1].name(), Some("profile4"));
    assert_eq!(profiles[2].name(), Some("profile2"));
}
