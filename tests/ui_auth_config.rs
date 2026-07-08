use openapi_terminal_app::spec::{SecurityScheme, SecuritySchemeKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::Widget,
};

#[test]
fn auth_config_renders_empty_security_schemes_message() {
    let widget = openapi_terminal_app::ui::auth_config::widget(&[], 0, None);
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

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes, 0, None);
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
            kind: SecuritySchemeKind::OAuth2 { token_url: None },
        },
    ];

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes, 0, None);
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
        line_1.contains("oauth2Auth: oauth2 (not editable yet)"),
        "expected row 1 to contain 'oauth2Auth: oauth2 (not editable yet)', got {line_1:?}"
    );
}

#[test]
fn auth_config_renders_api_key_bearer_and_basic_auth_text() {
    let schemes = three_security_schemes();

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes, 0, None);
    let area = Rect::new(0, 0, 60, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);

    assert!(
        line_0.contains("apiKeyAuth: apiKey (header)"),
        "expected row 0 to contain 'apiKeyAuth: apiKey (header)', got {line_0:?}"
    );
    assert!(
        line_1.contains("bearerAuth: http (bearer)"),
        "expected row 1 to contain 'bearerAuth: http (bearer)', got {line_1:?}"
    );
    assert!(
        line_2.contains("basicAuth: http (basic) — enter as user:pass"),
        "expected row 2 to contain 'basicAuth: http (basic) — enter as user:pass', got {line_2:?}"
    );
}

#[test]
fn auth_config_highlight_follows_selected_row() {
    let schemes = three_security_schemes();

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes, 1, None);
    let area = Rect::new(0, 0, 60, 5);
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
    assert!(
        !row_has_reversed_modifier(&buffer, area, 2),
        "expected row 2 to not have REVERSED highlight style"
    );
}

#[test]
fn auth_config_shows_inline_editing_buffer_only_for_selected_row() {
    let schemes = three_security_schemes();

    let widget =
        openapi_terminal_app::ui::auth_config::widget(&schemes, 2, Some("admin:secret"));
    let area = Rect::new(0, 0, 60, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);
    let line_1 = row_text(&buffer, area, 1);
    let line_2 = row_text(&buffer, area, 2);

    assert!(
        line_0.contains("apiKeyAuth: apiKey (header)"),
        "expected row 0 to still contain the static text, got {line_0:?}"
    );
    assert!(
        line_1.contains("bearerAuth: http (bearer)"),
        "expected row 1 to still contain the static text, got {line_1:?}"
    );
    assert!(
        line_2.contains("basicAuth: admin:secret"),
        "expected row 2 to contain 'basicAuth: admin:secret', got {line_2:?}"
    );
}

#[test]
fn auth_config_renders_oauth2_as_not_editable_yet() {
    let schemes = vec![SecurityScheme {
        name: "oauth2Auth".to_string(),
        kind: SecuritySchemeKind::OAuth2 { token_url: None },
    }];

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes, 0, None);
    let area = Rect::new(0, 0, 40, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("oauth2Auth: oauth2 (not editable yet)"),
        "expected row 0 to contain 'oauth2Auth: oauth2 (not editable yet)', got {line_0:?}"
    );
}

#[test]
fn auth_config_renders_oauth2_client_credentials_hint_when_token_url_present() {
    let schemes = vec![SecurityScheme {
        name: "oauth2Auth".to_string(),
        kind: SecuritySchemeKind::OAuth2 {
            token_url: Some("https://auth.example.com/token".to_string()),
        },
    }];

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes, 0, None);
    let area = Rect::new(0, 0, 80, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains(
            "oauth2Auth: oauth2 (client_credentials) - enter as client_id:client_secret"
        ),
        "expected row 0 to contain 'oauth2Auth: oauth2 (client_credentials) - enter as client_id:client_secret', got {line_0:?}"
    );
}

#[test]
fn auth_config_shows_raw_buffer_while_editing_oauth2_with_token_url() {
    let schemes = vec![SecurityScheme {
        name: "oauth2Auth".to_string(),
        kind: SecuritySchemeKind::OAuth2 {
            token_url: Some("https://auth.example.com/token".to_string()),
        },
    }];

    let widget =
        openapi_terminal_app::ui::auth_config::widget(&schemes, 0, Some("client-1:secret-1"));
    let area = Rect::new(0, 0, 80, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("oauth2Auth: client-1:secret-1"),
        "expected row 0 to contain 'oauth2Auth: client-1:secret-1', got {line_0:?}"
    );
}

#[test]
fn auth_config_renders_open_id_connect_as_not_editable_yet() {
    let schemes = vec![SecurityScheme {
        name: "oidcAuth".to_string(),
        kind: SecuritySchemeKind::OpenIdConnect,
    }];

    let widget = openapi_terminal_app::ui::auth_config::widget(&schemes, 0, None);
    let area = Rect::new(0, 0, 50, 5);
    let mut buffer = Buffer::empty(area);

    Widget::render(widget, area, &mut buffer);

    let line_0 = row_text(&buffer, area, 0);

    assert!(
        line_0.contains("oidcAuth: openIdConnect (not editable yet)"),
        "expected row 0 to contain 'oidcAuth: openIdConnect (not editable yet)', got {line_0:?}"
    );
}

fn three_security_schemes() -> Vec<SecurityScheme> {
    vec![
        SecurityScheme {
            name: "apiKeyAuth".to_string(),
            kind: SecuritySchemeKind::ApiKey {
                location: "header".to_string(),
                param_name: "X-API-Key".to_string(),
            },
        },
        SecurityScheme {
            name: "bearerAuth".to_string(),
            kind: SecuritySchemeKind::Http {
                scheme: "bearer".to_string(),
            },
        },
        SecurityScheme {
            name: "basicAuth".to_string(),
            kind: SecuritySchemeKind::Http {
                scheme: "basic".to_string(),
            },
        },
    ]
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
