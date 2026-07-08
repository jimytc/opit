pub struct SecurityScheme {
    pub name: String,
    pub kind: SecuritySchemeKind,
}

pub enum SecuritySchemeKind {
    ApiKey {
        location: String,
        param_name: String,
    },
    Http {
        scheme: String,
    },
    OAuth2 {
        token_url: Option<String>,
    },
    OpenIdConnect,
}

fn api_key_location_str(location: &openapiv3::APIKeyLocation) -> &'static str {
    match location {
        openapiv3::APIKeyLocation::Query => "query",
        openapiv3::APIKeyLocation::Header => "header",
        openapiv3::APIKeyLocation::Cookie => "cookie",
    }
}

pub(super) fn security_schemes_from(components: &openapiv3::Components) -> Vec<SecurityScheme> {
    components
        .security_schemes
        .iter()
        .filter_map(|(name, reference)| {
            let scheme = reference.as_item()?;
            let kind = match scheme {
                openapiv3::SecurityScheme::APIKey { location, name, .. } => {
                    SecuritySchemeKind::ApiKey {
                        location: api_key_location_str(location).to_string(),
                        param_name: name.clone(),
                    }
                }
                openapiv3::SecurityScheme::HTTP { scheme, .. } => SecuritySchemeKind::Http {
                    scheme: scheme.clone(),
                },
                openapiv3::SecurityScheme::OAuth2 { flows, .. } => SecuritySchemeKind::OAuth2 {
                    token_url: flows
                        .client_credentials
                        .as_ref()
                        .map(|flow| flow.token_url.clone()),
                },
                openapiv3::SecurityScheme::OpenIDConnect { .. } => {
                    SecuritySchemeKind::OpenIdConnect
                }
            };
            Some(SecurityScheme {
                name: name.clone(),
                kind,
            })
        })
        .collect()
}
