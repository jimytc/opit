use std::collections::HashMap;

use openapi_terminal_app::request::gather_request_inputs;
use openapi_terminal_app::spec::{Operation, Parameter};

fn operation_with_split_parameters() -> Operation {
    Operation {
        path: "/pets/{petId}".to_string(),
        method: "GET".to_string(),
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
            Parameter {
                name: "limit".to_string(),
                location: "query".to_string(),
                required: false,
            },
        ],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    }
}

#[test]
fn maps_header_and_non_header_inputs_by_their_independent_row_indexes() {
    let operation = operation_with_split_parameters();
    let header_inputs = HashMap::from([(0, "abc".to_string())]);
    let parameter_inputs = HashMap::from([(0, "123".to_string()), (1, "10".to_string())]);

    let inputs = gather_request_inputs(
        &operation,
        &header_inputs,
        &parameter_inputs,
        &[],
        &[],
        &HashMap::new(),
    );

    assert_eq!(inputs.param_values.len(), 3);
    assert_eq!(
        inputs.param_values.get("X-Request-Id"),
        Some(&"abc".to_string())
    );
    assert_eq!(inputs.param_values.get("petId"), Some(&"123".to_string()));
    assert_eq!(inputs.param_values.get("limit"), Some(&"10".to_string()));
}

#[test]
fn omits_parameters_when_their_split_input_row_is_absent() {
    let operation = operation_with_split_parameters();
    let header_inputs = HashMap::new();
    let parameter_inputs = HashMap::from([(0, "123".to_string())]);

    let inputs = gather_request_inputs(
        &operation,
        &header_inputs,
        &parameter_inputs,
        &[],
        &[],
        &HashMap::new(),
    );

    assert_eq!(inputs.param_values.len(), 1);
    assert_eq!(inputs.param_values.get("petId"), Some(&"123".to_string()));
    assert!(!inputs.param_values.contains_key("X-Request-Id"));
    assert!(!inputs.param_values.contains_key("limit"));
}

#[test]
fn ignores_stray_parameter_inputs_when_operation_has_no_parameters() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };
    let header_inputs = HashMap::from([(0, "abc".to_string())]);
    let parameter_inputs = HashMap::from([(0, "123".to_string())]);

    let inputs = gather_request_inputs(
        &operation,
        &header_inputs,
        &parameter_inputs,
        &[],
        &[],
        &HashMap::new(),
    );

    assert!(inputs.param_values.is_empty());
}

#[test]
fn collects_custom_headers_after_spec_header_parameter_rows() {
    let operation = operation_with_split_parameters();
    let header_inputs = HashMap::from([(1, "beta".to_string())]);
    let custom_headers = vec!["X-Feature-Flag".to_string()];

    let inputs = gather_request_inputs(
        &operation,
        &header_inputs,
        &HashMap::new(),
        &custom_headers,
        &[],
        &HashMap::new(),
    );

    assert_eq!(
        inputs.extra_headers,
        vec![("X-Feature-Flag".to_string(), "beta".to_string())]
    );
}

#[test]
fn collects_custom_query_params_after_spec_non_header_parameter_rows() {
    let operation = operation_with_split_parameters();
    let parameter_inputs = HashMap::from([(2, "true".to_string())]);
    let custom_query_params = vec!["debug".to_string()];

    let inputs = gather_request_inputs(
        &operation,
        &HashMap::new(),
        &parameter_inputs,
        &[],
        &custom_query_params,
        &HashMap::new(),
    );

    assert_eq!(
        inputs.extra_query,
        vec![("debug".to_string(), "true".to_string())]
    );
}
