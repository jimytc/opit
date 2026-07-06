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
