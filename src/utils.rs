use color_eyre::eyre::ContextCompat;
use home::home_dir;
use std::path::PathBuf;

use crate::Args;

/// Get existing filepath from a list.
///
/// Args:
///     `file_names`: A list of files to look for. In order of most important first.
///
/// Returns:
///     First existing path found. `None` if none of the given files exist.
///
pub fn get_filepath(args: &Args, filenames: &[&str]) -> color_eyre::Result<Option<PathBuf>> {
    let path = if args.global {
        home_dir().wrap_err("Could not find home path.")?
    } else {
        let mut cwd = PathBuf::new();
        cwd.push(".");
        cwd
    };
    for name in filenames {
        let file = path.join(name);
        if file.is_file() {
            return Ok(Some(file));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {}
