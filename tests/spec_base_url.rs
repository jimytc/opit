#[test]
fn base_url_returns_first_server_url() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Example API",
        "version": "1.0.0"
      },
      "servers": [
        { "url": "https://api.example.com/v1" },
        { "url": "https://staging.example.com/v1" }
      ],
      "paths": {}
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");

    assert_eq!(
        spec.base_url(),
        Some("https://api.example.com/v1".to_string())
    );
}

#[test]
fn base_url_returns_none_when_servers_field_is_missing() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Example API",
        "version": "1.0.0"
      },
      "paths": {}
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");

    assert_eq!(spec.base_url(), None);
}

#[test]
fn servers_returns_all_server_urls_in_order() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Example API",
        "version": "1.0.0"
      },
      "servers": [
        { "url": "https://api.example.com/v1" },
        { "url": "https://staging.example.com/v1" }
      ],
      "paths": {}
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");

    assert_eq!(
        spec.servers(),
        vec![
            "https://api.example.com/v1".to_string(),
            "https://staging.example.com/v1".to_string()
        ]
    );
}

#[test]
fn servers_returns_empty_vec_when_servers_field_is_missing() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Example API",
        "version": "1.0.0"
      },
      "paths": {}
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");

    assert_eq!(spec.servers(), Vec::<String>::new());
}
