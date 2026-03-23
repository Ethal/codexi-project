// tests/account_test.rs

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use codexi::core::parse_date;
use codexi::logic::account::{Account, AccountType, SearchParamsBuilder, search};
use codexi::logic::balance::Balance;
use codexi::logic::operation::RegularKind;
use codexi::logic::operation::{OperationFlow, OperationKind};

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

// Helper function to initialize with known data
fn setup_account_with_data() -> Account {
    let mut cb = setup_empty_account();

    // #0 Init (2025-09-01) : 200.00 => OP_ID = 0
    // credit: 200.00, debit: 0.0, balance: 200.0 <- 2025-09
    // #1 Debit (2025-10-04) : 14.20 => OP_ID = 4
    // #2 Credit (2025-10-08) : 50.00 => OP_ID = 2
    // #3 Debit (2025-10-21) : 44.80 => OP_ID = 5
    // #4 Debit (2025-10-21) : 11.00 => OP_ID = 8
    // credit; 50.00, debit: 70.00 balance: -20.0  <- 2025-10
    // #5 Credit (2025-11-05) : 100.00 => OP_ID = 1
    // #6 Debit (2025-11-12) : 15.70 => OP_ID = 7
    // #7 Debit (2025-11-20) : 23.60 => OP_ID = 10
    // credit: 100.00, debit: 39.30, balance: 60.70 <- 2025-11
    // #8 Debit (2025-12-05) : 25.50 => OP_ID = 3
    // #9 Credit (2025-12-10) : 10.00 => OP_ID = 9
    // #10 Credit (2025-12-15) : 150.00 => OP_ID = 6
    // credit: 160.00, debit: 25.50, balance: 134,50 <- 2025-12
    //
    // balance: 510.00 - 134.80 = 375.20
    // credit: 200 + 50.00 + 100.00 + 160 = 510.00
    // debit: 0 + 70.00 + 39.30 + 25.50 = 134.80

    // #0 Init (2025-01-01) : 200.00 => OP_ID = 0
    cb.initialize(parse_date("2025-09-01").unwrap(), dec!(200.0))
        .unwrap();

    // #5 Credit (2025-11-05) : 100.00 => OP_ID = 1
    cb.register_transaction(
        parse_date("2025-11-05").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Credit,
        dec!(100.0),
        "Atm".into(),
    )
    .unwrap();

    // #2 Credit (2025-10-08) : 50.00 => OP_ID = 2
    cb.register_transaction(
        parse_date("2025-10-08").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Credit,
        dec!(50.0),
        "Atm".into(),
    )
    .unwrap();

    // #8 Debit (2025-12-05) : 25.50 => OP_ID = 3
    cb.register_transaction(
        parse_date("2025-12-05").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(25.50),
        "Minimarket".into(),
    )
    .unwrap();

    // #1 Debit (2025-10-04) : 14.20 => OP_ID = 4
    cb.register_transaction(
        parse_date("2025-10-04").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(14.20),
        "Book".into(),
    )
    .unwrap();

    // #3 Debit (2025-10-21) : 44.80 => OP_ID = 5
    cb.register_transaction(
        parse_date("2025-10-21").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(44.80),
        "Post office".into(),
    )
    .unwrap();

    // #10 Credit (2025-12-15) : 150.00 => OP_ID = 6
    cb.register_transaction(
        parse_date("2025-12-15").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Credit,
        dec!(150.0),
        "Atm".into(),
    )
    .unwrap();

    // #6 Debit (2025-11-12) : 15.70 => OP_ID = 7
    cb.register_transaction(
        parse_date("2025-11-12").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(15.70),
        "Bakery".into(),
    )
    .unwrap();

    // #4 Debit (2025-10-21) : 11.00 => OP_ID = 8
    cb.register_transaction(
        parse_date("2025-10-21").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(11.00),
        "Fruits".into(),
    )
    .unwrap();

    // #9 Credit (2025-12-10) : 10.00 => OP_ID = 9
    cb.register_transaction(
        parse_date("2025-12-10").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Credit,
        dec!(10.0),
        "Refund".into(),
    )
    .unwrap();

    // #7 Debit (2025-11-20) : 23.60 => OP_ID = 10
    cb.register_transaction(
        parse_date("2025-11-20").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(23.60),
        "Newspapers".into(),
    )
    .unwrap();

    cb
}

