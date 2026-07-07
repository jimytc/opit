use openapi_terminal_app::spec::{Operation, Parameter};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::Widget,
};

#[test]
fn request_builder_without_operation_renders_no_operation_selected() {
    let widget = openapi_terminal_app::ui::request_builder::widget(None, 0, None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("No operation selected"),
        "expected row 0 to contain 'No operation selected', got {line_0:?}"
    );
}

#[test]
fn request_builder_with_no_parameters_renders_no_parameters() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(Some(&operation), 0, None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("No parameters"),
        "expected row 0 to contain 'No parameters', got {line_0:?}"
    );
}

fn two_parameter_operation() -> Operation {
    Operation {
        path: "/pets/{petId}".to_string(),
        method: "GET".to_string(),
        parameters: vec![
            Parameter {
                name: "petId".to_string(),
                location: "path".to_string(),
                required: true,
            },
            Parameter {
                name: "limit".to_string(),
                location: "query".to_string(),
                required: false,
            },
        ],
        has_request_body: false,
    }
}

#[test]
fn request_builder_renders_required_and_optional_parameters() {
    let operation = two_parameter_operation();

    let widget = openapi_terminal_app::ui::request_builder::widget(Some(&operation), 0, None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("petId (path, required)"),
        "expected row 0 to contain 'petId (path, required)', got {line_0:?}"
    );
    assert!(
        line_1.contains("limit (query, optional)"),
        "expected row 1 to contain 'limit (query, optional)', got {line_1:?}"
    );
}

#[test]
fn request_builder_highlights_selected_row_zero() {
    let operation = two_parameter_operation();

    let widget = openapi_terminal_app::ui::request_builder::widget(Some(&operation), 0, None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    assert!(
        row_has_reversed_modifier(&buffer, area, 0),
        "expected row 0 to have REVERSED highlight style"
    );
    assert!(
        !row_has_reversed_modifier(&buffer, area, 1),
        "expected row 1 to not have REVERSED highlight style"
    );
}

#[test]
fn request_builder_highlight_follows_selected_row() {
    let operation = two_parameter_operation();

    let widget = openapi_terminal_app::ui::request_builder::widget(Some(&operation), 1, None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    assert!(
        !row_has_reversed_modifier(&buffer, area, 0),
        "expected row 0 to not have REVERSED highlight style"
    );
    assert!(
        row_has_reversed_modifier(&buffer, area, 1),
        "expected row 1 to have REVERSED highlight style"
    );
}

#[test]
fn request_builder_shows_inline_editing_buffer_only_for_selected_row() {
    let operation = two_parameter_operation();

    let widget =
        openapi_terminal_app::ui::request_builder::widget(Some(&operation), 0, Some("123"));
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("petId: 123"),
        "expected row 0 to contain 'petId: 123', got {line_0:?}"
    );
    assert!(
        line_1.contains("limit (query, optional)"),
        "expected row 1 to still contain the static text, got {line_1:?}"
    );
}

#[test]
fn request_builder_with_only_request_body_renders_body_row() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(Some(&operation), 0, None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("Body"),
        "expected row 0 to contain 'Body', got {line_0:?}"
    );
}

#[test]
fn request_builder_shows_inline_editing_buffer_for_body_row() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        Some(&operation),
        0,
        Some("{\"name\":\"fido\"}"),
    );
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("Body: {\"name\":\"fido\"}"),
        "expected row 0 to contain the body editing buffer, got {line_0:?}"
    );
    assert!(
        row_has_reversed_modifier(&buffer, area, 0),
        "expected row 0 to have REVERSED highlight style"
    );
}

#[test]
fn request_builder_body_row_follows_parameters() {
    let operation = Operation {
        path: "/pets/{petId}".to_string(),
        method: "POST".to_string(),
        parameters: vec![Parameter {
            name: "petId".to_string(),
            location: "path".to_string(),
            required: true,
        }],
        has_request_body: true,
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(Some(&operation), 1, None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("petId (path, required)"),
        "expected row 0 to still contain the static parameter text, got {line_0:?}"
    );
    assert!(
        !row_has_reversed_modifier(&buffer, area, 0),
        "expected row 0 to not have REVERSED highlight style"
    );
    assert!(
        line_1.contains("Body"),
        "expected row 1 to contain 'Body', got {line_1:?}"
    );
    assert!(
        row_has_reversed_modifier(&buffer, area, 1),
        "expected row 1 to have REVERSED highlight style"
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
