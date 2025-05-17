use std::path::PathBuf;
use std::process;

mod error;
use clap::{Parser, Subcommand};

/// A framework for managing and maintaining git pre-commit hooks
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install the angler pre-commit script
    Install {
        /// Where to install the angler script (defaults to .git/hooks)
        #[clap(long)]
        dir: Option<PathBuf>,
    },

    /// Run pre-commit hooks
    Run {
        /// Run on all files, not just staged files
        #[clap(long)]
        all_files: bool,

        /// Specific filenames to run hooks on
        #[clap(value_parser)]
        files: Vec<PathBuf>,
    },
}

fn install(dir: Option<PathBuf>) -> Result<(), error::AnglerError> {
    println!("Installing angler pre-commit hook...");

    // This would contain the actual installation logic
    // For now, we'll just print what we would do

    let install_dir = dir.unwrap_or_else(|| PathBuf::from(".git/hooks"));
    println!("- Installing to directory: {}", install_dir.display());
    println!("- Config file is required");

    // Mock success for now
    Ok(())
}

fn run_hooks(all_files: bool, files: &[PathBuf]) -> Result<(), error::AnglerError> {
    println!("Running angler pre-commit hooks...");

    // This would contain the actual hook running logic
    // For now, we'll just print what we would do

    if all_files {
        println!("- Running on all files in the repository");
    } else if !files.is_empty() {
        println!("- Running on specific files: {:?}", files);
    } else {
        println!("- Running on staged files");
    }

    println!("- Running hooks sequentially");
    println!("- Running all hooks from config");

    // Mock success for now
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Install { dir } => install(dir),
        Commands::Run { all_files, files } => run_hooks(all_files, &files),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
