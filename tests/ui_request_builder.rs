use openapi_terminal_app::spec::{Operation, Parameter};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[test]
fn request_builder_without_operation_renders_no_operation_selected() {
    let widget = openapi_terminal_app::ui::request_builder::widget(None);
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

    let widget = openapi_terminal_app::ui::request_builder::widget(Some(&operation));
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("No parameters"),
        "expected row 0 to contain 'No parameters', got {line_0:?}"
    );
}

#[test]
fn request_builder_renders_required_and_optional_parameters() {
    let operation = Operation {
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
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(Some(&operation));
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

fn row_text(buffer: &Buffer, area: Rect, row: u16) -> String {
    (area.left()..area.right())
        .map(|x| buffer[(x, row)].symbol())
        .collect()
}
