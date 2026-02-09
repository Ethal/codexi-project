// src/ccore/wallet/codexi.rs

use anyhow::{Result, anyhow};
use std::mem;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, Local};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::core::wallet::operation_flow::OperationFlow;
use crate::core::wallet::operation_kind::OperationKind;
use crate::core::wallet::system_kind::SystemKind;
use crate::core::wallet::regular_kind::RegularKind;
use crate::core::wallet::operation::Operation;

/// Struct representing the codexi
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Codexi {
    pub operations: Vec<Operation>,
    pub next_op_id: usize,
}
/// Methods for codexi
impl Codexi {

    /// This function adds a new operation to the codexi while ensuring data integrity.
    /// ex: codexi.add_operation(...);
    /// It checks for date conflicts with existing system operations (Init, Close, Adjust)
    /// and ensures that debit operations do not exceed the current balance.
    pub fn add_operation(&mut self,
        kind:OperationKind,
        flow: OperationFlow,
        date: &str,
        amount: Decimal,
        description: &str,
        void_of: Option<usize>,
    ) -> Result<()>
    {

        let search_items = self.search(None, None, None, None, None, None, None, None)?;
        if search_items.is_empty() && !matches!(kind, OperationKind::System(SystemKind::Init)) {
            log::warn!("No data, considered to perform a system init command (codexi system init YYYY-MM-DD AMOUNT)");
            return Ok(());
        }


        let new_op_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;

        let latest_close_date = self.operations.iter()
            .filter(|op| matches!(op.kind, OperationKind::System(SystemKind::Close)))
            .map(|op| op.date)
            .max();

        let latest_non_strict_date = self.operations.iter()
            .filter(|op| matches!(op.kind, OperationKind::System(SystemKind::Init) | OperationKind::System(SystemKind::Adjust)))
            .map(|op| op.date)
            .max();


        if let Some(close_date) = latest_close_date {
            if new_op_date <= close_date {
                log::warn!(
                    "Operation date ({}) cannot be on or before the last period close date ({}).",
                    new_op_date, close_date
                );
                return Ok(());
            }
        }

        if let Some(anchor_date) = latest_non_strict_date {
            if new_op_date < anchor_date {
                log::warn!(
                    "Operation date ({}) cannot be before the latest system anchor date ({}).",
                    new_op_date, anchor_date
                );
                return Ok(());
            }
        }

        if amount <= Decimal::ZERO {
            log::warn!("Operation amount ({:.2}) cannot be negative or egal ro zero).", amount);
            return Ok(());
        }

        let op = Operation::new(self.next_op_id, kind, flow, date, amount, description, void_of)?;
        self.next_op_id += 1;
        self.operations.push(op.clone());
        self.operations.sort_by_key(|o| o.date);
        log::info!("Operation added : {}", op);
        Ok(())
    }

    /// This function void an operation at the specified index.
    /// ex: codexi.system void 3;
    /// It checks if the operation is a system operation (Init, Close, Void, Adjust) and prevents void if so.
    /// It returns an error if the index is out of bounds or if void is not allowed.
    pub fn void_operation(&mut self, index: usize) -> Result<()> {
        let all_ops: Vec<&Operation> = self.operations.iter().collect();
        let original = self.operations
                .iter()
                .find(|op| op.id == index)
                .ok_or_else(|| anyhow!("Operation #{} not found", index))?;

        if original.is_voided(&all_ops) {
            log::warn!("Operation #{} has already been voided.", index);
            return Ok(());
        }

        if matches!(original.kind, OperationKind::System(_)) {
            log::warn!("Operation #{} cannot be void: it is a protected system entry (Initial Balance, Void, Adjustment or Carried Forward Balance).",
                index
            );
            return Ok(());
        }

        let void_date_str = Local::now().date_naive().to_string();
        let void_flow = original.flow.opposite();
        let void_amount = original.amount;
        let void_description = format!(
            "VOID #{}: {} ({} {})",
            index,
            original.description,
            original.flow,
            original.amount
        );

        self.add_operation(
            OperationKind::System(SystemKind::Void),
            void_flow,
            &void_date_str,
            void_amount,
            &void_description,
            Some(index),
        )?;
        log::info!("Operation #{} successfully void.", index);

        Ok(())
    }

