use std::collections::HashMap;

use openapi_terminal_app::auth::{credentials_from_inputs, Credential};
use openapi_terminal_app::spec::{SecurityScheme, SecuritySchemeKind};

#[test]
fn api_key_scheme_produces_api_key_credential() {
    let schemes = vec![SecurityScheme {
        name: "apiKeyAuth".to_string(),
        kind: SecuritySchemeKind::ApiKey {
            location: "header".to_string(),
            param_name: "X-API-Key".to_string(),
        },
    }];
    let inputs = HashMap::from([(0, "secret123".to_string())]);

    let result = credentials_from_inputs(&schemes, &inputs);

    assert_eq!(result.len(), 1);
    assert!(matches!(
        &result[0],
        Credential::ApiKey { location, param_name, value }
            if location == "header" && param_name == "X-API-Key" && value == "secret123"
    ));
}

#[test]
fn bearer_scheme_produces_bearer_credential() {
    let schemes = vec![SecurityScheme {
        name: "bearerAuth".to_string(),
        kind: SecuritySchemeKind::Http {
            scheme: "bearer".to_string(),
        },
    }];
    let inputs = HashMap::from([(0, "tok-abc".to_string())]);

    let result = credentials_from_inputs(&schemes, &inputs);

    assert_eq!(result.len(), 1);
    assert!(matches!(&result[0], Credential::Bearer { token } if token == "tok-abc"));
}

#[test]
fn basic_scheme_with_colon_splits_username_and_password() {
    let schemes = vec![SecurityScheme {
        name: "basicAuth".to_string(),
        kind: SecuritySchemeKind::Http {
            scheme: "basic".to_string(),
        },
    }];
    let inputs = HashMap::from([(0, "alice:hunter2".to_string())]);

    let result = credentials_from_inputs(&schemes, &inputs);

    assert_eq!(result.len(), 1);
    assert!(matches!(
        &result[0],
        Credential::Basic { username, password }
            if username == "alice" && password == "hunter2"
    ));
}

#[test]
fn basic_scheme_without_colon_uses_whole_string_as_username() {
    let schemes = vec![SecurityScheme {
        name: "basicAuth".to_string(),
        kind: SecuritySchemeKind::Http {
            scheme: "basic".to_string(),
        },
    }];
    let inputs = HashMap::from([(0, "justauser".to_string())]);

    let result = credentials_from_inputs(&schemes, &inputs);

    assert_eq!(result.len(), 1);
    assert!(matches!(
        &result[0],
        Credential::Basic { username, password }
            if username == "justauser" && password.is_empty()
    ));
}

#[test]
fn oauth2_scheme_produces_no_credential_even_with_input() {
    let schemes = vec![SecurityScheme {
        name: "oauth2Auth".to_string(),
        kind: SecuritySchemeKind::OAuth2 {
            token_url: Some("https://auth.example.com/token".to_string()),
        },
    }];
    let inputs = HashMap::from([(0, "irrelevant".to_string())]);

    let result = credentials_from_inputs(&schemes, &inputs);

    assert!(result.is_empty());
}

#[test]
fn open_id_connect_scheme_produces_no_credential_even_with_input() {
    let schemes = vec![SecurityScheme {
        name: "oidcAuth".to_string(),
        kind: SecuritySchemeKind::OpenIdConnect,
    }];
    let inputs = HashMap::from([(0, "irrelevant".to_string())]);

    let result = credentials_from_inputs(&schemes, &inputs);

    assert!(result.is_empty());
}

#[test]
fn row_without_input_contributes_nothing() {
    let schemes = vec![
        SecurityScheme {
            name: "apiKeyAuth".to_string(),
            kind: SecuritySchemeKind::ApiKey {
                location: "header".to_string(),
                param_name: "X-API-Key".to_string(),
            },
        },
        SecurityScheme {
            name: "bearerAuth".to_string(),
            kind: SecuritySchemeKind::Http {
                scheme: "bearer".to_string(),
            },
        },
    ];
    let inputs = HashMap::from([(0, "secret123".to_string())]);

    let result = credentials_from_inputs(&schemes, &inputs);

    assert_eq!(result.len(), 1);
    assert!(matches!(&result[0], Credential::ApiKey { .. }));
}
