use openapi_terminal_app::spec::Operation;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::{ListState, StatefulWidget, Widget},
};

#[test]
fn endpoint_list_renders_method_and_path_for_each_operation() {
    let operations = vec![
        Operation {
            path: "/pets".to_string(),
            method: "GET".to_string(),
            parameters: vec![],
            has_request_body: false,
            request_body_media_type: None,
            summary: None,
            request_body_example: None,
            tags: vec![],
        },
        Operation {
            path: "/pets".to_string(),
            method: "POST".to_string(),
            parameters: vec![],
            has_request_body: false,
            request_body_media_type: None,
            summary: None,
            request_body_example: None,
            tags: vec![],
        },
    ];

    let widget = openapi_terminal_app::ui::endpoint_list::widget(&operations);
    let area = Rect::new(0, 0, 20, 4);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("GET /pets"),
        "expected row 0 to contain 'GET /pets', got {line_0:?}"
    );
    assert!(
        line_1.contains("POST /pets"),
        "expected row 1 to contain 'POST /pets', got {line_1:?}"
    );
}

#[test]
fn endpoint_list_renders_summary_on_indented_second_line() {
    let operations = vec![Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: Some("List all pets".to_string()),
        request_body_example: None,
        tags: vec![],
    }];

    let widget = openapi_terminal_app::ui::endpoint_list::widget(&operations);
    let area = Rect::new(0, 0, 40, 3);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("GET /pets"),
        "expected row 0 to contain 'GET /pets', got {line_0:?}"
    );
    assert!(
        line_1.contains("  List all pets"),
        "expected row 1 to contain indented summary, got {line_1:?}"
    );
}

#[test]
fn endpoint_list_without_summary_stays_single_line() {
    let operations = vec![Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    }];

    let widget = openapi_terminal_app::ui::endpoint_list::widget(&operations);
    let area = Rect::new(0, 0, 40, 3);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("GET /pets"),
        "expected row 0 to contain 'GET /pets', got {line_0:?}"
    );
    assert!(
        line_1.trim().is_empty(),
        "expected row 1 to stay empty when summary is None, got {line_1:?}"
    );
}

#[test]
fn endpoint_list_mixes_summary_and_plain_operations_without_row_drift() {
    let operations = vec![
        Operation {
            path: "/pets".to_string(),
            method: "GET".to_string(),
            parameters: vec![],
            has_request_body: false,
            request_body_media_type: None,
            summary: Some("List all pets".to_string()),
            request_body_example: None,
            tags: vec![],
        },
        Operation {
            path: "/pets".to_string(),
            method: "POST".to_string(),
            parameters: vec![],
            has_request_body: false,
            request_body_media_type: None,
            summary: None,
            request_body_example: None,
            tags: vec![],
        },
    ];

    let widget = openapi_terminal_app::ui::endpoint_list::widget(&operations);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);
    let line_3 = row_text(&buffer, area, 3);

    assert!(
        line_0.contains("GET /pets"),
        "expected row 0 to contain 'GET /pets', got {line_0:?}"
    );
    assert!(
        line_1.contains("  List all pets"),
        "expected row 1 to contain indented summary, got {line_1:?}"
    );
    assert!(
        line_2.contains("POST /pets"),
        "expected row 2 to contain 'POST /pets' after summary row, got {line_2:?}"
    );
    assert!(
        line_3.trim().is_empty(),
        "expected row 3 to stay empty because POST has no summary, got {line_3:?}"
    );
}

#[test]
fn endpoint_list_applies_reversed_highlight_style_to_selected_row_only() {
    let operations = vec![
        Operation {
            path: "/pets".to_string(),
            method: "GET".to_string(),
            parameters: vec![],
            has_request_body: false,
            request_body_media_type: None,
            summary: None,
            request_body_example: None,
            tags: vec![],
        },
        Operation {
            path: "/pets".to_string(),
            method: "POST".to_string(),
            parameters: vec![],
            has_request_body: false,
            request_body_media_type: None,
            summary: None,
            request_body_example: None,
            tags: vec![],
        },
    ];

    let widget = openapi_terminal_app::ui::endpoint_list::widget(&operations);
    let area = Rect::new(0, 0, 20, 4);
    let mut buffer = Buffer::empty(area);
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    StatefulWidget::render(widget, area, &mut buffer, &mut list_state);

    assert!(
        row_has_reversed_modifier(&buffer, area, 0),
        "expected selected row 0 to have REVERSED highlight style"
    );
    assert!(
        !row_has_reversed_modifier(&buffer, area, 1),
        "expected unselected row 1 to not have REVERSED highlight style"
    );
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
