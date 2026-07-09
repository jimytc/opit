use std::collections::HashSet;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use openapi_terminal_app::app::{AppState, Pane, RequestBuilderTab};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn tab() -> KeyEvent {
    key(KeyCode::Tab)
}

fn backtab() -> KeyEvent {
    KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT)
}

fn ctrl_s() -> KeyEvent {
    KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL)
}

fn commit_request_builder_input(state: &mut AppState, value: &str) {
    state.request_builder.headers.set_row_count(1);
    state.request_builder.headers.start_editing();
    for c in value.chars() {
        state.request_builder.headers.push_char(c);
    }
    state.request_builder.headers.commit();
}

fn commit_auth_config_input(state: &mut AppState, value: &str) {
    state.auth_config.set_row_count(1);
    state.auth_config.start_editing();
    for c in value.chars() {
        state.auth_config.push_char(c);
    }
    state.auth_config.commit();
}

#[test]
fn is_editing_is_false_on_fresh_app_state() {
    let state = AppState::new();

    assert!(!state.is_editing());
}

#[test]
fn enter_starts_editing_request_builder_when_focused() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(2);

    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    assert_eq!(state.focused, Pane::RequestBuilder);
    assert!(state.is_editing());
    assert_eq!(state.request_builder.headers.editing_buffer(), Some(""));
}

#[test]
fn request_builder_char_backspace_and_enter_commit_through_handle_key() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(2);
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_key(key(KeyCode::Char('a')));
    state.handle_key(key(KeyCode::Char('b')));
    assert_eq!(state.request_builder.headers.editing_buffer(), Some("ab"));

    state.handle_key(key(KeyCode::Backspace));
    assert_eq!(state.request_builder.headers.editing_buffer(), Some("a"));

    state.handle_key(key(KeyCode::Enter));

    assert!(!state.is_editing());
    assert_eq!(
        state.request_builder.headers.inputs().get(&0),
        Some(&"a".to_string())
    );
}

#[test]
fn request_builder_enter_inserts_newline_when_editing_multiline_row() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    state
        .request_builder
        .headers
        .set_multiline_rows(HashSet::from([0]));
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_key(key(KeyCode::Char('a')));
    state.handle_key(key(KeyCode::Char('b')));
    state.handle_key(key(KeyCode::Enter));
    state.handle_key(key(KeyCode::Char('c')));
    state.handle_key(key(KeyCode::Char('d')));

    assert!(state.is_editing());
    assert_eq!(
        state.request_builder.headers.editing_buffer(),
        Some("ab\ncd")
    );
    assert!(state.request_builder.headers.inputs().is_empty());
}

#[test]
fn request_builder_enter_commits_non_multiline_row() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_key(key(KeyCode::Char('a')));
    state.handle_key(key(KeyCode::Enter));

    assert!(!state.is_editing());
    assert_eq!(
        state.request_builder.headers.inputs().get(&0),
        Some(&"a".to_string())
    );
}

#[test]
fn ctrl_s_commits_multiline_request_builder_buffer() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    state
        .request_builder
        .headers
        .set_multiline_rows(HashSet::from([0]));
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));
    state.handle_key(key(KeyCode::Char('a')));
    state.handle_key(key(KeyCode::Enter));
    state.handle_key(key(KeyCode::Char('b')));

    state.handle_key(ctrl_s());

    assert!(!state.is_editing());
    assert_eq!(
        state.request_builder.headers.inputs().get(&0),
        Some(&"a\nb".to_string())
    );
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
}

#[test]
fn ctrl_s_commits_auth_config_buffer() {
    let mut state = AppState::new();
    state.auth_config.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));
    state.handle_key(key(KeyCode::Char('s')));
    state.handle_key(key(KeyCode::Char('e')));
    state.handle_key(key(KeyCode::Char('c')));

    state.handle_key(ctrl_s());

    assert!(!state.is_editing());
    assert_eq!(state.auth_config.inputs().get(&0), Some(&"sec".to_string()));
    assert_eq!(state.auth_config.editing_buffer(), None);
}

#[test]
fn ctrl_s_is_no_op_when_not_editing() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);

    state.handle_key(ctrl_s());

    assert!(!state.is_editing());
    assert_eq!(state.focused, Pane::EndpointList);
    assert!(state.request_builder.headers.inputs().is_empty());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
    assert!(state.auth_config.inputs().is_empty());
    assert_eq!(state.auth_config.editing_buffer(), None);
}

#[test]
fn request_builder_esc_cancels_without_committing() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_key(key(KeyCode::Char('x')));
    state.handle_key(key(KeyCode::Char('y')));
    state.handle_key(key(KeyCode::Esc));

    assert!(!state.is_editing());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
    assert_eq!(state.request_builder.headers.inputs().get(&0), None);
}

#[test]
fn tab_is_no_op_while_editing_request_builder() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_key(tab());
    state.handle_key(backtab());

    assert_eq!(state.focused, Pane::RequestBuilder);
    assert!(state.is_editing());
    assert_eq!(state.request_builder.headers.editing_buffer(), Some(""));
}

