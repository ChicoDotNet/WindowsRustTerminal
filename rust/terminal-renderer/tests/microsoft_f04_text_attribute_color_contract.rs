use terminal_buffer::{
    text_attribute::TextAttribute,
    text_color::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND, Rgb, TABLE_SIZE, TextColor},
};
use terminal_renderer::{
    RenderSettingsPolicy, ResolvedAttributeColors, resolve_text_attribute_colors,
};

fn color_table() -> [Rgb; TABLE_SIZE] {
    let mut table = [Rgb::default(); TABLE_SIZE];
    table[DEFAULT_FOREGROUND] = Rgb::new(1, 2, 3);
    table[DEFAULT_BACKGROUND] = Rgb::new(4, 5, 6);
    table
}

fn colors(foreground: Rgb, background: Rgb) -> ResolvedAttributeColors {
    ResolvedAttributeColors {
        foreground,
        background,
    }
}

fn resolve(attribute: TextAttribute, table: &[Rgb; TABLE_SIZE]) -> ResolvedAttributeColors {
    resolve_text_attribute_colors(
        attribute,
        table,
        DEFAULT_FOREGROUND,
        DEFAULT_BACKGROUND,
        RenderSettingsPolicy::default(),
    )
}

#[test]
fn microsoft_f04_text_attribute_color_getters_match_source_contract() {
    let table = color_table();
    let red = Rgb::new(255, 0, 0);
    let faint_red = Rgb::new(127, 0, 0);
    let green = Rgb::new(0, 255, 0);
    let mut attribute = TextAttribute::from_rgb(red, green);

    assert!(!attribute.is_reverse_video());
    assert_eq!(
        attribute
            .foreground()
            .resolve(&table, DEFAULT_FOREGROUND, false),
        red
    );
    assert_eq!(
        attribute
            .background()
            .resolve(&table, DEFAULT_BACKGROUND, false),
        green
    );
    assert_eq!(resolve(attribute, &table), colors(red, green));

    attribute.set_reverse_video(true);
    assert_eq!(resolve(attribute, &table), colors(green, red));

    attribute.set_reverse_video(false);
    attribute.set_faint(true);
    assert_eq!(resolve(attribute, &table), colors(faint_red, green));

    attribute.set_reverse_video(true);
    assert_eq!(resolve(attribute, &table), colors(green, faint_red));

    attribute.set_reverse_video(false);
    attribute.set_faint(false);
    attribute.set_invisible(true);
    assert_eq!(resolve(attribute, &table), colors(green, green));

    attribute.set_reverse_video(true);
    assert_eq!(resolve(attribute, &table), colors(red, red));
}

#[test]
fn microsoft_f04_reverse_default_colors_match_source_contract() {
    let table = color_table();
    let default_foreground = Rgb::new(1, 2, 3);
    let default_background = Rgb::new(4, 5, 6);
    let red = Rgb::new(255, 0, 0);
    let green = Rgb::new(0, 255, 0);
    let mut attribute = TextAttribute::default();

    assert!(!attribute.is_reverse_video());
    assert_eq!(
        attribute
            .foreground()
            .resolve(&table, DEFAULT_FOREGROUND, false),
        default_foreground
    );
    assert_eq!(
        attribute
            .background()
            .resolve(&table, DEFAULT_BACKGROUND, false),
        default_background
    );
    assert_eq!(
        resolve(attribute, &table),
        colors(default_foreground, default_background)
    );

    attribute.set_reverse_video(true);
    assert!(attribute.is_reverse_video());
    assert_eq!(
        resolve(attribute, &table),
        colors(default_background, default_foreground)
    );

    attribute.set_foreground(TextColor::rgb(red.r, red.g, red.b));
    assert!(attribute.is_reverse_video());
    assert_eq!(resolve(attribute, &table), colors(default_background, red));

    attribute.invert();
    assert!(!attribute.is_reverse_video());
    attribute.set_default_foreground();
    attribute.set_background(TextColor::rgb(green.r, green.g, green.b));
    assert_eq!(
        resolve(attribute, &table),
        colors(default_foreground, green)
    );
}
