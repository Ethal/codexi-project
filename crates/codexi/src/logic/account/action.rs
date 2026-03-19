// src/logic/account/actions.rs

use chrono::{Local, NaiveDate};
use nulid::Nulid;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::mem;
use std::path::PathBuf;

use crate::core::{DataPaths, format_date, format_id, format_id_short};
use crate::logic::{
    account::{
        Account, AccountArchive, AccountError, CheckpointRef, ComplianceAction,
        SearchParamsBuilder, TemporalAction, search,
    },
    balance::Balance,
    operation::{
        OperationBuilder, OperationContext, OperationFlow, OperationKind, OperationLinks,
        RegularKind, SystemKind,
    },
};

use crate::file_management::FileManagement;

/// Methods for account
impl Account {
    /// This function adds a new transaction(Regular operation) to the account while ensuring data integrity.
    /// It checks for date conflicts with existing system operations (Init, Close, Adjust)
    pub fn register_transaction(
        &mut self,
        date: NaiveDate,
        kind: OperationKind,
        flow: OperationFlow,
        amount: Decimal,
        description: String,
    ) -> Result<Nulid, AccountError> {
        self.temporal_policy(TemporalAction::Create(&kind), date)?;
        self.compliance_policy(ComplianceAction::Create(&kind, flow, amount))?;

        let mut context = OperationContext::default();
        context.currency_id = self.currency_id;

        // On prépare le builder
        let op = OperationBuilder::default()
            .date(date)
            .kind(kind)
            .flow(flow)
            .amount(amount)
            .description(description)
            .context(context)
            .build()?;

        // record the operation
        let op_id = self.commit_operation(op);
        // rebuild the balance
        self.rebuild_balances_from(date);

        Ok(op_id)
    }
    /// This function void an operation at the specified index.
    ///
    /// It checks if the operation is a system operation (Init, Close, Void, Adjust) and prevents void if so.
    /// It returns an error if the index is out of bounds or if void is not allowed.
    pub fn void_operation(&mut self, void_id: Nulid) -> Result<Nulid, AccountError> {
        let today = Local::now().date_naive();
        // check the temporal policy
        let kind = OperationKind::System(SystemKind::Void);
        self.temporal_policy(TemporalAction::Void(void_id), today)?;

        // locate and get the target operation to void
        let target_op = match self.get_operation_by_id(void_id) {
            Some(op) => op,
            None => return Err(AccountError::OperationNotFound(void_id.to_string())),
        };

        let op_flow = target_op.flow.opposite();
        let op_amount = target_op.amount;
        let description = format!(
            "VOID #{}: {} ({} {:.2})",
            format_id_short(&format_id(void_id)),
            target_op.description,
            target_op.flow,
            target_op.amount
        );

        self.compliance_policy(ComplianceAction::Create(&kind, op_flow, op_amount))?;

        let mut links = OperationLinks::default();
        links.void_of = Some(void_id);

        let mut context = OperationContext::default();
        context.currency_id = self.currency_id;

        // Create the void operation
        let op = OperationBuilder::default()
            .date(today)
            .kind(kind)
            .flow(op_flow)
            .amount(op_amount)
            .description(description)
            .links(links)
            .context(context)
            .build()?;

        // record the void operation
        let op_id = self.commit_operation(op);
        // rebuild the balance
        self.rebuild_balances_from(today);

        // 4. update'void_by' on the void operation
        if let Some(target_op) = self.get_operation_by_id_mut(void_id) {
            target_op.links.void_by = Some(op_id);
        }

        Ok(op_id)
    }

    /// Sets the initial balance of an account.
    ///
    /// This function creates an initial operation representing the starting balance.
    /// It should only be called when the account is empty.
    pub fn initialize(&mut self, date: NaiveDate, amount: Decimal) -> Result<Nulid, AccountError> {
        // Check temporal policy
        let kind = OperationKind::System(SystemKind::Init);
        self.temporal_policy(TemporalAction::Create(&kind), date)?;

        // logic
        let op_flow = OperationFlow::from_sign(amount);
        let op_amount = amount.abs();
        let description = format!("INITIAL AMOUNT {}", op_amount);

        self.compliance_policy(ComplianceAction::Create(&kind, op_flow, op_amount))?;

        let mut context = OperationContext::default();
        context.currency_id = self.currency_id;

        // Create the init operation
        let op = OperationBuilder::default()
            .date(date)
            .kind(kind)
            .flow(op_flow)
            .amount(op_amount)
            .description(description)
            .context(context)
            .build()?;

        // record the init operation
        let op_id = self.commit_operation(op);
        // rebuild the balance
        self.rebuild_balances_from(date);

        Ok(op_id)
    }

