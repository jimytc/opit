use openapi_terminal_app::spec::Operation;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[test]
fn endpoint_list_renders_method_and_path_for_each_operation() {
    let operations = vec![
        Operation {
            path: "/pets".to_string(),
            method: "GET".to_string(),
            parameters: vec![],
            has_request_body: false,
        },
        Operation {
            path: "/pets".to_string(),
            method: "POST".to_string(),
            parameters: vec![],
            has_request_body: false,
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

fn row_text(buffer: &Buffer, area: Rect, row: u16) -> String {
    (area.left()..area.right())
        .map(|x| buffer[(x, row)].symbol())
        .collect()
}
