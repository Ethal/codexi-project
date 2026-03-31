// src/logic/account/account.rs

use chrono::NaiveDate;
use nulid::Nulid;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::core::{CoreWarning, validate_text_rules};
use crate::logic::{
    account::{
        AccountAnchors, AccountError, AccountType, CheckpointRef, OperationContainer,
        TemporalAction, policy::AccountContext,
    },
    operation::{AccountOperations, Operation},
    utils::{HasName, HasNulid},
};

// meta data relaed to the account
#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct AccountMeta {
    pub iban: Option<String>,
    pub color: Option<String>,
    pub display_order: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub note: Option<String>,
}

/// Struct representing the an account
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: Nulid,    // Id
    pub name: String, // Account name
    pub context: AccountContext,
    pub bank_id: Option<Nulid>,             // Nulid of the Bank
    pub currency_id: Option<Nulid>,         // Main currency id for the account
    pub carry_forward_balance: Decimal,     // for internal calculation
    pub open_date: NaiveDate, // Open date of the account,typivcaly the date of the init.
    pub terminated_date: Option<NaiveDate>, // Close date of the account.
    pub operations: Vec<Operation>, // Operation list
    pub(crate) current_balance: Decimal,
    pub(crate) checkpoints: Vec<CheckpointRef>,
    pub anchors: AccountAnchors,
    #[serde(default)]
    pub meta: AccountMeta, // Account meta data
}
/// Methods for account
impl Account {
    pub fn new(
        open_date: NaiveDate,
        name_user: String,
        account_type: AccountType,
        bank_id: Option<Nulid>,
        currency_id: Option<Nulid>,
    ) -> Result<Self, AccountError> {
        let id = Nulid::new()?;

        let min = 0;
        let max = 50;
        let name: String = if let Err(_e) = validate_text_rules(&name_user, min, max) {
            "New Account".to_string()
        } else {
            name_user
        };

        let meta = AccountMeta::default();
        let operations: Vec<Operation> = Vec::new();
        let checkpoints: Vec<CheckpointRef> = Vec::new();

        let context = AccountContext::from_type(account_type);

        Ok(Self {
            id,
            name,
            context,
            bank_id,
            currency_id,
            operations,
            carry_forward_balance: Decimal::ZERO, // will be update with init, or close.
            open_date,                            // wiil be update with the init date.
            terminated_date: None,                // wil be update by a close account.
            current_balance: Decimal::ZERO,
            checkpoints,
            anchors: AccountAnchors::default(),
            meta,
        })
    }

    pub fn update(
        &mut self,
        name: &str,
        currency_id: Option<Nulid>,
        bank_id: Option<Nulid>,
        context: AccountContext,
        meta: AccountMeta,
    ) {
        self.name = name.into();
        self.currency_id = currency_id;
        self.bank_id = bank_id;

        self.context = context;
        self.meta = meta;
    }

    pub fn update_context(&mut self, ctx: AccountContext) {
        self.context = ctx;
    }
    pub fn update_meta(&mut self, meta: AccountMeta) {
        self.meta = meta;
    }

    pub fn set_context(
        &mut self,
        overdraft_limit: Option<Decimal>,
        min_balance: Option<Decimal>,
        max_monthly_transactions: Option<Option<u32>>, // Some(None) = no limit
        deposit_locked_until: Option<NaiveDate>,
        allows_interest: Option<bool>,
        allows_joint_signers: Option<bool>,
    ) -> Result<Vec<CoreWarning>, AccountError> {
        self.is_terminated()?;
        let warnings = self.context.update_context(
            overdraft_limit,
            min_balance,
            max_monthly_transactions,
            deposit_locked_until,
            allows_interest,
            allows_joint_signers,
        )?;

        Ok(warnings)
    }

