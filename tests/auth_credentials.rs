use openapi_terminal_app::auth::{apply, Credential};
use openapi_terminal_app::request::HttpRequest;

fn fresh_request() -> HttpRequest {
    HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/pets".to_string(),
        headers: vec![],
    }
}

#[test]
fn applies_header_api_key_credential() {
    let mut request = fresh_request();
    let credential = Credential::ApiKey {
        location: "header".to_string(),
        param_name: "X-API-Key".to_string(),
        value: "secret123".to_string(),
    };

    apply(&mut request, &credential);

    assert_eq!(
        request.headers,
        vec![("X-API-Key".to_string(), "secret123".to_string())]
    );
}

#[test]
fn applies_bearer_credential() {
    let mut request = fresh_request();
    let credential = Credential::Bearer {
        token: "abc123".to_string(),
    };

    apply(&mut request, &credential);

    assert_eq!(
        request.headers,
        vec![("Authorization".to_string(), "Bearer abc123".to_string())]
    );
}

#[test]
fn applies_basic_credential() {
    let mut request = fresh_request();
    let credential = Credential::Basic {
        username: "user".to_string(),
        password: "pass".to_string(),
    };

    apply(&mut request, &credential);

    assert_eq!(
        request.headers,
        vec![(
            "Authorization".to_string(),
            "Basic dXNlcjpwYXNz".to_string()
        )]
    );
}

#[test]
fn applies_raw_credential() {
    let mut request = fresh_request();
    let credential = Credential::Raw {
        header_name: "X-Custom".to_string(),
        header_value: "whatever".to_string(),
    };

    apply(&mut request, &credential);

    assert_eq!(
        request.headers,
        vec![("X-Custom".to_string(), "whatever".to_string())]
    );
}
