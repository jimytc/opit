use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use async_trait::async_trait;
use openapi_terminal_app::auth::oauth2::{
    build_token_caches, resolve_oauth2_credentials, Clock,
};
use openapi_terminal_app::auth::Credential;
use openapi_terminal_app::request::{HttpClient, HttpError, HttpRequest, HttpResponse};
use openapi_terminal_app::spec::{SecurityScheme, SecuritySchemeKind};

struct FakeHttpClient {
    call_count: Mutex<u32>,
    response_body: String,
    should_fail: bool,
}

impl FakeHttpClient {
    fn new(response_body: &str) -> Self {
        Self {
            call_count: Mutex::new(0),
            response_body: response_body.to_string(),
            should_fail: false,
        }
    }

    fn new_failing() -> Self {
        Self {
            call_count: Mutex::new(0),
            response_body: String::new(),
            should_fail: true,
        }
    }

    fn call_count(&self) -> u32 {
        *self.call_count.lock().expect("call count lock poisoned")
    }
}

#[async_trait]
impl HttpClient for FakeHttpClient {
    async fn send(&self, _request: HttpRequest) -> Result<HttpResponse, HttpError> {
        *self.call_count.lock().expect("call count lock poisoned") += 1;

        if self.should_fail {
            return Err(HttpError::Failed("simulated failure".to_string()));
        }

        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: self.response_body.clone(),
        })
    }
}

struct FakeClock {
    now: Mutex<Instant>,
}

impl FakeClock {
    fn new() -> Self {
        Self {
            now: Mutex::new(Instant::now()),
        }
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        *self.now.lock().expect("clock lock poisoned")
    }
}

#[tokio::test]
async fn resolves_oauth2_scheme_with_configured_credentials() {
    let schemes = vec![SecurityScheme {
        name: "oauth2Auth".to_string(),
        kind: SecuritySchemeKind::OAuth2 {
            token_url: Some("https://auth.example.com/token".to_string()),
        },
    }];
    let inputs = HashMap::from([(0, "client-1:secret-1".to_string())]);
    let token_caches = build_token_caches(&schemes);
    let http = FakeHttpClient::new(
        r#"{"access_token":"tok-abc","expires_in":60,"token_type":"Bearer"}"#,
    );
    let clock = FakeClock::new();

    let credentials =
        resolve_oauth2_credentials(&schemes, &inputs, &token_caches, &http, &clock)
            .await
            .expect("OAuth2 credentials should resolve");

    assert_eq!(credentials.len(), 1);
    match &credentials[0] {
        Credential::Bearer { token } => assert_eq!(token, "tok-abc"),
        _ => panic!("expected bearer credential"),
    }
}

#[tokio::test]
async fn skips_oauth2_scheme_with_no_input_configured() {
    let schemes = vec![SecurityScheme {
        name: "oauth2Auth".to_string(),
        kind: SecuritySchemeKind::OAuth2 {
            token_url: Some("https://auth.example.com/token".to_string()),
        },
    }];
    let inputs = HashMap::new();
    let token_caches = build_token_caches(&schemes);
    let http = FakeHttpClient::new(
        r#"{"access_token":"tok-abc","expires_in":60,"token_type":"Bearer"}"#,
    );
    let clock = FakeClock::new();

    let credentials =
        resolve_oauth2_credentials(&schemes, &inputs, &token_caches, &http, &clock)
            .await
            .expect("unconfigured OAuth2 scheme should be skipped");

    assert!(credentials.is_empty());
    assert_eq!(http.call_count(), 0);
}

#[tokio::test]
async fn skips_non_oauth2_and_oauth2_without_token_url_schemes() {
    let schemes = vec![
        SecurityScheme {
            name: "api_key".to_string(),
            kind: SecuritySchemeKind::ApiKey {
                location: "header".to_string(),
                param_name: "X-API-Key".to_string(),
            },
        },
        SecurityScheme {
            name: "oauth_without_token_url".to_string(),
            kind: SecuritySchemeKind::OAuth2 { token_url: None },
        },
        SecurityScheme {
            name: "bearer".to_string(),
            kind: SecuritySchemeKind::Http {
                scheme: "bearer".to_string(),
            },
        },
    ];
    let inputs = HashMap::from([
        (0, "api-key-value".to_string()),
        (1, "client-1:secret-1".to_string()),
        (2, "bearer-token".to_string()),
    ]);
    let token_caches = build_token_caches(&schemes);
    let http = FakeHttpClient::new(
        r#"{"access_token":"tok-abc","expires_in":60,"token_type":"Bearer"}"#,
    );
    let clock = FakeClock::new();

    let credentials =
        resolve_oauth2_credentials(&schemes, &inputs, &token_caches, &http, &clock)
            .await
            .expect("non-fetchable schemes should be skipped");

    assert!(credentials.is_empty());
    assert_eq!(http.call_count(), 0);
}

#[tokio::test]
async fn fails_fast_on_token_fetch_error_without_processing_later_schemes() {
    let schemes = vec![
        SecurityScheme {
            name: "oauth_a".to_string(),
            kind: SecuritySchemeKind::OAuth2 {
                token_url: Some("https://a.example.com/token".to_string()),
            },
        },
        SecurityScheme {
            name: "oauth_b".to_string(),
            kind: SecuritySchemeKind::OAuth2 {
                token_url: Some("https://b.example.com/token".to_string()),
            },
        },
    ];
    let inputs = HashMap::from([
        (0, "client-a:secret-a".to_string()),
        (1, "client-b:secret-b".to_string()),
    ]);
    let token_caches = build_token_caches(&schemes);
    let http = FakeHttpClient::new_failing();
    let clock = FakeClock::new();

    let credentials =
        resolve_oauth2_credentials(&schemes, &inputs, &token_caches, &http, &clock).await;

    assert!(credentials.is_err());
    assert_eq!(http.call_count(), 1);
}
