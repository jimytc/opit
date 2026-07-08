use openapi_terminal_app::auth::oauth2::build_token_caches;
use openapi_terminal_app::spec::{SecurityScheme, SecuritySchemeKind};

#[test]
fn builds_token_caches_for_oauth2_schemes_with_token_urls() {
    let schemes = vec![
        SecurityScheme {
            name: "api_key".to_string(),
            kind: SecuritySchemeKind::ApiKey {
                location: "header".to_string(),
                param_name: "X-API-Key".to_string(),
            },
        },
        SecurityScheme {
            name: "oauth_a".to_string(),
            kind: SecuritySchemeKind::OAuth2 {
                token_url: Some("https://a.example.com/token".to_string()),
            },
        },
        SecurityScheme {
            name: "bearer".to_string(),
            kind: SecuritySchemeKind::Http {
                scheme: "bearer".to_string(),
            },
        },
        SecurityScheme {
            name: "oauth_b".to_string(),
            kind: SecuritySchemeKind::OAuth2 {
                token_url: Some("https://b.example.com/token".to_string()),
            },
        },
    ];

    let map = build_token_caches(&schemes);

    let mut keys: Vec<_> = map.keys().copied().collect();
    keys.sort();
    assert_eq!(keys, vec![1, 3]);
}

#[test]
fn skips_oauth2_schemes_without_token_urls_and_openid_connect_schemes() {
    let schemes = vec![
        SecurityScheme {
            name: "oauth_without_client_credentials".to_string(),
            kind: SecuritySchemeKind::OAuth2 { token_url: None },
        },
        SecurityScheme {
            name: "openid".to_string(),
            kind: SecuritySchemeKind::OpenIdConnect,
        },
    ];

    let map = build_token_caches(&schemes);

    assert!(map.is_empty());
}

#[test]
fn empty_schemes_produce_no_token_caches() {
    let schemes: Vec<SecurityScheme> = Vec::new();

    let map = build_token_caches(&schemes);

    assert!(map.is_empty());
}
