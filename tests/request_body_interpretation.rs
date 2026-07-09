use std::collections::HashMap;

use openapi_terminal_app::request::gather_request_inputs;
use openapi_terminal_app::spec::Operation;

fn operation_with_request_body_flag(has_request_body: bool) -> Operation {
    Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    }
}

#[test]
fn reads_payload_index_zero_when_operation_declares_request_body() {
    let operation = operation_with_request_body_flag(true);
    let payload_inputs = HashMap::from([(0, "{\"name\":\"fido\"}".to_string())]);

    let inputs = gather_request_inputs(
        &operation,
        &HashMap::new(),
        &HashMap::new(),
        &[],
        &[],
        &payload_inputs,
    );

    assert_eq!(inputs.body, Some("{\"name\":\"fido\"}".to_string()));
}

#[test]
fn reads_payload_index_zero_when_operation_does_not_declare_request_body() {
    let operation = operation_with_request_body_flag(false);
    let payload_inputs = HashMap::from([(0, "raw payload text".to_string())]);

    let inputs = gather_request_inputs(
        &operation,
        &HashMap::new(),
        &HashMap::new(),
        &[],
        &[],
        &payload_inputs,
    );

    assert_eq!(inputs.body, Some("raw payload text".to_string()));
}

#[test]
fn returns_no_body_when_payload_index_zero_is_absent() {
    let operation = operation_with_request_body_flag(false);

    let inputs = gather_request_inputs(
        &operation,
        &HashMap::new(),
        &HashMap::new(),
        &[],
        &[],
        &HashMap::new(),
    );

    assert_eq!(inputs.body, None);
}
