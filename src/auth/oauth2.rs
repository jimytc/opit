use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Deserialize;
use thiserror::Error;

use crate::request::{HttpClient, HttpError, HttpRequest};

const EXPIRY_SKEW: Duration = Duration::from_secs(30);

pub trait Clock: Send + Sync {
    fn now(&self) -> Instant;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("token request failed: {0:?}")]
    Http(HttpError),
    #[error("failed to parse token response: {0}")]
    InvalidResponse(#[from] serde_json::Error),
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

struct CachedToken {
    value: String,
    expires_at: Instant,
}

pub struct TokenCache {
    cached: Mutex<Option<CachedToken>>,
}

impl TokenCache {
    pub fn new() -> Self {
        Self {
            cached: Mutex::new(None),
        }
    }

    pub async fn get_or_fetch(
        &self,
        http: &dyn HttpClient,
        clock: &dyn Clock,
        token_url: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<String, AuthError> {
        if let Some(cached) = self.cached.lock().expect("token cache lock poisoned").as_ref() {
            if clock.now() < cached.expires_at - EXPIRY_SKEW {
                return Ok(cached.value.clone());
            }
        }

        let request = HttpRequest {
            method: "POST".to_string(),
            url: token_url.to_string(),
            headers: vec![(
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            )],
            body: Some(format!(
                "grant_type=client_credentials&client_id={client_id}&client_secret={client_secret}"
            )),
        };

        let response = http.send(request).await.map_err(AuthError::Http)?;
        let token_response: TokenResponse = serde_json::from_str(&response.body)?;

        let expires_at = clock.now() + Duration::from_secs(token_response.expires_in);
        *self.cached.lock().expect("token cache lock poisoned") = Some(CachedToken {
            value: token_response.access_token.clone(),
            expires_at,
        });

        Ok(token_response.access_token)
    }
}

impl Default for TokenCache {
    fn default() -> Self {
        Self::new()
    }
}