    /// Sets the initial balance of the codexi.
    /// ex: codexi.initialize(1000.0, "2024-07-01");
    /// This function creates an initial operation representing the starting balance.
    /// It should only be called when the codexi is empty.
    pub fn initialize(
        &mut self,
        amount: Decimal,
        date_str: &str,
    ) -> Result<()>
    {
        if !self.operations.is_empty() {
            log::warn!("The codexi is not empty. Cannot set initial balance.");
            return Ok(());
        }

        let op_flow = OperationFlow::from_sign(amount);
        let description = format!("INITIAL AMOUNT");

        // 3. Créer l'opération
        self.add_operation(
            OperationKind::System(SystemKind::Init) ,
            op_flow,
            &date_str,
            amount.abs(), // Utiliser la valeur absolue
            &description,
            None,
        )?;

        log::info!("codexi initialized with a balance of {:.2} on {}.",
            amount,
            date_str,
        );
        Ok(())
    }

    /// This function adjusts the codexi to match a physical balance.
    /// It calculates the difference and creates an adjustment operation if needed.
    /// Negative physical balances are not allowed.
    /// ex: codexi.system adjust 2024-07-15 950.0 ;
    pub fn adjust_balance(
        &mut self,
        physical_amount: Decimal,
        date_str: &str,
    ) -> Result<()>
    {

        let balance_items = self.search(None, None, None, None, None, None, None, None)?;
        let current_balance = self
            .balance(&balance_items)
            .unwrap_or_default();

        let difference = physical_amount - current_balance.total;

        if difference.abs() < dec!(0.001) || difference.abs() == Decimal::ZERO {
            log::info!("No adjustment needed. Theoretical balance ({:.2}) matches physical balance ({:.2}).",
                    current_balance.total,
                    physical_amount,
            );
            return Ok(());
        }

        let adjustment_flow = OperationFlow::from_sign(difference);
        let adjustment_amount = difference.abs();

        let description = format!("ADJUSTMENT: Deviation of {:.2} to reach physical balance {:.2}",
            adjustment_amount,
            physical_amount,
        );

        self.add_operation(
            OperationKind::System(SystemKind::Adjust),
            adjustment_flow,
            &date_str,
            adjustment_amount,
            &description,
            None,
        )?;

        log::warn!("ADJUSTMENT MADE: Added a {} of {:.2} to correct the balance.",
                adjustment_flow,
                adjustment_amount,
        );

        Ok(())
    }

