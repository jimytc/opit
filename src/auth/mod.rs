use base64::Engine;

use crate::request::HttpRequest;

pub mod oauth2;

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
