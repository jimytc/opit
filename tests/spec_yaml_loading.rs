#[test]
fn parses_minimal_openapi_yaml_get_operation() {
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

    let spec = openapi_terminal_app::spec::Spec::from_yaml_str(yaml).expect("spec parses");
    let operations = spec.operations();

    assert_eq!(operations.len(), 1);
    assert_eq!(operations[0].path, "/pets");
    assert_eq!(operations[0].method, "GET");
}
