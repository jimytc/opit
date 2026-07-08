use std::collections::HashMap;

use openapi_terminal_app::request::body_from_inputs;
use openapi_terminal_app::spec::{Operation, Parameter};

fn operation_with_two_parameters(has_request_body: bool) -> Operation {
    Operation {
        path: "/pets/{petId}".to_string(),
        method: "POST".to_string(),
        parameters: vec![
            Parameter {
                name: "petId".to_string(),
                location: "path".to_string(),
                required: true,
            },
            Parameter {
                name: "X-Request-Id".to_string(),
                location: "header".to_string(),
                required: false,
            },
        ],
        has_request_body,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    }
}

#[test]
fn returns_none_when_operation_has_no_request_body_even_if_body_row_input_exists() {
    let operation = operation_with_two_parameters(false);
    let inputs = HashMap::from([(2, "{\"name\":\"fido\"}".to_string())]);

    let body = body_from_inputs(&operation, &inputs);

    assert_eq!(body, None);
}

#[test]
fn returns_none_when_body_row_input_is_absent() {
    let operation = operation_with_two_parameters(true);
    let inputs = HashMap::new();

    let body = body_from_inputs(&operation, &inputs);

    assert_eq!(body, None);
}

#[test]
fn returns_body_row_input_when_request_body_exists() {
    let operation = operation_with_two_parameters(true);
    let inputs = HashMap::from([(2, "{\"name\":\"fido\"}".to_string())]);

    let body = body_from_inputs(&operation, &inputs);

    assert_eq!(body, Some("{\"name\":\"fido\"}".to_string()));
}

#[test]
fn uses_row_zero_as_body_row_when_operation_has_no_parameters() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };
    let inputs = HashMap::from([(0, "raw body text".to_string())]);

    let body = body_from_inputs(&operation, &inputs);

    assert_eq!(body, Some("raw body text".to_string()));
}
