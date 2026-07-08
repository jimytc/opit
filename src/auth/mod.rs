use std::collections::HashMap;

use base64::Engine;

use crate::request::HttpRequest;
use crate::spec::{SecurityScheme, SecuritySchemeKind};

pub mod oauth2;

#[derive(Clone)]
pub enum Credential {
    ApiKey {
        location: String,
        param_name: String,
        value: String,
    },
    Bearer {
        token: String,
    },
    Basic {
        username: String,
        password: String,
    },
    Raw {
        header_name: String,
        header_value: String,
    },
}

pub fn split_credential_pair(raw: &str) -> (String, String) {
    let (first, second) = raw.split_once(':').unwrap_or((raw, ""));
    (first.to_string(), second.to_string())
}

pub fn apply(request: &mut HttpRequest, credential: &Credential) {
    match credential {
        Credential::ApiKey {
            location,
            param_name,
            value,
        } => match location.as_str() {
            "header" => request.headers.push((param_name.clone(), value.clone())),
            "query" => crate::request::append_query_param(&mut request.url, param_name, value),
            "cookie" => {
                crate::request::append_cookie_header(&mut request.headers, param_name, value)
            }
            _ => {}
        },
        Credential::Bearer { token } => {
            request
                .headers
                .push(("Authorization".to_string(), format!("Bearer {token}")));
        }
        Credential::Basic { username, password } => {
            let encoded =
                base64::engine::general_purpose::STANDARD.encode(format!("{username}:{password}"));
            request
                .headers
                .push(("Authorization".to_string(), format!("Basic {encoded}")));
        }
        Credential::Raw {
            header_name,
            header_value,
        } => {
            request
                .headers
                .push((header_name.clone(), header_value.clone()));
        }
    }
}

pub fn credentials_from_inputs(
    schemes: &[SecurityScheme],
    inputs: &HashMap<usize, String>,
) -> Vec<Credential> {
    schemes
        .iter()
        .enumerate()
        .filter_map(|(index, scheme)| {
            let raw = inputs.get(&index)?;
            match &scheme.kind {
                SecuritySchemeKind::ApiKey {
                    location,
                    param_name,
                } => Some(Credential::ApiKey {
                    location: location.clone(),
                    param_name: param_name.clone(),
                    value: raw.clone(),
                }),
                SecuritySchemeKind::Http { scheme } if scheme == "bearer" => {
                    Some(Credential::Bearer { token: raw.clone() })
                }
                SecuritySchemeKind::Http { scheme } if scheme == "basic" => {
                    let (username, password) = split_credential_pair(raw);
                    Some(Credential::Basic { username, password })
                }
                // OAuth2 is intentionally never resolved here: this function is
                // sync and used by the per-frame live preview, but a real
                // client_credentials token fetch requires an async network
                // call. See auth::oauth2::resolve_oauth2_credentials for the
                // real (send-time-only) resolution path.
                _ => None,
            }
        })
        .collect()
}
