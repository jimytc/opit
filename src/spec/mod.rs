use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpecError {
    #[error("failed to parse OpenAPI JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct Operation {
    pub path: String,
    pub method: String,
}

pub struct Spec {
    inner: openapiv3::OpenAPI,
}

impl Spec {
    pub fn from_json_str(json: &str) -> Result<Self, SpecError> {
        let inner: openapiv3::OpenAPI = serde_json::from_str(json)?;
        Ok(Self { inner })
    }

    pub fn operations(&self) -> Vec<Operation> {
        let mut operations = Vec::new();
        for (path, item) in &self.inner.paths.paths {
            let item = match item.as_item() {
                Some(item) => item,
                None => continue,
            };
            if item.get.is_some() {
                operations.push(Operation {
                    path: path.clone(),
                    method: "GET".to_string(),
                });
            }
        }
        operations
    }
}
