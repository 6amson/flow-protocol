use clap::Parser;
use flow::{utils::errors::FlowError, Workspace};
use std::path::PathBuf;

/// Simple Flow user creator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    username: String,
    dir: Option<PathBuf>,
}

fn main() -> Result<(), FlowError> {
    let cli = Cli::parse();

    let base_dir = cli.dir.unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join("Flow")
    });

    let workspace = Workspace::new(&cli.username, base_dir);

    println!("Workspace path: {}", workspace.path.display());

    Ok(())
}