#[test]
fn test_default_account_is_empty() {
    let account = setup_empty_account();

    let params = SearchParamsBuilder::default().build().unwrap();
    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::new(&balance_items);

    assert_eq!(
        account.operations.len(),
        0,
        "The default account should have 0 operations."
    );

    assert_eq!(
        balance_result.total(),
        Decimal::ZERO,
        "The balance of an empty codexi must be 0.0."
    );
}

#[test]
fn test_account_nb_op() {
    let account = setup_account_with_data();
    assert_eq!(
        account.operations.len(),
        11,
        "The account should have 11 operations."
    );
}

#[test]
fn test_2025_10_account_balance() {
    let account = setup_account_with_data();

    let params = SearchParamsBuilder::default()
        .from(Some(parse_date("2025-10-01").unwrap()))
        .to(Some(parse_date("2025-10-31").unwrap()))
        .build()
        .unwrap();
    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::new(&balance_items);

    // #1 Debit (2025-10-04) : 14.20 => OP_ID = 4
    // #2 Credit (2025-10-08) : 50.00 => OP_ID = 2
    // #3 Debit (2025-10-21) : 44.80 => OP_ID = 5
    // #4 Debit (2025-10-21) : 11.00 => OP_ID = 8
    // credit; 50.00, debit: 70.00 balance: -20.0

    assert_eq!(
        balance_result.credit,
        dec!(50.00),
        "The total credits are incorrect"
    );
    assert_eq!(
        balance_result.debit,
        dec!(70.00),
        "The total debits are incorrect."
    );
    assert_eq!(
        balance_result.total(),
        dec!(-20.00),
        "The final account balance is incorrect."
    );
}

#[test]
fn test_2025_11_account_balance() {
    let account = setup_account_with_data();

    let params = SearchParamsBuilder::default()
        .from(Some(parse_date("2025-11-01").unwrap()))
        .to(Some(parse_date("2025-11-30").unwrap()))
        .build()
        .unwrap();
    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::new(&balance_items);

    // #5 Credit (2025-11-05) : 100.00 => OP_ID = 1
    // #6 Debit (2025-11-12) : 15.70 => OP_ID = 7
    // #7 Debit (2025-11-20) : 23.60 => OP_ID = 10
    // credit: 100.00, debit: 39.30, balance: 60.70

    assert_eq!(
        balance_result.credit,
        dec!(100.00),
        "The total credits are incorrect"
    );
    assert_eq!(
        balance_result.debit,
        dec!(39.30),
        "The total debits are incorrect."
    );
    assert_eq!(
        balance_result.total(),
        dec!(60.70),
        "The final account balance is incorrect."
    );
}

#[test]
fn test_2025_12_account_balance() {
    let account = setup_account_with_data();

    let params = SearchParamsBuilder::default()
        .from(Some(parse_date("2025-12-01").unwrap()))
        .to(Some(parse_date("2025-12-31").unwrap()))
        .build()
        .unwrap();
    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::new(&balance_items);

    // #8 Debit (2025-12-05) : 25.50 => OP_ID = 3
    // #9 Credit (2025-12-10) : 10.00 => OP_ID = 9
    // #10 Credit (2025-12-15) : 150.00 => OP_ID = 6
    // credit: 160.00, debit: 25.50, balance: 134,50

    assert_eq!(
        balance_result.credit,
        dec!(160.00),
        "The total credits are incorrect"
    );
    assert_eq!(
        balance_result.debit,
        dec!(25.50),
        "The total debits are incorrect."
    );
    assert_eq!(
        balance_result.total(),
        dec!(134.50),
        "The final account balance is incorrect."
    );
}

#[test]
fn test_full_account_balance() {
    let account = setup_account_with_data();

    let params = SearchParamsBuilder::default().build().unwrap();

    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::new(&balance_items);

    // ASSERT: Verification of expected results
    // Expected total balance: 510.00 - 134.80 = 375.20
    // Expected total credit: 200 + 50.00 + 100.00 + 160 = 510.00
    // Expected total debit: 0 + 70.00 + 39.30 + 25.50 = 134.80

    assert_eq!(
        balance_result.credit,
        dec!(510.00),
        "The total credits are incorrect"
    );
    assert_eq!(
        balance_result.debit,
        dec!(134.80),
        "The total debits are incorrect."
    );
    assert_eq!(
        balance_result.total(),
        dec!(375.20),
        "The final account balance is incorrect."
    );
}

