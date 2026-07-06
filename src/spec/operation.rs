pub struct Operation {
    pub path: String,
    pub method: String,
    pub parameters: Vec<Parameter>,
    pub has_request_body: bool,
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
