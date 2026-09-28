use terminal_host::console_argument_parser::{ConsoleArgumentError, parse_console_arguments};

fn tokens(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn explicit_separator_alone_is_a_successful_empty_client_commandline() {
    let parsed = parse_console_arguments(&tokens(&["--"])).expect("separator-only command line");
    assert!(parsed.client_commandline.is_empty());
}

#[test]
fn first_unknown_token_owns_the_remaining_client_commandline() {
    let parsed = parse_console_arguments(&tokens(&[
        "console",
        "--vtmode",
        "foo",
        "--outpipe",
        "bar",
        "--",
        "baz",
    ]))
    .expect("implicit client command line");

    assert_eq!(
        parsed.client_commandline,
        "console --vtmode foo --outpipe bar -- baz"
    );
}

#[test]
fn explicit_separator_prevents_following_tokens_from_becoming_host_switches() {
    let parsed = parse_console_arguments(&tokens(&["--", "--headless", "cmd.exe"]))
        .expect("explicit client command line");

    assert!(!parsed.headless());
    assert_eq!(parsed.client_commandline, "--headless cmd.exe");
}

#[test]
fn duplicate_signal_handle_fails_closed() {
    assert_eq!(
        parse_console_arguments(&tokens(&["--signal", "0x10", "--signal", "0x20"])),
        Err(ConsoleArgumentError::DuplicateHandle("signal"))
    );
}

#[test]
fn dimensions_preserve_cpp_lower_bound_cast_behavior() {
    let parsed = parse_console_arguments(&tokens(&["--width", "-32769", "--height", "-65536"]))
        .expect("C++ short cast compatibility");

    assert_eq!(parsed.width, 32767);
    assert_eq!(parsed.height, 0);
}

#[test]
fn malformed_dimension_and_unknown_feature_fail_closed() {
    assert_eq!(
        parse_console_arguments(&tokens(&["--height", "8foo"])),
        Err(ConsoleArgumentError::InvalidValue("dimension"))
    );
    assert_eq!(
        parse_console_arguments(&tokens(&["--feature", "future"])),
        Err(ConsoleArgumentError::InvalidValue("feature"))
    );
}
