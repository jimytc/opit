use std::collections::HashSet;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use openapi_terminal_app::app::{AppState, Pane};
use openapi_terminal_app::auth::oauth2::{build_token_caches, resolve_oauth2_credentials, SystemClock};
use openapi_terminal_app::auth::Credential;
use openapi_terminal_app::cli::Cli;
use openapi_terminal_app::request::{self, HttpClient, ReqwestClient};
use openapi_terminal_app::spec::{Operation, SecurityScheme, SecuritySchemeKind, Spec};
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
    let credentials = credentials_from_cli(&cli);

    run_app(&operations, &security_schemes, &servers, &credentials).await
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
    credentials: &[Credential],
) -> anyhow::Result<()> {
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

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
        credentials,
        &http_client,
        &token_caches,
    )
    .await;

    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    result
}

async fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut AppState,
    operations: &[Operation],
    security_schemes: &[SecurityScheme],
    servers: &[String],
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
        let request_builder_row_count = selected_operation
            .map(|operation| operation.parameters.len() + usize::from(operation.has_request_body))
            .unwrap_or(0);
        app.request_builder.set_row_count(request_builder_row_count);
        app.auth_config.set_row_count(security_schemes.len());
        app.auth_config
            .set_non_editable_rows(non_editable_auth_rows(security_schemes));
        let base_url = servers
            .get(app.selected_server_index)
            .map(String::as_str)
            .unwrap_or_default();

        terminal.draw(|frame| draw(frame, app, &filtered, security_schemes, base_url, servers, cli_credentials))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if app.is_editing() {
                    match key.code {
                        KeyCode::Enter | KeyCode::Esc | KeyCode::Backspace | KeyCode::Char(_) => {
                            app.handle_key(key)
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Enter if app.focused == Pane::EndpointList => {
                            if let Some(operation) = selected_operation {
                                let missing = request::missing_required_params(
                                    operation,
                                    app.request_builder.inputs(),
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
                                    app.request_builder.inputs(),
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
                                            Err(error) => app.set_response(request::HttpResponse {
                                                status: 0,
                                                headers: vec![],
                                                body: format!("{error:?}"),
                                            }),
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
    app: &AppState,
    filtered: &[&Operation],
    security_schemes: &[SecurityScheme],
    base_url: &str,
    servers: &[String],
    cli_credentials: &[Credential],
) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(rows[0]);
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(rows[1]);

    let endpoint_title = match (servers.len() > 1, app.endpoint_filter.is_empty()) {
        (true, true) => format!(
            "Endpoints — server {}/{} (press 's')",
            app.selected_server_index + 1,
            servers.len()
        ),
        (true, false) => format!(
            "Endpoints — server {}/{} (press 's') — filter: {}",
            app.selected_server_index + 1,
            servers.len(),
            app.endpoint_filter
        ),
        (false, true) => "Endpoints".to_string(),
        (false, false) => format!("Endpoints — filter: {}", app.endpoint_filter),
    };
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
        top[0],
        &mut list_state,
    );

    let selected_operation = filtered.get(app.selected_operation_index).copied();
    let request_builder_block = Block::bordered()
        .title("Request Builder")
        .border_style(pane_border_style(app.focused, Pane::RequestBuilder));
    let request_builder_inner = request_builder_block.inner(top[1]);
    frame.render_widget(request_builder_block, top[1]);

    let request_builder_sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Percentage(40)])
        .split(request_builder_inner);
    let body_committed = selected_operation
        .map(|operation| {
            app.request_builder
                .inputs()
                .contains_key(&operation.parameters.len())
        })
        .unwrap_or(false);
    frame.render_widget(
        request_builder::widget(
            selected_operation,
            app.request_builder.selected_row(),
            app.request_builder.editing_buffer(),
            body_committed,
        ),
        request_builder_sections[0],
    );

    if let Some(operation) = selected_operation {
        let preview_request = request::build_preview(
            base_url,
            operation,
            &app.request_builder.inputs_with_live_edit(),
            security_schemes,
            &app.auth_config.inputs_with_live_edit(),
            cli_credentials,
        );
        let curl_text = request::to_curl(&preview_request);
        frame.render_widget(
            Paragraph::new(curl_text).wrap(Wrap { trim: false }),
            request_builder_sections[1],
        );
    }

    let auth_config_block = Block::bordered()
        .title("Auth Config")
        .border_style(pane_border_style(app.focused, Pane::AuthConfig));
    frame.render_widget(
        auth_config::widget(
            security_schemes,
            app.auth_config.selected_row(),
            app.auth_config.editing_buffer(),
        )
        .block(auth_config_block),
        bottom[0],
    );

    let response_viewer_block = Block::bordered()
        .title("Response Viewer")
        .border_style(pane_border_style(app.focused, Pane::ResponseViewer));
    frame.render_widget(
        response_viewer::widget(app.response()).block(response_viewer_block),
        bottom[1],
    );
}

fn pane_border_style(focused: Pane, pane: Pane) -> Style {
    if focused == pane {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    }
}
