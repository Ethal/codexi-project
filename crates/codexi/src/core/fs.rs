// src/core/fs.rs

use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use crate::core::CoreError;

const fn project_dirs_args() -> (&'static str, &'static str, &'static str) {
    ("fr", "ethal", "codexi")
}

fn get_project_dirs() -> Result<ProjectDirs, CoreError> {
    let (q, o, a) = project_dirs_args();
    ProjectDirs::from(q, o, a)
        .ok_or_else(|| CoreError::NoDataDirectory("Could not determine project directories".to_string()))
}

/// Returns the OS data directory for codexi.
/// Linux   : ~/.local/share/fr.ethal.codexi/
/// macOS   : ~/Library/Application Support/fr.ethal.codexi/
pub fn get_data_dir() -> Result<PathBuf, CoreError> {
    let dirs = get_project_dirs()?;
    let path = dirs.data_dir().to_path_buf();
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Returns the OS config directory for codexi.
/// Linux   : ~/.config/codexi/
/// macOS   : ~/Library/Application Support/fr.ethal.codexi/
pub fn get_config_dir() -> Result<PathBuf, CoreError> {
    let dirs = get_project_dirs()?;
    let path = dirs.config_dir().to_path_buf();
    fs::create_dir_all(&path)?;
    Ok(path)
}
