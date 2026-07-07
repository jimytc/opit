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

pub fn apply(request: &mut HttpRequest, credential: &Credential) {
    match credential {
        Credential::ApiKey {
            location,
            param_name,
            value,
        } => {
            if location == "header" {
                request.headers.push((param_name.clone(), value.clone()));
            }
        }
        Credential::Bearer { token } => {
            request
                .headers
                .push(("Authorization".to_string(), format!("Bearer {token}")));
        }
        Credential::Basic { username, password } => {
            let encoded = base64::engine::general_purpose::STANDARD
                .encode(format!("{username}:{password}"));
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
                    let (username, password) = raw.split_once(':').unwrap_or((raw.as_str(), ""));
                    Some(Credential::Basic {
                        username: username.to_string(),
                        password: password.to_string(),
                    })
                }
                _ => None,
            }
        })
        .collect()
}