    /// This function closes the current accounting period by archiving all operations
    /// up to the specified closing date and creating a new "Carried Forward Solde" operation.
    /// ex: codexi.close_period("2024-07-31", vec!["End of July".to_string()]);
    /// It saves the archived operations to a file and updates the codexi accordingly.
    /// The description_parts are concatenated to describe the closing operation.
    pub fn close_period(
        &mut self,
        close_date_str: &str,
        description_parts: Vec<String>,
    ) -> Result<()>
    {
        let close_date = NaiveDate::parse_from_str(close_date_str, "%Y-%m-%d")?;

        let mut current_closing_balance  = Decimal::ZERO;
        let mut archived_operations = Vec::new();

        let original_operations = mem::take(&mut self.operations);

        for op in original_operations.into_iter() {
            let op_date = op.date;

            if op_date <= close_date {

                match op.kind {
                    OperationKind::System(SystemKind::Init) | OperationKind::System(SystemKind::Close) => {
                        archived_operations.push(op.clone());
                        match op.flow {
                            OperationFlow::Credit => current_closing_balance = op.amount,
                            OperationFlow::Debit => current_closing_balance = -op.amount,
                            OperationFlow::None => {},
                        }
                    }
                    OperationKind::System(SystemKind::Adjust) |
                    OperationKind::System(SystemKind::Void) |
                    OperationKind::Regular(RegularKind::Transaction) |
                    OperationKind::Regular(RegularKind::Fee) |
                    OperationKind::Regular(RegularKind::Transfer) |
                    OperationKind::Regular(RegularKind::Refund) => {
                        match op.flow {
                            OperationFlow::Credit => current_closing_balance += op.amount,
                            OperationFlow::Debit => current_closing_balance -= op.amount,
                            OperationFlow::None => {},
                        }
                        archived_operations.push(op);
                    }
                }
            } else {
                self.operations.push(op);
            }
        }

        // If there's nothing to close, we stop.
        if archived_operations.is_empty() && self.operations.iter().all(|op| !matches!(op.kind,
            OperationKind::System(SystemKind::Init) |
            OperationKind::System(SystemKind::Close)))
        {
            // Management logic if the codexi is empty or contains only previous anchors.
            // If there are no transactions to archive, nothing is done.
            log::info!("No transactions (Adjust/Others) found to archive on or before {}.", close_date_str);
            return Ok(());
        }

        // --- PART 1: ARCHIVE MANAGEMENT ---

        // Save the archive if there are transactions to archive.
        if !archived_operations.is_empty() {
            let mut archive_export = self.clone();
            archive_export.operations = archived_operations.clone();
            self.save_archive(&archive_export, close_date_str)?;
            log::info!("{} operations archived", archived_operations.len());
        }

        // --- PART 2: CREATION OF THE NEW ANCHOR ---

        let net_solde = current_closing_balance;

        // 1. Create the new Carry Forward Balance operation
        let new_flow = OperationFlow::from_sign(net_solde);
        let new_amount = net_solde.abs();
        let description = format!("BALANCE DEFERRED: {:.2} {}",
            new_amount,
            description_parts.join(" "),
        );

        let new_op = Operation::new(
            self.next_op_id,
            OperationKind::System(SystemKind::Close),
            new_flow,
            close_date_str,
            new_amount,
            description,
            None,
        )?;
        self.next_op_id += 1;

        // 2. Add the new anchor to the vector.
        // This new anchor replaces all old anchors and transactions up to close_date.
        self.operations.push(new_op);

        // 3. Sort the final vector (so that the new anchor is in the correct position)
        // We sort by both date and type to resolve conflicts on the same day.
        self.operations.sort_by(|a, b| {
            // Primary sorting by date
            let date_order = a.date.cmp(&b.date);
            if date_order != Ordering::Equal {
                return date_order;
            }
            // Secondary sorting for equal dates
            a.kind.cmp(&b.kind)
        });

        log::warn!("PERIOD CLOSED: All transactions up to {} archived and replaced by single Close entry.", close_date_str);

        Ok(())
    }

    /// Get the operations with balance
    pub fn get_operations_with_balance(&self) -> Vec<(&Operation, Decimal)> {
        let mut cur_bal = Decimal::ZERO;
        let mut out = Vec::new();

        for op in &self.operations {
            cur_bal = Self::calculate_new_balance(cur_bal, op).unwrap_or(Decimal::ZERO);
            out.push((op, cur_bal));
        }

        out
    }

    /// calculate the new balance.
    fn calculate_new_balance(
        mut cur_bal: Decimal,
        op: &Operation,
    ) -> Result<Decimal>
    {
        match op.flow {
            OperationFlow::Credit => cur_bal += op.amount,
            OperationFlow::Debit => cur_bal -= op.amount,
            OperationFlow::None => {},
        };
        Ok(cur_bal)
    }

}

/*---------------- TEST -------------------------*/
#[cfg(test)]
mod tests {

    use super::*;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    fn setup_empty_codexi() -> Codexi {
        // init
        Codexi::default()
    }

