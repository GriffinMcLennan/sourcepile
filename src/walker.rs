use crate::filter::is_source_file;
use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Recursively collects file paths under `root`.
///
/// * `include_all` – include every file (ignoring extension filter)
/// * `respect_gitignore` – apply `.gitignore` & other VCS ignore rules
pub fn collect_files(
    root: &Path,
    include_all: bool,
    respect_gitignore: bool,
) -> Result<Vec<PathBuf>> {
    let mut builder = WalkBuilder::new(root);

    if !respect_gitignore {
        builder
            .git_exclude(false)
            .git_ignore(false)
            .git_global(false)
            .ignore(false);
    }

    let walker = builder.build();
    let mut files = Vec::new();

    for dent in walker {
        let dent = dent?;
        if dent.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            // Skip any file literally named "sourcepile.txt"
            if let Some(name) = dent.path().file_name().and_then(|s| s.to_str()) {
                if name == "sourcepile.txt" {
                    continue;
                }
            }
            if include_all || is_source_file(dent.path()) {
                files.push(dent.into_path());
            }
        }
    }

    Ok(files)
}