#[test]
fn test_balance_with_range_filter() {
    let account = setup_account_with_data();

    let params = SearchParamsBuilder::default()
        .from(Some(parse_date("2025-12-04").unwrap()))
        .to(Some(parse_date("2025-12-06").unwrap()))
        .build()
        .unwrap();

    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::new(&balance_items);

    assert_eq!(
        balance_result.credit,
        Decimal::ZERO,
        "The total filtered credit must be 0.0."
    );
    assert_eq!(
        balance_result.debit,
        dec!(25.50),
        "The total debits are incorrect."
    );
    assert_eq!(
        balance_result.total(),
        dec!(-25.50),
        "The balance filtered by date range is incorrect."
    );
}

#[test]
fn test_balance_with_day_filter_no_operations() {
    let account = setup_account_with_data();
    let params = SearchParamsBuilder::default()
        .from(Some(parse_date("2025-12-06").unwrap()))
        .to(Some(parse_date("2025-12-06").unwrap()))
        .build()
        .unwrap();

    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::new(&balance_items);

    assert_eq!(
        balance_result.credit,
        Decimal::ZERO,
        "The total filtered credit must be 0.0."
    );
    assert_eq!(
        balance_result.debit,
        Decimal::ZERO,
        "The total filtered debit must be 0.0."
    );
    assert_eq!(
        balance_result.total(),
        Decimal::ZERO,
        "The balance filtered by date range is incorrect."
    );
}

#[test]
fn test_balance_with_filter_month() {
    let account = setup_account_with_data();
    let params = SearchParamsBuilder::default()
        .from(Some(parse_date("2025-11-01").unwrap()))
        .to(Some(parse_date("2025-11-30").unwrap()))
        .build()
        .unwrap();

    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::new(&balance_items);

    assert_eq!(
        balance_result.credit,
        dec!(100.00),
        "The total credits are incorrect."
    );
    assert_eq!(
        balance_result.debit,
        dec!(39.30),
        "The total debits are incorrect"
    );
    assert_eq!(
        balance_result.total(),
        dec!(60.70),
        "The balance filtered by date range is incorrect."
    );
}

#[test]
fn test_stats_void_outside_period_has_no_effect() {
    // #1 Debit (2025-10-04) : 14.20 => OP_ID = 4
    // #2 Credit (2025-10-08) : 50.00 => OP_ID = 2
    // #3 Debit (2025-10-21) : 44.80 => OP_ID = 5
    // #4 Debit (2025-10-21) : 11.00 => OP_ID = 8
    // credit; 50.00, debit: 70.00 balance: -20.0

    let mut account = setup_account_with_data();

    // original op id 11
    let op_id_voided = account
        .register_transaction(
            parse_date("2025-10-22").unwrap(),
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(10.0),
            "Atm".into(),
        )
        .unwrap();

    // #11 Debit (2025-10-22) : 10.00 => OP_ID = 11
    // credit; 50.00, debit: 80.00 balance: -30.0

    // VOID  id 11
    account.void_operation(op_id_voided).unwrap(); // void op id 11  create a op VOID credit 10

    let params = SearchParamsBuilder::default()
        .from(Some(parse_date("2025-10-01").unwrap()))
        .to(Some(parse_date("2025-10-31").unwrap()))
        .build()
        .unwrap();

    // Stat does not take into account init and close amount
    let stats_no_net = account.stats_entry(&params, false).unwrap();
    let stats_net = account.stats_entry(&params, true).unwrap();

    assert_eq!(stats_no_net.balance, dec!(-30.00));
    assert_eq!(stats_net.balance, dec!(-30.00));

    assert_eq!(stats_no_net.total_credit, dec!(50.00));
    assert_eq!(stats_net.total_credit, dec!(50.00));

    assert_eq!(stats_no_net.total_debit, dec!(80.00));
    assert_eq!(stats_net.total_debit, dec!(80.00));
}

