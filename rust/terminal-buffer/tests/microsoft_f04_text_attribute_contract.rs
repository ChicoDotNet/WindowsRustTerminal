use terminal_buffer::text_attribute::{LegacyColorDefaults, TextAttribute};

#[test]
fn microsoft_f04_text_attribute_exhaustive_legacy_roundtrip_matches_source_contract() {
    const ALL_ATTRS: u16 = 0xdfff;
    const COMMON_LVB_LEADING_BYTE: u16 = 0x0100;
    const COMMON_LVB_TRAILING_BYTE: u16 = 0x0200;
    const NON_META_RESERVED_BIT: u16 = 0x2000;

    let defaults = LegacyColorDefaults::default();

    for legacy in 0..ALL_ATTRS {
        if legacy & (NON_META_RESERVED_BIT | COMMON_LVB_LEADING_BYTE | COMMON_LVB_TRAILING_BYTE)
            != 0
        {
            continue;
        }

        let attribute = TextAttribute::from_legacy(legacy, defaults);
        assert_eq!(
            attribute.legacy_attributes(defaults),
            legacy,
            "legacy attribute 0x{legacy:04x} must round-trip exactly"
        );
    }
}
