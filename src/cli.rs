use std::path::PathBuf;

use clap::{ArgGroup, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(group(ArgGroup::new("stream").args(["target", "stdio"]).multiple(false).required(true)))]
pub struct Cli {
    /// A file to be processed.
    pub target: Option<PathBuf>,

    /// Read the target from stdin and write the formatted content to stdout.
    #[arg(long)]
    pub stdio: bool,

    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Format the target file or stdin.
    Format {
        #[arg(long)]
        no_check: bool,
    },

    /// Check if the target file or stdin is correctly formatted.
    Check,
}
