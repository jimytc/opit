use std::sync::Mutex;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use openapi_terminal_app::auth::oauth2::{Clock, TokenCache};
use openapi_terminal_app::request::{HttpClient, HttpError, HttpRequest, HttpResponse};

struct FakeHttpClient {
    call_count: Mutex<u32>,
    last_request: Mutex<Option<HttpRequest>>,
    response_body: String,
}

impl FakeHttpClient {
    fn new(response_body: &str) -> Self {
        Self {
            call_count: Mutex::new(0),
            last_request: Mutex::new(None),
            response_body: response_body.to_string(),
        }
    }

    fn call_count(&self) -> u32 {
        *self.call_count.lock().expect("call count lock poisoned")
    }
}

#[async_trait]
impl HttpClient for FakeHttpClient {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        *self.call_count.lock().expect("call count lock poisoned") += 1;
        *self.last_request.lock().expect("last request lock poisoned") = Some(request);

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

    fn advance(&self, duration: Duration) {
        let mut now = self.now.lock().expect("clock lock poisoned");
        *now = *now + duration;
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        *self.now.lock().expect("clock lock poisoned")
    }
}

fn token_response(access_token: &str) -> String {
    format!(
        r#"{{"access_token":"{}","expires_in":60,"token_type":"Bearer"}}"#,
        access_token
    )
}

#[tokio::test]
async fn fetches_and_returns_token_on_first_call() {
    let http = FakeHttpClient::new(&token_response("tok-abc"));
    let clock = FakeClock::new();
    let cache = TokenCache::new();

    let token = cache
        .get_or_fetch(
            &http,
            &clock,
            "https://auth.example.com/token",
            "client-1",
            "secret-1",
        )
        .await
        .expect("token fetch should succeed");

    assert_eq!(token, "tok-abc");

    let last_request = http
        .last_request
        .lock()
        .expect("last request lock poisoned");
    let last_request = last_request
        .as_ref()
        .expect("fake client should record the last request");

    assert_eq!(last_request.method, "POST");
    assert_eq!(last_request.url, "https://auth.example.com/token");
    assert!(last_request.headers.contains(&(
        "Content-Type".to_string(),
        "application/x-www-form-urlencoded".to_string()
    )));
    assert_eq!(
        last_request.body,
        Some("grant_type=client_credentials&client_id=client-1&client_secret=secret-1".to_string())
    );
}

#[tokio::test]
async fn reuses_cached_token_before_expiry() {
    let http = FakeHttpClient::new(&token_response("tok-abc"));
    let clock = FakeClock::new();
    let cache = TokenCache::new();

    let first_token = cache
        .get_or_fetch(
            &http,
            &clock,
            "https://auth.example.com/token",
            "client-1",
            "secret-1",
        )
        .await
        .expect("first token fetch should succeed");
    let second_token = cache
        .get_or_fetch(
            &http,
            &clock,
            "https://auth.example.com/token",
            "client-1",
            "secret-1",
        )
        .await
        .expect("cached token fetch should succeed");

    assert_eq!(first_token, "tok-abc");
    assert_eq!(second_token, "tok-abc");
    assert_eq!(http.call_count(), 1);
}

#[tokio::test]
async fn refetches_token_after_expiry_with_skew() {
    let http = FakeHttpClient::new(&token_response("tok-abc"));
    let clock = FakeClock::new();
    let cache = TokenCache::new();

    cache
        .get_or_fetch(
            &http,
            &clock,
            "https://auth.example.com/token",
            "client-1",
            "secret-1",
        )
        .await
        .expect("first token fetch should succeed");

    clock.advance(Duration::from_secs(40));

    cache
        .get_or_fetch(
            &http,
            &clock,
            "https://auth.example.com/token",
            "client-1",
            "secret-1",
        )
        .await
        .expect("refetched token should succeed");

    assert_eq!(http.call_count(), 2);
}
