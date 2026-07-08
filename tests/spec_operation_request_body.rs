#[test]
fn operations_report_whether_request_body_is_present() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets": {
          "post": {
            "operationId": "createPet",
            "requestBody": {
              "content": {
                "application/json": {
                  "schema": {
                    "type": "object"
                  }
                }
              }
            },
            "responses": {}
          },
          "get": {
            "operationId": "listPets",
            "responses": {}
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");
    let operations = spec.operations();

    assert_eq!(operations.len(), 2);

    let post_operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "POST")
        .expect("POST /pets operation exists");
    let get_operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "GET")
        .expect("GET /pets operation exists");

    assert!(post_operation.has_request_body);
    assert!(!get_operation.has_request_body);
}

#[test]
fn post_operation_reports_request_body_media_type() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets": {
          "post": {
            "operationId": "createPet",
            "requestBody": {
              "content": {
                "application/json": {
                  "schema": {
                    "type": "object"
                  }
                }
              }
            },
            "responses": {}
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let operations = spec.operations();
    let operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "POST")
        .expect("POST /pets operation exists");

    assert_eq!(
        operation.request_body_media_type,
        Some("application/json".to_string())
    );
}

#[test]
fn get_operation_without_request_body_reports_no_request_body_media_type() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets": {
          "get": {
            "operationId": "listPets",
            "responses": {}
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let operations = spec.operations();
    let operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "GET")
        .expect("GET /pets operation exists");

    assert_eq!(operation.request_body_media_type, None);
}

#[test]
fn get_operation_reports_summary_and_tags() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets": {
          "get": {
            "operationId": "listPets",
            "summary": "List all pets",
            "tags": ["Pets"],
            "responses": {}
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let operations = spec.operations();
    let operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "GET")
        .expect("GET /pets operation exists");

    assert_eq!(operation.summary, Some("List all pets".to_string()));
    assert_eq!(operation.tags, vec!["Pets".to_string()]);
}

#[test]
fn get_operation_uses_description_as_summary_when_summary_is_absent() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets": {
          "get": {
            "operationId": "listPets",
            "description": "Returns every pet in the store",
            "responses": {}
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let operations = spec.operations();
    let operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "GET")
        .expect("GET /pets operation exists");

    assert_eq!(
        operation.summary,
        Some("Returns every pet in the store".to_string())
    );
}

#[test]
fn get_operation_without_summary_or_description_reports_no_summary() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets": {
          "get": {
            "operationId": "listPets",
            "responses": {}
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let operations = spec.operations();
    let operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "GET")
        .expect("GET /pets operation exists");

    assert_eq!(operation.summary, None);
}

#[test]
fn get_operation_without_tags_reports_empty_tags() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets": {
          "get": {
            "operationId": "listPets",
            "responses": {}
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let operations = spec.operations();
    let operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "GET")
        .expect("GET /pets operation exists");

    assert_eq!(operation.tags, Vec::<String>::new());
}
