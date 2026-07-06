#[test]
fn operation_exposes_parameters_from_openapi_operation() {
    let json = r#"
    {
      "openapi": "3.0.0",
      "info": {
        "title": "Pets API",
        "version": "1.0.0"
      },
      "paths": {
        "/pets/{petId}": {
          "get": {
            "operationId": "getPet",
            "parameters": [
              {
                "name": "petId",
                "in": "path",
                "required": true,
                "schema": {
                  "type": "string"
                }
              },
              {
                "name": "limit",
                "in": "query",
                "required": false,
                "schema": {
                  "type": "integer"
                }
              },
              {
                "name": "X-Request-Id",
                "in": "header",
                "required": false,
                "schema": {
                  "type": "string"
                }
              }
            ],
            "responses": {
              "200": {
                "description": "A pet"
              }
            }
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");
    let operations = spec.operations();

    assert_eq!(operations.len(), 1);

    let operation = &operations[0];

    assert_eq!(operation.parameters.len(), 3);
    assert!(operation
        .parameters
        .iter()
        .any(|parameter| parameter.name == "petId"
            && parameter.location == "path"
            && parameter.required));
    assert!(operation
        .parameters
        .iter()
        .any(|parameter| parameter.name == "limit"
            && parameter.location == "query"
            && !parameter.required));
    assert!(operation
        .parameters
        .iter()
        .any(|parameter| parameter.name == "X-Request-Id"
            && parameter.location == "header"
            && !parameter.required));
}
