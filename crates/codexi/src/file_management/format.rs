// src/file_management/format.rs
//
// This is the only layer that deals with I/O AND serde.
// Central dispatch: given a format + a type T, handle the serde round-trip.
// This is the only place in the codebase that knows about serde_json and toml.
// It bridges ExchangeError (domain) → FileExchangeError (I/O) via From.

use std::fs;
use std::path::Path;

use crate::exchange::Exchangeable;
use crate::file_management::FileExchangeError;

pub enum ExchangeSerdeFormat {
    Json,
    Toml,
}

impl ExchangeSerdeFormat {
    /// Export: domain value → file on disk.
    pub fn export<T: Exchangeable>(&self, value: &T, dir: &Path) -> Result<(), FileExchangeError> {
        // Build filename: e.g. "account.json" or "currencies.toml"
        let filename = format!("{}.{}", T::exchange_filename(), self.extension());
        let file_path = dir.join(&filename);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let exchange = value.to_exchange();
        let content = match self {
            Self::Json => serde_json::to_string_pretty(&exchange)?,
            Self::Toml => toml::to_string_pretty(&exchange)?,
        };
        fs::write(&file_path, content)?;
        Ok(())
    }

    /// Import: file on disk → domain value + warnings.
    pub fn import<T: Exchangeable>(
        &self,
        dir: &Path,
    ) -> Result<(T, Vec<T::Warning>), FileExchangeError> {
        let filename = format!("{}.{}", T::exchange_filename(), self.extension());
        let file_path = dir.join(&filename);
        let content = fs::read_to_string(&file_path)?;

        let exchange: T::Exchange = match self {
            Self::Json => serde_json::from_str(&content)?,
            Self::Toml => toml::from_str(&content)?,
        };
        T::from_exchange(exchange).map_err(FileExchangeError::from)
    }

    fn extension(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Toml => "toml",
        }
    }
}
