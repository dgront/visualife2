use std::fs;
use std::path::{Path, PathBuf};

/// Loads SVG files from the given directory and file list.
/// Returns a vector of (file_name, file_contents) pairs.
#[allow(dead_code)]
pub(crate) fn load_expected_svgs<P: AsRef<Path>>(directory: P, file_names: &[&str]) -> Result<Vec<String>, String> {
    let base_path = directory.as_ref();
    let mut results = Vec::new();

    for &file_name in file_names {
        let full_path: PathBuf = base_path.join(file_name);
        let content = fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read {}: {}", full_path.display(), e))?;
        results.push(content);
    }

    Ok(results)
}