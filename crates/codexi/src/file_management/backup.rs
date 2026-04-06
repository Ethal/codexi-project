// src/file_management/backup.rs

use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use chrono::Local;
use directories::UserDirs;
use walkdir::WalkDir;
use zip::ZipArchive;
use zip::write::{FileOptions, ZipWriter};

use crate::CODEXI_DATA_FORMAT_VERSION;
use crate::core::DataPaths;
use crate::file_management::{FileBackupError, FileManagement};

impl FileManagement {
    /// Creates a complete ZIP backup of the application's data directory.
    /// The `target_path` is the FULL path where the ZIP file should be written.
    /// It includes all files except internal snapshots, tmp, trash.
    pub fn create_backup(paths: &DataPaths, target_dir_arg: Option<&str>) -> Result<PathBuf, FileBackupError> {
        let target_path = get_final_backup_path(target_dir_arg)?;

        let codexi_file = &paths.main_file;
        let snapshot_dir = &paths.snapshots_dir;
        let tmp_dir = &paths.tmp_dir;
        let trash_dir = &paths.trash_dir;

        // The data directory SHALL exist and contain at least the codexi.dat file
        if !codexi_file.exists() {
            return Err(FileBackupError::NoDirOrFile(format!(
                "The data directory ({:?}) does not exist or contains no file.",
                paths.root
            )));
        }

        // Create the ZIP file
        let file = File::create(&target_path)?;
        let mut zip = ZipWriter::new(file);

        // Compression options — unix permissions only on Unix/macOS
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        #[cfg(unix)]
        let options = options.unix_permissions(0o755);

        // Iterate the data directory (including codexi.dat and archives/,
        // exclude snapshot, tmp, trash)
        for entry in WalkDir::new(&paths.root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.starts_with(snapshot_dir) || path.starts_with(tmp_dir) || path.starts_with(trash_dir) {
                continue;
            }

            // Paths in the ZIP to be relative to the data_dir, not absolute.
            let name_in_zip = path
                .strip_prefix(&paths.root)
                .map_err(|_| {
                    FileBackupError::RelativePath("Failure to calculate relative path for archive.".to_string())
                })?
                .to_path_buf();

            if path.is_file() {
                let name_in_zip_str = name_in_zip
                    .to_str()
                    .ok_or_else(|| FileBackupError::InvalidPath("Path invalid (non-UTF8).".to_string()))?
                    .replace('\\', "/"); // normalize separators for Windows

                // Avoid adding temporary or locked files
                if name_in_zip_str.contains(".temp") {
                    continue;
                }

                zip.start_file(&name_in_zip_str, options)?;
                io::copy(&mut File::open(path)?, &mut zip)?;
            } else if path.is_dir() && !name_in_zip.as_os_str().is_empty() {
                // Add the directory (only if it is not the root directory itself)
                let name_in_zip_str = name_in_zip
                    .to_str()
                    .ok_or_else(|| FileBackupError::InvalidPath("Path invalid (non-UTF8).".to_string()))?
                    .replace('\\', "/"); // normalize separators for Windows

                zip.add_directory(&name_in_zip_str, options)?;
            }
        }

        zip.finish()?;
        Ok(target_path)
    }

    /// Restores the contents of a full ZIP backup to the application's data directory.
    /// The `zip_path` is the FULL path to the backup ZIP file.
    /// Existing files in the data directory will be overwritten.
    pub fn restore_backup(paths: &DataPaths, zip_path: &Path) -> Result<(), FileBackupError> {
        let codexi = &paths.main_file;

        // If active ledger exists → move to trash, otherwise clean dirs manually
        if codexi.exists() {
            Self::clear_data(paths)?;
        } else {
            let snapshots = &paths.snapshots_dir;
            let archives = &paths.archives_dir;

            if snapshots.exists() {
                fs::remove_dir_all(snapshots)?;
            }
            if archives.exists() {
                fs::remove_dir_all(archives)?;
            }
        }

        let file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;

        // Iterate over all files in the archive
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            // The destination path is data_dir + the path to the file in the ZIP archive
            let outpath = &paths.root.join(file.mangled_name());

            if file.is_dir() {
                fs::create_dir_all(outpath)?;
            } else if file.is_file() {
                // Ensure that the parent directory exists
                if let Some(p) = outpath.parent()
                    && !p.exists()
                {
                    fs::create_dir_all(p)?;
                }

                let mut outfile = File::create(outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }
}

/// Determines the full path to the ZIP backup file.
/// Uses `target_dir_arg` (optional string) or the default user directory.
fn get_final_backup_path(target_dir_arg: Option<&str>) -> Result<PathBuf, FileBackupError> {
    let now = Local::now();
    let default_filename = format!(
        "codexi_backup_{}_v{}.zip",
        now.format("%Y%m%d_%H%M%S"),
        CODEXI_DATA_FORMAT_VERSION
    );

    let target_dir: PathBuf;
    let final_filename: String;

    if let Some(path_str) = target_dir_arg {
        let path = PathBuf::from(path_str);

        if path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip")) {
            final_filename = path
                .file_name()
                .ok_or_else(|| {
                    FileBackupError::InvalidBackupPath("The path specified for the backup is invalid.".to_string())
                })?
                .to_string_lossy()
                .into_owned();

            target_dir = path
                .parent()
                .map(|p| {
                    if p.as_os_str().is_empty() {
                        PathBuf::from(".")
                    } else {
                        p.to_path_buf()
                    }
                })
                .unwrap_or(PathBuf::from("."));
        } else {
            target_dir = path;
            final_filename = default_filename;
        }
    } else {
        let user_dirs = UserDirs::new()
            .ok_or_else(|| FileBackupError::NoUserDirectory("Unable to find user directory (UserDirs).".to_string()))?;

        target_dir = user_dirs
            .document_dir()
            .unwrap_or_else(|| user_dirs.home_dir())
            .to_path_buf();

        final_filename = default_filename;
    };

    fs::create_dir_all(&target_dir)?;
    let final_path = target_dir.join(final_filename);

    Ok(final_path)
}
