pub struct Operation {
    pub path: String,
    pub method: String,
    pub parameters: Vec<Parameter>,
    pub has_request_body: bool,
    pub request_body_media_type: Option<String>,
    pub summary: Option<String>,
    pub request_body_example: Option<String>,
    pub tags: Vec<String>,
}

pub(super) fn request_body_media_type_from(operation: &openapiv3::Operation) -> Option<String> {
    operation
        .request_body
        .as_ref()
        .and_then(|body| body.as_item())
        .and_then(|body| body.content.keys().next().cloned())
}

pub(super) fn summary_from(operation: &openapiv3::Operation) -> Option<String> {
    operation
        .summary
        .clone()
        .or_else(|| operation.description.clone())
}

pub struct Parameter {
    pub name: String,
    pub location: String,
    pub required: bool,
}

pub(super) fn parameters_from(operation: &openapiv3::Operation) -> Vec<Parameter> {
    operation
        .parameters
        .iter()
        .filter_map(|reference| reference.as_item())
        .map(|parameter| {
            let data = parameter.parameter_data_ref();
            let location = match parameter {
                openapiv3::Parameter::Query { .. } => "query",
                openapiv3::Parameter::Header { .. } => "header",
                openapiv3::Parameter::Path { .. } => "path",
                openapiv3::Parameter::Cookie { .. } => "cookie",
            };
            Parameter {
                name: data.name.clone(),
                location: location.to_string(),
                required: data.required,
            }
        })
        .collect()
}
