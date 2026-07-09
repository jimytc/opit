use openapi_terminal_app::spec::{Operation, Parameter};

#[test]
fn operation_exposes_parameters_from_openapi_operation() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets/{petId}": {
          "get": {
            "operationId": "getPet",
            "parameters": [
              {
                "name": "petId",
                "in": "path",
                "required": true,
                "schema": {
                  "type": "string"
                }
              },
              {
                "name": "limit",
                "in": "query",
                "required": false,
                "schema": {
                  "type": "integer"
                }
              },
              {
                "name": "X-Request-Id",
                "in": "header",
                "required": false,
                "schema": {
                  "type": "string"
                }
              }
            ],
            "responses": {
              "200": {
                "description": "A pet"
              }
            }
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");
    let operations = spec.operations();

    assert_eq!(operations.len(), 1);

    let operation = &operations[0];

    assert_eq!(operation.parameters.len(), 3);
    assert!(operation
        .parameters
        .iter()
        .any(|parameter| parameter.name == "petId"
            && parameter.location == "path"
            && parameter.required));
    assert!(operation
        .parameters
        .iter()
        .any(|parameter| parameter.name == "limit"
            && parameter.location == "query"
            && !parameter.required));
    assert!(operation
        .parameters
        .iter()
        .any(|parameter| parameter.name == "X-Request-Id"
            && parameter.location == "header"
            && !parameter.required));
}

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

fn parameter(name: &str, location: &str) -> Parameter {
    Parameter {
        name: name.to_string(),
        location: location.to_string(),
        required: false,
    }
}

fn parameter_names(parameters: Vec<&Parameter>) -> Vec<&str> {
    parameters
        .iter()
        .map(|parameter| parameter.name.as_str())
        .collect()
}

#[test]
fn operation_splits_header_and_non_header_parameters_preserving_relative_order() {
    let operation = operation_with_parameters(vec![
        parameter("petId", "path"),
        parameter("x-request-id", "header"),
        parameter("limit", "query"),
        parameter("x-client-version", "header"),
        parameter("session", "cookie"),
    ]);

    assert_eq!(
        parameter_names(operation.header_parameters()),
        vec!["x-request-id", "x-client-version"]
    );
    assert_eq!(
        parameter_names(operation.non_header_parameters()),
        vec!["petId", "limit", "session"]
    );
}

#[test]
fn operation_with_no_parameters_returns_empty_parameter_groups() {
    let operation = operation_with_parameters(vec![]);

    assert_eq!(parameter_names(operation.header_parameters()), Vec::<&str>::new());
    assert_eq!(
        parameter_names(operation.non_header_parameters()),
        Vec::<&str>::new()
    );
}

#[test]
fn operation_with_only_header_parameters_returns_all_headers_and_no_non_headers() {
    let operation = operation_with_parameters(vec![
        parameter("x-request-id", "header"),
        parameter("x-client-version", "header"),
    ]);

    assert_eq!(
        parameter_names(operation.header_parameters()),
        vec!["x-request-id", "x-client-version"]
    );
    assert_eq!(
        parameter_names(operation.non_header_parameters()),
        Vec::<&str>::new()
    );
}

#[test]
fn operation_with_only_non_header_parameters_returns_no_headers_and_all_non_headers() {
    let operation = operation_with_parameters(vec![
        parameter("petId", "path"),
        parameter("limit", "query"),
        parameter("session", "cookie"),
    ]);

    assert_eq!(parameter_names(operation.header_parameters()), Vec::<&str>::new());
    assert_eq!(
        parameter_names(operation.non_header_parameters()),
        vec!["petId", "limit", "session"]
    );
}
