// src/file_management/toml.rs

use std::path::Path;

use crate::{
    exchange::Exchangeable,
    file_management::{ExchangeSerdeFormat, FileExchangeError, FileManagement},
};

impl FileManagement {
    pub fn export_toml<T: Exchangeable>(data: &T, dir: &Path) -> Result<(), FileExchangeError> {
        ExchangeSerdeFormat::Toml.export(data, dir)
    }

    pub fn import_toml<T: Exchangeable>(dir: &Path) -> Result<(T, Vec<T::Warning>), FileExchangeError> {
        ExchangeSerdeFormat::Toml.import(dir)
    }
}
