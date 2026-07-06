#[test]
fn lists_all_operations_defined_on_a_path_item() {
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
          },
          "post": {
            "operationId": "createPet",
            "responses": {}
          },
          "put": {
            "operationId": "replacePets",
            "responses": {}
          },
          "delete": {
            "operationId": "deletePets",
            "responses": {}
          }
        }
      }
    }
    "#;

    let spec = openapi_terminal_app::spec::Spec::from_json_str(json).expect("spec parses");
    let operations = spec.operations();

    assert_eq!(operations.len(), 4);
    assert!(operations
        .iter()
        .any(|operation| operation.path == "/pets" && operation.method == "GET"));
    assert!(operations
        .iter()
        .any(|operation| operation.path == "/pets" && operation.method == "POST"));
    assert!(operations
        .iter()
        .any(|operation| operation.path == "/pets" && operation.method == "PUT"));
    assert!(operations
        .iter()
        .any(|operation| operation.path == "/pets" && operation.method == "DELETE"));
}
