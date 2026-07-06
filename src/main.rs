use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use openapi_terminal_app::cli::Cli;
use openapi_terminal_app::spec::{Operation, Spec};
use openapi_terminal_app::ui::endpoint_list;
use ratatui::widgets::Block;

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

    let result = event_loop(&mut terminal, operations);

    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    operations: &[Operation],
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| {
            let list = endpoint_list::widget(operations).block(Block::bordered().title("Endpoints"));
            frame.render_widget(list, frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                break;
            }
        }
    }
    Ok(())
}
