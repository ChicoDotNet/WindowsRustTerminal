use terminal_adapter::{AdaptDispatchPresentationState, PageGeometry};
use terminal_buffer::{
    text_attribute::{TextAttribute, UnderlineStyle},
    text_color::TextColor,
};
use terminal_parser::{
    output_engine::{OutputAction, TermDispatch},
    state_machine::Parameters,
};

fn state() -> AdaptDispatchPresentationState {
    AdaptDispatchPresentationState::new(PageGeometry::new(20, 100, 29))
}

fn apply_single(starting: TextAttribute, option: i32) -> TextAttribute {
    let mut state = state();
    state.set_current_attributes(starting);
    state.dispatch(OutputAction::SetGraphicsRendition(Parameters::from_values(vec![
        Some(option),
    ])));
    state.current_attributes()
}

#[test]
fn microsoft_graphics_base_empty_sgr_resets_attributes() {
    let mut state = state();
    let mut starting = TextAttribute::default();
    starting.set_intense(true);
    starting.set_faint(true);
    starting.set_reverse_video(true);
    starting.set_invisible(true);
    starting.set_crossed_out(true);
    starting.set_overlined(true);
    starting.set_underline_style(UnderlineStyle::Curly);
    starting.set_foreground(TextColor::index16(TextColor::BRIGHT_RED));
    starting.set_background(TextColor::index16(TextColor::BRIGHT_BLUE));
    state.set_current_attributes(starting);

    state.dispatch(OutputAction::SetGraphicsRendition(Parameters::default()));

    assert_eq!(state.current_attributes(), TextAttribute::default());
}

#[test]
fn microsoft_graphics_single_sgr_options_match_source_contract() {
    let default = TextAttribute::default();

    let mut fully_set = default;
    fully_set.set_intense(true);
    fully_set.set_faint(true);
    fully_set.set_reverse_video(true);
    fully_set.set_invisible(true);
    fully_set.set_crossed_out(true);
    fully_set.set_overlined(true);
    fully_set.set_underline_style(UnderlineStyle::Curly);
    fully_set.set_foreground(TextColor::index16(TextColor::BRIGHT_RED));
    fully_set.set_background(TextColor::index16(TextColor::BRIGHT_BLUE));
    assert_eq!(apply_single(fully_set, 0), default);

    let mut expected = default;
    expected.set_intense(true);
    assert_eq!(apply_single(default, 1), expected);

    expected = default;
    expected.set_faint(true);
    assert_eq!(apply_single(default, 2), expected);

    expected = default;
    expected.set_underline_style(UnderlineStyle::Single);
    assert_eq!(apply_single(default, 4), expected);

    expected = default;
    expected.set_reverse_video(true);
    assert_eq!(apply_single(default, 7), expected);

    expected = default;
    expected.set_invisible(true);
    assert_eq!(apply_single(default, 8), expected);

    expected = default;
    expected.set_crossed_out(true);
    assert_eq!(apply_single(default, 9), expected);

    expected = default;
    expected.set_underline_style(UnderlineStyle::Double);
    assert_eq!(apply_single(default, 21), expected);

    let mut intense_faint = default;
    intense_faint.set_intense(true);
    intense_faint.set_faint(true);
    assert_eq!(apply_single(intense_faint, 22), default);

    let mut underlined = default;
    underlined.set_underline_style(UnderlineStyle::Curly);
    assert_eq!(apply_single(underlined, 24), default);

    let mut reversed = default;
    reversed.set_reverse_video(true);
    assert_eq!(apply_single(reversed, 27), default);

    let mut invisible = default;
    invisible.set_invisible(true);
    assert_eq!(apply_single(invisible, 28), default);

    let mut crossed = default;
    crossed.set_crossed_out(true);
    assert_eq!(apply_single(crossed, 29), default);

    let mut overlined = default;
    overlined.set_overlined(true);
    assert_eq!(apply_single(overlined, 55), default);

    expected = default;
    expected.set_overlined(true);
    assert_eq!(apply_single(default, 53), expected);

    for (option, index) in (30..=37).zip(0..=7) {
        let mut starting = default;
        starting.set_background(TextColor::index16(TextColor::BRIGHT_MAGENTA));
        let mut expected = starting;
        expected.set_foreground(TextColor::index16(index));
        assert_eq!(apply_single(starting, option), expected, "SGR {option}");
    }

    let mut starting = default;
    starting.set_foreground(TextColor::index16(TextColor::BRIGHT_GREEN));
    starting.set_background(TextColor::index16(TextColor::BRIGHT_MAGENTA));
    expected = starting;
    expected.set_default_foreground();
    assert_eq!(apply_single(starting, 39), expected);

    for (option, index) in (40..=47).zip(0..=7) {
        let mut starting = default;
        starting.set_foreground(TextColor::index16(TextColor::BRIGHT_CYAN));
        let mut expected = starting;
        expected.set_background(TextColor::index16(index));
        assert_eq!(apply_single(starting, option), expected, "SGR {option}");
    }

    starting = default;
    starting.set_foreground(TextColor::index16(TextColor::BRIGHT_CYAN));
    starting.set_background(TextColor::index16(TextColor::BRIGHT_GREEN));
    expected = starting;
    expected.set_default_background();
    assert_eq!(apply_single(starting, 49), expected);

    for (option, index) in (90..=97).zip(8..=15) {
        let mut starting = default;
        starting.set_background(TextColor::index16(TextColor::DARK_GREEN));
        let mut expected = starting;
        expected.set_foreground(TextColor::index16(index));
        assert_eq!(apply_single(starting, option), expected, "SGR {option}");
    }

    for (option, index) in (100..=107).zip(8..=15) {
        let mut starting = default;
        starting.set_foreground(TextColor::index16(TextColor::DARK_YELLOW));
        let mut expected = starting;
        expected.set_background(TextColor::index16(index));
        assert_eq!(apply_single(starting, option), expected, "SGR {option}");
    }
}
