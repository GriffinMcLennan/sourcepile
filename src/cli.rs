use clap::{ArgAction, Parser};

/// Flatten a directory of source files into a single text file.
///
/// By default `sourcepile` walks the current directory, honours `.gitignore`,
/// and writes `./sourcepile.txt`.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Directory to scan (defaults to current directory)
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: std::path::PathBuf,

    /// Output file (defaults to ./sourcepile.txt)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<std::path::PathBuf>,

    /// Include *all* files (otherwise only common source-file extensions)
    #[arg(long)]
    pub all: bool,

    /// Ignore `.gitignore` rules (include files that would normally be skipped)
    #[arg(long, action = ArgAction::SetTrue)]
    pub ignore_gitignore: bool,

    /// Maximum lines to read from each file (0 = unlimited)
    #[arg(short = 'l', long = "max-lines", value_name = "N")]
    pub max_lines: Option<usize>,
}