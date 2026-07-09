use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use openapi_terminal_app::app::{AppState, Pane, RequestBuilderTab};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn tab() -> KeyEvent {
    key(KeyCode::Tab)
}

fn right_bracket() -> KeyEvent {
    key(KeyCode::Char(']'))
}

fn ctrl_s() -> KeyEvent {
    KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL)
}

fn focus_request_builder(state: &mut AppState) {
    state.handle_key(tab());
    state.handle_key(tab());

    assert_eq!(state.focused, Pane::RequestBuilder);
}

fn type_text(state: &mut AppState, text: &str) {
    for c in text.chars() {
        state.handle_key(key(KeyCode::Char(c)));
    }
}

fn add_header_with_enter(state: &mut AppState, text: &str) {
    state.request_builder.headers.set_row_count(1);
    focus_request_builder(state);
    state.handle_key(key(KeyCode::Enter));
    type_text(state, text);
    state.handle_key(key(KeyCode::Enter));
}

#[test]
fn header_add_row_enter_adds_custom_header_name_and_stores_value() {
    let mut state = AppState::new();

    add_header_with_enter(&mut state, "X-Api-Version=2");

    assert_eq!(
        state.request_builder.custom_headers,
        vec!["X-Api-Version".to_string()]
    );
    assert_eq!(
        state.request_builder.headers.inputs().get(&0).map(String::as_str),
        Some("2")
    );
    assert!(!state.is_editing());
    assert_eq!(state.request_builder.headers.selected_row(), 1);
}

#[test]
fn header_add_row_ctrl_s_adds_custom_header_name_and_stores_value() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    focus_request_builder(&mut state);
    state.handle_key(key(KeyCode::Enter));
    type_text(&mut state, "X-Api-Version=2");

    state.handle_key(ctrl_s());

    assert_eq!(
        state.request_builder.custom_headers,
        vec!["X-Api-Version".to_string()]
    );
    assert_eq!(
        state.request_builder.headers.inputs().get(&0).map(String::as_str),
        Some("2")
    );
    assert!(!state.is_editing());
    assert_eq!(state.request_builder.headers.selected_row(), 1);
}

#[test]
fn header_add_row_enter_without_equals_cancels_without_adding_custom_header() {
    let mut state = AppState::new();

    add_header_with_enter(&mut state, "invalidtext");

    assert!(state.request_builder.custom_headers.is_empty());
    assert!(state.request_builder.headers.inputs().is_empty());
    assert!(!state.is_editing());
}

#[test]
fn header_add_row_enter_with_empty_name_cancels_without_adding_custom_header() {
    let mut state = AppState::new();

    add_header_with_enter(&mut state, "=onlyvalue");

    assert!(state.request_builder.custom_headers.is_empty());
    assert!(state.request_builder.headers.inputs().is_empty());
    assert!(!state.is_editing());
}

#[test]
fn header_add_row_enter_duplicate_custom_name_cancels_without_adding_second_header() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(2);
    state
        .request_builder
        .custom_headers
        .push("X-Api-Version".to_string());
    state
        .request_builder
        .headers
        .set_input(0, "1".to_string());
    focus_request_builder(&mut state);
    state.handle_key(key(KeyCode::Down));
    state.handle_key(key(KeyCode::Enter));
    type_text(&mut state, "X-Api-Version=999");

    state.handle_key(key(KeyCode::Enter));

    assert_eq!(
        state.request_builder.custom_headers,
        vec!["X-Api-Version".to_string()]
    );
    assert_eq!(state.request_builder.headers.inputs().get(&1), None);
    assert!(!state.is_editing());
}

#[test]
fn header_add_row_enter_duplicate_spec_name_cancels_without_adding_custom_header() {
    let mut state = AppState::new();
    state.set_header_param_names(vec!["Authorization".to_string()]);

    add_header_with_enter(&mut state, "Authorization=Bearer xyz");

    assert!(state.request_builder.custom_headers.is_empty());
    assert!(state.request_builder.headers.inputs().is_empty());
    assert!(!state.is_editing());
}

#[test]
fn header_add_row_non_commit_keys_behave_like_normal_editing() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    focus_request_builder(&mut state);
    state.handle_key(key(KeyCode::Enter));

    type_text(&mut state, "abc");

    assert!(state.is_editing());
    assert_eq!(state.request_builder.headers.editing_buffer(), Some("abc"));
    assert!(state.request_builder.custom_headers.is_empty());

    state.handle_key(key(KeyCode::Backspace));

    assert_eq!(state.request_builder.headers.editing_buffer(), Some("ab"));

    state.handle_key(key(KeyCode::Esc));

    assert!(!state.is_editing());
    assert!(state.request_builder.custom_headers.is_empty());
    assert!(state.request_builder.headers.inputs().is_empty());
}

#[test]
fn header_add_row_navigation_before_editing_behaves_normally() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(2);
    focus_request_builder(&mut state);

    state.handle_key(key(KeyCode::Down));

    assert_eq!(state.request_builder.headers.selected_row(), 1);
    assert!(!state.is_editing());
    assert!(state.request_builder.custom_headers.is_empty());
    assert!(state.request_builder.headers.inputs().is_empty());
}

#[test]
fn parameters_add_row_enter_adds_custom_query_param_name_and_stores_value() {
    let mut state = AppState::new();
    state.request_builder.parameters.set_row_count(1);
    focus_request_builder(&mut state);
    state.handle_key(right_bracket());
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Parameters);
    state.handle_key(key(KeyCode::Enter));
    type_text(&mut state, "region=us-west");

    state.handle_key(key(KeyCode::Enter));

    assert_eq!(
        state.request_builder.custom_query_params,
        vec!["region".to_string()]
    );
    assert_eq!(
        state
            .request_builder
            .parameters
            .inputs()
            .get(&0)
            .map(String::as_str),
        Some("us-west")
    );
    assert!(!state.is_editing());
    assert_eq!(state.request_builder.parameters.selected_row(), 1);
}

#[test]
fn parameters_add_row_enter_duplicate_spec_name_cancels_without_adding_custom_query_param() {
    let mut state = AppState::new();
    state.set_parameter_param_names(vec!["region".to_string()]);
    state.request_builder.parameters.set_row_count(1);
    focus_request_builder(&mut state);
    state.handle_key(right_bracket());
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Parameters);
    state.handle_key(key(KeyCode::Enter));
    type_text(&mut state, "region=us-west");

    state.handle_key(key(KeyCode::Enter));

    assert!(state.request_builder.custom_query_params.is_empty());
    assert!(state.request_builder.parameters.inputs().is_empty());
    assert!(!state.is_editing());
}

#[test]
fn editing_existing_custom_header_row_after_add_row_commit_uses_normal_commit_behavior() {
    let mut state = AppState::new();

    add_header_with_enter(&mut state, "X-Api-Version=2");
    state.request_builder.headers.set_row_count(2);
    state.handle_key(key(KeyCode::Up));
    state.handle_key(key(KeyCode::Enter));

    assert!(state.is_editing());
    assert_eq!(state.request_builder.headers.editing_buffer(), Some("2"));

    state.handle_key(key(KeyCode::Backspace));
    state.handle_key(key(KeyCode::Char('3')));
    state.handle_key(key(KeyCode::Enter));

    assert_eq!(
        state.request_builder.headers.inputs().get(&0).map(String::as_str),
        Some("3")
    );
    assert_eq!(
        state.request_builder.custom_headers,
        vec!["X-Api-Version".to_string()]
    );
    assert!(!state.is_editing());
}
