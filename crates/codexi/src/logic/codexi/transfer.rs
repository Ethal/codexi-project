// src/logic/codexi/transfer.rs

use chrono::NaiveDate;
use nulid::Nulid;
use rust_decimal::Decimal;

use crate::core::{format_id, format_id_short};
use crate::logic::{
    account::{AccountError, ComplianceAction, TemporalAction},
    codexi::{Codexi, CodexiError},
    operation::{
        OperationBuilder, OperationContext, OperationFlow, OperationKind, OperationLinks,
        RegularKind,
    },
};

impl Codexi {
    /// Creates a transfer between two accounts.
    /// Two linked Regular::Transfer operations are created — one Debit on source,
    /// one Credit on destination. Exchange rate is calculated automatically.
    ///
    /// Returns (source_op_id, destination_op_id)
    pub fn transfer(
        &mut self,
        date: NaiveDate,
        amount_from: Decimal,
        to_id: Nulid,
        amount_to: Decimal,
        description: String,
    ) -> Result<(Nulid, Nulid), CodexiError> {
        let from_id = self.current_account;

        // Source and destination must be different
        if from_id == to_id {
            return Err(CodexiError::TransferSameAccount);
        }

        // Both accounts must exist and have a currency set
        let currency_from = self
            .get_account_by_id(&from_id)?
            .currency_id
            .ok_or_else(|| CodexiError::TransferNoCurrency(format_id(from_id)))?;

        let currency_to = self
            .get_account_by_id(&to_id)?
            .currency_id
            .ok_or_else(|| CodexiError::TransferNoCurrency(format_id(to_id)))?;

        let kind = OperationKind::Regular(RegularKind::Transfer);

        // --- Validate policies on source account ---
        {
            let acc_from = self.get_account_by_id(&from_id)?;
            acc_from
                .temporal_policy(TemporalAction::Create(&kind), date)
                .map_err(AccountError::TemporalViolation)?;
            acc_from
                .compliance_policy(ComplianceAction::Create(
                    &kind,
                    OperationFlow::Debit,
                    amount_from,
                ))
                .map_err(AccountError::ComplianceViolation)?;
        }

        // --- Validate policies on destination account ---
        {
            let acc_to = self.get_account_by_id(&to_id)?;
            acc_to
                .temporal_policy(TemporalAction::Create(&kind), date)
                .map_err(AccountError::TemporalViolation)?;
            acc_to
                .compliance_policy(ComplianceAction::Create(
                    &kind,
                    OperationFlow::Credit,
                    amount_to,
                ))
                .map_err(AccountError::ComplianceViolation)?;
        }

        // --- Build source operation (Debit) ---
        // We need op_to_id first to cross-link — so we pre-generate the destination id
        let op_to_id = Nulid::new().map_err(AccountError::Id)?;

        let desc_from = format!("TRANSFER TO {}: {}", format_id(to_id), description);
        let mut links_from = OperationLinks::default();
        links_from.transfer_id = Some(op_to_id);
        links_from.transfer_account_id = Some(to_id);

        let mut ctx_from = OperationContext::default();
        ctx_from.currency_id = Some(currency_from);

        let mut op_from = OperationBuilder::default()
            .date(date)
            .kind(kind)
            .flow(OperationFlow::Debit)
            .amount(amount_from)
            .description(desc_from)
            .links(links_from)
            .context(ctx_from.clone())
            .build()
            .map_err(AccountError::Operation)?;

        let op_from_id = op_from.id;
        // Calculate effective exchange rate : amount_to / amount_from
        let exchange_rate = amount_to / amount_from;
        op_from.context.exchange_rate = exchange_rate;

        // --- Build destination operation (Credit) ---
        let desc_to = format!(
            "TRANSFER FROM {}: {}",
            format_id_short(&format_id(from_id)),
            description
        );
        let mut links_to = OperationLinks::default();
        links_to.transfer_id = Some(op_from_id);
        links_to.transfer_account_id = Some(from_id);

        let mut ctx_to = OperationContext::default();
        ctx_to.currency_id = Some(currency_to);

        // Pre-set the id we reserved earlier
        let mut op_to = OperationBuilder::default()
            .date(date)
            .kind(kind)
            .flow(OperationFlow::Credit)
            .amount(amount_to)
            .description(desc_to)
            .links(links_to)
            .context(ctx_to)
            .build()
            .map_err(AccountError::Operation)?;
        op_to.id = op_to_id; // use the pre-generated id for cross-linking

        // --- Commit both operations ---
        let acc_from = self.get_account_by_id_mut(&from_id)?;
        acc_from.commit_operation(op_from);

        let acc_to = self.get_account_by_id_mut(&to_id)?;
        acc_to.commit_operation(op_to);

        Ok((op_from_id, op_to_id))
    }

