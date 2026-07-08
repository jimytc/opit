use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use openapi_terminal_app::app::{AppState, Pane};

#[test]
fn new_app_state_starts_with_first_operation_selected() {
    let state = AppState::new();

    assert_eq!(state.selected_operation_index, 0);
}

#[test]
fn new_app_state_starts_with_first_server_selected() {
    let state = AppState::new();

    assert_eq!(state.selected_server_index, 0);
}

#[test]
fn new_app_state_starts_with_empty_endpoint_filter() {
    let state = AppState::new();

    assert_eq!(state.endpoint_filter, "");
}

#[test]
fn set_operation_count_clamps_selected_operation_index_when_count_shrinks() {
    let mut state = AppState::new();
    state.set_operation_count(4);
    state.selected_operation_index = 3;

    state.set_operation_count(2);

    assert_eq!(state.selected_operation_index, 1);
}

#[test]
fn down_moves_selection_until_last_operation_and_clamps() {
    let mut state = AppState::new();
    state.set_operation_count(3);

    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(state.selected_operation_index, 1);

    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(state.selected_operation_index, 2);

    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(state.selected_operation_index, 2);
}

#[test]
fn up_from_first_operation_stays_at_zero() {
    let mut state = AppState::new();
    state.set_operation_count(3);

    state.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));

    assert_eq!(state.selected_operation_index, 0);
}

#[test]
fn up_and_down_do_not_change_selection_when_endpoint_list_is_not_focused() {
    let mut state = AppState::new();
    state.set_operation_count(3);

    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(state.selected_operation_index, 1);

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::RequestBuilder);

    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(state.selected_operation_index, 1);
}

#[test]
fn s_cycles_selected_server_index_and_wraps() {
    let mut state = AppState::new();
    state.set_server_count(3);

    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(state.selected_server_index, 1);

    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(state.selected_server_index, 2);

    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(state.selected_server_index, 0);
}

#[test]
fn s_does_not_change_selected_server_index_when_server_count_is_zero() {
    let mut state = AppState::new();
    state.set_server_count(0);

    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));

    assert_eq!(state.selected_server_index, 0);
}

#[test]
fn s_does_not_change_selected_server_index_when_endpoint_list_is_not_focused() {
    let mut state = AppState::new();
    state.set_server_count(3);

    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(state.selected_server_index, 1);

    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(state.focused, Pane::RequestBuilder);

    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(state.selected_server_index, 1);
}

#[test]
fn slash_starts_endpoint_filtering_when_endpoint_list_is_focused() {
    let mut state = AppState::new();

    state.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));

    assert_eq!(state.focused, Pane::EndpointList);
    assert!(state.is_editing());
    assert_eq!(state.endpoint_filter, "");
}

#[test]
fn endpoint_filter_appends_characters_backspaces_and_enter_keeps_text() {
    let mut state = AppState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));

    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert!(!state.is_editing());
    assert_eq!(state.endpoint_filter, "ge");
}

#[test]
fn endpoint_filter_esc_exits_filtering_and_clears_text() {
    let mut state = AppState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));

    state.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert!(!state.is_editing());
    assert_eq!(state.endpoint_filter, "");
}

#[test]
fn endpoint_filter_consumes_up_and_down_without_changing_selection() {
    let mut state = AppState::new();
    state.set_operation_count(3);
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(state.selected_operation_index, 1);

    state.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));

    assert_eq!(state.selected_operation_index, 1);
    assert!(state.is_editing());
}

#[test]
fn endpoint_filter_treats_s_as_filter_text_without_cycling_server() {
    let mut state = AppState::new();
    state.set_server_count(3);
    state.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));

    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));

    assert_eq!(state.selected_server_index, 0);
    assert_eq!(state.endpoint_filter, "s");
    assert!(state.is_editing());
}
