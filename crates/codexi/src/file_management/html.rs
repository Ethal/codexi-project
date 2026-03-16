// src/file_management/html.rs

use std::fs;
use std::path::{Path, PathBuf};

use crate::file_management::{FileManagement, FileManagementError};

impl FileManagement {
    /// export as html
    pub fn export_html(data: &str, dir: &Path) -> Result<PathBuf, FileManagementError> {
        let file_path = dir.join("report.html");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, data)?;

        Ok(file_path)
    }
}
