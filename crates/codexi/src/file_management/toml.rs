// src/file_management/toml.rs

use crate::exchange::Exchangeable;
use crate::file_management::{FileExchangeError, FileManagement, format::ExchangeSerdeFormat};
use std::path::Path;

impl FileManagement {
    pub fn export_toml<T: Exchangeable>(data: &T, dir: &Path) -> Result<(), FileExchangeError> {
        ExchangeSerdeFormat::Toml.export(data, dir)
    }

    pub fn import_toml<T: Exchangeable>(
        dir: &Path,
    ) -> Result<(T, Vec<T::Warning>), FileExchangeError> {
        ExchangeSerdeFormat::Toml.import(dir)
    }
}