    // Helper function to initialize with known data
    fn setup_codexi_with_data() -> Codexi {
        let mut cb = Codexi::default();

        // #0 Init (2025-09-01) : 200.00 ID = 0
        // credit: 200.00, debit: 0.0, balance: 200.0
        // #1 Debit (2025-10-04) : 14.20 ID = 4
        // #2 Credit (2025-10-08) : 50.00 ID = 2
        // #3 Debit (2025-10-21) : 44.80 ID = 5
        // #4 Debit (2025-10-21) : 11.00 ID = 8
        // credit; 50.00, debit: 70.00 balance: -20.0
        // #5 Credit (2025-11-05) : 100.00 ID = 1
        // #6 Debit (2025-11-12) : 15.70 ID = 7
        // #7 Debit (2025-11-20) : 23.60 ID = 10
        // credit: 100.00, debit: 39.30, balance: 60.70
        // #8 Debit (2025-12-05) : 25.50 ID = 3
        // #9 Credit (2025-12-10) : 10.00 ID = 9
        // #10 Credit (2025-12-15) : 150.00 ID = 6
        // credit: 160.00, debit: 25.50, balance: 134,50
        // credit: 510.00, debit: 25.50, balance: 124,50 <- Toatal credit/vebit and balance

        // #0 Init (2025-01-01) : 200.00 ID = 0
        cb.initialize(
            dec!(200.0),
            "2025-09-01".to_string().as_str(),
        ).unwrap();

        // #5 Credit (2025-11-05) : 100.00 ID = 1
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            "2025-11-05".to_string().as_str(),
            dec!(100.0),
            format!("Atm").as_str(),
            None,
        ).unwrap();

