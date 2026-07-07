use std::path::PathBuf;

use clap::Parser;
use openapi_terminal_app::cli::Cli;

#[test]
fn parses_required_spec_path_positional_argument() {
    let cli = Cli::try_parse_from(["openapi-terminal-app", "./spec.json"]).unwrap();

    assert_eq!(cli.spec_path, PathBuf::from("./spec.json"));
}

#[test]
fn returns_error_when_spec_path_is_missing() {
    let result = Cli::try_parse_from(["openapi-terminal-app"]);

    assert!(result.is_err());
}

#[test]
fn defaults_bearer_token_and_header_when_absent() {
    let cli = Cli::try_parse_from(["openapi-terminal-app", "./spec.json"]).unwrap();

    assert_eq!(cli.bearer_token, None);
    assert_eq!(cli.header, Vec::<String>::new());
}

#[test]
fn parses_bearer_token_flag() {
    let cli = Cli::try_parse_from([
        "openapi-terminal-app",
        "./spec.json",
        "--bearer-token",
        "abc123",
    ])
    .unwrap();

    assert_eq!(cli.bearer_token, Some("abc123".to_string()));
}

#[test]
fn parses_repeated_header_flags() {
    let cli = Cli::try_parse_from([
        "openapi-terminal-app",
        "./spec.json",
        "--header",
        "X-Foo=bar",
        "--header",
        "X-Baz=qux",
    ])
    .unwrap();

    assert_eq!(
        cli.header,
        vec!["X-Foo=bar".to_string(), "X-Baz=qux".to_string()]
    );
}
