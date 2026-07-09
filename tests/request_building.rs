use std::collections::HashMap;

use openapi_terminal_app::request::{build, RequestInputs};
use openapi_terminal_app::spec::{Operation, Parameter};

#[test]
fn build_replaces_path_parameters_and_adds_query_and_header_values() {
    let operation = Operation {
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
        summary: None,
        request_body_example: None,
        tags: vec![],
    };
    let param_values = HashMap::from([
        ("petId".to_string(), "123".to_string()),
        ("limit".to_string(), "10".to_string()),
        ("X-Request-Id".to_string(), "abc".to_string()),
    ]);
    let inputs = RequestInputs {
        param_values,
        extra_headers: vec![],
        extra_query: vec![],
        body: None,
    };

    let request = build("https://api.example.com", &operation, &inputs);

    assert_eq!(request.method, "GET");
    assert_eq!(request.url, "https://api.example.com/pets/123?limit=10");
    assert_eq!(
        request.headers,
        vec![("X-Request-Id".to_string(), "abc".to_string())]
    );
}

#[test]
fn build_adds_cookie_parameters_to_single_cookie_header() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![
            Parameter {
                name: "session".to_string(),
                location: "cookie".to_string(),
                required: false,
            },
            Parameter {
                name: "limit".to_string(),
                location: "query".to_string(),
                required: false,
            },
            Parameter {
                name: "theme".to_string(),
                location: "cookie".to_string(),
                required: false,
            },
        ],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };
    let param_values = HashMap::from([
        ("session".to_string(), "abc123".to_string()),
        ("limit".to_string(), "10".to_string()),
        ("theme".to_string(), "dark".to_string()),
    ]);
    let inputs = RequestInputs {
        param_values,
        extra_headers: vec![],
        extra_query: vec![],
        body: None,
    };

    let request = build("https://api.example.com", &operation, &inputs);

    assert_eq!(request.url, "https://api.example.com/pets?limit=10");
    assert_eq!(
        request.headers,
        vec![(
            "Cookie".to_string(),
            "session=abc123; theme=dark".to_string()
        )]
    );
}

#[test]
fn build_without_parameters_uses_method_and_base_url_plus_path() {
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
    let inputs = RequestInputs {
        param_values: HashMap::new(),
        extra_headers: vec![],
        extra_query: vec![],
        body: None,
    };

    let request = build("https://api.example.com", &operation, &inputs);

    assert_eq!(request.method, "POST");
    assert_eq!(request.url, "https://api.example.com/pets");
    assert!(request.headers.is_empty());
}

#[test]
fn build_appends_extra_headers_and_extra_query_params() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![
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
        summary: None,
        request_body_example: None,
        tags: vec![],
    };
    let inputs = RequestInputs {
        param_values: HashMap::from([
            ("limit".to_string(), "10".to_string()),
            ("X-Request-Id".to_string(), "abc".to_string()),
        ]),
        extra_headers: vec![("X-Debug".to_string(), "enabled".to_string())],
        extra_query: vec![("debug".to_string(), "true".to_string())],
        body: None,
    };

    let request = build("https://api.example.com", &operation, &inputs);

    assert!(request
        .headers
        .contains(&("X-Request-Id".to_string(), "abc".to_string())));
    assert!(request
        .headers
        .contains(&("X-Debug".to_string(), "enabled".to_string())));
    assert!(request.url.starts_with("https://api.example.com/pets?"));
    assert!(request.url.contains("limit=10"));
    assert!(request.url.contains("debug=true"));
}
