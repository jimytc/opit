use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use openapi_terminal_app::app::{AppState, Pane};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn tab() -> KeyEvent {
    key(KeyCode::Tab)
}

fn backtab() -> KeyEvent {
    KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT)
}

fn commit_request_builder_input(state: &mut AppState, value: &str) {
    state.request_builder.set_row_count(1);
    state.request_builder.start_editing();
    for c in value.chars() {
        state.request_builder.push_char(c);
    }
    state.request_builder.commit();
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
    state.request_builder.set_row_count(2);

    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    assert_eq!(state.focused, Pane::RequestBuilder);
    assert!(state.is_editing());
    assert_eq!(state.request_builder.editing_buffer(), Some(""));
}

#[test]
fn request_builder_char_backspace_and_enter_commit_through_handle_key() {
    let mut state = AppState::new();
    state.request_builder.set_row_count(2);
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_key(key(KeyCode::Char('a')));
    state.handle_key(key(KeyCode::Char('b')));
    assert_eq!(state.request_builder.editing_buffer(), Some("ab"));

    state.handle_key(key(KeyCode::Backspace));
    assert_eq!(state.request_builder.editing_buffer(), Some("a"));

    state.handle_key(key(KeyCode::Enter));

    assert!(!state.is_editing());
    assert_eq!(
        state.request_builder.inputs().get(&0),
        Some(&"a".to_string())
    );
}

#[test]
fn request_builder_esc_cancels_without_committing() {
    let mut state = AppState::new();
    state.request_builder.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_key(key(KeyCode::Char('x')));
    state.handle_key(key(KeyCode::Char('y')));
    state.handle_key(key(KeyCode::Esc));

    assert!(!state.is_editing());
    assert_eq!(state.request_builder.editing_buffer(), None);
    assert_eq!(state.request_builder.inputs().get(&0), None);
}

#[test]
fn tab_is_no_op_while_editing_request_builder() {
    let mut state = AppState::new();
    state.request_builder.set_row_count(1);
    state.handle_key(tab());
    state.handle_key(key(KeyCode::Enter));

    state.handle_key(tab());
    state.handle_key(backtab());

    assert_eq!(state.focused, Pane::RequestBuilder);
    assert!(state.is_editing());
    assert_eq!(state.request_builder.editing_buffer(), Some(""));
}

#[test]
fn auth_config_routes_editing_keys_without_affecting_request_builder() {
    let mut state = AppState::new();
    state.auth_config.set_row_count(1);
    state.handle_key(tab());
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
    assert!(state.request_builder.inputs().is_empty());
    assert_eq!(state.request_builder.editing_buffer(), None);
}

#[test]
fn endpoint_down_that_changes_selection_resets_request_builder() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    commit_request_builder_input(&mut state, "x");

    state.handle_key(key(KeyCode::Down));

    assert_eq!(state.selected_operation_index, 1);
    assert!(state.request_builder.inputs().is_empty());
    assert_eq!(state.request_builder.editing_buffer(), None);
}

#[test]
fn endpoint_up_at_first_selection_does_not_reset_request_builder() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    commit_request_builder_input(&mut state, "x");

    state.handle_key(key(KeyCode::Up));

    assert_eq!(state.selected_operation_index, 0);
    assert_eq!(
        state.request_builder.inputs().get(&0),
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
fn response_viewer_ignores_editing_and_navigation_keys() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    commit_request_builder_input(&mut state, "request");
    commit_auth_config_input(&mut state, "auth");

    state.handle_key(tab());
    state.handle_key(tab());
    state.handle_key(tab());
    assert_eq!(state.focused, Pane::ResponseViewer);

    state.request_builder.start_editing();
    state.request_builder.push_char('!');
    state.auth_config.start_editing();
    state.auth_config.push_char('?');

    let selected_operation_index_before = state.selected_operation_index;
    let request_inputs_before = state.request_builder.inputs().clone();
    let request_buffer_before = state.request_builder.editing_buffer().map(str::to_owned);
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
    assert_eq!(state.request_builder.inputs(), &request_inputs_before);
    assert_eq!(
        state.request_builder.editing_buffer(),
        request_buffer_before.as_deref()
    );
    assert_eq!(state.auth_config.inputs(), &auth_inputs_before);
    assert_eq!(
        state.auth_config.editing_buffer(),
        auth_buffer_before.as_deref()
    );
}
