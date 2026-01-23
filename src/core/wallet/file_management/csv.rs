// src/core/wallet/file_management/csv.rs

use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::imex::OperationCsv;
use crate::core::wallet::imex::OperationExport;
use crate::core::wallet::imex::LedgerExport;
use crate::core::wallet::imex::EXPORT_VERSION;

impl Codexi {
    /// Export to csv
    pub fn export_csv(&self, dir: &Path) -> Result<()> {
        let file_path = dir.join("codexi.csv");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = fs::File::create(&file_path)?;
        let mut wtr = csv::Writer::from_writer(file);

        let export = self.export_ledger();
        for op in &export.operations {
            let csv_op = OperationCsv::from(op);
            wtr.serialize(csv_op)?;
        }

        wtr.flush()?;
        log::info!("Export CSV saved to {:?}", file_path);
        Ok(())
    }
    /// Import from csv
    pub fn import_csv(dir: &Path) -> Result<Self> {
        let file_path = dir.join("codexi.csv");

        let file = fs::File::open(&file_path)?;
        let mut rdr = csv::Reader::from_reader(file);

        let mut operations: Vec<OperationExport> = Vec::new();
        for result in rdr.deserialize::<OperationCsv>() {
            let csv_op = result
                .map_err(|e| anyhow!("Import CSV: {}", e))?;
            operations.push(csv_op.into());
        }
        let next_op_id = operations
            .iter()
            .map(|op| op.id)
            .max()
            .map(|id| id + 1)
            .unwrap_or(0);

        let import = LedgerExport {
            version: EXPORT_VERSION,
            operations:operations,
            next_op_id:next_op_id,
        };

        let mut codexi = Self::import_ledger(&import)?;

        codexi.operations.sort_by_key(|o| o.date);
        log::info!("Import CSV: {:?} loaded", file_path);
        Ok(codexi)
    }
}
