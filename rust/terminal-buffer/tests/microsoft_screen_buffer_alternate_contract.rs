use terminal_buffer::alternate_buffer::{AlternateBufferState, CursorShape};

#[test]
fn microsoft_screen_buffer_single_alternate_buffer_creation_contract() {
    let mut state = AlternateBufferState::new();
    assert!(!state.is_alternate_active());
    assert!(state.alternate().is_none());
    state.use_alternate();
    assert!(state.is_alternate_active());
    assert!(state.alternate().is_some());
    state.use_main();
    assert!(!state.is_alternate_active());
    assert!(state.alternate().is_none());
}

#[test]
fn microsoft_screen_buffer_multiple_alternate_buffer_creation_contract() {
    let mut state = AlternateBufferState::new();
    state.use_alternate();
    assert_eq!(state.generation(), 1);
    state.active_mut().cursor.x = 9;
    state.use_alternate();
    assert_eq!(state.generation(), 2);
    assert_eq!(state.alternate().unwrap().cursor.x, 9);
    state.use_main();
    assert!(!state.is_alternate_active());
}

#[test]
fn microsoft_screen_buffer_multiple_alternates_from_main_contract() {
    let mut state = AlternateBufferState::new();
    state.use_alternate();
    state.use_main();
    state.main_mut().cursor.x = 7;
    state.use_alternate();
    assert_eq!(state.generation(), 2);
    assert_eq!(state.alternate().unwrap().cursor.x, 7);
}

#[test]
fn microsoft_screen_buffer_alternate_cursor_inheritance_contract() {
    let mut state = AlternateBufferState::new();
    let main = &mut state.main_mut().cursor;
    main.x = 3;
    main.y = 5;
    main.visible = false;
    main.size = 33;
    main.shape = CursorShape::DoubleUnderscore;
    main.blinking = false;

    state.use_alternate();
    assert_eq!(state.alternate().unwrap().cursor, state.main().cursor);
    {
        let alt = &mut state.active_mut().cursor;
        alt.x = 5;
        alt.y = 3;
        alt.visible = true;
        alt.size = 66;
        alt.shape = CursorShape::EmptyBox;
        alt.blinking = true;
    }
    state.use_main();
    assert_eq!((state.main().cursor.x, state.main().cursor.y), (3, 5));
    assert!(state.main().cursor.visible);
    assert_eq!(state.main().cursor.size, 66);
    assert_eq!(state.main().cursor.shape, CursorShape::EmptyBox);
    assert!(state.main().cursor.blinking);
}

#[test]
fn microsoft_screen_buffer_alt_buffer_cursor_state_contract() {
    let mut state = AlternateBufferState::new();
    state.main_mut().cursor.size = 47;
    state.main_mut().cursor.shape = CursorShape::DoubleUnderscore;
    state.use_alternate();
    let alt = state.alternate().unwrap();
    assert_eq!(alt.cursor.size, 47);
    assert_eq!(alt.cursor.shape, CursorShape::DoubleUnderscore);
}

#[test]
fn microsoft_screen_buffer_alt_buffer_vt_dispatching_contract() {
    let mut state = AlternateBufferState::new();
    state.use_alternate();
    state.dispatch_vt("\u{1b}[5;6H");
    assert_eq!((state.main().cursor.x, state.main().cursor.y), (0, 0));
    assert_eq!((state.alternate().unwrap().cursor.x, state.alternate().unwrap().cursor.y), (5, 4));
    state.dispatch_vt("\u{1b}[48;2;255;0;255m");
    assert!(!state.main().magenta_background);
    assert!(state.alternate().unwrap().magenta_background);
    state.dispatch_vt("X");
    assert!(state.main().text.is_empty());
    assert_eq!(state.alternate().unwrap().text, "X");
    assert_eq!(state.alternate().unwrap().cursor.x, 6);
}

#[test]
fn microsoft_screen_buffer_alt_buffer_ris_contract() {
    let mut state = AlternateBufferState::new();
    state.use_alternate();
    assert!(state.is_alternate_active());
    state.ris();
    assert!(!state.is_alternate_active());
}
