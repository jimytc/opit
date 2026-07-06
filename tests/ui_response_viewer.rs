use openapi_terminal_app::request::HttpResponse;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[test]
fn response_viewer_renders_empty_state_when_no_response_exists() {
    let widget = openapi_terminal_app::ui::response_viewer::widget(None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert_eq!(line_0.trim_end(), "No response yet");
}

#[test]
fn response_viewer_renders_status_and_body_when_response_exists() {
    let response = HttpResponse {
        status: 200,
        headers: vec![],
        body: "{\"ok\":true}".to_string(),
    };

    let widget = openapi_terminal_app::ui::response_viewer::widget(Some(&response));
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert_eq!(line_0.trim_end(), "Status: 200");
    assert!(
        line_1.contains("{\"ok\":true}"),
        "expected row 1 to contain '{{\"ok\":true}}', got {line_1:?}"
    );
}

fn row_text(buffer: &Buffer, area: Rect, row: u16) -> String {
    (area.left()..area.right())
        .map(|x| buffer[(x, row)].symbol())
        .collect()
}
