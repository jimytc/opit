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

#[test]
fn post_operation_generates_request_body_example_from_inline_object_schema() {
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
                    "type": "object",
                    "properties": {
                      "name": {
                        "type": "string"
                      },
                      "weight": {
                        "type": "number"
                      },
                      "vaccinated": {
                        "type": "boolean"
                      },
                      "nicknames": {
                        "type": "array",
                        "items": {
                          "type": "string"
                        }
                      }
                    }
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

    let request_body_example = operation
        .request_body_example
        .as_ref()
        .expect("request body example is generated");
    let parsed_example: serde_json::Value =
        serde_json::from_str(request_body_example).expect("request body example is JSON");

    assert!(
        request_body_example.starts_with("{\n"),
        "expected request body example to be pretty-printed, got {request_body_example:?}"
    );
    assert_eq!(
        parsed_example,
        serde_json::json!({
            "name": "",
            "weight": 0,
            "vaccinated": false,
            "nicknames": [""]
        })
    );
}

#[test]
fn post_operation_skips_request_body_properties_that_are_references() {
    let json = r##"
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
                    "type": "object",
                    "properties": {
                      "name": {
                        "type": "string"
                      },
                      "owner": {
                        "$ref": "#/components/schemas/Owner"
                      }
                    }
                  }
                }
              }
            },
            "responses": {}
          }
        }
      },
      "components": {
        "schemas": {
          "Owner": {
            "type": "object",
            "properties": {
              "id": {
                "type": "string"
              }
            }
          }
        }
      }
    }
    "##;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let operations = spec.operations();
    let operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "POST")
        .expect("POST /pets operation exists");

    let request_body_example = operation
        .request_body_example
        .as_ref()
        .expect("request body example is generated");
    let parsed_example: serde_json::Value =
        serde_json::from_str(request_body_example).expect("request body example is JSON");

    assert_eq!(
        parsed_example,
        serde_json::json!({
            "name": ""
        })
    );
    assert!(
        parsed_example.get("owner").is_none(),
        "expected referenced request body property to be skipped, got {parsed_example:?}"
    );
}

#[test]
fn post_operation_does_not_generate_request_body_example_for_top_level_schema_reference() {
    let json = r##"
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
                    "$ref": "#/components/schemas/Pet"
                  }
                }
              }
            },
            "responses": {}
          }
        }
      },
      "components": {
        "schemas": {
          "Pet": {
            "type": "object",
            "properties": {
              "name": {
                "type": "string"
              }
            }
          }
        }
      }
    }
    "##;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).unwrap();
    let operations = spec.operations();
    let operation = operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == "POST")
        .expect("POST /pets operation exists");

    assert_eq!(operation.request_body_example, None);
}

#[test]
fn get_operation_without_request_body_reports_no_request_body_example() {
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

    assert_eq!(operation.request_body_example, None);
}
