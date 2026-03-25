// src/excnage/summary.rs

#[derive(Debug, Default, Clone)]
pub struct ImportSummary {
    pub name: String,
    pub created: usize,
    pub updated: usize,
    pub total_processed: usize,
}

impl ImportSummary {
    pub fn is_empty(&self) -> bool {
        self.total_processed == 0
    }
}