        // #2 Credit (2025-10-08) : 50.00 ID = 2
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            "2025-10-08".to_string().as_str(),
            dec!(50.0),
            format!("Atm").as_str(),
            None,
        ).unwrap();

        // #8 Debit (2025-12-05) : 25.50 ID = 3
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            "2025-12-05".to_string().as_str(),
            dec!(25.50),
            format!("Minimarket").as_str(),
            None,
        ).unwrap();

        // #1 Debit (2025-10-04) : 14.20 ID = 4
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            "2025-10-04".to_string().as_str(),
            dec!(14.20),
            format!("Book").as_str(),
            None,
        ).unwrap();

        // #3 Debit (2025-10-21) : 44.80 ID = 5
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            "2025-10-21".to_string().as_str(),
            dec!(44.80),
            format!("Post office").as_str(),
            None,
        ).unwrap();

        // #10 Credit (2025-12-15) : 150.00 ID = 6
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            "2025-12-15".to_string().as_str(),
            dec!(150.0),
            format!("Atm").as_str(),
            None,
        ).unwrap();

        // #6 Debit (2025-11-12) : 15.70 ID = 7
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            "2025-11-12".to_string().as_str(),
            dec!(15.70),
            format!("Bakery").as_str(),
            None,
        ).unwrap();

        // #4 Debit (2025-10-21) : 11.00 ID = 8
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            "2025-10-21".to_string().as_str(),
            dec!(11.00),
            format!("Fruits").as_str(),
            None,
        ).unwrap();

        // #9 Credit (2025-12-10) : 10.00 ID = 9
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            "2025-12-10".to_string().as_str(),
            dec!(10.0),
            format!("Refund").as_str(),
            None,
        ).unwrap();

        // #7 Debit (2025-11-20) : 23.60 ID = 10
        cb.add_operation(
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            "2025-11-20".to_string().as_str(),
            dec!(23.60),
            format!("Newspapers").as_str(),
            None,
        ).unwrap();

        cb
    }

    #[test]
    fn test_default_codexi_is_empty() -> Result<()> {
        let codexi = setup_empty_codexi();

        assert_eq!(codexi.operations.len(), 0, "The default codexi should have 0 operations.");

        let balance_items = codexi.search(None, None, None, None, None, None, None, None)?;
        let balance_result = codexi
            .balance(&balance_items)
            .unwrap_or_default();

        assert_eq!(balance_result.total, Decimal::ZERO, "The balance of an empty codexi must be 0.0.");

        Ok(())
    }

    #[test]
    fn test_codexi_nb_op() -> Result<()> {
        let codexi = setup_codexi_with_data();
        assert_eq!(codexi.operations.len(), 11, "The codexi should have 11 operations.");
        Ok(())
    }

    #[test]
    fn test_2025_10_account_balance() -> Result<()> {
        let codexi = setup_codexi_with_data();

        let balance_items = codexi.search(Some("2025-10-01".to_string()), Some("2025-10-31".to_string()), None, None, None, None, None, None)?;
        let balance_result = codexi
            .balance(&balance_items)
            .unwrap_or_default();

        // #1 Debit (2025-10-04) : 14.20 ID = 4
        // #2 Credit (2025-10-08) : 50.00 ID = 2
        // #3 Debit (2025-10-21) : 44.80 ID = 5
        // #4 Debit (2025-10-21) : 11.00 ID = 8
        // credit; 50.00, debit: 70.00 balance: -20.0

        assert_eq!(balance_result.credit, dec!(50.00), "The total credits are incorrect");
        assert_eq!(balance_result.debit, dec!(70.00), "The total debits are incorrect.");
        assert_eq!(balance_result.total, dec!(-20.00), "The final account balance is incorrect.");

        Ok(())
    }

    #[test]
    fn test_2025_11_account_balance() -> Result<()> {
        let codexi = setup_codexi_with_data();

        let balance_items = codexi.search(Some("2025-11-01".to_string()), Some("2025-11-30".to_string()), None, None, None, None, None, None)?;
        let balance_result = codexi
            .balance(&balance_items)
            .unwrap_or_default();

        // #5 Credit (2025-11-05) : 100.00 ID = 1
        // #6 Debit (2025-11-12) : 15.70 ID = 7
        // #7 Debit (2025-11-20) : 23.60 ID = 10
        // credit: 100.00, debit: 39.30, balance: 60.70

        assert_eq!(balance_result.credit, dec!(100.00), "The total credits are incorrect");
        assert_eq!(balance_result.debit, dec!(39.30), "The total debits are incorrect.");
        assert_eq!(balance_result.total, dec!(60.70), "The final account balance is incorrect.");

        Ok(())
    }

    #[test]
    fn test_2025_12_account_balance() -> Result<()> {
        let codexi = setup_codexi_with_data();

        let balance_items = codexi.search(Some("2025-12-01".to_string()), Some("2025-12-31".to_string()), None, None, None, None, None, None)?;
        let balance_result = codexi
            .balance(&balance_items)
            .unwrap_or_default();

        // #8 Debit (2025-12-05) : 25.50 ID = 3
        // #9 Credit (2025-12-10) : 10.00 ID = 9
        // #10 Credit (2025-12-15) : 150.00 ID = 6
        // credit: 160.00, debit: 25.50, balance: 134,50

        assert_eq!(balance_result.credit, dec!(160.00), "The total credits are incorrect");
        assert_eq!(balance_result.debit, dec!(25.50), "The total debits are incorrect.");
        assert_eq!(balance_result.total, dec!(134.50), "The final account balance is incorrect.");

        Ok(())
    }


    #[test]
    fn test_full_account_balance() -> Result<()> {
        let codexi = setup_codexi_with_data();

        let balance_items = codexi.search(None, None, None, None, None, None, None, None)?;
        let balance_result = codexi
            .balance(&balance_items)
            .unwrap_or_default();

        // ASSERT: Verification of expected results
        // Expected total balance: 510.00 - 134.80 = 375.20
        // Expected total credit: 200 + 50.00 + 100.00 + 160 = 510.00
        // Expected total debit: 0 + 70.00 + 39.30 + 25.50 = 134.80

        assert_eq!(balance_result.credit, dec!(510.00), "The total credits are incorrect");
        assert_eq!(balance_result.debit, dec!(134.80), "The total debits are incorrect.");
        assert_eq!(balance_result.total, dec!(375.20), "The final account balance is incorrect.");

        Ok(())
    }


    #[test]
    fn test_balance_with_range_filter() -> Result<()> {
        let codexi = setup_codexi_with_data();

        let balance_items = codexi.search(Some("2025-12-04".to_string()), Some("2025-12-06".to_string()), None, None, None, None, None, None)?;
        let balance_result = codexi
            .balance(&balance_items)
            .unwrap_or_default();

        assert_eq!(balance_result.credit, Decimal::ZERO, "The total filtered credit must be 0.0.");
        assert_eq!(balance_result.debit, dec!(25.50), "The total debits are incorrect.");
        assert_eq!(balance_result.total, dec!(-25.50), "The balance filtered by date range is incorrect.");

        Ok(())
    }

    #[test]
    fn test_balance_with_day_filter_no_operations() -> Result<()> {
        let codexi = setup_codexi_with_data();

        let balance_items = codexi.search(Some("2025-12-06".to_string()), Some("2025-12-06".to_string()), None, None, None, None, None, None)?;
        let balance_result = codexi
            .balance(&balance_items)
            .unwrap_or_default();

        assert_eq!(balance_result.credit, Decimal::ZERO, "The total filtered credit must be 0.0.");
        assert_eq!(balance_result.debit, Decimal::ZERO, "The total filtered debit must be 0.0.");
        assert_eq!(balance_result.total, Decimal::ZERO, "The balance filtered by date range is incorrect.");

        Ok(())
    }

    #[test]
    fn test_balance_with_filter_month() -> Result<()> {
        let codexi = setup_codexi_with_data();

        let balance_items = codexi.search(Some("2025-11-01".to_string()), Some("2025-11-30".to_string()), None, None, None, None, None, None)?;
        let balance_result = codexi
            .balance(&balance_items)
            .unwrap_or_default();

        assert_eq!(balance_result.credit, dec!(100.00), "The total credits are incorrect.");
        assert_eq!(balance_result.debit, dec!(39.30), "The total debits are incorrect");
        assert_eq!(balance_result.total, dec!(60.70), "The balance filtered by date range is incorrect.");

        Ok(())
    }

    #[test]
    fn test_stats_void_outside_period_has_no_effect() -> Result<()> {
        let mut codexi = setup_codexi_with_data();

        // VOID created Local::now()
        codexi.void_operation(5)?;

        let items = codexi.search(
            Some("2025-10-01".to_string()),
            Some("2025-10-31".to_string()),
            None, None, None, None, None, None
        )?;

        let stats_no_net = codexi.stats(&items, false).unwrap();
        let stats_net    = codexi.stats(&items, true).unwrap();

        assert_eq!(stats_no_net.balance, dec!(-20.00));
        assert_eq!(stats_net.balance, dec!(-20.00));

        assert_eq!(stats_no_net.total_credit, dec!(50.00));
        assert_eq!(stats_no_net.total_debit, dec!(70.00));

        assert_eq!(stats_net.total_credit, dec!(50.00));
        assert_eq!(stats_net.total_debit, dec!(70.00));

        Ok(())
    }


    #[test]
    fn test_stats_void_in_period_produces_expected_net_result()-> Result<()> {
        let mut codexi = setup_codexi_with_data();

        let _ = codexi.void_operation(5);

        let items = codexi.search(
            Some("2025-10-01".into()),
            Some("2026-01-31".into()),
            None, None, None, None, None, None
        )?;

        let stats = codexi.stats(&items, true).unwrap();

        assert_eq!(stats.balance, dec!(130.40));
        Ok(())
    }


}