#[test]
fn test_stats_void_in_period_produces_expected_net_result() {
    let mut account = setup_account_with_data();

    // #0 Init (2025-09-01) : 200.00 => OP_ID = 0
    // credit: 200.00, debit: 0.0, balance: 200.0 <- 2025-09
    // #1 Debit (2025-10-04) : 14.20 => OP_ID = 4
    // #2 Credit (2025-10-08) : 50.00 => OP_ID = 2
    // #3 Debit (2025-10-21) : 44.80 => OP_ID = 5
    // #4 Debit (2025-10-21) : 11.00 => OP_ID = 8
    // credit; 50.00, debit: 70.00 balance: -20.0  <- 2025-10
    // #5 Credit (2025-11-05) : 100.00 => OP_ID = 1
    // #6 Debit (2025-11-12) : 15.70 => OP_ID = 7
    // #7 Debit (2025-11-20) : 23.60 => OP_ID = 10
    // credit: 100.00, debit: 39.30, balance: 60.70 <- 2025-11
    // #8 Debit (2025-12-05) : 25.50 => OP_ID = 3
    // #9 Credit (2025-12-10) : 10.00 => OP_ID = 9
    // #10 Credit (2025-12-15) : 150.00 => OP_ID = 6
    // credit: 160.00, debit: 25.50, balance: 135,50 <- 2025-12
    //
    // balance: 510.00 - 134.80 = 375.20
    // credit: 200 + 50.00 + 100.00 + 160 = 510.00
    // debit: 0 + 70.00 + 39.30 + 25.50 = 134.80

    // original op id 11
    let op_id_voided = account
        .register_transaction(
            parse_date("2025-10-22").unwrap(),
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(10.0),
            "Atm".into(),
        )
        .unwrap();

    // #11 Debit (2025-10-22) : 10.00 => OP_ID = 11

    // balance: 510.00 - 134.80 = 375.20  = 375.20 - init(200) = 175.20
    // credit: 200 + 50.00 + 100.00 + 160  = 520.00
    // debit: 0 + 70.00 + 39.30 + 25.50 + 10 = 144.80

    // VOID  id 11 / a Void op is at local date now
    account.void_operation(op_id_voided).unwrap(); // void op id 11  create a op VOID credit 10

    let params = SearchParamsBuilder::default()
        .from(Some(parse_date("2025-09-01").unwrap()))
        .to(Some(parse_date("4000-12-31").unwrap()))
        .build()
        .unwrap();

    // balance: 510.00 - 134.80 = 375.20  = 375.20 - init(+200) = 175.20
    // credit: 200 + 50.00 + 100.00 + 160 (+10 Void)= 510.00 - init(+200) = 310.00
    // debit: 0 + 70.00 + 39.30 + 25.50 (-10 Voided)= 134.80

    let stats_no_net = account.stats_entry(&params, false).unwrap();
    let stats_net = account.stats_entry(&params, true).unwrap();

    assert_eq!(
        stats_no_net.total_credit,
        dec!(310.00),
        "stat credit no net"
    );
    assert_eq!(stats_net.total_credit, dec!(310.00), "stat credit net");

    assert_eq!(stats_no_net.total_debit, dec!(144.80), "debit no net");
    assert_eq!(stats_net.total_debit, dec!(154.80), "debit net");

    assert_eq!(stats_net.balance, dec!(155.20));
}

#[test]
fn operation_is_not_voided_by_default() {
    let mut account = setup_account_with_data();

    // id 11
    let op_id = account
        .register_transaction(
            parse_date("2026-01-30").unwrap(),
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            dec!(50.0),
            "Atm".into(),
        )
        .unwrap();

    let op = account.get_operation_by_id(op_id).unwrap();
    assert!(op.links.void_by.is_none());
}

#[test]
fn init_operation_fail() {
    let mut account = setup_account_with_data();

    let res = account.initialize(parse_date("2026-01-15").unwrap(), dec!(100.0));

    assert!(res.is_err());
}

#[test]
fn init_operation_ok() {
    let mut account = setup_empty_account();

    let res = account.initialize(parse_date("2026-01-15").unwrap(), dec!(100.0));

    assert!(res.is_ok());
}

#[test]
fn void_operation() {
    let mut account = setup_account_with_data();

    // original op id 11
    let op_id_voided = account
        .register_transaction(
            parse_date("2026-01-15").unwrap(),
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            dec!(50.0),
            "Atm".into(),
        )
        .unwrap();

    // void op id 11 , op void id 12
    let op_id_void = account.void_operation(op_id_voided).unwrap(); // id 12

    let op_voided = account.get_operation_by_id(op_id_voided).unwrap();
    let op_void = account.get_operation_by_id(op_id_void).unwrap();

    assert!(op_voided.links.void_by.is_some());
    assert!(op_void.links.void_of.is_some());

    assert_eq!(op_voided.links.void_by, Some(op_id_void));
    assert_eq!(op_void.links.void_of, Some(op_id_voided));

    assert_eq!(op_voided.amount, op_void.amount);
    assert_eq!(op_voided.flow, op_void.flow.opposite());
}

