// src/file_management/backup.rs

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use chrono::Local;
use directories::UserDirs;
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use tar::{Archive, Builder};
use walkdir::WalkDir;

use crate::CODEXI_DATA_FORMAT_VERSION;
use crate::core::DataPaths;
use crate::file_management::{FileBackupError, FileManagement};

impl FileManagement {
    /// Creates a complete TAR.GZ backup of the application's data directory.
    ///
    /// The backup contains all application data except:
    /// - snapshots/
    /// - tmp/
    /// - trash/
    ///
    /// The returned path is the FULL path to the created archive.
    pub fn create_backup(paths: &DataPaths, target_dir_arg: Option<&str>) -> Result<PathBuf, FileBackupError> {
        let target_path = get_final_backup_path(target_dir_arg)?;

        // The data directory SHALL exist and contain at least the main data file.
        if !paths.main_file.exists() {
            return Err(FileBackupError::NoDirOrFile(format!(
                "The data directory ({:?}) does not exist or contains no file.",
                paths.root
            )));
        }

        let file = File::create(&target_path)?;

        // gzip compression:
        // Fast    = Compression::fast()
        // Default = Compression::default()
        // Best    = Compression::best()
        let encoder = GzEncoder::new(file, Compression::default());

        let mut tar = Builder::new(encoder);

        // Iterate through the data directory.
        for entry in WalkDir::new(&paths.root).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            // Exclude internal directories that must never be backed up.
            if path.starts_with(&paths.snapshots_dir)
                || path.starts_with(&paths.tmp_dir)
                || path.starts_with(&paths.trash_dir)
            {
                continue;
            }

            // Paths stored in the archive must be relative to the data directory.
            let relative_path = path.strip_prefix(&paths.root).map_err(|_| {
                FileBackupError::RelativePath("Failure to calculate relative path for archive.".to_string())
            })?;

            // Skip the root directory itself.
            if relative_path.as_os_str().is_empty() {
                continue;
            }

            // Avoid temporary files.
            if relative_path.to_string_lossy().contains(".temp") {
                continue;
            }

            if path.is_file() {
                tar.append_path_with_name(path, relative_path)?;
            } else if path.is_dir() {
                tar.append_dir(relative_path, path)?;
            }
        }

        let encoder = tar.into_inner()?;
        encoder.finish()?;

        Ok(target_path)
    }

    /// Restores the contents of a TAR.GZ backup into the application's
    /// data directory.
    ///
    /// Existing application data is removed before restoration.
    pub fn restore_backup(paths: &DataPaths, backup_path: &Path) -> Result<(), FileBackupError> {
        let codexi = &paths.main_file;

        // If an active ledger exists, use the normal cleanup process.
        // Otherwise remove leftover internal directories manually.
        if codexi.exists() {
            Self::clear_data(paths)?;
        } else {
            if paths.snapshots_dir.exists() {
                fs::remove_dir_all(&paths.snapshots_dir)?;
            }

            if paths.archives_dir.exists() {
                fs::remove_dir_all(&paths.archives_dir)?;
            }
        }

        fs::create_dir_all(&paths.root)?;

        let file = File::open(backup_path)?;
        let decoder = GzDecoder::new(file);

        let mut archive = Archive::new(decoder);

        // Extract all files into the application data directory.
        archive.unpack(&paths.root)?;

        Ok(())
    }
}

/// Determines the full path to the backup archive.
///
/// If `target_dir_arg` is:
/// - a directory -> a default filename is generated
/// - a .tar.gz file -> that filename is used
/// - None -> the user's Documents directory is used
fn get_final_backup_path(target_dir_arg: Option<&str>) -> Result<PathBuf, FileBackupError> {
    let now = Local::now();

    let default_filename = format!(
        "codexi_backup_{}_v{}.tar.gz",
        now.format("%Y%m%d_%H%M%S"),
        CODEXI_DATA_FORMAT_VERSION
    );

    let (target_dir, final_filename) = if let Some(path_str) = target_dir_arg {
        let path = PathBuf::from(path_str);

        let is_backup_file = path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().to_ascii_lowercase().ends_with(".tar.gz"));

        if is_backup_file {
            let filename = path
                .file_name()
                .ok_or_else(|| {
                    FileBackupError::InvalidBackupPath("The path specified for the backup is invalid.".to_string())
                })?
                .to_string_lossy()
                .into_owned();

            let dir = path
                .parent()
                .map(|p| {
                    if p.as_os_str().is_empty() {
                        PathBuf::from(".")
                    } else {
                        p.to_path_buf()
                    }
                })
                .unwrap_or_else(|| PathBuf::from("."));

            (dir, filename)
        } else {
            (path, default_filename)
        }
    } else {
        let user_dirs = UserDirs::new()
            .ok_or_else(|| FileBackupError::NoUserDirectory("Unable to find user directory (UserDirs).".to_string()))?;

        (
            user_dirs
                .document_dir()
                .unwrap_or_else(|| user_dirs.home_dir())
                .to_path_buf(),
            default_filename,
        )
    };

    fs::create_dir_all(&target_dir)?;

    Ok(target_dir.join(final_filename))
}
