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

pub fn example_json_from_schema(schema: &openapiv3::Schema) -> serde_json::Value {
    use openapiv3::{SchemaKind, Type};

    match &schema.schema_kind {
        SchemaKind::Type(Type::String(_)) => serde_json::Value::String(String::new()),
        SchemaKind::Type(Type::Number(_)) | SchemaKind::Type(Type::Integer(_)) => {
            serde_json::json!(0)
        }
        SchemaKind::Type(Type::Boolean(_)) => serde_json::Value::Bool(false),
        SchemaKind::Type(Type::Array(array)) => {
            match array.items.as_ref().and_then(|items| items.as_item()) {
                Some(item_schema) => {
                    serde_json::Value::Array(vec![example_json_from_schema(item_schema)])
                }
                None => serde_json::Value::Array(vec![]),
            }
        }
        SchemaKind::Type(Type::Object(object)) => {
            let mut map = serde_json::Map::new();
            for (name, property) in &object.properties {
                if let Some(property_schema) = property.as_item() {
                    map.insert(name.clone(), example_json_from_schema(property_schema));
                }
            }
            serde_json::Value::Object(map)
        }
        _ => serde_json::Value::Null,
    }
}

pub(super) fn request_body_example_from(operation: &openapiv3::Operation) -> Option<String> {
    let body = operation.request_body.as_ref()?.as_item()?;
    let media_type = body.content.values().next()?;
    let schema = media_type.schema.as_ref()?.as_item()?;
    let value = example_json_from_schema(schema);
    serde_json::to_string_pretty(&value).ok()
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
