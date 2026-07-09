use openapi_terminal_app::{
    app::RequestBuilderTab,
    spec::{Operation, Parameter},
};
use ratatui::{buffer::Buffer, layout::Rect, style::Modifier, widgets::Widget};

#[test]
fn request_builder_without_operation_renders_no_operation_selected() {
    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Header,
        None,
        &[],
        &[],
        0,
        None,
        false,
    );
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
fn request_builder_with_no_header_parameters_renders_add_header() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Header,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("+ Add Header"),
        "expected row 0 to contain '+ Add Header', got {line_0:?}"
    );
    assert!(
        !line_0.contains("No parameters"),
        "expected row 0 to not contain 'No parameters', got {line_0:?}"
    );
}

#[test]
fn request_builder_with_no_non_header_parameters_renders_add_parameter() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Parameters,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("+ Add Parameter"),
        "expected row 0 to contain '+ Add Parameter', got {line_0:?}"
    );
    assert!(
        !line_0.contains("No parameters"),
        "expected row 0 to not contain 'No parameters', got {line_0:?}"
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
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    }
}

#[test]
fn request_builder_renders_required_and_optional_parameters() {
    let operation = two_parameter_operation();

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Parameters,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
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

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Parameters,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
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

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Parameters,
        Some(&operation),
        &[],
        &[],
        1,
        None,
        false,
    );
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

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Parameters,
        Some(&operation),
        &[],
        &[],
        0,
        Some("123"),
        false,
    );
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
fn request_builder_with_custom_header_renders_custom_row_before_add_header() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };
    let custom_headers = vec!["X-Api-Version".to_string()];

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Header,
        Some(&operation),
        &custom_headers,
        &[],
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 60, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("X-Api-Version (header, custom)"),
        "expected row 0 to contain the custom header row, got {line_0:?}"
    );
    assert!(
        line_1.contains("+ Add Header"),
        "expected row 1 to contain '+ Add Header', got {line_1:?}"
    );
}

#[test]
fn request_builder_with_custom_query_param_renders_custom_row_before_add_parameter() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };
    let custom_query_params = vec!["region".to_string()];

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Parameters,
        Some(&operation),
        &[],
        &custom_query_params,
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 60, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("region (query, custom)"),
        "expected row 0 to contain the custom query parameter row, got {line_0:?}"
    );
    assert!(
        line_1.contains("+ Add Parameter"),
        "expected row 1 to contain '+ Add Parameter', got {line_1:?}"
    );
}

#[test]
fn request_builder_shows_add_placeholder_inline_editing_without_label_colon() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Header,
        Some(&operation),
        &[],
        &[],
        0,
        Some("X-Api"),
        false,
    );
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("+ X-Api"),
        "expected row 0 to contain '+ X-Api', got {line_0:?}"
    );
    assert!(
        !line_0.contains(": X-Api"),
        "expected add placeholder editing to omit label-colon style, got {line_0:?}"
    );
}

#[test]
fn request_builder_with_only_request_body_renders_body_row() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
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
fn request_builder_payload_tab_renders_body_row_without_request_body_flag() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "GET".to_string(),
        parameters: vec![],
        has_request_body: false,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("Body"),
        "expected row 0 to contain 'Body' even without request body flag, got {line_0:?}"
    );
}

#[test]
fn request_builder_shows_inline_editing_buffer_for_body_row() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        Some("{\"name\":\"fido\"}"),
        false,
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
fn request_builder_payload_tab_renders_only_body_row_when_parameters_exist() {
    let operation = Operation {
        path: "/pets/{petId}".to_string(),
        method: "POST".to_string(),
        parameters: vec![Parameter {
            name: "petId".to_string(),
            location: "path".to_string(),
            required: true,
        }],
        has_request_body: true,
        request_body_media_type: None,
        summary: None,
        request_body_example: None,
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("Body"),
        "expected row 0 to contain 'Body', got {line_0:?}"
    );
    assert!(
        !line_0.contains("petId"),
        "expected payload tab row 0 to not contain parameter text, got {line_0:?}"
    );
    assert!(
        line_1.trim().is_empty(),
        "expected payload tab to render no parameter row at row 1, got {line_1:?}"
    );
    assert!(
        row_has_reversed_modifier(&buffer, area, 0),
        "expected row 0 to have REVERSED highlight style"
    );
}

#[test]
fn request_builder_shows_body_example_hint_before_body_is_committed() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: Some("application/json".to_string()),
        summary: None,
        request_body_example: Some("{\"name\":\"\"}".to_string()),
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 80, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("Body — e.g. {\"name\":\"\"}"),
        "expected row 0 to contain the body example hint, got {line_0:?}"
    );
}

#[test]
fn request_builder_keeps_multiline_body_example_hint_on_one_row() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: Some("application/json".to_string()),
        summary: None,
        request_body_example: Some("{\n  \"name\": \"\",\n  \"age\": 0\n}".to_string()),
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 80, 6);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("Body"),
        "expected row 0 to contain 'Body', got {line_0:?}"
    );
    assert!(
        line_0.contains("e.g."),
        "expected row 0 to contain the body example hint, got {line_0:?}"
    );
    assert!(
        line_1.trim().is_empty(),
        "expected multiline body example hint to occupy only row 0, got row 1 {line_1:?}"
    );
}

#[test]
fn request_builder_keeps_plain_body_text_after_body_is_committed() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: Some("application/json".to_string()),
        summary: None,
        request_body_example: Some("{\"name\":\"\"}".to_string()),
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        true,
    );
    let area = Rect::new(0, 0, 80, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("Body"),
        "expected row 0 to contain plain body text, got {line_0:?}"
    );
    assert!(
        !line_0.contains("e.g."),
        "expected committed body row to omit example hint, got {line_0:?}"
    );
}

#[test]
fn request_builder_keeps_plain_body_text_when_no_example_exists() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: Some("application/json".to_string()),
        summary: None,
        request_body_example: None,
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        None,
        false,
    );
    let area = Rect::new(0, 0, 80, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("Body"),
        "expected row 0 to contain plain body text, got {line_0:?}"
    );
    assert!(
        !line_0.contains("e.g."),
        "expected body row without an example to omit hint text, got {line_0:?}"
    );
}

#[test]
fn request_builder_body_example_hint_does_not_replace_inline_editing_buffer() {
    let operation = Operation {
        path: "/pets".to_string(),
        method: "POST".to_string(),
        parameters: vec![],
        has_request_body: true,
        request_body_media_type: Some("application/json".to_string()),
        summary: None,
        request_body_example: Some("{\"name\":\"\"}".to_string()),
        tags: vec![],
    };

    let widget = openapi_terminal_app::ui::request_builder::widget(
        RequestBuilderTab::Payload,
        Some(&operation),
        &[],
        &[],
        0,
        Some("{\"name\":\"fido\"}"),
        false,
    );
    let area = Rect::new(0, 0, 80, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("Body: {\"name\":\"fido\"}"),
        "expected row 0 to contain the body editing buffer, got {line_0:?}"
    );
    assert!(
        !line_0.contains("e.g."),
        "expected editing body row to omit example hint, got {line_0:?}"
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
