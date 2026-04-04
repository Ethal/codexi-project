// src/logic/loan/settings.rs

use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{
    core::parse_optional_decimal,
    logic::loan::{LoanError, LoanPolicy},
};

const LOAN_POLICY_FILENAME: &str = "loan_policy.json";

/// Persisted loan policy — all fields are primitives for easy serde
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanPolicySettings {
    pub type_interest: String,
    pub rate: String,
    pub free_days: u32,
    pub max_cap: Option<String>,
    pub max_days: Option<u32>,
    pub min_capital: Option<String>,
    pub max_penalty: Option<String>,
}

impl Default for LoanPolicySettings {
    fn default() -> Self {
        Self {
            type_interest: "linear".to_string(),
            rate: "1".to_string(),
            free_days: 7,
            max_cap: Some("50".to_string()),
            max_days: Some(30),
            min_capital: Some("100".to_string()),
            max_penalty: None,
        }
    }
}

impl LoanPolicySettings {
    /// Load from tmp_dir, or create default if absent
    pub fn load_or_create(tmp_dir: &Path) -> Self {
        let path = tmp_dir.join(LOAN_POLICY_FILENAME);
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            let settings = Self::default();
            // best effort — silent if tmp_dir does not exist yet
            let _ = settings.save(tmp_dir);
            settings
        }
    }

    /// Save to tmp_dir
    pub fn save(&self, tmp_dir: &Path) -> Result<(), LoanError> {
        let path = tmp_dir.join(LOAN_POLICY_FILENAME);
        let content = serde_json::to_string_pretty(self).map_err(|_| LoanError::PolicySerialize)?;
        std::fs::write(&path, content).map_err(|_| LoanError::PolicyWrite)?;
        Ok(())
    }

    /// Reset to default and save
    pub fn reset(tmp_dir: &Path) -> Result<Self, LoanError> {
        let settings = Self::default();
        settings.save(tmp_dir)?;
        Ok(settings)
    }

    /// Convert to domain LoanPolicy for use in loan calculations
    pub fn to_loan_policy(&self) -> Result<LoanPolicy, LoanError> {
        Ok(LoanPolicy {
            max_interest_cap: parse_optional_decimal(&self.max_cap, "max_cap")
                .map_err(|e| LoanError::PolicyParse(e.to_string()))?,

            max_duration_days: self.max_days.map(|d| Duration::days(d as i64)),

            min_capital: parse_optional_decimal(&self.min_capital, "min_cap")
                .map_err(|e| LoanError::PolicyParse(e.to_string()))?,

            max_penality: parse_optional_decimal(&self.max_penalty, "min_penality")
                .map_err(|e| LoanError::PolicyParse(e.to_string()))?,
        })
    }
}
