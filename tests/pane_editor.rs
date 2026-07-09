use std::collections::HashSet;

use openapi_terminal_app::app::PaneEditor;

#[test]
fn new_editor_has_default_state() {
    let editor = PaneEditor::new();

    assert_eq!(editor.selected_row(), 0);
    assert_eq!(editor.editing_buffer(), None);
    assert!(editor.inputs().is_empty());
}

#[test]
fn set_row_count_clamps_selected_row_when_count_shrinks() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(5);
    editor.move_down();
    editor.move_down();
    editor.move_down();
    editor.move_down();

    editor.set_row_count(2);

    assert_eq!(editor.selected_row(), 1);
}

#[test]
fn set_row_count_with_same_count_preserves_in_progress_editing_buffer() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(3);
    editor.start_editing();
    editor.push_char('a');

    editor.set_row_count(3);

    assert_eq!(editor.editing_buffer(), Some("a"));
}

#[test]
fn set_row_count_with_larger_count_preserves_committed_inputs() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(3);
    editor.start_editing();
    editor.push_char('a');
    editor.commit();

    editor.set_row_count(5);

    assert_eq!(editor.inputs().get(&0).map(String::as_str), Some("a"));
}

#[test]
fn row_count_is_zero_on_fresh_editor() {
    let editor = PaneEditor::new();

    assert_eq!(editor.row_count(), 0);
}

#[test]
fn row_count_returns_last_set_row_count() {
    let mut editor = PaneEditor::new();

    editor.set_row_count(7);

    assert_eq!(editor.row_count(), 7);
}

#[test]
fn row_count_returns_smaller_last_set_row_count_after_shrink() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(7);

    editor.set_row_count(2);

    assert_eq!(editor.row_count(), 2);
}

#[test]
fn move_up_is_bounded_at_first_row() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(3);

    editor.move_up();

    assert_eq!(editor.selected_row(), 0);
}

#[test]
fn move_down_is_bounded_at_last_row() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(2);

    editor.move_down();
    editor.move_down();

    assert_eq!(editor.selected_row(), 1);
}

#[test]
fn move_up_and_down_are_no_ops_when_row_count_is_zero() {
    let mut editor = PaneEditor::new();

    editor.move_down();
    editor.move_up();

    assert_eq!(editor.selected_row(), 0);
}

#[test]
fn moving_is_no_op_while_editing() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(3);
    editor.move_down();
    editor.start_editing();

    editor.move_down();
    editor.move_up();

    assert_eq!(editor.selected_row(), 1);
    assert_eq!(editor.editing_buffer(), Some(""));
}

#[test]
fn start_editing_prefills_from_existing_input() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);
    editor.start_editing();
    editor.push_char('x');
    editor.commit();

    editor.start_editing();

    assert_eq!(editor.editing_buffer(), Some("x"));
}

#[test]
fn start_editing_uses_empty_buffer_when_selected_row_has_no_input() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);

    editor.start_editing();

    assert_eq!(editor.editing_buffer(), Some(""));
}

#[test]
fn start_editing_is_no_op_when_row_count_is_zero() {
    let mut editor = PaneEditor::new();

    editor.start_editing();

    assert_eq!(editor.editing_buffer(), None);
}

#[test]
fn start_editing_is_no_op_when_selected_row_is_not_editable() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(2);
    editor.move_down();
    editor.set_non_editable_rows(HashSet::from([1]));

    editor.start_editing();

    assert_eq!(editor.selected_row(), 1);
    assert_eq!(editor.editing_buffer(), None);
}

#[test]
fn is_editing_multiline_row_is_false_on_fresh_editor() {
    let editor = PaneEditor::new();

    assert!(!editor.is_editing_multiline_row());
}

#[test]
fn is_editing_multiline_row_is_true_when_editing_selected_multiline_row() {
    let mut editor = PaneEditor::new();
    editor.set_multiline_rows(HashSet::from([0]));
    editor.set_row_count(2);

    editor.start_editing();

    assert!(editor.is_editing_multiline_row());
}

#[test]
fn is_editing_multiline_row_is_false_when_editing_non_multiline_row() {
    let mut editor = PaneEditor::new();
    editor.set_multiline_rows(HashSet::from([0]));
    editor.set_row_count(2);
    editor.move_down();

    editor.start_editing();

    assert_eq!(editor.selected_row(), 1);
    assert!(!editor.is_editing_multiline_row());
    assert_eq!(editor.editing_buffer(), Some(""));
}

#[test]
fn is_editing_multiline_row_is_false_when_multiline_row_selected_but_not_editing() {
    let mut editor = PaneEditor::new();
    editor.set_multiline_rows(HashSet::from([0]));
    editor.set_row_count(2);

    assert_eq!(editor.selected_row(), 0);
    assert_eq!(editor.editing_buffer(), None);
    assert!(!editor.is_editing_multiline_row());
}

#[test]
fn set_input_on_fresh_editor_stores_value_without_editing() {
    let mut editor = PaneEditor::new();

    editor.set_input(2, "stored".to_string());

    assert_eq!(editor.inputs().get(&2).map(String::as_str), Some("stored"));
    assert_eq!(editor.editing_buffer(), None);
}

#[test]
fn set_input_does_not_start_editing_multiline_row() {
    let mut editor = PaneEditor::new();
    editor.set_multiline_rows(HashSet::from([1]));

    editor.set_input(1, "stored".to_string());

    assert_eq!(editor.inputs().get(&1).map(String::as_str), Some("stored"));
    assert_eq!(editor.editing_buffer(), None);
    assert!(!editor.is_editing_multiline_row());
}

