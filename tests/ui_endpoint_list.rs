use openapi_terminal_app::spec::Operation;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::{ListState, StatefulWidget, Widget},
};

#[test]
fn endpoint_list_renders_method_and_path_for_each_operation_in_untagged_group() {
    let operations = vec![
        operation("/pets", "GET", None, vec![]),
        operation("/pets", "POST", None, vec![]),
    ];
    let filtered = operation_refs(&operations);

    let (widget, _highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let area = Rect::new(0, 0, 20, 4);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);

    assert!(
        line_0.contains("Untagged"),
        "expected row 0 to contain the Untagged header, got {line_0:?}"
    );
    assert!(
        line_1.contains("GET /pets"),
        "expected row 1 to contain 'GET /pets', got {line_1:?}"
    );
    assert!(
        line_2.contains("POST /pets"),
        "expected row 2 to contain 'POST /pets', got {line_2:?}"
    );
}

#[test]
fn endpoint_list_renders_summary_on_indented_second_line_after_group_header() {
    let operations = vec![operation("/pets", "GET", Some("List all pets"), vec![])];
    let filtered = operation_refs(&operations);

    let (widget, _highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let area = Rect::new(0, 0, 40, 3);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);

    assert!(
        line_0.contains("Untagged"),
        "expected row 0 to contain the Untagged header, got {line_0:?}"
    );
    assert!(
        line_1.contains("GET /pets"),
        "expected row 1 to contain 'GET /pets', got {line_1:?}"
    );
    assert!(
        line_2.contains("  List all pets"),
        "expected row 2 to contain indented summary, got {line_2:?}"
    );
}

#[test]
fn endpoint_list_without_summary_stays_single_line_after_group_header() {
    let operations = vec![operation("/pets", "GET", None, vec![])];
    let filtered = operation_refs(&operations);

    let (widget, _highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let area = Rect::new(0, 0, 40, 3);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);

    assert!(
        line_0.contains("Untagged"),
        "expected row 0 to contain the Untagged header, got {line_0:?}"
    );
    assert!(
        line_1.contains("GET /pets"),
        "expected row 1 to contain 'GET /pets', got {line_1:?}"
    );
    assert!(
        line_2.trim().is_empty(),
        "expected row 2 to stay empty when summary is None, got {line_2:?}"
    );
}

#[test]
fn endpoint_list_mixes_summary_and_plain_operations_without_row_drift_after_group_header() {
    let operations = vec![
        operation("/pets", "GET", Some("List all pets"), vec![]),
        operation("/pets", "POST", None, vec![]),
    ];
    let filtered = operation_refs(&operations);

    let (widget, _highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);
    let line_3 = row_text(&buffer, area, 3);
    let line_4 = row_text(&buffer, area, 4);

    assert!(
        line_0.contains("Untagged"),
        "expected row 0 to contain the Untagged header, got {line_0:?}"
    );
    assert!(
        line_1.contains("GET /pets"),
        "expected row 1 to contain 'GET /pets', got {line_1:?}"
    );
    assert!(
        line_2.contains("  List all pets"),
        "expected row 2 to contain indented summary, got {line_2:?}"
    );
    assert!(
        line_3.contains("POST /pets"),
        "expected row 3 to contain 'POST /pets' after summary row, got {line_3:?}"
    );
    assert!(
        line_4.trim().is_empty(),
        "expected row 4 to stay empty because POST has no summary, got {line_4:?}"
    );
}

