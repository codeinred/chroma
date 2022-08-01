use std::env::set_current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::report_err;
use crate::Result;

/// Description of the location of the project root, relative to the current working directory. Provides a 'cd_to_root' function, which moves the current working directory to the project root
pub struct ProjectRoot {
    /// Absolute path to project root
    pub project_root: PathBuf,
    /// Current working directory, relative to the project root
    pub current_dir: PathBuf,
    /// Name of file (either chroma.toml or Chroma.toml)
    pub file: &'static Path,
}

impl ProjectRoot {
    /// Change the current directory to the project root.
    pub fn cd_to_root(&self) -> Result<()> {
        Ok(set_current_dir(self.project_root.as_path())?)
    }
}

/// Get a relative path from the first path to the second path. Requires that both paths exist.
pub fn relative_path(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<PathBuf> {
    let from = from.as_ref().canonicalize()?;
    let to = to.as_ref().canonicalize()?;

    let mut from = from.iter();
    let mut to = to.iter();

    Ok(loop {
        let a = from.next();
        if a == None {
            break to.collect();
        }
        let b = to.next();
        if b == None {
            // The result is going to be ../../../.. with n repetitions of '..'
            let n_back = 1 + from.count();
            break std::iter::repeat("..").take(n_back).collect();
        }
        let a = a.unwrap();
        let b = b.unwrap();
        // If these path components aren't the same, this is the point where they diverge
        if a != b {
            let n_back = 1 + from.count();
            let mut result: PathBuf = std::iter::repeat("..").take(n_back).collect();
            result.push(b);
            for remaining in to {
                result.push(remaining);
            }
            break result;
        }
    })
}

/// Attempt to find either 'chroma.toml' or 'Chroma.toml' in the given starting directory, or one of it's parent directories
pub fn find_project_root_from(start: impl AsRef<Path>) -> Result<ProjectRoot> {
    let root: PathBuf = fs::canonicalize(start)?;

    for path in root.ancestors() {
        if path.join("chroma.toml").exists() {
            return Ok(ProjectRoot {
                project_root: path.into(),
                current_dir: relative_path(path, ".")?,
                file: "chroma.toml".as_ref(),
            });
        }
        if path.join("Chroma.toml").exists() {
            return Ok(ProjectRoot {
                project_root: path.into(),
                current_dir: relative_path(path, ".")?,
                file: "Chroma.toml".as_ref(),
            });
        }
    }
    report_err("Unable to find either 'chroma.toml' or 'Chroma.toml' in current directory, or it's parents.")
}

/// Attempt to find either 'chroma.toml' or 'Chroma.toml' in the current directory or one of it's parent directories
pub fn find_project_root() -> Result<ProjectRoot> {
    find_project_root_from(".")
}
