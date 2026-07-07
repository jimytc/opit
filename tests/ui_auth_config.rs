use openapi_terminal_app::spec::{SecurityScheme, SecuritySchemeKind};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[test]
fn auth_config_renders_empty_security_schemes_message() {
    let widget = openapi_terminal_app::ui::auth_config::widget(&[]);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("No security schemes defined"),
        "expected row 0 to contain 'No security schemes defined', got {line_0:?}"
    );
}

#[test]
fn auth_config_renders_api_key_security_scheme() {
    let schemes = vec![SecurityScheme {
        name: "apiKeyAuth".to_string(),
        kind: SecuritySchemeKind::ApiKey {
            location: "header".to_string(),
            param_name: "X-API-Key".to_string(),
        },
    }];

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("apiKeyAuth: apiKey (header)"),
        "expected row 0 to contain 'apiKeyAuth: apiKey (header)', got {line_0:?}"
    );
}

#[test]
fn auth_config_renders_multiple_security_scheme_kinds() {
    let schemes = vec![
        SecurityScheme {
            name: "bearerAuth".to_string(),
            kind: SecuritySchemeKind::Http {
                scheme: "bearer".to_string(),
            },
        },
        SecurityScheme {
            name: "oauth2Auth".to_string(),
            kind: SecuritySchemeKind::OAuth2,
        },
    ];

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);

    assert!(
        line_0.contains("bearerAuth: http (bearer)"),
        "expected row 0 to contain 'bearerAuth: http (bearer)', got {line_0:?}"
    );
    assert!(
        line_1.contains("oauth2Auth: oauth2"),
        "expected row 1 to contain 'oauth2Auth: oauth2', got {line_1:?}"
    );
}

fn row_text(buffer: &Buffer, area: Rect, row: u16) -> String {
    (area.left()..area.right())
        .map(|x| buffer[(x, row)].symbol())
        .collect()
}
