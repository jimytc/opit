use openapi_terminal_app::spec::SecuritySchemeKind;

#[test]
fn spec_exposes_security_schemes_from_components() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Secure API",
        "version": "1.0.0"
      },
      "paths": {},
      "components": {
        "securitySchemes": {
          "apiKeyAuth": {
            "type": "apiKey",
            "in": "header",
            "name": "X-API-Key"
          },
          "bearerAuth": {
            "type": "http",
            "scheme": "bearer"
          },
          "oauth2Auth": {
            "type": "oauth2",
            "flows": {
              "clientCredentials": {
                "tokenUrl": "https://example.com/token",
                "scopes": {}
              }
            }
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");
    let security_schemes = spec.security_schemes();

    assert_eq!(security_schemes.len(), 3);
    assert!(security_schemes.iter().any(|scheme| {
        scheme.name == "apiKeyAuth"
            && matches!(
                &scheme.kind,
                SecuritySchemeKind::ApiKey {
                    location,
                    param_name
                } if location == "header" && param_name == "X-API-Key"
            )
    }));
    assert!(security_schemes.iter().any(|scheme| {
        scheme.name == "bearerAuth"
            && matches!(
                &scheme.kind,
                SecuritySchemeKind::Http { scheme } if scheme == "bearer"
            )
    }));
    assert!(security_schemes.iter().any(|scheme| {
        scheme.name == "oauth2Auth" && matches!(&scheme.kind, SecuritySchemeKind::OAuth2 { .. })
    }));
}

#[test]
fn oauth2_without_client_credentials_flow_has_no_token_url() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Secure API",
        "version": "1.0.0"
      },
      "paths": {},
      "components": {
        "securitySchemes": {
          "oauth2NoClientCreds": {
            "type": "oauth2",
            "flows": {
              "authorizationCode": {
                "authorizationUrl": "https://example.com/authorize",
                "tokenUrl": "https://example.com/token",
                "scopes": {}
              }
            }
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let security_schemes = spec.security_schemes();

    assert!(security_schemes.iter().any(|scheme| {
        scheme.name == "oauth2NoClientCreds"
            && matches!(
                &scheme.kind,
                SecuritySchemeKind::OAuth2 { token_url } if token_url.is_none()
            )
    }));
}
