use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    pub spec_path: PathBuf,

    #[arg(long)]
    pub bearer_token: Option<String>,

    #[arg(long = "header")]
    pub header: Vec<String>,
}
