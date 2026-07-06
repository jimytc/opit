use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use openapi_terminal_app::app::{AppState, Pane};

#[test]
fn new_app_state_starts_with_first_operation_selected() {
    let state = AppState::new();

    assert_eq!(state.selected_operation_index, 0);
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
