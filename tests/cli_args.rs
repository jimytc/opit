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