    ///Return the Operation or None
    pub fn get_operation_by_id(&self, op_id: Nulid) -> Option<&Operation> {
        self.operations.iter().find(|op| op.id == op_id)
    }
    /// Return the mutable Operation or None
    pub fn get_operation_by_id_mut(&mut self, id: Nulid) -> Option<&mut Operation> {
        self.operations.iter_mut().find(|op| op.id == id)
    }

    /// Returns an AccountOperations snapshot for export.
    pub fn to_account_operations(&self) -> AccountOperations {
        AccountOperations {
            account_id: self.id,
            operations: self.operations.clone(),
        }
    }

    /// Recalculates all anchors based on the current operation vector.
    pub fn refresh_anchors(&mut self) {
        self.anchors.rebuild_from(&self.operations);
    }

    /// Update Operations with an operation, rebuild the balance and return the id of the operation
    pub fn commit_operation(&mut self, op: Operation) -> Nulid {
        self.anchors.update(&op);
        let id = op.id;
        let op_date = op.date;
        self.operations.push(op);
        self.operations
            .sort_by(|a, b| a.date.cmp(&b.date).then(a.id.cmp(&b.id)));
        self.rebuild_balances_from(op_date);
        id
    }

    /// Rebuild the balance of all operations from the given date onwards.
    /// Called after every commit_operation() to keep op.balance and
    /// current_balance consistent.
    pub fn rebuild_balances_from(&mut self, from_date: NaiveDate) {
        // Compute running balance just before from_date
        let running_before: Decimal = self
            .operations
            .iter()
            .filter(|op| op.date < from_date)
            .fold(Decimal::ZERO, |acc, op| op.flow.apply(acc, op.amount));

        // Update op.balance for all operations from from_date onwards
        let mut running = running_before;
        for op in self.operations.iter_mut().filter(|op| op.date >= from_date) {
            running = op.flow.apply(running, op.amount);
            op.balance = running;
        }

        self.current_balance = self
            .operations
            .last()
            .map(|op| op.balance)
            .unwrap_or(Decimal::ZERO);
    }

    /// Returns the running balance at the end of the given date.
    /// Used by compliance_policy to validate against the correct historical
    /// balance rather than current_balance.
    pub fn balance_at(&self, date: NaiveDate) -> Decimal {
        self.operations
            .iter()
            .filter(|op| op.date <= date)
            .fold(Decimal::ZERO, |acc, op| op.flow.apply(acc, op.amount))
    }
    /// Determine if a specific operation can be undone (Void)
    pub fn can_void(&self, op_id: Nulid) -> Result<bool, AccountError> {
        // current date
        let today = chrono::Local::now().date_naive();

        // check if void is allow as per the current date
        match self.temporal_policy(TemporalAction::Void(op_id), today) {
            Ok(_) => Ok(true),   // Ok
            Err(_) => Ok(false), // not Ok
        }
    }
}

impl HasNulid for Account {
    fn id(&self) -> Nulid {
        self.id
    }
}
impl HasName for Account {
    fn name(&self) -> &str {
        &self.name
    }
}

impl OperationContainer for Account {
    fn operations(&self) -> &[Operation] {
        &self.operations
    }
}

