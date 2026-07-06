use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use openapi_terminal_app::app::{AppState, Pane};
use openapi_terminal_app::cli::Cli;
use openapi_terminal_app::spec::{Operation, Spec};
use openapi_terminal_app::ui::{endpoint_list, request_builder};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, ListState};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let spec = Spec::load_from_path(&cli.spec_path)?;
    let operations = spec.operations();

    run_app(&operations)
}

fn run_app(operations: &[Operation]) -> anyhow::Result<()> {
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

    let mut app = AppState::new();
    app.set_operation_count(operations.len());

    let result = event_loop(&mut terminal, &mut app, operations);

    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut AppState,
    operations: &[Operation],
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(frame.area());

            let endpoint_block = Block::bordered()
                .title("Endpoints")
                .border_style(pane_border_style(app.focused, Pane::EndpointList));
            let mut list_state = ListState::default();
            if !operations.is_empty() {
                list_state.select(Some(app.selected_operation_index));
            }
            frame.render_stateful_widget(
                endpoint_list::widget(operations).block(endpoint_block),
                chunks[0],
                &mut list_state,
            );

            let selected_operation = operations.get(app.selected_operation_index);
            let request_builder_block = Block::bordered()
                .title("Request Builder")
                .border_style(pane_border_style(app.focused, Pane::RequestBuilder));
            frame.render_widget(
                request_builder::widget(selected_operation).block(request_builder_block),
                chunks[1],
            );
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    break;
                }
                app.handle_key(key);
            }
        }
    }
    Ok(())
}

fn pane_border_style(focused: Pane, pane: Pane) -> Style {
    if focused == pane {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    }
}
