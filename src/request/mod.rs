use std::collections::HashMap;

use crate::spec::Operation;

mod http_client;

pub use http_client::{HttpClient, HttpError, HttpResponse, ReqwestClient};

pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
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
    }
}
