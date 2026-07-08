use openapi_terminal_app::auth::split_credential_pair;

#[test]
fn splits_username_and_password_on_colon() {
    assert_eq!(
        split_credential_pair("alice:hunter2"),
        ("alice".to_string(), "hunter2".to_string())
    );
}

#[test]
fn returns_empty_second_half_when_no_colon_exists() {
    assert_eq!(
        split_credential_pair("justauser"),
        ("justauser".to_string(), "".to_string())
    );
}

#[test]
fn returns_two_empty_strings_for_empty_input() {
    assert_eq!(
        split_credential_pair(""),
        ("".to_string(), "".to_string())
    );
}

#[test]
fn splits_only_on_the_first_colon() {
    assert_eq!(
        split_credential_pair("a:b:c"),
        ("a".to_string(), "b:c".to_string())
    );
}

#[test]
fn allows_empty_first_half_when_colon_starts_input() {
    assert_eq!(
        split_credential_pair(":secret"),
        ("".to_string(), "secret".to_string())
    );
}