#[test]
fn endpoint_filter_treats_digit_as_filter_text_not_focus_jump() {
    let mut state = AppState::new();

    state.handle_key(key(KeyCode::Char('/')));
    state.handle_key(key(KeyCode::Char('2')));

    assert_eq!(state.focused, Pane::EndpointList);
    assert!(state.is_editing());
    assert_eq!(state.endpoint_filter, "2");
}

#[test]
fn request_builder_editing_treats_digit_as_input_not_focus_jump() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);

    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));
    state.handle_key(key(KeyCode::Char('3')));

    assert_eq!(state.focused, Pane::RequestBuilder);
    assert!(state.is_editing());
    assert_eq!(state.request_builder.headers.editing_buffer(), Some("3"));
}

#[test]
fn auth_config_routes_editing_keys_without_affecting_request_builder() {
    let mut state = AppState::new();
    state.auth_config.set_row_count(1);
    state.handle_key(tab());

    state.handle_key(key(KeyCode::Enter));
    state.handle_key(key(KeyCode::Char('a')));
    state.handle_key(key(KeyCode::Char('b')));
    state.handle_key(key(KeyCode::Backspace));
    assert_eq!(state.focused, Pane::AuthConfig);
    assert!(state.is_editing());
    assert_eq!(state.auth_config.editing_buffer(), Some("a"));

    state.handle_key(key(KeyCode::Esc));
    assert!(!state.is_editing());
    assert!(state.auth_config.inputs().is_empty());

    state.handle_key(key(KeyCode::Enter));
    state.handle_key(key(KeyCode::Char('z')));
    state.handle_key(key(KeyCode::Enter));

    assert_eq!(state.auth_config.inputs().get(&0), Some(&"z".to_string()));
    assert!(state.request_builder.headers.inputs().is_empty());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
}

#[test]
fn handle_paste_appends_to_editing_request_builder() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_paste("pasted\ntext");

    assert_eq!(
        state.request_builder.headers.editing_buffer(),
        Some("pasted\ntext")
    );
    assert!(state.request_builder.headers.inputs().is_empty());
}

#[test]
fn handle_paste_appends_to_editing_auth_config_without_affecting_request_builder() {
    let mut state = AppState::new();
    state.auth_config.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_paste("secret");

    assert_eq!(state.auth_config.editing_buffer(), Some("secret"));
    assert!(state.auth_config.inputs().is_empty());
    assert!(state.request_builder.headers.inputs().is_empty());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
}

#[test]
fn handle_paste_is_no_op_when_endpoint_list_is_focused() {
    let mut state = AppState::new();

    state.handle_paste("ignored");

    assert_eq!(state.focused, Pane::EndpointList);
    assert!(state.request_builder.headers.inputs().is_empty());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
    assert!(state.auth_config.inputs().is_empty());
    assert_eq!(state.auth_config.editing_buffer(), None);
}

#[test]
fn handle_paste_is_no_op_when_request_builder_is_focused_but_not_editing() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(tab());

    state.handle_paste("ignored");

    assert_eq!(state.focused, Pane::RequestBuilder);
    assert!(state.request_builder.headers.inputs().is_empty());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
}

#[test]
fn endpoint_down_that_changes_selection_resets_request_builder() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    commit_request_builder_input(&mut state, "x");

    state.handle_key(key(KeyCode::Down));

    assert_eq!(state.selected_operation_index, 1);
    assert!(state.request_builder.headers.inputs().is_empty());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
}

#[test]
fn endpoint_up_at_first_selection_does_not_reset_request_builder() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    commit_request_builder_input(&mut state, "x");

    state.handle_key(key(KeyCode::Up));

    assert_eq!(state.selected_operation_index, 0);
    assert_eq!(
        state.request_builder.headers.inputs().get(&0),
        Some(&"x".to_string())
    );
}

#[test]
fn endpoint_navigation_never_resets_auth_config() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    commit_auth_config_input(&mut state, "secret");
    let auth_inputs_before = state.auth_config.inputs().clone();

    state.handle_key(key(KeyCode::Down));

    assert_eq!(state.selected_operation_index, 1);
    assert_eq!(state.auth_config.inputs(), &auth_inputs_before);
}

#[test]
fn sync_selected_operation_resets_request_builder_when_identity_changes_at_same_index() {
    let mut state = AppState::new();
    state.sync_selected_operation(Some(("GET", "/pets")));
    commit_request_builder_input(&mut state, "x");

    state.sync_selected_operation(Some(("POST", "/pets")));

    assert!(state.request_builder.headers.inputs().is_empty());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
    assert_eq!(state.selected_operation_index, 0);
}

