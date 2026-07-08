use openapi_terminal_app::spec::Operation;

#[test]
fn filtered_operations_with_empty_filter_returns_all_operations_in_original_order() {
    let operations = vec![
        operation("GET", "/pets", Some("List all pets")),
        operation("POST", "/pets", Some("Create a pet")),
        operation("DELETE", "/pets/{petId}", None),
    ];

    let filtered = openapi_terminal_app::ui::endpoint_list::filtered_operations(&operations, "");

    assert_eq!(filtered.len(), 3);
    assert!(std::ptr::eq(filtered[0], &operations[0]));
    assert!(std::ptr::eq(filtered[1], &operations[1]));
    assert!(std::ptr::eq(filtered[2], &operations[2]));
}

#[test]
fn filtered_operations_matches_path_case_insensitively() {
    let operations = vec![
        operation("GET", "/Pets", Some("List all animals")),
        operation("GET", "/users", Some("List all users")),
    ];

    let filtered = openapi_terminal_app::ui::endpoint_list::filtered_operations(&operations, "pet");

    assert_eq!(filtered.len(), 1);
    assert!(std::ptr::eq(filtered[0], &operations[0]));
}

#[test]
fn filtered_operations_matches_summary_case_insensitively() {
    let operations = vec![
        operation("GET", "/pets", Some("List all pets")),
        operation("POST", "/pets", Some("Create a pet")),
    ];

    let filtered =
        openapi_terminal_app::ui::endpoint_list::filtered_operations(&operations, "LIST");

    assert_eq!(filtered.len(), 1);
    assert!(std::ptr::eq(filtered[0], &operations[0]));
}

#[test]
fn filtered_operations_excludes_non_matching_operations() {
    let operations = vec![
        operation("GET", "/pets", Some("List all pets")),
        operation("POST", "/users", Some("Create a user")),
        operation("DELETE", "/orders/{orderId}", Some("Delete an order")),
    ];

    let filtered =
        openapi_terminal_app::ui::endpoint_list::filtered_operations(&operations, "user");

    assert_eq!(filtered.len(), 1);
    assert!(std::ptr::eq(filtered[0], &operations[1]));
}

#[test]
fn filtered_operations_does_not_panic_when_summary_is_none() {
    let operations = vec![
        operation("GET", "/health", None),
        operation("GET", "/pets", Some("List all pets")),
    ];

    let filtered =
        openapi_terminal_app::ui::endpoint_list::filtered_operations(&operations, "health");

    assert_eq!(filtered.len(), 1);
    assert!(std::ptr::eq(filtered[0], &operations[0]));
}

#[test]
fn filtered_operations_preserves_relative_order_for_non_contiguous_matches() {
    let operations = vec![
        operation("GET", "/pets", Some("List all pets")),
        operation("POST", "/users", Some("Create a user")),
        operation("DELETE", "/pets/{petId}", Some("Delete a pet")),
    ];

    let filtered = openapi_terminal_app::ui::endpoint_list::filtered_operations(&operations, "pet");

    assert_eq!(filtered.len(), 2);
    assert!(std::ptr::eq(filtered[0], &operations[0]));
    assert!(std::ptr::eq(filtered[1], &operations[2]));
}

#[test]
fn filtered_operations_groups_by_first_tag_in_first_appearance_order() {
    let operations = vec![
        operation_with_tags("GET", "/zebra/one", None, vec!["Zebra"]),
        operation_with_tags("GET", "/apple/one", None, vec!["Apple"]),
        operation_with_tags("POST", "/zebra/two", None, vec!["Zebra"]),
        operation_with_tags("GET", "/status", None, vec![]),
    ];

    let filtered = openapi_terminal_app::ui::endpoint_list::filtered_operations(&operations, "");

    assert_eq!(filtered.len(), 4);
    assert!(std::ptr::eq(filtered[0], &operations[0]));
    assert!(std::ptr::eq(filtered[1], &operations[2]));
    assert!(std::ptr::eq(filtered[2], &operations[1]));
    assert!(std::ptr::eq(filtered[3], &operations[3]));
}

#[test]
fn filtered_operations_preserves_relative_order_within_a_tag_group() {
    let operations = vec![
        operation_with_tags("GET", "/pets/first", None, vec!["Pets"]),
        operation_with_tags("POST", "/pets/second", None, vec!["Pets"]),
    ];

    let filtered = openapi_terminal_app::ui::endpoint_list::filtered_operations(&operations, "");

    assert_eq!(filtered.len(), 2);
    assert!(std::ptr::eq(filtered[0], &operations[0]));
    assert!(std::ptr::eq(filtered[1], &operations[1]));
}

fn operation(method: &str, path: &str, summary: Option<&str>) -> Operation {
    operation_with_tags(method, path, summary, vec![])
}

fn operation_with_tags(
    method: &str,
    path: &str,
    summary: Option<&str>,
    tags: Vec<&str>,
) -> Operation {
    Operation {
        path: path.to_string(),
        method: method.to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: summary.map(str::to_string),
        request_body_example: None,
        tags: tags.into_iter().map(str::to_string).collect(),
    }
}