    /// Checks that the twin transfer operation exists and is not archived.
    /// Returns (twin_op_id, twin_account_id) if the void is allowed.
    fn check_transfer_void(&self, op_id: Nulid) -> Result<(Nulid, Nulid), CodexiError> {
        let acc = self.get_current_account()?;

        let op = acc
            .get_operation_by_id(op_id)
            .ok_or_else(|| AccountError::OperationNotFound(format_id(op_id)))?;

        // Must be a transfer operation
        let twin_op_id = op
            .links
            .transfer_id
            .ok_or_else(|| CodexiError::NotATransfer(format_id(op_id)))?;

        let twin_account_id = op
            .links
            .transfer_account_id
            .ok_or_else(|| CodexiError::NotATransfer(format_id(op_id)))?;

        // Twin must exist in the other account — if not, it is archived
        let twin_acc = self.get_account_by_id(&twin_account_id)?;
        if twin_acc.get_operation_by_id(twin_op_id).is_none() {
            return Err(CodexiError::TransferTwinArchived);
        }

        Ok((twin_op_id, twin_account_id))
    }

    /// Voids an operation from the current account.
    /// If the operation is a transfer, both linked operations are voided atomically.
    /// If the twin operation is archived, the void is rejected.
    /// For regular operations, delegates directly to Account::void_operation.
    pub fn void_from_current(&mut self, op_id: Nulid) -> Result<(), CodexiError> {
        let current_id = self.current_account;

        // Detect if the operation is a transfer
        let transfer_info = {
            let acc = self.get_current_account()?;
            let op = acc
                .get_operation_by_id(op_id)
                .ok_or_else(|| AccountError::OperationNotFound(format_id(op_id)))?;

            // Only collect twin info if transfer_id AND transfer_account_id are set
            match (op.links.transfer_id, op.links.transfer_account_id) {
                (Some(tid), Some(aid)) => Some((tid, aid)),
                _ => None,
            }
        };

        match transfer_info {
            Some((twin_op_id, twin_account_id)) => {
                // Verify twin is not archived
                self.check_transfer_void(op_id)?;

                // Void source operation on current account
                self.get_account_by_id_mut(&current_id)?
                    .void_operation(op_id)?;

                // Void twin operation on destination account
                self.get_account_by_id_mut(&twin_account_id)?
                    .void_operation(twin_op_id)?;
            }
            None => {
                // Regular void — delegate directly to Account::void_operation
                self.get_current_account_mut()?.void_operation(op_id)?;
            }
        }

        Ok(())
    }
}

