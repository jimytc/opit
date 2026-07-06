use std::sync::Mutex;

use async_trait::async_trait;
use openapi_terminal_app::request::{HttpClient, HttpError, HttpRequest, HttpResponse};

struct FakeHttpClient {
    last_request: Mutex<Option<HttpRequest>>,
}

impl FakeHttpClient {
    fn new() -> Self {
        Self {
            last_request: Mutex::new(None),
        }
    }
}

#[async_trait]
impl HttpClient for FakeHttpClient {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        *self.last_request.lock().expect("last request lock poisoned") = Some(request);

        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: "{\"ok\":true}".to_string(),
        })
    }
}

#[tokio::test]
async fn fake_http_client_returns_success_response() {
    let fake_client = FakeHttpClient::new();
    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/pets".to_string(),
        headers: vec![],
    };

    let response = fake_client.send(request).await.expect("request should succeed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body, "{\"ok\":true}");
}

#[tokio::test]
async fn fake_http_client_records_request_url() {
    let fake_client = FakeHttpClient::new();
    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/pets".to_string(),
        headers: vec![],
    };

    fake_client.send(request).await.expect("request should succeed");

    let last_request = fake_client
        .last_request
        .lock()
        .expect("last request lock poisoned");
    let last_request = last_request
        .as_ref()
        .expect("fake client should record the last request");

    assert_eq!(last_request.url, "https://api.example.com/pets");
}