    /// This function adjusts the codexi to match a physical balance.
    /// It calculates the difference and creates an adjustment operation if needed.
    pub fn adjust_balance(
        &mut self,
        date: NaiveDate,
        physical_amount: Decimal,
    ) -> Result<Nulid, AccountError> {
        // Check temporal policy
        let kind = OperationKind::System(SystemKind::Adjust);
        self.temporal_policy(TemporalAction::Create(&kind), date)?;

        // logic
        let params = SearchParamsBuilder::default().to(Some(date)).build()?;
        let balance_items = search(self, &params)?;

        let current_balance = Balance::new(&balance_items);
        let difference = physical_amount - current_balance.total();
        // check the difference if equal to zero or below < 0.001 -> Error
        if difference.abs() < dec!(0.001) || difference.abs() == Decimal::ZERO {
            return Err(AccountError::NoAdjust);
        }
        let op_flow = OperationFlow::from_sign(difference);
        let op_amount = difference.abs();
        let description = format!(
            "ADJUSTMENT: Deviation of {:.2} to reach physical balance {:.2}",
            op_amount, physical_amount,
        );

        self.compliance_policy(ComplianceAction::Create(&kind, op_flow, op_amount))?;

        let mut context = OperationContext::default();
        context.currency_id = self.currency_id;

        // Create the close(checkpoint) operation
        let op = OperationBuilder::default()
            .date(date)
            .kind(kind)
            .flow(op_flow)
            .amount(op_amount)
            .description(description)
            .context(context)
            .build()?;

        // record the adjust operation
        let op_id = self.commit_operation(op);
        // rebuild the balance
        self.rebuild_balances_from(date);

        Ok(op_id)
    }

    /// This function closes the current accounting period by archiving all operations
    /// up to the specified closing date and creating a new "Carried Forward Solde" operation.
    ///
    /// It saves the archived operations to a file and updates the account accordingly.
    /// The description_parts are concatenated to describe the closing operation.
    pub fn checkpoint(
        &mut self,
        checkpoint_date: NaiveDate,
        desc: String,
        paths: &DataPaths,
    ) -> Result<Nulid, AccountError> {
        // Check finaancial policy
        let kind = OperationKind::System(SystemKind::Checkpoint);
        self.temporal_policy(TemporalAction::Create(&kind), checkpoint_date)?;

        let mut checkpoint_balance = Decimal::ZERO;
        let mut archived_operations = Vec::new();

        let original_operations = mem::take(&mut self.operations);

        for op in original_operations.into_iter() {
            let op_date = op.date;

            if op_date <= checkpoint_date {
                match op.kind {
                    OperationKind::System(SystemKind::Init)
                    | OperationKind::System(SystemKind::Checkpoint) => {
                        archived_operations.push(op.clone());
                        match op.flow {
                            OperationFlow::Credit => checkpoint_balance = op.amount,
                            OperationFlow::Debit => checkpoint_balance = -op.amount,
                            OperationFlow::None => {}
                        }
                    }
                    OperationKind::System(SystemKind::Adjust)
                    | OperationKind::System(SystemKind::Void)
                    | OperationKind::Regular(RegularKind::Transaction)
                    | OperationKind::Regular(RegularKind::Fee)
                    | OperationKind::Regular(RegularKind::Transfer)
                    | OperationKind::Regular(RegularKind::Refund) => {
                        match op.flow {
                            OperationFlow::Credit => checkpoint_balance += op.amount,
                            OperationFlow::Debit => checkpoint_balance -= op.amount,
                            OperationFlow::None => {}
                        }
                        archived_operations.push(op);
                    }
                }
            } else {
                self.operations.push(op);
            }
        }

        // If there's nothing to close, we stop.
        if archived_operations.is_empty()
            && self.operations.iter().all(|op| {
                !matches!(
                    op.kind,
                    OperationKind::System(SystemKind::Init)
                        | OperationKind::System(SystemKind::Checkpoint)
                )
            })
        {
            // Management logic if the codexi is empty or contains only previous anchors.
            // If there are no transactions to archive, nothing is done.
            return Err(AccountError::NothingClose);
        }

        // --- PART 1: ARCHIVE MANAGEMENT ---

        // Save the archive if there are transactions to archive.
        if !archived_operations.is_empty() {
            let mut archive_export = self.clone();
            archive_export.operations = archived_operations;
            let codexi_archive =
                AccountArchive::new(&archive_export, checkpoint_date, checkpoint_balance);
            FileManagement::save_archive(&codexi_archive, paths)?;
        }

        // --- PART 2: CREATION OF THE NEW ANCHOR ---

        // Create the new Carry Forward Balance operation
        let op_flow = OperationFlow::from_sign(checkpoint_balance);
        let op_amount = checkpoint_balance.abs();
        let description = format!("BALANCE DEFERRED: {}: {:.2} {}", op_flow, op_amount, desc);

        self.compliance_policy(ComplianceAction::Create(&kind, op_flow, op_amount))?;

        let mut context = OperationContext::default();
        context.currency_id = self.currency_id;

        // Build the operation
        let op = OperationBuilder::default()
            .date(checkpoint_date)
            .kind(kind)
            .flow(op_flow)
            .amount(op_amount)
            .description(description)
            .context(context)
            .balance(checkpoint_balance)
            .build()?;

        // record the operation
        let op_id = self.commit_operation(op);

        // update the account data
        self.current_balance = checkpoint_balance;
        self.carry_forward_balance = checkpoint_balance;

        let archive_file: PathBuf =
            format!("{}_codexi_{}.cld", self.id, format_date(checkpoint_date)).into();
        let ck = CheckpointRef {
            checkpoint_date,
            checkpoint_balance,
            archive_file,
        };
        self.checkpoints.push(ck);

        Ok(op_id)
    }
}