#[test]
fn set_input_overwrites_committed_value() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);
    editor.start_editing();
    editor.push_str("before");
    editor.commit();

    editor.set_input(0, "after".to_string());

    assert_eq!(editor.inputs().get(&0).map(String::as_str), Some("after"));
    assert_eq!(editor.editing_buffer(), None);
}

#[test]
fn select_row_sets_selected_row_without_clamping() {
    let mut editor = PaneEditor::new();

    editor.select_row(5);

    assert_eq!(editor.selected_row(), 5);
}

#[test]
fn select_row_does_not_affect_editing_buffer_or_inputs() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);
    editor.start_editing();
    editor.push_str("stored");
    editor.commit();

    editor.select_row(3);

    assert_eq!(editor.selected_row(), 3);
    assert_eq!(editor.inputs().get(&0).map(String::as_str), Some("stored"));
    assert_eq!(editor.editing_buffer(), None);
}

#[test]
fn push_char_is_no_op_when_not_editing() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);

    editor.push_char('x');

    assert_eq!(editor.editing_buffer(), None);
    assert!(editor.inputs().is_empty());
}

#[test]
fn push_str_appends_full_string_including_embedded_newline() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);
    editor.start_editing();

    editor.push_str("line one\nline two");

    assert_eq!(editor.editing_buffer(), Some("line one\nline two"));
}

#[test]
fn push_str_is_no_op_when_not_editing() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);

    editor.push_str("ignored");

    assert_eq!(editor.editing_buffer(), None);
    assert!(editor.inputs().is_empty());
}

#[test]
fn pop_char_is_no_op_when_not_editing() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);

    editor.pop_char();

    assert_eq!(editor.editing_buffer(), None);
    assert!(editor.inputs().is_empty());
}

#[test]
fn pop_char_on_empty_editing_buffer_keeps_buffer_empty() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);
    editor.start_editing();

    editor.pop_char();

    assert_eq!(editor.editing_buffer(), Some(""));
}

#[test]
fn commit_stores_buffer_at_selected_row_and_clears_editing_buffer() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(2);
    editor.move_down();
    editor.start_editing();
    editor.push_char('o');
    editor.push_char('k');

    editor.commit();

    assert_eq!(editor.inputs().get(&1).map(String::as_str), Some("ok"));
    assert_eq!(editor.editing_buffer(), None);
}

#[test]
fn commit_is_no_op_when_not_editing() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);

    editor.commit();

    assert!(editor.inputs().is_empty());
    assert_eq!(editor.editing_buffer(), None);
}

#[test]
fn cancel_clears_buffer_without_replacing_prior_committed_input() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);
    editor.start_editing();
    for c in "first".chars() {
        editor.push_char(c);
    }
    editor.commit();

    editor.start_editing();
    for c in "second".chars() {
        editor.push_char(c);
    }
    editor.cancel();

    assert_eq!(editor.inputs().get(&0), Some(&"first".to_string()));
    assert_eq!(editor.editing_buffer(), None);
}

#[test]
fn cancel_is_no_op_when_not_editing() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);

    editor.cancel();

    assert_eq!(editor.editing_buffer(), None);
    assert!(editor.inputs().is_empty());
}

#[test]
fn reset_clears_inputs_editing_buffer_and_selected_row() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(3);
    editor.move_down();
    editor.start_editing();
    editor.push_char('x');
    editor.commit();
    editor.move_down();
    editor.start_editing();
    editor.push_char('y');

    editor.reset();

    assert_eq!(editor.selected_row(), 0);
    assert_eq!(editor.editing_buffer(), None);
    assert!(editor.inputs().is_empty());
}

#[test]
fn inputs_with_live_edit_matches_inputs_when_not_editing() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(2);
    editor.start_editing();
    for c in "committed".chars() {
        editor.push_char(c);
    }
    editor.commit();

    let inputs_with_live_edit = editor.inputs_with_live_edit();

    assert_eq!(inputs_with_live_edit, editor.inputs().clone());
    assert_eq!(inputs_with_live_edit.len(), 1);
    assert_eq!(
        inputs_with_live_edit.get(&0),
        Some(&"committed".to_string())
    );
}

#[test]
fn inputs_with_live_edit_includes_uncommitted_value_for_empty_selected_row() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);
    editor.start_editing();
    editor.push_char('a');
    editor.push_char('b');

    assert_eq!(editor.editing_buffer(), Some("ab"));
    assert!(editor.inputs().is_empty());

    let inputs_with_live_edit = editor.inputs_with_live_edit();

    assert_eq!(inputs_with_live_edit.len(), 1);
    assert_eq!(inputs_with_live_edit.get(&0), Some(&"ab".to_string()));
    assert!(editor.inputs().is_empty());
}

#[test]
fn inputs_with_live_edit_uses_uncommitted_value_over_committed_value() {
    let mut editor = PaneEditor::new();
    editor.set_row_count(1);
    editor.start_editing();
    editor.push_char('o');
    editor.push_char('l');
    editor.push_char('d');
    editor.commit();

    editor.start_editing();
    editor.pop_char();
    editor.pop_char();
    editor.pop_char();
    editor.push_char('n');
    editor.push_char('e');
    editor.push_char('w');

    assert_eq!(editor.editing_buffer(), Some("new"));
    assert_eq!(editor.inputs().get(&0), Some(&"old".to_string()));

    let inputs_with_live_edit = editor.inputs_with_live_edit();

    assert_eq!(inputs_with_live_edit.len(), 1);
    assert_eq!(inputs_with_live_edit.get(&0), Some(&"new".to_string()));
    assert_eq!(editor.inputs().get(&0), Some(&"old".to_string()));
}
