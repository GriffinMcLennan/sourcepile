mod cli;
mod filter;
mod formatter;
mod walker;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use std::{fs::File, io::Write, path::PathBuf};

fn main() -> Result<()> {
    // Parse flags / args
    let cli = Cli::parse();

    // Canonicalise the target directory
    let root = cli
        .path
        .canonicalize()
        .with_context(|| format!("`{}` is not a valid directory", cli.path.display()))?;

    // Discover files
    let files = walker::collect_files(&root, cli.all, !cli.ignore_gitignore)?;

    // Concatenate them all
    let dump = formatter::format_files(&root, &files, cli.max_lines)?;

    // Determine output path (default: ./sourcepile.txt)
    let outfile: PathBuf = cli
        .output
        .unwrap_or_else(|| std::env::current_dir().unwrap().join("sourcepile.txt"));

    // Write
    let mut f = File::create(&outfile)
        .with_context(|| format!("failed to create {}", outfile.display()))?;
    f.write_all(dump.as_bytes())
        .with_context(|| format!("failed to write {}", outfile.display()))?;

    println!("Wrote {} files → {}", files.len(), outfile.display());
    Ok(())
}