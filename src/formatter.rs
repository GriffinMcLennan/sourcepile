use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Concatenate every file in `files`.
///
/// Each section is prefixed with `// relative/path`.
pub fn format_files(
    root: &Path,
    files: &[PathBuf],
    max_lines: Option<usize>,
) -> Result<String> {
    let mut out = String::new();

    for file in files {
        let rel = file.strip_prefix(root).unwrap_or(file);
        out.push_str("// ");
        out.push_str(&rel.to_string_lossy());
        out.push('\n');

        let contents = fs::read_to_string(file)
            .with_context(|| format!("reading {}", rel.display()))?;

        let body = if let Some(limit) = max_lines.filter(|&n| n > 0) {
            // keep only the first `limit` lines
            contents
                .lines()
                .take(limit)
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            contents
        };

        out.push_str(&body);
        if !body.ends_with('\n') {
            out.push('\n');
        }
        out.push('\n');
    }

    Ok(out)
}