/*------------------------ TEST ------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parse_date;
    use crate::logic::operation::{
        OperationBuilder, OperationFlow, OperationKind, RegularKind, SystemKind,
    };
    use rust_decimal_macros::dec;

    // Helper to create an empty account
    fn setup_empty_account() -> Account {
        // init
        Account::new(
            parse_date("2025-09-01").unwrap(),
            "Test".into(),
            AccountType::Current,
            None,
            None,
        )
        .unwrap()
    }

    #[test]
    fn test_refresh_anchors_unordered_data() {
        let mut account = setup_empty_account();

        // add operation not in the order and not by using the commit_operation
        // Build the operation
        let op = OperationBuilder::default()
            .date(parse_date("2026-01-10").unwrap())
            .kind(OperationKind::System(SystemKind::Checkpoint))
            .flow(OperationFlow::Credit)
            .amount(dec!(10))
            .description("description".to_string())
            .build()
            .unwrap();
        account.operations.push(op);

        let op = OperationBuilder::default()
            .date(parse_date("2026-01-20").unwrap())
            .kind(OperationKind::System(SystemKind::Checkpoint))
            .flow(OperationFlow::Credit)
            .amount(dec!(10))
            .description("description".to_string())
            .build()
            .unwrap();
        account.operations.push(op); // the latest one

        let op = OperationBuilder::default()
            .date(parse_date("2026-01-15").unwrap())
            .kind(OperationKind::System(SystemKind::Checkpoint))
            .flow(OperationFlow::Credit)
            .amount(dec!(10))
            .description("description".to_string())
            .build()
            .unwrap();
        account.operations.push(op);

        // before refresh anchor is none
        assert_eq!(account.anchors.last_checkpoint, None);

        account.refresh_anchors();

        // after the refresh, shall be the latest date
        assert_eq!(
            account.anchors.last_checkpoint.map(|la| la.date),
            Some(parse_date("2026-01-20").unwrap())
        );
    }

    #[test]
    fn test_audit_invalid_history() {
        let mut account = setup_empty_account();

        // 1. Initialisation normal at 01/03
        let init_date = parse_date("2026-03-01").unwrap();
        let op = OperationBuilder::default()
            .date(init_date)
            .kind(OperationKind::System(SystemKind::Init))
            .flow(OperationFlow::Credit)
            .amount(dec!(10))
            .description("description".to_string())
            .build()
            .unwrap();
        account.commit_operation(op);

        // 2. manual  corruption : insert of  débit at 01/02 (BEFORE the init)
        // we dont use the temporal policy , direct push
        let op = OperationBuilder::default()
            .date(parse_date("2026-02-01").unwrap())
            .kind(OperationKind::Regular(RegularKind::Transaction))
            .flow(OperationFlow::Credit)
            .amount(dec!(10))
            .description("fraudulent".to_string())
            .build()
            .unwrap();

        account.operations.push(op);

        // sort to simulate a file properly corrupted
        account.operations.sort_by_key(|o| o.date);
        account.refresh_anchors();

        // 3. audit shall fail temporal_policy can not accept  a débit before the Init
        let result = account.audit();
        assert!(
            result.is_err(),
            "The audit should detect an operation before init"
        );
    }

    #[test]
    fn test_audit_locked_period_violation() {
        let mut account = setup_empty_account();
        let d1 = parse_date("2026-01-01").unwrap();
        let d2 = parse_date("2026-01-15").unwrap();
        let d3 = parse_date("2026-01-30").unwrap();

        // Init -> Checkpoint at 30/01
        account.commit_operation(
            OperationBuilder::default()
                .date(d1)
                .kind(OperationKind::System(SystemKind::Init))
                .flow(OperationFlow::Credit)
                .amount(dec!(100))
                .description("ok".to_string())
                .build()
                .unwrap(),
        );
        account.commit_operation(
            OperationBuilder::default()
                .date(d3)
                .kind(OperationKind::System(SystemKind::Checkpoint))
                .flow(OperationFlow::Credit)
                .amount(dec!(100))
                .description("ok".to_string())
                .build()
                .unwrap(),
        );

        // Corrupted : insert débit at 15/01 (period close par d3)
        account.operations.push(
            OperationBuilder::default()
                .date(d2)
                .kind(OperationKind::Regular(RegularKind::Transaction))
                .flow(OperationFlow::Debit)
                .amount(dec!(10))
                .description("fraudulent".to_string())
                .build()
                .unwrap(),
        );

        account.operations.sort_by_key(|o| o.date);
        account.refresh_anchors();

        // The audit shall détect than d2 is <= the checkpoint d3 during the replay
        assert!(
            account.audit().is_err(),
            "The audit shall reject an operation in a close period"
        );
    }
}
