use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use openapi_terminal_app::app::{AppState, Pane};

#[test]
fn new_app_state_focuses_endpoint_list() {
    let state = AppState::new();

    assert_eq!(state.focused, Pane::EndpointList);
}

#[test]
fn new_app_state_starts_read_only_pane_scrolls_at_zero() {
    let state = AppState::new();

    assert_eq!(state.curl_preview_scroll(), 0);
    assert_eq!(state.response_viewer_scroll(), 0);
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

#[test]
fn curl_preview_up_and_down_change_curl_preview_scroll() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::CurlPreview);

    state.set_curl_preview_max_scroll(5);
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(state.curl_preview_scroll(), 3);

    state.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(state.curl_preview_scroll(), 2);
}

#[test]
fn curl_preview_down_stops_at_max_scroll() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::CurlPreview);

    state.set_curl_preview_max_scroll(2);
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    assert_eq!(state.curl_preview_scroll(), 2);
}

#[test]
fn curl_preview_up_at_zero_keeps_scroll_at_zero() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::CurlPreview);

    state.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));

    assert_eq!(state.curl_preview_scroll(), 0);
}

#[test]
fn response_viewer_up_and_down_change_response_viewer_scroll_without_affecting_curl_preview() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::ResponseViewer);

    state.set_response_viewer_max_scroll(5);
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    assert_eq!(state.response_viewer_scroll(), 2);
    assert_eq!(state.curl_preview_scroll(), 0);
}

#[test]
fn set_curl_preview_max_scroll_clamps_existing_scroll_when_max_shrinks() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::CurlPreview);

    state.set_curl_preview_max_scroll(5);
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(state.curl_preview_scroll(), 5);

    state.set_curl_preview_max_scroll(3);

    assert_eq!(state.curl_preview_scroll(), 3);
}

#[test]
fn endpoint_list_up_and_down_do_not_change_read_only_pane_scrolls() {
    let mut state = AppState::new();

    assert_eq!(state.focused, Pane::EndpointList);

    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));

    assert_eq!(state.curl_preview_scroll(), 0);
    assert_eq!(state.response_viewer_scroll(), 0);
}
