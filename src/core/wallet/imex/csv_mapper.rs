// src/core/wallet/imex/csv_mapper.rs

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::core::wallet::operation_kind::OperationKind;
use crate::core::wallet::operation_flow::OperationFlow;
use crate::core::wallet::system_kind::SystemKind;
use crate::core::wallet::regular_kind::RegularKind;
use crate::core::wallet::imex::OperationExport;

/// CSV-safe enums
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OperationKindCsv {
    Init,
    Close,
    Adjust,
    Void,
    Transaction,
    Fee,
    Transfer,
    Refund,
}

/// CSV row representation
#[derive(Debug, Serialize, Deserialize)]
pub struct OperationCsv {
    pub id: usize,
    pub kind: OperationKindCsv,
    pub flow: OperationFlow,
    pub date: NaiveDate,
    pub amount: Decimal,
    pub description: String,
    pub void_of: Option<usize>,
}

/// Mapping: operation export -> CSV
impl From<&OperationExport> for OperationCsv {
    fn from(op: &OperationExport) -> Self {
        let kind = match &op.kind {
            OperationKind::System(SystemKind::Init) => OperationKindCsv::Init,
            OperationKind::System(SystemKind::Close) => OperationKindCsv::Close,
            OperationKind::System(SystemKind::Adjust) => OperationKindCsv::Adjust,
            OperationKind::System(SystemKind::Void) => OperationKindCsv::Void,
            OperationKind::Regular(RegularKind::Transaction) => OperationKindCsv::Transaction,
            OperationKind::Regular(RegularKind::Fee) => OperationKindCsv::Fee,
            OperationKind::Regular(RegularKind::Transfer) => OperationKindCsv::Transfer,
            OperationKind::Regular(RegularKind::Refund) => OperationKindCsv::Refund,
        };

        Self {
            id: op.id,
            kind,
            flow: op.flow,
            date: op.date,
            amount: op.amount,
            description: op.description.clone(),
            void_of: op.void_of,
        }
    }
}

/// Mapping: CSV -> operation export
impl From<OperationCsv> for OperationExport {
    fn from(csv: OperationCsv) -> Self {
        let kind = match csv.kind {
            OperationKindCsv::Init => {
                OperationKind::System(SystemKind::Init)
            }
            OperationKindCsv::Close => {
                OperationKind::System(SystemKind::Close)
            }
            OperationKindCsv::Adjust => {
                OperationKind::System(SystemKind::Adjust)
            }
            OperationKindCsv::Void => {
                OperationKind::System(SystemKind::Void)
            }
            OperationKindCsv::Transaction => {
                OperationKind::Regular(RegularKind::Transaction)
            }
            OperationKindCsv::Fee => {
                OperationKind::Regular(RegularKind::Fee)
            }
            OperationKindCsv::Transfer => {
                OperationKind::Regular(RegularKind::Transfer)
            }
            OperationKindCsv::Refund => {
                OperationKind::Regular(RegularKind::Refund)
            }
        };


        OperationExport {
            id: csv.id,
            kind: kind,
            flow: csv.flow,
            date: csv.date,
            amount: csv.amount,
            description: csv.description,
            void_of: csv.void_of,
        }
    }
}

/*---------------- TEST -------------------------*/

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;


    #[test]
    fn csv_round_trip_void_of_none() {
        let op = OperationExport {
            id : 1,
            kind: OperationKind::Regular(RegularKind::Transaction),
            flow: OperationFlow::Credit,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount: Decimal::ONE_HUNDRED,
            description: "test".to_string(),
            void_of: None,
        };

        let csv = OperationCsv::from(&op);
        let op2: OperationExport = csv.into();

        assert_eq!(op.id, op2.id);
        assert_eq!(op.kind, op2.kind);
        assert_eq!(op.flow, op2.flow);
        assert_eq!(op.date, op2.date);
        assert_eq!(op.amount, op2.amount);
        assert_eq!(op.description, op2.description);
        assert_eq!(op.void_of, op2.void_of);
    }

    #[test]
    fn csv_round_trip_void_of_exist() {
        let op = OperationExport {
            id : 1,
            kind: OperationKind::System(SystemKind::Void),
            flow: OperationFlow::Credit,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount: Decimal::ONE_HUNDRED,
            description: "test".to_string(),
            void_of: Some(5),
        };

        let csv = OperationCsv::from(&op);
        let op2: OperationExport = csv.into();

        assert_eq!(op.id, op2.id);
        assert_eq!(op.kind, op2.kind);
        assert_eq!(op.flow, op2.flow);
        assert_eq!(op.date, op2.date);
        assert_eq!(op.amount, op2.amount);
        assert_eq!(op.description, op2.description);
        assert_eq!(op.void_of, op2.void_of);
    }

    #[test]
    fn csv_round_trip_all_kinds() {
        let kinds = vec![
            OperationKind::System(SystemKind::Init),
            OperationKind::System(SystemKind::Close),
            OperationKind::System(SystemKind::Adjust),
            OperationKind::System(SystemKind::Void),
            OperationKind::Regular(RegularKind::Transaction),
            OperationKind::Regular(RegularKind::Fee),
            OperationKind::Regular(RegularKind::Transfer),
            OperationKind::Regular(RegularKind::Refund),
        ];

        for (i, kind) in kinds.into_iter().enumerate() {
            let op = OperationExport {
                id: i,
                kind,
                flow: OperationFlow::Debit,
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                amount: Decimal::ONE,
                description: "test".into(),
                void_of: None,
            };

            let csv = OperationCsv::from(&op);
            let op2: OperationExport = csv.into();
            assert_eq!(op.kind, op2.kind);
        }
    }

}
