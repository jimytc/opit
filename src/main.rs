use clap::Parser;
use openapi_terminal_app::cli::Cli;
use openapi_terminal_app::spec::Spec;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let spec = Spec::load_from_path(&cli.spec_path)?;
    for op in spec.operations() {
        println!("{} {}", op.method, op.path);
    }
    Ok(())
}
