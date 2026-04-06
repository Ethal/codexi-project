// src/core/paths.rs

use chrono::{Local, NaiveDate};
use nulid::Nulid;
use std::path::{Path, PathBuf};

use crate::core::{format_date, format_date_time_short, format_id};

// Disk structure of Codexi
/*
├── <APP_NAME>.<EXT_MAIN>
├── <APP_NAME>.<EXT_CFG>
├── <DIR_ARCHIVES>
│   └── <ACCOUNT_ID>
│       ├── <ACCOUNT_ID>_<APP_NAME>_<YYYY>-<MM>-<DD>.<EXT_ARCHIVE>
│       ├── ...
│       └── ...
├── <DIR_SNAPSHOTS>
│   └── <APP_NAME>_<YYYYMMDD>_<HHMMSS>.<EXT_SNAPSHOT>
│   └── ....
├── <DIR_TMP>
│   └── ......
└── <DIR_TRASH>
    └── <YYYYMMDD>_<HHMMSS>
        ├── archives/...
        ├── codexi.dat
        └── snapshots/...
*/

/// A resolved file path with its filename
pub struct ResolvedPath {
    pub path: PathBuf,
    pub filename: String,
}

pub struct DataPaths {
    pub root: PathBuf,          // data_dir
    pub main_file: PathBuf,     // data_dir/codexi.dat
    pub config_file: PathBuf,   // data_dir/codexi.cfg
    pub archives_dir: PathBuf,  // data_dir/archives/
    pub snapshots_dir: PathBuf, // data_dir/snapshots/
    pub tmp_dir: PathBuf,       // data_dir/tmp/
    pub trash_dir: PathBuf,     // data_dir/trash/
}

impl DataPaths {
    pub(crate) const APP_NAME: &'static str = "codexi";
    pub(crate) const EXT_MAIN: &'static str = "dat";
    pub(crate) const EXT_CFG: &'static str = "cfg";
    pub(crate) const EXT_ARCHIVE: &'static str = "cld";
    pub(crate) const EXT_SNAPSHOT: &'static str = "snp";
    pub(crate) const DIR_ARCHIVES: &'static str = "archives";
    pub(crate) const DIR_SNAPSHOTS: &'static str = "snapshots";
    pub(crate) const DIR_TMP: &'static str = "tmp";
    pub(crate) const DIR_TRASH: &'static str = "trash";

    pub fn new(data_dir: &Path) -> Self {
        let root = data_dir.to_path_buf();

        Self {
            config_file: root.join(format!("{}.{}", Self::APP_NAME, Self::EXT_CFG)),
            main_file: root.join(format!("{}.{}", Self::APP_NAME, Self::EXT_MAIN)),
            archives_dir: root.join(Self::DIR_ARCHIVES),
            snapshots_dir: root.join(Self::DIR_SNAPSHOTS),
            tmp_dir: root.join(Self::DIR_TMP),
            trash_dir: root.join(Self::DIR_TRASH),
            root,
        }
    }

    /// archives/<account_id>/
    pub fn archive_dir(&self, account_id: &Nulid) -> PathBuf {
        self.archives_dir.join(format_id(*account_id))
    }

    /// archives/<account_id>/<account_id>_codexi_<date>.cld
    pub fn archive_path(&self, account_id: &Nulid, date: &NaiveDate) -> ResolvedPath {
        let filename = format!(
            "{}_{}_{}.{}",
            format_id(*account_id),
            Self::APP_NAME,
            format_date(*date),
            Self::EXT_ARCHIVE,
        );
        let path = self.archive_dir(account_id).join(&filename);
        ResolvedPath { path, filename }
    }

    /// snapshots/codexi_<timestamp>.snp
    pub fn snapshot_path(&self) -> ResolvedPath {
        let filename = format!(
            "{}_{}.{}",
            Self::APP_NAME,
            format_date_time_short(Local::now().naive_local()),
            Self::EXT_SNAPSHOT,
        );
        let path = self.snapshots_dir.join(&filename);

        ResolvedPath { path, filename }
    }

    /// trash/<timestamp>/
    pub fn trash_path(&self) -> PathBuf {
        self.trash_dir.join(format_date_time_short(Local::now().naive_local()))
    }
}
