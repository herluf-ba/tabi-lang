pub(crate) mod tabi;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs tabi file
    Run { file: String },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let result = match &cli.command {
        Commands::Run { file } => run(file),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn run(file_name: &String) -> Result<()> {
    let program = std::fs::read_to_string(file_name).context("could not read '{file}'")?;
    let value = tabi::run(file_name.to_string(), program)?;
    println!("{value}");
    Ok(())
}
