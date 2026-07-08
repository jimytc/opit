use std::collections::HashMap;

use openapi_terminal_app::auth::Credential;
use openapi_terminal_app::request::build_preview;
use openapi_terminal_app::spec::{Operation, Parameter, SecurityScheme, SecuritySchemeKind};

fn get_pets_operation() -> Operation {
    Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
    }
}

#[test]
fn build_preview_replaces_path_parameters_from_indexed_inputs() {
    let operation = Operation {
        path: "/pets/{petId}".to_string(),
        method: "GET".to_string(),
        parameters: vec![Parameter {
            name: "petId".to_string(),
            location: "path".to_string(),
            required: true,
        }],
        has_request_body: false,
        request_body_media_type: None,
    };
    let param_inputs = HashMap::from([(0, "123".to_string())]);
    let auth_inputs = HashMap::new();

    let request = build_preview(
        "https://api.example.com",
        &operation,
        &param_inputs,
        &[],
        &auth_inputs,
        &[],
    );

    assert_eq!(request.url, "https://api.example.com/pets/123");
    assert_eq!(request.body, None);
}

#[test]
fn build_preview_stores_raw_body_from_body_row() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: None,
    };
    let param_inputs = HashMap::from([(0, "{\"name\":\"fido\"}".to_string())]);
    let auth_inputs = HashMap::new();

    let request = build_preview(
        "https://api.example.com",
        &operation,
        &param_inputs,
        &[],
        &auth_inputs,
        &[],
    );

    assert_eq!(request.body, Some("{\"name\":\"fido\"}".to_string()));
}

#[test]
fn build_preview_adds_content_type_header_when_body_value_is_present() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: Some("application/json".to_string()),
    };
    let param_inputs = HashMap::from([(0, "{\"name\":\"fido\"}".to_string())]);

    let request = build_preview(
        "https://api.example.com",
        &operation,
        &param_inputs,
        &[],
        &HashMap::new(),
        &[],
    );

    assert!(request.headers.contains(&(
        "Content-Type".to_string(),
        "application/json".to_string()
    )));
}

#[test]
fn build_preview_omits_content_type_header_when_body_value_is_absent() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: Some("application/json".to_string()),
    };
    let param_inputs = HashMap::new();

    let request = build_preview(
        "https://api.example.com",
        &operation,
        &param_inputs,
        &[],
        &HashMap::new(),
        &[],
    );

    assert!(!request
        .headers
        .iter()
        .any(|(name, _value)| name == "Content-Type"));
}

#[test]
fn build_preview_applies_cli_sourced_credentials() {
    let operation = get_pets_operation();
    let param_inputs = HashMap::new();
    let auth_inputs = HashMap::new();
    let cli_credentials = vec![Credential::Bearer {
        token: "cli-token".to_string(),
    }];

    let request = build_preview(
        "https://api.example.com",
        &operation,
        &param_inputs,
        &[],
        &auth_inputs,
        &cli_credentials,
    );

    assert!(request.headers.contains(&(
        "Authorization".to_string(),
        "Bearer cli-token".to_string()
    )));
}

#[test]
fn build_preview_applies_interactively_entered_auth_credentials() {
    let operation = get_pets_operation();
    let param_inputs = HashMap::new();
    let security_schemes = vec![SecurityScheme {
        name: "apiKeyAuth".to_string(),
        kind: SecuritySchemeKind::ApiKey {
            location: "header".to_string(),
            param_name: "X-API-Key".to_string(),
        },
    }];
    let auth_inputs = HashMap::from([(0, "secret123".to_string())]);

    let request = build_preview(
        "https://api.example.com",
        &operation,
        &param_inputs,
        &security_schemes,
        &auth_inputs,
        &[],
    );

    assert!(request
        .headers
        .contains(&("X-API-Key".to_string(), "secret123".to_string())));
}

#[test]
fn build_preview_combines_cli_and_interactive_credentials() {
    let operation = get_pets_operation();
    let param_inputs = HashMap::new();
    let security_schemes = vec![SecurityScheme {
        name: "apiKeyAuth".to_string(),
        kind: SecuritySchemeKind::ApiKey {
            location: "header".to_string(),
            param_name: "X-API-Key".to_string(),
        },
    }];
    let auth_inputs = HashMap::from([(0, "secret123".to_string())]);
    let cli_credentials = vec![Credential::Bearer {
        token: "cli-token".to_string(),
    }];

    let request = build_preview(
        "https://api.example.com",
        &operation,
        &param_inputs,
        &security_schemes,
        &auth_inputs,
        &cli_credentials,
    );

    assert!(request.headers.contains(&(
        "Authorization".to_string(),
        "Bearer cli-token".to_string()
    )));
    assert!(request
        .headers
        .contains(&("X-API-Key".to_string(), "secret123".to_string())));
}
