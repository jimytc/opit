use std::collections::HashMap;

use openapi_terminal_app::request::param_values_from_inputs;
use openapi_terminal_app::spec::{Operation, Parameter};

fn operation_with_three_parameters() -> Operation {
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
                name: "limit".to_string(),
                location: "query".to_string(),
                required: false,
            },
            Parameter {
                name: "X-Request-Id".to_string(),
                location: "header".to_string(),
                required: false,
            },
        ],
        has_request_body: false,
        request_body_media_type: None,
    }
}

#[test]
fn maps_present_parameter_rows_by_parameter_name() {
    let operation = operation_with_three_parameters();
    let inputs = HashMap::from([(0, "123".to_string()), (2, "abc".to_string())]);

    let param_values = param_values_from_inputs(&operation, &inputs);

    assert_eq!(param_values.len(), 2);
    assert_eq!(param_values.get("petId"), Some(&"123".to_string()));
    assert_eq!(param_values.get("X-Request-Id"), Some(&"abc".to_string()));
    assert!(!param_values.contains_key("limit"));
}

#[test]
fn ignores_body_row_input_that_is_not_a_parameter() {
    let operation = operation_with_three_parameters();
    let inputs = HashMap::from([(3, "leaked body value".to_string())]);

    let param_values = param_values_from_inputs(&operation, &inputs);

    assert!(param_values.is_empty());
}

#[test]
fn returns_empty_map_when_operation_has_no_parameters() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: None,
    };
    let inputs = HashMap::from([(0, "raw body text".to_string())]);

    let param_values = param_values_from_inputs(&operation, &inputs);

    assert!(param_values.is_empty());
}
