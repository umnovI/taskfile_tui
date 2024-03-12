use serde_yaml;
use std::{array, fs, path::Path};

/// Get existing filepath from a list.
///
/// Args:
///     `file_names`: A list of files to look for. In order of most important first.
///
/// Returns:
///     First existing path found. `None` if none of the given files exist.
///
pub fn get_filepath<'a>(filenames: &'a [&str]) -> Option<&'a Path> {
    for name in filenames {
        let file = Path::new(name);
        if file.is_file() {
            return Some(file);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::{clone, fs::File};
    use tempfile::tempdir;
}