#[test]
fn void_operation_can_not_void_itself() {
    let mut account = setup_account_with_data();

    // id 11
    let op_id_voided = account
        .register_transaction(
            parse_date("2026-01-15").unwrap(),
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            dec!(50.0),
            "Atm".into(),
        )
        .unwrap();

    // id 12
    let op_id_void1 = account.void_operation(op_id_voided).unwrap();
    // try to void a void operation
    let op_id_void2 = account.void_operation(op_id_void1);

    assert!(op_id_void2.is_err());
}

#[test]
fn cannot_void_same_operation_twice() {
    let mut account = setup_account_with_data();

    // id 11
    let op_id_voided = account
        .register_transaction(
            parse_date("2026-01-15").unwrap(),
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            dec!(50.0),
            "Atm".into(),
        )
        .unwrap();

    // id 12
    account.void_operation(op_id_voided).unwrap();
    // try to void a opearation already voided
    let op_id_void2 = account.void_operation(op_id_voided);

    assert!(op_id_void2.is_err());
}

// ── balance_at ───────────────────────────────────────────────

#[test]
fn balance_at_returns_correct_historical_balance() {
    let account = setup_account_with_data();

    // At 2025-09-30 — only init (200)
    assert_eq!(
        account.balance_at(parse_date("2025-09-30").unwrap()),
        dec!(200.00)
    );

    // At 2025-10-31 — init + oct ops (200 + 50 - 14.20 - 44.80 - 11 = 180)
    assert_eq!(
        account.balance_at(parse_date("2025-10-31").unwrap()),
        dec!(180.00)
    );

    // At 2025-11-30 — + nov ops (180 + 100 - 15.70 - 23.60 = 240.70)
    assert_eq!(
        account.balance_at(parse_date("2025-11-30").unwrap()),
        dec!(240.70)
    );

    // At 2025-12-31 — full balance (240.70 - 25.50 + 10 + 150 = 375.20)
    assert_eq!(
        account.balance_at(parse_date("2025-12-31").unwrap()),
        dec!(375.20)
    );
}

#[test]
fn balance_at_empty_account_returns_zero() {
    let account = setup_empty_account();
    assert_eq!(
        account.balance_at(parse_date("2025-12-31").unwrap()),
        Decimal::ZERO
    );
}

#[test]
fn balance_at_before_first_op_returns_zero() {
    let account = setup_account_with_data();
    // Before init date 2025-09-01
    assert_eq!(
        account.balance_at(parse_date("2025-08-31").unwrap()),
        Decimal::ZERO
    );
}

// ── compliance with past date ─────────────────────────────────

#[test]
fn compliance_rejects_overdraft_on_past_date() {
    // Reproduces the bug fixed by balance_at:
    // an operation inserted at a past date must be validated
    // against the balance at that date, not current_balance.

    let mut account = Account::new(
        parse_date("2026-01-01").unwrap(),
        "Test".into(),
        AccountType::Current,
        None,
        None,
    )
    .unwrap();

    // Set overdraft limit to 10
    account
        .context
        .update_context(Some(dec!(10)), None, None, None, None, None)
        .unwrap();

    // Init with 50 on 2026-01-01
    account
        .initialize(parse_date("2026-01-01").unwrap(), dec!(50))
        .unwrap();

    // Debit 55 on 2026-01-01 → balance -5 (within overdraft -10) → OK
    account
        .register_transaction(
            parse_date("2026-01-01").unwrap(),
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(55),
            "ok debit".into(),
        )
        .unwrap();

    // Debit 10 on 2026-01-01 → balance -15 < -10 → must fail
    let res = account.register_transaction(
        parse_date("2026-01-01").unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(10),
        "exceeds overdraft".into(),
    );
    assert!(res.is_err(), "overdraft at past date should be rejected");
}

// ── adjust lock same day ──────────────────────────────────────

#[test]
fn void_allowed_for_op_after_adjust_same_day() {
    use chrono::Local;
    let today = Local::now().date_naive();

    let mut account = Account::new(today, "Test".into(), AccountType::Current, None, None).unwrap();

    account.initialize(today, dec!(500)).unwrap();

    // OP1 before adjust
    account
        .register_transaction(
            today,
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(50),
            "op1".into(),
        )
        .unwrap();

    // Adjust — locks OP1
    account.adjust_balance(today, dec!(400)).unwrap();

    // OP3 after adjust — same day
    let op3_id = account
        .register_transaction(
            today,
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            dec!(100),
            "op3".into(),
        )
        .unwrap();

    // Void OP3 must succeed
    let res = account.void_operation(op3_id);
    assert!(
        res.is_ok(),
        "void of op after adjust same day should be allowed"
    );
}

