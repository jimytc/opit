use std::collections::HashSet;

use clap::Parser;
use crossterm::event::{
    self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEventKind,
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, supports_keyboard_enhancement, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use openapi_terminal_app::app::{AppState, Pane, PaneEditor, RequestBuilderTab};
use openapi_terminal_app::auth::oauth2::{
    build_token_caches, resolve_oauth2_credentials, SystemClock,
};
use openapi_terminal_app::auth::Credential;
use openapi_terminal_app::cli::Cli;
use openapi_terminal_app::request::{self, HttpClient, ReqwestClient};
use openapi_terminal_app::spec::{Operation, Parameter, SecurityScheme, SecuritySchemeKind, Spec};
use openapi_terminal_app::ui::{auth_config, endpoint_list, request_builder, response_viewer};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, ListState, Paragraph, Wrap};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let spec = Spec::load_from_path(&cli.spec_path)?;
    let operations = spec.operations();
    let security_schemes = spec.security_schemes();
    let servers = spec.servers();
    let title = spec.title();
    let version = spec.version();
    let credentials = credentials_from_cli(&cli);

    run_app(
        &operations,
        &security_schemes,
        &servers,
        &title,
        &version,
        &credentials,
    )
    .await
}

fn credentials_from_cli(cli: &Cli) -> Vec<Credential> {
    let mut credentials = Vec::new();
    if let Some(token) = &cli.bearer_token {
        credentials.push(Credential::Bearer {
            token: token.clone(),
        });
    }
    for header in &cli.header {
        if let Some((name, value)) = header.split_once('=') {
            credentials.push(Credential::Raw {
                header_name: name.to_string(),
                header_value: value.to_string(),
            });
        }
    }
    credentials
}

async fn run_app(
    operations: &[Operation],
    security_schemes: &[SecurityScheme],
    servers: &[String],
    title: &str,
    version: &str,
    credentials: &[Credential],
) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let keyboard_enhancement_supported = supports_keyboard_enhancement().unwrap_or(false);
    std::io::stdout()
        .execute(EnterAlternateScreen)?
        .execute(EnableBracketedPaste)?;
    if keyboard_enhancement_supported {
        std::io::stdout().execute(PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
        ))?;
    }
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

    let mut app = AppState::new();
    app.set_operation_count(operations.len());
    app.set_server_count(servers.len());
    let http_client = ReqwestClient::new();
    let token_caches = build_token_caches(security_schemes);

    let result = event_loop(
        &mut terminal,
        &mut app,
        operations,
        security_schemes,
        servers,
        title,
        version,
        credentials,
        &http_client,
        &token_caches,
    )
    .await;

    disable_raw_mode()?;
    if keyboard_enhancement_supported {
        std::io::stdout().execute(PopKeyboardEnhancementFlags)?;
    }
    std::io::stdout()
        .execute(DisableBracketedPaste)?
        .execute(LeaveAlternateScreen)?;

    result
}

async fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut AppState,
    operations: &[Operation],
    security_schemes: &[SecurityScheme],
    servers: &[String],
    title: &str,
    version: &str,
    cli_credentials: &[Credential],
    http_client: &dyn HttpClient,
    token_caches: &std::collections::HashMap<usize, openapi_terminal_app::auth::oauth2::TokenCache>,
) -> anyhow::Result<()> {
    loop {
        let filtered = endpoint_list::filtered_operations(operations, &app.endpoint_filter);
        app.set_operation_count(filtered.len());
        app.sync_selected_operation(
            filtered
                .get(app.selected_operation_index)
                .map(|operation| (operation.method.as_str(), operation.path.as_str())),
        );
        let selected_operation = filtered.get(app.selected_operation_index).copied();
        let header_params: Vec<&Parameter> = selected_operation
            .map(|operation| operation.header_parameters())
            .unwrap_or_default();
        let non_header_params: Vec<&Parameter> = selected_operation
            .map(|operation| operation.non_header_parameters())
            .unwrap_or_default();

        app.request_builder.headers.set_row_count(
            header_params.len() + app.request_builder.custom_headers.len() + 1,
        );
        app.request_builder.parameters.set_row_count(
            non_header_params.len() + app.request_builder.custom_query_params.len() + 1,
        );
        app.request_builder.payload.set_row_count(1);
        app.request_builder
            .payload
            .set_multiline_rows(HashSet::from([0]));
        app.set_header_param_names(
            header_params
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect(),
        );
        app.set_parameter_param_names(
            non_header_params
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect(),
        );
        app.auth_config.set_row_count(security_schemes.len());
        app.auth_config
            .set_non_editable_rows(non_editable_auth_rows(security_schemes));
        let base_url = servers
            .get(app.selected_server_index)
            .map(String::as_str)
            .unwrap_or_default();

        terminal.draw(|frame| {
            draw(
                frame,
                app,
                &filtered,
                security_schemes,
                base_url,
                servers,
                title,
                version,
                cli_credentials,
            )
        })?;

        match event::read()? {
            Event::Paste(text) => app.handle_paste(&text),
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    if app.is_editing() {
                        match key.code {
                            KeyCode::Enter
                            | KeyCode::Esc
                            | KeyCode::Backspace
                            | KeyCode::Char(_) => app.handle_key(key),
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Enter if app.focused == Pane::EndpointList => {
                                if let Some(operation) = selected_operation {
                                    let request_inputs = request::gather_request_inputs(
                                        operation,
                                        app.request_builder.headers.inputs(),
                                        app.request_builder.parameters.inputs(),
                                        &app.request_builder.custom_headers,
                                        &app.request_builder.custom_query_params,
                                        app.request_builder.payload.inputs(),
                                    );
                                    let missing = request::missing_required_params(
                                        operation,
                                        &request_inputs,
                                    );
                                    if !missing.is_empty() {
                                        app.set_response(request::HttpResponse {
                                            status: 0,
                                            headers: vec![],
                                            body: format!(
                                                "Missing required parameter(s): {}",
                                                missing.join(", ")
                                            ),
                                        });
                                        continue;
                                    }
                                    let mut request = request::build_preview(
                                        base_url,
                                        operation,
                                        &request_inputs,
                                        security_schemes,
                                        app.auth_config.inputs(),
                                        cli_credentials,
                                    );

                                    match resolve_oauth2_credentials(
                                        security_schemes,
                                        app.auth_config.inputs(),
                                        token_caches,
                                        http_client,
                                        &SystemClock,
                                    )
                                    .await
                                    {
                                        Ok(oauth2_credentials) => {
                                            for credential in &oauth2_credentials {
                                                openapi_terminal_app::auth::apply(
                                                    &mut request,
                                                    credential,
                                                );
                                            }

                                            match http_client.send(request).await {
                                                Ok(response) => app.set_response(response),
                                                Err(error) => {
                                                    app.set_response(request::HttpResponse {
                                                        status: 0,
                                                        headers: vec![],
                                                        body: format!("{error:?}"),
                                                    })
                                                }
                                            }
                                        }
                                        Err(error) => app.set_response(request::HttpResponse {
                                            status: 0,
                                            headers: vec![],
                                            body: format!("{error:?}"),
                                        }),
                                    }
                                }
                            }
                            _ => app.handle_key(key),
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn non_editable_auth_rows(security_schemes: &[SecurityScheme]) -> HashSet<usize> {
    security_schemes
        .iter()
        .enumerate()
        .filter_map(|(index, scheme)| {
            matches!(
                scheme.kind,
                SecuritySchemeKind::OAuth2 { token_url: None } | SecuritySchemeKind::OpenIdConnect
            )
            .then_some(index)
        })
        .collect()
}

fn draw(
    frame: &mut ratatui::Frame,
    app: &mut AppState,
    filtered: &[&Operation],
    security_schemes: &[SecurityScheme],
    base_url: &str,
    servers: &[String],
    title: &str,
    version: &str,
    cli_credentials: &[Credential],
) {
    let body_and_footer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
        ])
        .split(body_and_footer[0]);
    let middle = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(columns[1]);
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(columns[2]);

    let mut endpoint_title = format!("Endpoints — {title} v{version}");
    if servers.len() > 1 {
        endpoint_title.push_str(&format!(
            " — server {}/{} (press 's')",
            app.selected_server_index + 1,
            servers.len()
        ));
    }
    if !app.endpoint_filter.is_empty() {
        endpoint_title.push_str(&format!(" — filter: {}", app.endpoint_filter));
    }
    let endpoint_block = Block::bordered()
        .title(endpoint_title)
        .border_style(pane_border_style(app.focused, Pane::EndpointList));
    let (endpoint_list_widget, visual_index) =
        endpoint_list::render(filtered, app.selected_operation_index);
    let mut list_state = ListState::default();
    if !filtered.is_empty() {
        list_state.select(Some(visual_index));
    }
    frame.render_stateful_widget(
        endpoint_list_widget.block(endpoint_block),
        columns[0],
        &mut list_state,
    );

    let selected_operation = filtered.get(app.selected_operation_index).copied();

    let auth_config_block = Block::bordered()
        .title("Auth Config")
        .border_style(pane_border_style(app.focused, Pane::AuthConfig));
    let mut auth_config_state = ListState::default();
    auth_config_state.select(Some(app.auth_config.selected_row()));
    frame.render_stateful_widget(
        auth_config::widget(
            security_schemes,
            app.auth_config.selected_row(),
            app.auth_config.editing_buffer(),
        )
        .block(auth_config_block),
        middle[0],
        &mut auth_config_state,
    );

    let request_builder_title = format!(
        "Request Builder — {} {} {}",
        tab_label(app.request_builder_tab, RequestBuilderTab::Header, "Header"),
        tab_label(
            app.request_builder_tab,
            RequestBuilderTab::Parameters,
            "Parameters"
        ),
        tab_label(
            app.request_builder_tab,
            RequestBuilderTab::Payload,
            "Payload"
        ),
    );
    let request_builder_block = Block::bordered()
        .title(request_builder_title)
        .border_style(pane_border_style(app.focused, Pane::RequestBuilder));
    let body_committed = app.request_builder.payload.inputs().contains_key(&0);
    let active_editor: &PaneEditor = match app.request_builder_tab {
        RequestBuilderTab::Header => &app.request_builder.headers,
        RequestBuilderTab::Parameters => &app.request_builder.parameters,
        RequestBuilderTab::Payload => &app.request_builder.payload,
    };
    let mut request_builder_state = ListState::default();
    request_builder_state.select(Some(active_editor.selected_row()));
    frame.render_stateful_widget(
        request_builder::widget(
            app.request_builder_tab,
            selected_operation,
            &app.request_builder.custom_headers,
            &app.request_builder.custom_query_params,
            active_editor.selected_row(),
            active_editor.editing_buffer(),
            body_committed,
        )
        .block(request_builder_block),
        middle[1],
        &mut request_builder_state,
    );

    let curl_preview_block = Block::bordered()
        .title("Curl Preview")
        .border_style(pane_border_style(app.focused, Pane::CurlPreview));
    let curl_preview_inner = curl_preview_block.inner(right[0]);
    frame.render_widget(curl_preview_block, right[0]);
    if let Some(operation) = selected_operation {
        let preview_request_inputs = request::gather_request_inputs(
            operation,
            &app.request_builder.headers.inputs_with_live_edit(),
            &app.request_builder.parameters.inputs_with_live_edit(),
            &app.request_builder.custom_headers,
            &app.request_builder.custom_query_params,
            &app.request_builder.payload.inputs_with_live_edit(),
        );
        let preview_request = request::build_preview(
            base_url,
            operation,
            &preview_request_inputs,
            security_schemes,
            &app.auth_config.inputs_with_live_edit(),
            cli_credentials,
        );
        let curl_text = request::to_curl(&preview_request);
        let curl_paragraph = Paragraph::new(curl_text).wrap(Wrap { trim: false });
        let line_count = curl_paragraph.line_count(curl_preview_inner.width);
        app.set_curl_preview_max_scroll(
            line_count.saturating_sub(curl_preview_inner.height as usize) as u16,
        );
        frame.render_widget(
            curl_paragraph.scroll((app.curl_preview_scroll(), 0)),
            curl_preview_inner,
        );
    } else {
        app.set_curl_preview_max_scroll(0);
    }

    let response_viewer_block = Block::bordered()
        .title("Response Viewer")
        .border_style(pane_border_style(app.focused, Pane::ResponseViewer));
    let response_viewer_inner = response_viewer_block.inner(right[1]);
    let response_viewer_paragraph = response_viewer::widget(app.response());
    let response_line_count = response_viewer_paragraph.line_count(response_viewer_inner.width);
    app.set_response_viewer_max_scroll(
        response_line_count.saturating_sub(response_viewer_inner.height as usize) as u16,
    );
    frame.render_widget(
        response_viewer_paragraph
            .scroll((app.response_viewer_scroll(), 0))
            .block(response_viewer_block),
        right[1],
    );

    frame.render_widget(
        Paragraph::new(HOTKEY_HINTS).style(Style::default().fg(Color::DarkGray)),
        body_and_footer[1],
    );
}

const HOTKEY_HINTS: &str = "Tab: cycle panes  Ctrl+Alt+1-5: jump to pane  [ / ]: switch Request Builder tab  ↑/↓: navigate/scroll  /: filter  s: server  Enter: edit/send  Ctrl+S: commit  Esc: cancel/quit  q: quit";

fn tab_label(current: RequestBuilderTab, tab: RequestBuilderTab, label: &str) -> String {
    if current == tab {
        format!("[{label}]")
    } else {
        label.to_string()
    }
}

fn pane_border_style(focused: Pane, pane: Pane) -> Style {
    if focused == pane {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    }
}
