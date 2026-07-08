use openapi_terminal_app::request::{pretty_print_if_json, HttpResponse};
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
    let body = "{\"ok\":true}";
    let response = HttpResponse {
        status: 200,
        headers: vec![],
        body: body.to_string(),
    };

    let widget = openapi_terminal_app::ui::response_viewer::widget(Some(&response));
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let pretty_body = pretty_print_if_json(body);
    let pretty_body_lines = pretty_body.lines().collect::<Vec<_>>();
    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);

    assert_eq!(line_0.trim_end(), "Status: 200");
    assert_eq!(line_1.trim_end(), pretty_body_lines[0]);
    assert!(
        line_2.contains(pretty_body_lines[1].trim()),
        "expected row 2 to contain {:?}, got {line_2:?}",
        pretty_body_lines[1].trim()
    );
}

#[test]
fn response_viewer_renders_non_json_body_unchanged() {
    let response = HttpResponse {
        status: 200,
        headers: vec![],
        body: "plain text response".to_string(),
    };

    let widget = openapi_terminal_app::ui::response_viewer::widget(Some(&response));
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert_eq!(line_0.trim_end(), "Status: 200");
    assert_eq!(line_1.trim_end(), "plain text response");
}

#[test]
fn response_viewer_wraps_long_body_lines() {
    let response = HttpResponse {
        status: 200,
        headers: vec![],
        body: "x".repeat(60),
    };

    let widget = openapi_terminal_app::ui::response_viewer::widget(Some(&response));
    let area = Rect::new(0, 0, 20, 6);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);

    assert!(
        line_1.trim_end().len() < 60,
        "expected row 1 to show less than the full long body, got {line_1:?}"
    );
    assert!(
        line_2.contains('x'),
        "expected row 2 to contain wrapped body text, got {line_2:?}"
    );
}

fn row_text(buffer: &Buffer, area: Rect, row: u16) -> String {
    (area.left()..area.right())
        .map(|x| buffer[(x, row)].symbol())
        .collect()
}