#[test]
fn void_blocked_for_op_before_adjust_same_day() {
    use chrono::Local;
    let today = Local::now().date_naive();

    let mut account = Account::new(today, "Test".into(), AccountType::Current, None, None).unwrap();

    account.initialize(today, dec!(500)).unwrap();

    // OP1 before adjust
    let op1_id = account
        .register_transaction(
            today,
            OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(50),
            "op1".into(),
        )
        .unwrap();

    // Adjust — locks OP1
    account.adjust_balance(today, dec!(400)).unwrap();

    // Void OP1 must fail — locked by adjust
    let res = account.void_operation(op1_id);
    assert!(
        res.is_err(),
        "void of op before adjust same day should be locked"
    );
}

// ── Lifecycle — set_account_type ─────────────────────────────

#[test]
fn set_account_type_allowed_on_empty_account() {
    let mut account = setup_empty_account();
    let res = account.set_account_type(AccountType::Saving);
    assert!(res.is_ok());
    assert_eq!(account.context.account_type, AccountType::Saving);
}

#[test]
fn set_account_type_blocked_once_operations_exist() {
    let mut account = setup_account_with_data();
    let res = account.set_account_type(AccountType::Saving);
    assert!(res.is_err());
    // Type must remain unchanged
    assert_eq!(account.context.account_type, AccountType::Current);
}

#[test]
fn set_account_type_blocked_after_init_only() {
    let mut account = setup_empty_account();
    account
        .initialize(parse_date("2026-01-01").unwrap(), dec!(100))
        .unwrap();
    // Even with just an init, type change is blocked
    let res = account.set_account_type(AccountType::Business);
    assert!(res.is_err());
}

// ── Lifecycle — validate_close_date ──────────────────────────

#[test]
fn close_date_valid() {
    use chrono::Local;
    let account = setup_account_with_data();
    let today = Local::now().date_naive();
    let res = account.validate_close_date(today);
    assert!(res.is_ok());
}

#[test]
fn close_date_in_future_fails() {
    use chrono::{Duration, Local};
    let account = setup_account_with_data();
    let tomorrow = Local::now().date_naive() + Duration::days(1);
    let res = account.validate_close_date(tomorrow);
    assert!(res.is_err());
}

#[test]
fn close_date_before_open_date_fails() {
    let account = setup_account_with_data();
    // open_date = 2025-09-01, trying 2025-08-31
    let before_open = parse_date("2025-08-31").unwrap();
    let res = account.validate_close_date(before_open);
    assert!(res.is_err());
}

#[test]
fn close_date_before_last_operation_fails() {
    let account = setup_account_with_data();
    // Last op is 2025-12-15, trying 2025-12-01
    let before_last = parse_date("2025-12-01").unwrap();
    let res = account.validate_close_date(before_last);
    assert!(res.is_err());
}

#[test]
fn close_date_on_last_operation_date_is_valid() {
    let account = setup_account_with_data();
    // Last op is 2025-12-15 — closing on same day is allowed
    let last_op_date = parse_date("2025-12-15").unwrap();
    let res = account.validate_close_date(last_op_date);
    assert!(res.is_ok());
}

// ── Lifecycle — audit transfer links ─────────────────────────

#[test]
fn audit_detects_broken_transfer_link() {
    use codexi::logic::operation::{OperationBuilder, OperationLinks};
    use nulid::Nulid;

    let mut account = setup_empty_account();
    account
        .initialize(parse_date("2026-01-01").unwrap(), dec!(500))
        .unwrap();

    // Manually insert an op with transfer_id but no transfer_account_id
    let fake_transfer_id = Nulid::new().unwrap();
    let links = OperationLinks {
        transfer_id: Some(fake_transfer_id),
        ..Default::default()
    };
    // transfer_account_id intentionally left None — broken link

    let op = OperationBuilder::default()
        .date(parse_date("2026-01-01").unwrap())
        .kind(OperationKind::Regular(RegularKind::Transfer))
        .flow(OperationFlow::Debit)
        .amount(dec!(50))
        .description("broken transfer".to_string())
        .links(links)
        .build()
        .unwrap();

    account.operations.push(op);
    account.refresh_anchors();

    let warnings = account.audit().unwrap();
    assert!(
        warnings.iter().any(|w| w.message.contains("TEST 8")),
        "audit should detect broken transfer link"
    );
}
