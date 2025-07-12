use std::path::Path;

/// Very small allow-list of typical “code” extensions.
pub fn is_source_file(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(
            ext,
            "rs" | "ts" | "tsx" | "js" | "jsx" | "json" |
            "py" | "java" | "kt" | "swift" | "c"  | "cpp" | "h" | "hpp" |
            "go" | "rb" | "php" | "scala" | "dart" |
            "css" | "html" | "md"  | "toml" | "yaml" | "yml" | "txt"
        ),
        None => false,
    }
}