/*------------------------ TEST ------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::account::{Account, AccountType, ComplianceViolation};
    use crate::logic::codexi::{Codexi, CodexiSettings};
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;

    // ── Helpers ──────────────────────────────────────────────

    fn new_codexi() -> Codexi {
        let setting = CodexiSettings::default();
        Codexi::new(setting).unwrap()
    }

    fn setup_codexi_two_accounts() -> (Codexi, Nulid, Nulid) {
        let mut codexi = new_codexi();

        // Account EUR
        let mut acc_eur = Account::new(
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            "acc_eur".into(),
            AccountType::Current,
            None,
            None,
        )
        .unwrap();
        // Set EUR currency — find it from codexi currencies
        let eur_id = codexi
            .currencies
            .currencies
            .iter()
            .find(|c| c.code == "EUR")
            .map(|c| c.id)
            .unwrap();

        acc_eur.currency_id = Some(eur_id);
        acc_eur
            .initialize(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(), dec!(1_000))
            .unwrap();
        let acc_eur_id = codexi.add_account(acc_eur);

        // Account IDR
        let mut acc_idr = Account::new(
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            "acc_idr".into(),
            AccountType::Current,
            None,
            None,
        )
        .unwrap();
        let idr_id = codexi
            .currencies
            .currencies
            .iter()
            .find(|c| c.code == "IDR")
            .map(|c| c.id)
            .unwrap();
        acc_idr.currency_id = Some(idr_id);
        acc_idr
            .initialize(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(), dec!(0))
            .unwrap();
        let acc_idr_id = codexi.add_account(acc_idr);

        // Set current account to EUR
        codexi.set_current_account(&acc_eur_id).unwrap();

        (codexi, acc_eur_id, acc_idr_id)
    }

    fn transfer_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
    }

    // ── transfer ─────────────────────────────────────────────

    #[test]
    fn transfer_ok() {
        let (mut codexi, _from_id, to_id) = setup_codexi_two_accounts();
        let res = codexi.transfer(
            transfer_date(),
            dec!(53.05),
            to_id,
            dec!(1_000_000),
            "ATM withdrawal".into(),
        );
        assert!(res.is_ok());
        let (op_from_id, op_to_id) = res.unwrap();

        // Source account — balance reduced
        let acc_eur = codexi.get_current_account().unwrap();
        assert_eq!(acc_eur.current_balance, dec!(946.95)); // 1000 - 53.05

        // Check cross-links on source op
        let op_from = acc_eur.get_operation_by_id(op_from_id).unwrap();
        assert_eq!(op_from.links.transfer_id, Some(op_to_id));
        assert_eq!(op_from.links.transfer_account_id, Some(to_id));

        // Destination account — balance increased
        let acc_idr = codexi.get_account_by_id(&to_id).unwrap();
        assert_eq!(acc_idr.current_balance, dec!(1_000_000));

        // Check cross-links on destination op
        let op_to = acc_idr.get_operation_by_id(op_to_id).unwrap();
        assert_eq!(op_to.links.transfer_id, Some(op_from_id));
    }

    #[test]
    fn transfer_exchange_rate_calculated() {
        let (mut codexi, _from_id, to_id) = setup_codexi_two_accounts();
        let (op_from_id, _) = codexi
            .transfer(
                transfer_date(),
                dec!(53.05),
                to_id,
                dec!(1_000_000),
                "ATM".into(),
            )
            .unwrap();

        // Rate = 1_000_000 / 53.05
        let acc_eur = codexi.get_current_account().unwrap();
        let op = acc_eur.get_operation_by_id(op_from_id).unwrap();
        let expected_rate = dec!(1_000_000) / dec!(53.05);
        assert_eq!(op.context.exchange_rate, expected_rate);
    }

    #[test]
    fn transfer_same_account_fails() {
        let (mut codexi, from_id, _) = setup_codexi_two_accounts();
        let res = codexi.transfer(
            transfer_date(),
            dec!(100),
            from_id, // same as current_account
            dec!(100),
            "self".into(),
        );
        assert!(matches!(res, Err(CodexiError::TransferSameAccount)));
    }

    #[test]
    fn transfer_amount_from_zero_fails() {
        let (mut codexi, _, to_id) = setup_codexi_two_accounts();
        let res = codexi.transfer(
            transfer_date(),
            dec!(0),
            to_id,
            dec!(100_000),
            "test".into(),
        );
        assert!(matches!(
            res,
            Err(CodexiError::Account(AccountError::ComplianceViolation(
                ComplianceViolation::InvalidAmount { amount: _ }
            )))
        ));
    }

    #[test]
    fn transfer_amount_to_zero_fails() {
        let (mut codexi, _, to_id) = setup_codexi_two_accounts();
        let res = codexi.transfer(transfer_date(), dec!(100), to_id, dec!(0), "test".into());
        assert!(matches!(
            res,
            Err(CodexiError::Account(AccountError::ComplianceViolation(
                ComplianceViolation::InvalidAmount { amount: _ }
            )))
        ));
    }

    #[test]
    fn transfer_no_currency_source_fails() {
        let (mut codexi, from_id, to_id) = setup_codexi_two_accounts();
        // Remove currency from source account
        codexi.get_account_by_id_mut(&from_id).unwrap().currency_id = None;
        let res = codexi.transfer(
            transfer_date(),
            dec!(100),
            to_id,
            dec!(100_000),
            "test".into(),
        );
        assert!(matches!(res, Err(CodexiError::TransferNoCurrency(_))));
    }

    #[test]
    fn transfer_no_currency_destination_fails() {
        let (mut codexi, _, to_id) = setup_codexi_two_accounts();
        // Remove currency from destination account
        codexi.get_account_by_id_mut(&to_id).unwrap().currency_id = None;
        let res = codexi.transfer(
            transfer_date(),
            dec!(100),
            to_id,
            dec!(100_000),
            "test".into(),
        );
        assert!(matches!(res, Err(CodexiError::TransferNoCurrency(_))));
    }

    #[test]
    fn transfer_exceeds_overdraft_fails() {
        let (mut codexi, _, to_id) = setup_codexi_two_accounts();
        // Current account has 1000 EUR, overdraft = 500 → max debit = 1500
        let res = codexi.transfer(
            transfer_date(),
            dec!(1_600), // 1000 + 600 > overdraft limit of 500
            to_id,
            dec!(30_000_000),
            "too much".into(),
        );
        assert!(matches!(res, Err(CodexiError::Account(_))));
    }

    // ── void_from_current — transfer ─────────────────────────

    #[test]
    fn void_transfer_ok() {
        let (mut codexi, _from_id, to_id) = setup_codexi_two_accounts();
        let (op_from_id, op_to_id) = codexi
            .transfer(
                transfer_date(),
                dec!(53.05),
                to_id,
                dec!(1_000_000),
                "ATM".into(),
            )
            .unwrap();

        // Void from current account (EUR)
        let res = codexi.void_from_current(op_from_id);
        assert!(res.is_ok());

        // Source balance restored
        let acc_eur = codexi.get_current_account().unwrap();
        assert_eq!(acc_eur.current_balance, dec!(1_000));

        // Source op has void_by set
        let op_from = acc_eur.get_operation_by_id(op_from_id).unwrap();
        assert!(op_from.links.void_by.is_some());

        // Destination balance restored
        let acc_idr = codexi.get_account_by_id(&to_id).unwrap();
        assert_eq!(acc_idr.current_balance, dec!(0));

        // Twin op has void_by set
        let op_to = acc_idr.get_operation_by_id(op_to_id).unwrap();
        assert!(op_to.links.void_by.is_some());
    }

    #[test]
    fn void_transfer_already_voided_fails() {
        let (mut codexi, _, to_id) = setup_codexi_two_accounts();
        let (op_from_id, _) = codexi
            .transfer(
                transfer_date(),
                dec!(53.05),
                to_id,
                dec!(1_000_000),
                "ATM".into(),
            )
            .unwrap();

        // First void — ok
        codexi.void_from_current(op_from_id).unwrap();

        // Second void — must fail
        let res = codexi.void_from_current(op_from_id);
        assert!(res.is_err());
    }

    // ── void_from_current — regular op ───────────────────────

    #[test]
    fn void_regular_op_ok() {
        let (mut codexi, _, _) = setup_codexi_two_accounts();
        let op_id = codexi
            .get_current_account_mut()
            .unwrap()
            .register_transaction(
                transfer_date(),
                OperationKind::Regular(RegularKind::Transaction),
                OperationFlow::Debit,
                dec!(50),
                "groceries".into(),
            )
            .unwrap();

        // Balance avant void : 1000 - 50 = 950
        assert_eq!(
            codexi.get_current_account().unwrap().current_balance,
            dec!(950)
        );

        let res = codexi.void_from_current(op_id);
        assert!(res.is_ok());

        // Balance après void : 950 + 50 = 1000 (restaurée)
        let acc = codexi.get_current_account().unwrap();
        assert_eq!(acc.current_balance, dec!(1_000));
    }

    #[test]
    fn void_op_not_found_fails() {
        let (mut codexi, _, _) = setup_codexi_two_accounts();
        let fake_id = Nulid::new().unwrap();
        let res = codexi.void_from_current(fake_id);
        assert!(res.is_err());
    }
}
