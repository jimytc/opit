use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use openapi_terminal_app::app::{AppState, Pane, RequestBuilderTab};

fn pane_jump_modifiers() -> KeyModifiers {
    KeyModifiers::CONTROL | KeyModifiers::ALT
}

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
fn new_app_state_starts_on_header_request_builder_tab() {
    let state = AppState::new();

    assert_eq!(state.request_builder_tab, RequestBuilderTab::Header);
}

#[test]
fn right_bracket_cycles_request_builder_tabs_forward_when_request_builder_is_focused() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::RequestBuilder);

    state.handle_key(KeyEvent::new(KeyCode::Char(']'), KeyModifiers::NONE));
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Parameters);

    state.handle_key(KeyEvent::new(KeyCode::Char(']'), KeyModifiers::NONE));
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Payload);

    state.handle_key(KeyEvent::new(KeyCode::Char(']'), KeyModifiers::NONE));
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Header);
}

#[test]
fn left_bracket_cycles_request_builder_tabs_backward_when_request_builder_is_focused() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::RequestBuilder);

    state.handle_key(KeyEvent::new(KeyCode::Char('['), KeyModifiers::NONE));
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Payload);

    state.handle_key(KeyEvent::new(KeyCode::Char('['), KeyModifiers::NONE));
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Parameters);

    state.handle_key(KeyEvent::new(KeyCode::Char('['), KeyModifiers::NONE));
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Header);
}

#[test]
fn brackets_do_not_cycle_request_builder_tabs_when_endpoint_list_is_focused() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Char('['), KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::EndpointList);
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Header);

    state.handle_key(KeyEvent::new(KeyCode::Char(']'), KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::EndpointList);
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Header);
}

#[test]
fn bracket_is_typed_when_request_builder_is_editing() {
    let mut state = AppState::new();
    state.request_builder.headers.set_row_count(1);

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char(']'), KeyModifiers::NONE));

    assert_eq!(state.focused, Pane::RequestBuilder);
    assert!(state.is_editing());
    assert_eq!(state.request_builder_tab, RequestBuilderTab::Header);
    assert_eq!(state.request_builder.headers.editing_buffer(), Some("]"));
}

#[test]
fn digit_four_jumps_directly_to_curl_preview() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Char('4'), pane_jump_modifiers()));

    assert_eq!(state.focused, Pane::CurlPreview);
}

#[test]
fn digit_one_jumps_back_to_endpoint_list_from_another_pane() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::AuthConfig);

    state.handle_key(KeyEvent::new(KeyCode::Char('1'), pane_jump_modifiers()));

    assert_eq!(state.focused, Pane::EndpointList);
}

#[test]
fn digit_keys_jump_to_their_mapped_panes() {
    for (digit, pane) in [
        ('1', Pane::EndpointList),
        ('2', Pane::AuthConfig),
        ('3', Pane::RequestBuilder),
        ('4', Pane::CurlPreview),
        ('5', Pane::ResponseViewer),
    ] {
        let mut state = AppState::new();

        state.handle_key(KeyEvent::new(KeyCode::Char(digit), pane_jump_modifiers()));

        assert_eq!(state.focused, pane);
    }
}

#[test]
fn out_of_range_digits_and_other_characters_do_not_change_focus() {
    for c in ['0', '6', 'x'] {
        let mut state = AppState::new();

        state.handle_key(KeyEvent::new(KeyCode::Char(c), pane_jump_modifiers()));

        assert_eq!(state.focused, Pane::EndpointList);
    }
}

#[test]
fn plain_digit_without_modifiers_does_not_jump_focus() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE));

    assert_eq!(state.focused, Pane::EndpointList);
}

#[test]
fn jumping_to_pane_preserves_selection_and_scrolls() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.set_curl_preview_max_scroll(5);
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    state.set_response_viewer_max_scroll(5);
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    state.handle_key(KeyEvent::new(KeyCode::Char('2'), pane_jump_modifiers()));

    assert_eq!(state.focused, Pane::AuthConfig);
    assert_eq!(state.selected_operation_index, 1);
    assert_eq!(state.curl_preview_scroll(), 2);
    assert_eq!(state.response_viewer_scroll(), 1);
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
