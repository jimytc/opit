use std::collections::HashMap;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use openapi_terminal_app::app::{AppState, Pane};
use openapi_terminal_app::auth::{self, Credential};
use openapi_terminal_app::cli::Cli;
use openapi_terminal_app::request::{self, HttpClient, ReqwestClient};
use openapi_terminal_app::spec::{Operation, Spec, SecurityScheme};
use openapi_terminal_app::ui::{auth_config, endpoint_list, request_builder, response_viewer};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, ListState};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let spec = Spec::load_from_path(&cli.spec_path)?;
    let operations = spec.operations();
    let security_schemes = spec.security_schemes();
    let base_url = spec.base_url().unwrap_or_default();
    let credentials = credentials_from_cli(&cli);

    run_app(&operations, &security_schemes, &base_url, &credentials).await
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
    base_url: &str,
    credentials: &[Credential],
) -> anyhow::Result<()> {
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

    let mut app = AppState::new();
    app.set_operation_count(operations.len());
    let http_client = ReqwestClient::new();

    let result = event_loop(
        &mut terminal,
        &mut app,
        operations,
        security_schemes,
        base_url,
        credentials,
        &http_client,
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
    base_url: &str,
    credentials: &[Credential],
    http_client: &dyn HttpClient,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, app, operations, security_schemes))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Enter => {
                        if let Some(operation) = operations.get(app.selected_operation_index) {
                            let mut request = request::build(base_url, operation, &HashMap::new());
                            for credential in credentials {
                                auth::apply(&mut request, credential);
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
                    }
                    _ => app.handle_key(key),
                }
            }
        }
    }
    Ok(())
}

fn draw(
    frame: &mut ratatui::Frame,
    app: &AppState,
    operations: &[Operation],
    security_schemes: &[SecurityScheme],
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

    let endpoint_block = Block::bordered()
        .title("Endpoints")
        .border_style(pane_border_style(app.focused, Pane::EndpointList));
    let mut list_state = ListState::default();
    if !operations.is_empty() {
        list_state.select(Some(app.selected_operation_index));
    }
    frame.render_stateful_widget(
        endpoint_list::widget(operations).block(endpoint_block),
        top[0],
        &mut list_state,
    );

    let selected_operation = operations.get(app.selected_operation_index);
    let request_builder_block = Block::bordered()
        .title("Request Builder")
        .border_style(pane_border_style(app.focused, Pane::RequestBuilder));
    frame.render_widget(
        request_builder::widget(selected_operation).block(request_builder_block),
        top[1],
    );

    let auth_config_block = Block::bordered()
        .title("Auth Config")
        .border_style(pane_border_style(app.focused, Pane::AuthConfig));
    frame.render_widget(
        auth_config::widget(security_schemes).block(auth_config_block),
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
