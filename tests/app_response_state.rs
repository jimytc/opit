use openapi_terminal_app::app::AppState;
use openapi_terminal_app::request::HttpResponse;

#[test]
fn fresh_app_state_has_no_response() {
    let state = AppState::new();

    assert!(state.response().is_none());
}

#[test]
fn set_response_stores_response() {
    let mut state = AppState::new();

    state.set_response(HttpResponse {
        status: 200,
        headers: vec![],
        body: "ok".to_string(),
    });

    let response = state.response().expect("response should be stored");
    assert_eq!(response.status, 200);
    assert_eq!(response.body, "ok");
}

#[test]
fn set_response_replaces_existing_response() {
    let mut state = AppState::new();

    state.set_response(HttpResponse {
        status: 200,
        headers: vec![],
        body: "ok".to_string(),
    });
    state.set_response(HttpResponse {
        status: 500,
        headers: vec![],
        body: "error".to_string(),
    });

    assert_eq!(state.response().unwrap().status, 500);
}
