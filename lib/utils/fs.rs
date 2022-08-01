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

/// Attempt to find either 'chroma.toml' or 'Chroma.toml' in the current directory or one of it's root directories
pub fn find_project_root() -> Result<ProjectRoot> {
    let root: PathBuf = fs::canonicalize(".")?;

    for path in root.ancestors() {
        if path.join("chroma.toml").exists() {
            return Ok(ProjectRoot {
                project_root: path.into(),
                current_dir: root.strip_prefix(path)?.into(),
                file: "chroma.toml".as_ref(),
            });
        }
        if path.join("Chroma.toml").exists() {
            return Ok(ProjectRoot {
                project_root: path.into(),
                current_dir: root.strip_prefix(path)?.into(),
                file: "Chroma.toml".as_ref(),
            });
        }
    }
    report_err("Unable to find either 'chroma.toml' or 'Chroma.toml' in current directory, or it's parents.")
}
