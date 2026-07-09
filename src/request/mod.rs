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

pub struct RequestInputs {
    pub param_values: HashMap<String, String>,
    pub extra_headers: Vec<(String, String)>,
    pub extra_query: Vec<(String, String)>,
    pub body: Option<String>,
}

pub fn gather_request_inputs(
    operation: &Operation,
    header_inputs: &HashMap<usize, String>,
    parameter_inputs: &HashMap<usize, String>,
    custom_headers: &[String],
    custom_query_params: &[String],
    payload_inputs: &HashMap<usize, String>,
) -> RequestInputs {
    let header_parameters = operation.header_parameters();
    let non_header_parameters = operation.non_header_parameters();

    let mut param_values = HashMap::new();
    for (index, parameter) in header_parameters.iter().enumerate() {
        if let Some(value) = header_inputs.get(&index) {
            param_values.insert(parameter.name.clone(), value.clone());
        }
    }
    for (index, parameter) in non_header_parameters.iter().enumerate() {
        if let Some(value) = parameter_inputs.get(&index) {
            param_values.insert(parameter.name.clone(), value.clone());
        }
    }

    let extra_headers = custom_headers
        .iter()
        .enumerate()
        .filter_map(|(index, name)| {
            header_inputs
                .get(&(header_parameters.len() + index))
                .map(|value| (name.clone(), value.clone()))
        })
        .collect();

    let extra_query = custom_query_params
        .iter()
        .enumerate()
        .filter_map(|(index, name)| {
            parameter_inputs
                .get(&(non_header_parameters.len() + index))
                .map(|value| (name.clone(), value.clone()))
        })
        .collect();

    let body = payload_inputs.get(&0).cloned();

    RequestInputs {
        param_values,
        extra_headers,
        extra_query,
        body,
    }
}

pub fn build(base_url: &str, operation: &Operation, inputs: &RequestInputs) -> HttpRequest {
    let mut path = operation.path.clone();
    let mut query_pairs = Vec::new();
    let mut headers = Vec::new();

    for parameter in &operation.parameters {
        let Some(value) = inputs.param_values.get(&parameter.name) else {
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

    for (name, value) in &inputs.extra_headers {
        headers.push((name.clone(), value.clone()));
    }
    for (name, value) in &inputs.extra_query {
        query_pairs.push(format!("{name}={value}"));
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

pub fn missing_required_params(operation: &Operation, inputs: &RequestInputs) -> Vec<String> {
    operation
        .parameters
        .iter()
        .filter(|parameter| {
            parameter.required
                && inputs
                    .param_values
                    .get(&parameter.name)
                    .map(String::as_str)
                    .unwrap_or("")
                    .is_empty()
        })
        .map(|parameter| parameter.name.clone())
        .collect()
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
    inputs: &RequestInputs,
    security_schemes: &[SecurityScheme],
    auth_inputs: &HashMap<usize, String>,
    cli_credentials: &[Credential],
) -> HttpRequest {
    let mut request = build(base_url, operation, inputs);
    if let Some(body) = &inputs.body {
        request.body = Some(body.clone());
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
