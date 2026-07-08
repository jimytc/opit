mod operation;
mod security;

use std::path::Path;

use thiserror::Error;

pub use operation::{Operation, Parameter};
pub use security::{SecurityScheme, SecuritySchemeKind};

#[derive(Debug, Error)]
pub enum SpecError {
    #[error("failed to parse OpenAPI JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("failed to parse OpenAPI YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("failed to read spec file: {0}")]
    Io(#[from] std::io::Error),
    #[error("unrecognized spec file extension: {0}")]
    UnknownExtension(String),
}

pub struct Spec {
    inner: openapiv3::OpenAPI,
}

impl Spec {
    pub fn from_json_str(json: &str) -> Result<Self, SpecError> {
        let inner: openapiv3::OpenAPI = serde_json::from_str(json)?;
        Ok(Self { inner })
    }

    pub fn from_yaml_str(yaml: &str) -> Result<Self, SpecError> {
        let inner: openapiv3::OpenAPI = serde_yaml::from_str(yaml)?;
        Ok(Self { inner })
    }

    pub fn load_from_path(path: &Path) -> Result<Self, SpecError> {
        let contents = std::fs::read_to_string(path)?;
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => Self::from_json_str(&contents),
            Some("yaml") | Some("yml") => Self::from_yaml_str(&contents),
            other => Err(SpecError::UnknownExtension(
                other.unwrap_or("").to_string(),
            )),
        }
    }

    pub fn base_url(&self) -> Option<String> {
        self.inner.servers.first().map(|server| server.url.clone())
    }

    pub fn servers(&self) -> Vec<String> {
        self.inner
            .servers
            .iter()
            .map(|server| server.url.clone())
            .collect()
    }

    pub fn security_schemes(&self) -> Vec<SecurityScheme> {
        match &self.inner.components {
            Some(components) => security::security_schemes_from(components),
            None => Vec::new(),
        }
    }

    pub fn operations(&self) -> Vec<Operation> {
        let mut operations = Vec::new();
        for (path, item) in &self.inner.paths.paths {
            let item = match item.as_item() {
                Some(item) => item,
                None => continue,
            };
            let methods: [(&str, &Option<openapiv3::Operation>); 7] = [
                ("GET", &item.get),
                ("POST", &item.post),
                ("PUT", &item.put),
                ("DELETE", &item.delete),
                ("PATCH", &item.patch),
                ("HEAD", &item.head),
                ("OPTIONS", &item.options),
            ];
            for (method, operation) in methods {
                if let Some(operation) = operation {
                    operations.push(Operation {
                        path: path.clone(),
                        method: method.to_string(),
                        parameters: operation::parameters_from(operation),
                        has_request_body: operation.request_body.is_some(),
                        request_body_media_type: operation::request_body_media_type_from(
                            operation,
                        ),
                        summary: operation::summary_from(operation),
                        request_body_example: operation::request_body_example_from(operation),
                        tags: operation.tags.clone(),
                    });
                }
            }
        }
        operations
    }
}