#[test]
fn sync_selected_operation_resets_request_builder_fields_but_preserves_current_request_builder_tab()
{
    let mut state = AppState::new();
    state.sync_selected_operation(Some(("GET", "/pets")));

    commit_request_builder_input(&mut state, "header");
    state.request_builder.parameters.set_row_count(1);
    state.request_builder.parameters.start_editing();
    state.request_builder.parameters.push_char('p');
    state.request_builder.parameters.commit();
    state.request_builder.payload.set_row_count(1);
    state.request_builder.payload.start_editing();
    state.request_builder.payload.push_char('b');
    state.request_builder.payload.commit();
    state
        .request_builder
        .custom_headers
        .push("x-custom: value".to_string());
    state
        .request_builder
        .custom_query_params
        .push("debug=true".to_string());

    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Char(']')));
    state.handle_key(key(KeyCode::Char(']')));
    assert_eq!(state.focused, Pane::RequestBuilder);
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Payload);

    state.sync_selected_operation(Some(("POST", "/pets")));

    assert_eq!(state.request_builder_tab, RequestBuilderTab::Payload);
    assert!(state.request_builder.headers.inputs().is_empty());
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
    assert!(state.request_builder.parameters.inputs().is_empty());
    assert_eq!(state.request_builder.parameters.editing_buffer(), None);
    assert!(state.request_builder.payload.inputs().is_empty());
    assert_eq!(state.request_builder.payload.editing_buffer(), None);
    assert!(state.request_builder.custom_headers.is_empty());
    assert!(state.request_builder.custom_query_params.is_empty());
    assert_eq!(state.selected_operation_index, 0);
}

#[test]
fn sync_selected_operation_does_not_reset_request_builder_when_identity_is_unchanged_at_same_index()
{
    let mut state = AppState::new();
    state.sync_selected_operation(Some(("GET", "/pets")));
    commit_request_builder_input(&mut state, "x");

    state.sync_selected_operation(Some(("GET", "/pets")));

    assert_eq!(
        state.request_builder.headers.inputs().get(&0),
        Some(&"x".to_string())
    );
    assert_eq!(state.request_builder.headers.editing_buffer(), None);
    assert_eq!(state.selected_operation_index, 0);
}

#[test]
fn response_viewer_ignores_editing_and_navigation_keys() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    commit_request_builder_input(&mut state, "request");
    commit_auth_config_input(&mut state, "auth");

    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(tab());
    assert_eq!(state.focused, Pane::ResponseViewer);

    state.request_builder.headers.start_editing();
    state.request_builder.headers.push_char('!');
    state.auth_config.start_editing();
    state.auth_config.push_char('?');

    let selected_operation_index_before = state.selected_operation_index;
    let request_inputs_before = state.request_builder.headers.inputs().clone();
    let request_buffer_before = state
        .request_builder
        .headers
        .editing_buffer()
        .map(str::to_owned);
    let auth_inputs_before = state.auth_config.inputs().clone();
    let auth_buffer_before = state.auth_config.editing_buffer().map(str::to_owned);

    state.handle_key(key(KeyCode::Down));
    state.handle_key(key(KeyCode::Enter));
    state.handle_key(key(KeyCode::Char('x')));
    state.handle_key(key(KeyCode::Backspace));
    state.handle_key(key(KeyCode::Esc));

    assert_eq!(
        state.selected_operation_index,
        selected_operation_index_before
    );
    assert_eq!(
        state.request_builder.headers.inputs(),
        &request_inputs_before
    );
    assert_eq!(
        state.request_builder.headers.editing_buffer(),
        request_buffer_before.as_deref()
    );
    assert_eq!(state.auth_config.inputs(), &auth_inputs_before);
    assert_eq!(
        state.auth_config.editing_buffer(),
        auth_buffer_before.as_deref()
    );
}

#[test]
fn response_viewer_up_and_down_scroll_without_affecting_editable_panes_or_selection() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    commit_request_builder_input(&mut state, "request");
    commit_auth_config_input(&mut state, "auth");

    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(tab());
    assert_eq!(state.focused, Pane::ResponseViewer);

    state.request_builder.headers.start_editing();
    state.request_builder.headers.push_char('!');
    state.auth_config.start_editing();
    state.auth_config.push_char('?');

    let selected_operation_index_before = state.selected_operation_index;
    let request_inputs_before = state.request_builder.headers.inputs().clone();
    let request_buffer_before = state
        .request_builder
        .headers
        .editing_buffer()
        .map(str::to_owned);
    let auth_inputs_before = state.auth_config.inputs().clone();
    let auth_buffer_before = state.auth_config.editing_buffer().map(str::to_owned);

    state.set_response_viewer_max_scroll(5);
    state.handle_key(key(KeyCode::Down));
    state.handle_key(key(KeyCode::Down));
    state.handle_key(key(KeyCode::Up));

    assert_eq!(state.response_viewer_scroll(), 1);
    assert_eq!(
        state.selected_operation_index,
        selected_operation_index_before
    );
    assert_eq!(
        state.request_builder.headers.inputs(),
        &request_inputs_before
    );
    assert_eq!(
        state.request_builder.headers.editing_buffer(),
        request_buffer_before.as_deref()
    );
    assert_eq!(state.auth_config.inputs(), &auth_inputs_before);
    assert_eq!(
        state.auth_config.editing_buffer(),
        auth_buffer_before.as_deref()
    );
}
