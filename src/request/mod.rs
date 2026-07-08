use std::collections::HashMap;

use crate::auth::{self, Credential};
use crate::spec::{Operation, SecurityScheme};

mod http_client;

pub use http_client::{HttpClient, HttpError, HttpResponse, ReqwestClient};

pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub fn build(
    base_url: &str,
    operation: &Operation,
    param_values: &HashMap<String, String>,
) -> HttpRequest {
    let mut path = operation.path.clone();
    let mut query_pairs = Vec::new();
    let mut headers = Vec::new();

    for parameter in &operation.parameters {
        let Some(value) = param_values.get(&parameter.name) else {
            continue;
        };
        match parameter.location.as_str() {
            "path" => {
                path = path.replace(&format!("{{{}}}", parameter.name), value);
            }
            "query" => {
                query_pairs.push(format!("{}={}", parameter.name, value));
            }
            "header" => {
                headers.push((parameter.name.clone(), value.clone()));
            }
            "cookie" => {
                append_cookie_header(&mut headers, &parameter.name, value);
            }
            _ => {}
        }
    }

    let mut url = format!("{base_url}{path}");
    if !query_pairs.is_empty() {
        url.push('?');
        url.push_str(&query_pairs.join("&"));
    }

    HttpRequest {
        method: operation.method.clone(),
        url,
        headers,
        body: None,
    }
}

pub fn append_cookie_header(headers: &mut Vec<(String, String)>, name: &str, value: &str) {
    if let Some(existing) = headers.iter_mut().find(|(key, _)| key == "Cookie") {
        existing.1.push_str(&format!("; {name}={value}"));
    } else {
        headers.push(("Cookie".to_string(), format!("{name}={value}")));
    }
}

pub fn append_query_param(url: &mut String, name: &str, value: &str) {
    url.push(if url.contains('?') { '&' } else { '?' });
    url.push_str(&format!("{name}={value}"));
}

pub fn param_values_from_inputs(
    operation: &Operation,
    inputs: &HashMap<usize, String>,
) -> HashMap<String, String> {
    operation
        .parameters
        .iter()
        .enumerate()
        .filter_map(|(index, parameter)| {
            inputs
                .get(&index)
                .map(|value| (parameter.name.clone(), value.clone()))
        })
        .collect()
}

pub fn missing_required_params(
    operation: &Operation,
    inputs: &HashMap<usize, String>,
) -> Vec<String> {
    operation
        .parameters
        .iter()
        .enumerate()
        .filter(|(index, parameter)| {
            parameter.required
                && inputs
                    .get(index)
                    .map(String::as_str)
                    .unwrap_or("")
                    .is_empty()
        })
        .map(|(_, parameter)| parameter.name.clone())
        .collect()
}

pub fn body_from_inputs(operation: &Operation, inputs: &HashMap<usize, String>) -> Option<String> {
    if !operation.has_request_body {
        return None;
    }
    inputs.get(&operation.parameters.len()).cloned()
}

pub fn pretty_print_if_json(body: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| body.to_string()),
        Err(_) => body.to_string(),
    }
}

pub fn build_preview(
    base_url: &str,
    operation: &Operation,
    param_inputs: &HashMap<usize, String>,
    security_schemes: &[SecurityScheme],
    auth_inputs: &HashMap<usize, String>,
    cli_credentials: &[Credential],
) -> HttpRequest {
    let param_values = param_values_from_inputs(operation, param_inputs);
    let mut request = build(base_url, operation, &param_values);
    if let Some(body) = body_from_inputs(operation, param_inputs) {
        request.body = Some(body);
        if let Some(media_type) = &operation.request_body_media_type {
            request
                .headers
                .push(("Content-Type".to_string(), media_type.clone()));
        }
    }

    let mut credentials = cli_credentials.to_vec();
    credentials.extend(auth::credentials_from_inputs(security_schemes, auth_inputs));
    for credential in &credentials {
        auth::apply(&mut request, credential);
    }

    request
}

/// Known limitation: header/body values containing a single quote are not
/// shell-escaped, so a value with a `'` will produce a curl command that
/// isn't directly copy-paste-safe.
pub fn to_curl(request: &HttpRequest) -> String {
    let mut curl = format!("curl -X {} '{}'", request.method, request.url);
    for (name, value) in &request.headers {
        curl.push_str(&format!(" -H '{name}: {value}'"));
    }
    if let Some(body) = &request.body {
        curl.push_str(&format!(" --data '{}'", pretty_print_if_json(body)));
    }
    curl
}
