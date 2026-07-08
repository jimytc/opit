use openapi_terminal_app::request::{append_cookie_header, append_query_param};

#[test]
fn append_cookie_header_adds_cookie_header_to_empty_headers() {
    let mut headers = Vec::new();

    append_cookie_header(&mut headers, "session", "abc123");

    assert_eq!(
        headers,
        vec![("Cookie".to_string(), "session=abc123".to_string())]
    );
}

#[test]
fn append_cookie_header_appends_to_existing_cookie_header() {
    let mut headers = vec![("Cookie".to_string(), "session=abc123".to_string())];

    append_cookie_header(&mut headers, "theme", "dark");

    assert_eq!(
        headers,
        vec![(
            "Cookie".to_string(),
            "session=abc123; theme=dark".to_string()
        )]
    );
}

#[test]
fn append_cookie_header_preserves_other_headers() {
    let mut headers = vec![("Authorization".to_string(), "Bearer xyz".to_string())];

    append_cookie_header(&mut headers, "session", "abc");

    assert_eq!(headers.len(), 2);
    assert!(headers.contains(&("Authorization".to_string(), "Bearer xyz".to_string())));
    assert!(headers.contains(&("Cookie".to_string(), "session=abc".to_string())));
}

#[test]
fn append_query_param_adds_question_mark_when_url_has_no_query_string() {
    let mut url = "https://api.example.com/pets".to_string();

    append_query_param(&mut url, "limit", "10");

    assert_eq!(url, "https://api.example.com/pets?limit=10");
}

#[test]
fn append_query_param_adds_ampersand_when_url_already_has_query_string() {
    let mut url = "https://api.example.com/pets?sort=asc".to_string();

    append_query_param(&mut url, "limit", "10");

    assert_eq!(url, "https://api.example.com/pets?sort=asc&limit=10");
}
