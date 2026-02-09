// src/core/wallet/migrations.rs

use anyhow::{Result, bail, Context};
use std::path::Path;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::operation_kind::OperationKind;
use crate::core::wallet::operation_flow::OperationFlow;
use crate::core::wallet::operation::Operation;

#[derive(Serialize, Deserialize)]
pub struct CodexiV1 {
    pub operations: Vec<OperationV1>,
}

#[derive(Serialize, Deserialize)]
pub struct OperationV1 {
    pub kind: OperationKind,
    pub flow: OperationFlow,
    pub date: NaiveDate,
    pub amount: f64,
    pub description: String,
}

impl Codexi {

    /// Migrate current ledger and associated archives
    pub fn migrate_v1(dir: &Path, archives: &[String]) -> Result<()> {
        let file_path = dir.join("codexi.dat");
        let old_codexi = Self::load_format_v1(&file_path)?;
        let codexi: Codexi = Self::migrate_v1_to_v2(old_codexi);
        codexi.save_current_ledger(&dir)?;
        log::info!("migarte current ledger: {:?}", &file_path);

        for arch in archives {
            let close_date = arch
                .trim_start_matches("codexi_")
                .trim_end_matches(".cld");
            let file_path = dir.join("archives").join(arch);
            let old_codexi = Self::load_format_v1(&file_path)?;
            let codexi: Codexi = Self::migrate_v1_to_v2(old_codexi);
            codexi.save_archive(&codexi, &close_date)?;
            log::info!("migarte archive: {:?}", &file_path);
        }
        Ok(())
    }

    /// load ledger v1
    pub fn load_format_v1(file_path: &Path) -> Result<CodexiV1> {
        if !file_path.exists() {
            bail!("No codexi file, cannot migrate");
        }

        let bytes = std::fs::read(file_path)?;
        let old = bincode::deserialize(&bytes)
            .context("the codexi.dat is not migrable")?;

        Ok(old)
    }
    /// Migrate structure from V1 to V2
    pub fn migrate_v1_to_v2(old: CodexiV1) -> Self {
        let operations: Vec<Operation> = old
            .operations
            .into_iter()
            .enumerate()
            .map(|(idx, op)| {
                let amount = Decimal::from_f64(op.amount)
                                .expect("Invalid f64 amount during migration");
                Operation {
                    id: idx,
                    kind: op.kind,
                    flow: op.flow,
                    date: op.date,
                    amount: amount,
                    description: op.description,
                    void_of: None,
                }
            })
            .collect();

        let next_op_id = operations
            .iter()
            .map(|op| op.id)
            .max()
            .map(|id| id + 1)
            .unwrap_or(0);

        Self { operations, next_op_id }
    }
}
