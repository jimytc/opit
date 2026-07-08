use std::collections::HashMap;

use openapi_terminal_app::request::missing_required_params;
use openapi_terminal_app::spec::{Operation, Parameter};

fn operation_with_parameters(parameters: Vec<Parameter>) -> Operation {
    Operation {
        path: "/pets/{petId}".to_string(),
        method: "GET".to_string(),
        parameters,
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    }
}

#[test]
fn reports_required_parameter_missing_when_only_optional_parameter_has_input() {
    let operation = operation_with_parameters(vec![
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
    ]);
    let inputs = HashMap::from([(1, "10".to_string())]);

    let missing = missing_required_params(&operation, &inputs);

    assert_eq!(missing, vec!["petId".to_string()]);
}

#[test]
fn reports_required_parameter_missing_when_committed_value_is_empty() {
    let operation = operation_with_parameters(vec![Parameter {
        name: "petId".to_string(),
        location: "path".to_string(),
        required: true,
    }]);
    let inputs = HashMap::from([(0, String::new())]);

    let missing = missing_required_params(&operation, &inputs);

    assert_eq!(missing, vec!["petId".to_string()]);
}

#[test]
fn returns_empty_vec_when_required_parameter_has_non_empty_input() {
    let operation = operation_with_parameters(vec![Parameter {
        name: "petId".to_string(),
        location: "path".to_string(),
        required: true,
    }]);
    let inputs = HashMap::from([(0, "123".to_string())]);

    let missing = missing_required_params(&operation, &inputs);

    assert!(missing.is_empty());
}

#[test]
fn reports_multiple_missing_required_parameters_in_operation_order() {
    let operation = operation_with_parameters(vec![
        Parameter {
            name: "petId".to_string(),
            location: "path".to_string(),
            required: true,
        },
        Parameter {
            name: "ownerId".to_string(),
            location: "query".to_string(),
            required: true,
        },
    ]);
    let inputs = HashMap::new();

    let missing = missing_required_params(&operation, &inputs);

    assert_eq!(missing, vec!["petId".to_string(), "ownerId".to_string()]);
}

#[test]
fn returns_empty_vec_when_operation_has_no_parameters() {
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
    let inputs = HashMap::new();

    let missing = missing_required_params(&operation, &inputs);

    assert!(missing.is_empty());
}
