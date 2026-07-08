use openapi_terminal_app::request::pretty_print_if_json;

#[test]
fn pretty_prints_compact_single_key_json_object() {
    let expected = serde_json::to_string_pretty(&serde_json::json!({ "ok": true })).unwrap();

    let formatted = pretty_print_if_json("{\"ok\":true}");

    assert_eq!(formatted, expected);
}

#[test]
fn pretty_prints_compact_json_array() {
    let expected = serde_json::to_string_pretty(&serde_json::json!([1, 2, 3])).unwrap();

    let formatted = pretty_print_if_json("[1,2,3]");

    assert_eq!(formatted, expected);
}

#[test]
fn returns_non_json_string_unchanged() {
    let body = "not json at all";

    let formatted = pretty_print_if_json(body);

    assert_eq!(formatted, body);
}

#[test]
fn returns_empty_string_unchanged() {
    let body = "";

    let formatted = pretty_print_if_json(body);

    assert_eq!(formatted, body);
}

#[test]
fn pretty_printing_already_pretty_json_is_idempotent() {
    let pretty_json = serde_json::to_string_pretty(&serde_json::json!({ "ok": true })).unwrap();

    let formatted = pretty_print_if_json(&pretty_json);

    assert_eq!(formatted, pretty_json);
}
