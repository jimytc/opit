use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use openapi_terminal_app::app::{AppState, Pane};

#[test]
fn new_app_state_focuses_endpoint_list() {
    let state = AppState::new();

    assert_eq!(state.focused, Pane::EndpointList);
}

#[test]
fn tab_cycles_forward_through_all_panes_and_wraps() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::AuthConfig);

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::RequestBuilder);

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::CurlPreview);

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::ResponseViewer);

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::EndpointList);
}

#[test]
fn backtab_from_endpoint_list_wraps_to_response_viewer() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT));

    assert_eq!(state.focused, Pane::ResponseViewer);
}

#[test]
fn unrelated_key_does_not_change_focus() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));

    assert_eq!(state.focused, Pane::EndpointList);
}

#[test]
fn curl_preview_does_not_enter_editing_or_handle_editing_keys() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));

    assert_eq!(state.focused, Pane::CurlPreview);
    assert!(!state.is_editing());

    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert_eq!(state.focused, Pane::CurlPreview);
    assert!(!state.is_editing());
}