#[test]
fn endpoint_list_applies_reversed_highlight_style_to_selected_operation_row_only() {
    let operations = vec![
        operation("/pets", "GET", None, vec![]),
        operation("/pets", "POST", None, vec![]),
    ];
    let filtered = operation_refs(&operations);

    let (widget, highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let area = Rect::new(0, 0, 20, 4);
    let mut buffer = Buffer::empty(area);
    let mut list_state = ListState::default();
    list_state.select(Some(highlight_index));

    StatefulWidget::render(widget, area, &mut buffer, &mut list_state);

    assert_eq!(
        highlight_index, 1,
        "expected selected operation 0 to map to row 1 after the group header"
    );
    assert!(
        !row_has_reversed_modifier(&buffer, area, 0),
        "expected header row 0 to not have REVERSED highlight style"
    );
    assert!(
        row_has_reversed_modifier(&buffer, area, 1),
        "expected selected operation row 1 to have REVERSED highlight style"
    );
    assert!(
        !row_has_reversed_modifier(&buffer, area, 2),
        "expected unselected operation row 2 to not have REVERSED highlight style"
    );
}

#[test]
fn endpoint_list_groups_operations_by_first_tag_in_first_appearance_order() {
    let operations = vec![
        operation("/zebra/one", "GET", None, vec!["Zebra"]),
        operation("/zebra/two", "POST", None, vec!["Zebra"]),
        operation("/apple/one", "GET", None, vec!["Apple"]),
    ];
    let filtered = operation_refs(&operations);

    let (widget, _highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let area = Rect::new(0, 0, 40, 6);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);
    let line_3 = row_text(&buffer, area, 3);
    let line_4 = row_text(&buffer, area, 4);

    assert!(
        line_0.contains("Zebra"),
        "expected first-appearing group 'Zebra' to render first, got {line_0:?}"
    );
    assert!(
        line_1.contains("GET /zebra/one"),
        "expected first Zebra operation to follow Zebra header, got {line_1:?}"
    );
    assert!(
        line_2.contains("POST /zebra/two"),
        "expected later Zebra operation to stay in the Zebra group, got {line_2:?}"
    );
    assert!(
        line_3.contains("Apple"),
        "expected second-appearing group 'Apple' to render after Zebra, got {line_3:?}"
    );
    assert!(
        line_4.contains("GET /apple/one"),
        "expected Apple operation to follow Apple header, got {line_4:?}"
    );
}

#[test]
fn endpoint_list_maps_selected_operation_index_to_visual_row_index_with_group_headers() {
    let operations = vec![
        operation("/group-a/one", "GET", None, vec!["Group A"]),
        operation("/group-a/two", "POST", None, vec!["Group A"]),
        operation("/group-b/one", "GET", None, vec!["Group B"]),
        operation("/group-b/two", "POST", None, vec!["Group B"]),
    ];
    let filtered = operation_refs(&operations);

    let (_widget, first_group_highlight_index) =
        openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let (_widget, second_group_highlight_index) =
        openapi_terminal_app::ui::endpoint_list::render(&filtered, 2);

    assert_eq!(
        first_group_highlight_index, 1,
        "expected filtered[0] to map to visual row 1 because header0 is row 0"
    );
    assert_eq!(
        second_group_highlight_index, 4,
        "expected filtered[2] to map to visual row 4 after header0/op0/op1/header1"
    );
}

#[test]
fn endpoint_list_renders_group_headers_as_separate_identifiable_rows() {
    let operations = vec![operation("/pets", "GET", None, vec!["Pets"])];
    let filtered = operation_refs(&operations);

    let (widget, highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let area = Rect::new(0, 0, 30, 3);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let header = row_text(&buffer, area, 0);
    let operation_row = row_text(&buffer, area, 1);

    assert!(
        header.contains("Pets"),
        "expected header row to contain group name, got {header:?}"
    );
    assert!(
        !header.contains("GET /pets"),
        "expected header row to be separate from operation text, got {header:?}"
    );
    assert!(
        operation_row.contains("GET /pets"),
        "expected operation row to contain method and path, got {operation_row:?}"
    );
    assert_ne!(
        highlight_index, 0,
        "expected header rows to never be returned as operation highlight indexes"
    );
}

#[test]
fn endpoint_list_rendered_row_count_includes_group_headers_and_summary_rows() {
    let operations = vec![
        operation("/pets", "GET", Some("List all pets"), vec!["Pets"]),
        operation("/pets", "POST", None, vec!["Pets"]),
        operation("/users", "GET", None, vec!["Users"]),
    ];
    let filtered = operation_refs(&operations);

    let (widget, _highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);
    let area = Rect::new(0, 0, 40, 7);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let rendered_rows = (0..area.height)
        .filter(|row| !row_text(&buffer, area, *row).trim().is_empty())
        .count();

    assert_eq!(
        rendered_rows, 6,
        "expected 3 operations across 2 groups with one summary row to render 6 non-empty rows"
    );
}

#[test]
fn endpoint_list_render_with_empty_filtered_slice_returns_empty_list_and_zero_highlight_index() {
    let filtered: Vec<&Operation> = vec![];

    let (widget, highlight_index) = openapi_terminal_app::ui::endpoint_list::render(&filtered, 0);

    assert!(
        widget.is_empty(),
        "expected empty filtered slice to return an empty list"
    );
    assert_eq!(
        highlight_index, 0,
        "there is no selectable operation for an empty filtered slice, so the highlight index is 0"
    );
}

fn operation(path: &str, method: &str, summary: Option<&str>, tags: Vec<&str>) -> Operation {
    Operation {
        path: path.to_string(),
        method: method.to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: summary.map(str::to_string),
        request_body_example: None,
        tags: tags.into_iter().map(str::to_string).collect(),
    }
}

fn operation_refs(operations: &[Operation]) -> Vec<&Operation> {
    operations.iter().collect()
}

fn row_text(buffer: &Buffer, area: Rect, row: u16) -> String {
    (area.left()..area.right())
        .map(|x| buffer[(x, row)].symbol())
        .collect()
}

fn row_has_reversed_modifier(buffer: &Buffer, area: Rect, row: u16) -> bool {
    (area.left()..area.right()).any(|x| {
        buffer[(x, row)]
            .style()
            .add_modifier
            .contains(Modifier::REVERSED)
    })
}
