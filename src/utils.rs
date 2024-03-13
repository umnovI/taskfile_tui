use color_eyre::eyre::{ContextCompat, ErrReport};
use home::home_dir;
use std::path::{Path, PathBuf};

/// Get existing filepath from a list.
///
/// Args:
///     `file_names`: A list of files to look for. In order of most important first.
///
/// Returns:
///     First existing path found. `None` if none of the given files exist.
///
pub fn get_filepath<'a>(filenames: &'a [&str]) -> color_eyre::Result<Option<PathBuf>> {
    let home: PathBuf = home_dir().wrap_err("Could not find home path.")?;
    for name in filenames {
        let file = home.join(name);
        if file.is_file() {
            return Ok(Some(file));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {}
