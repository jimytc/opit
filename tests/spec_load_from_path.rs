use std::path::PathBuf;

fn temp_spec_path(test_name: &str, extension: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "openapi-terminal-app-{test_name}-{}.{extension}",
        std::process::id()
    ))
}

#[test]
fn loads_minimal_openapi_json_get_operation_from_path() {
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
            "responses": {
              "200": {
                "description": "A list of pets"
              }
            }
          }
        }
      }
    }
    "#;
    let path = temp_spec_path("loads_minimal_openapi_json_get_operation_from_path", "json");

    std::fs::write(&path, json).expect("write temp JSON spec");
    let spec = openapi_terminal_app::spec::Spec::load_from_path(&path).expect("spec loads");
    let _ = std::fs::remove_file(&path);

    let operations = spec.operations();
    assert_eq!(operations.len(), 1);
    assert_eq!(operations[0].path, "/pets");
    assert_eq!(operations[0].method, "GET");
}

#[test]
fn loads_minimal_openapi_yaml_get_operation_from_path() {
    let yaml = r#"
openapi: 3.0.0
info:
  title: Pets API
  version: 1.0.0
paths:
  /pets:
    get:
      operationId: listPets
      responses:
        "200":
          description: A list of pets
"#;
    let path = temp_spec_path("loads_minimal_openapi_yaml_get_operation_from_path", "yaml");

    std::fs::write(&path, yaml).expect("write temp YAML spec");
    let spec = openapi_terminal_app::spec::Spec::load_from_path(&path).expect("spec loads");
    let _ = std::fs::remove_file(&path);

    let operations = spec.operations();
    assert_eq!(operations.len(), 1);
    assert_eq!(operations[0].path, "/pets");
    assert_eq!(operations[0].method, "GET");
}
