use openapi_terminal_app::request::{pretty_print_if_json, to_curl, HttpRequest};

#[test]
fn formats_get_request_without_headers_or_body() {
    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/pets".to_string(),
        headers: vec![],
        body: None,
    };

    let curl = to_curl(&request);

    assert_eq!(curl, "curl -X GET 'https://api.example.com/pets'");
}

#[test]
fn preserves_header_order() {
    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/pets".to_string(),
        headers: vec![
            ("Authorization".to_string(), "Bearer abc".to_string()),
            ("X-Custom".to_string(), "value".to_string()),
        ],
        body: None,
    };

    let curl = to_curl(&request);
    let authorization_header = "-H 'Authorization: Bearer abc'";
    let custom_header = "-H 'X-Custom: value'";
    let authorization_index = curl.find(authorization_header).unwrap();
    let custom_index = curl.find(custom_header).unwrap();

    assert!(authorization_index < custom_index);
}

#[test]
fn formats_json_body_with_pretty_printing() {
    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/pets".to_string(),
        headers: vec![],
        body: Some("{\"a\":1}".to_string()),
    };
    let pretty_body = pretty_print_if_json("{\"a\":1}");
    let expected = format!(
        "curl -X POST 'https://api.example.com/pets' --data '{}'",
        pretty_body
    );

    let curl = to_curl(&request);

    assert_eq!(curl, expected);
}

#[test]
fn formats_non_json_body_unchanged() {
    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/pets".to_string(),
        headers: vec![],
        body: Some("plain text body".to_string()),
    };

    let curl = to_curl(&request);

    assert!(curl.ends_with(" --data 'plain text body'"));
}

#[test]
fn formats_post_request_without_trailing_headers_or_data() {
    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/pets".to_string(),
        headers: vec![],
        body: None,
    };

    let curl = to_curl(&request);

    assert_eq!(curl, "curl -X POST 'https://api.example.com/pets'");
}
