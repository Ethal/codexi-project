// tests/account_test.rs

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use codexi::core::parse_date;
use codexi::logic::account::{Account, SearchParamsBuilder, StatsEntry, search};
use codexi::logic::balance::Balance;
use codexi::logic::operation::RegularKind;
use codexi::logic::operation::{OperationFlow, OperationKind};

fn setup_empty_account() -> Account {
    // init
    Account::new(
        parse_date("2025-09-01".into()).unwrap(),
        "Test".into(),
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
    cb.initialize(parse_date("2025-09-01".into()).unwrap(), dec!(200.0))
        .unwrap();

    // #5 Credit (2025-11-05) : 100.00 => OP_ID = 1
    cb.register_transaction(
        parse_date("2025-11-05".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Credit,
        dec!(100.0),
        "Atm".into(),
    )
    .unwrap();

    // #2 Credit (2025-10-08) : 50.00 => OP_ID = 2
    cb.register_transaction(
        parse_date("2025-10-08".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Credit,
        dec!(50.0),
        "Atm".into(),
    )
    .unwrap();

    // #8 Debit (2025-12-05) : 25.50 => OP_ID = 3
    cb.register_transaction(
        parse_date("2025-12-05".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(25.50),
        "Minimarket".into(),
    )
    .unwrap();

    // #1 Debit (2025-10-04) : 14.20 => OP_ID = 4
    cb.register_transaction(
        parse_date("2025-10-04".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(14.20),
        "Book".into(),
    )
    .unwrap();

    // #3 Debit (2025-10-21) : 44.80 => OP_ID = 5
    cb.register_transaction(
        parse_date("2025-10-21".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(44.80),
        "Post office".into(),
    )
    .unwrap();

    // #10 Credit (2025-12-15) : 150.00 => OP_ID = 6
    cb.register_transaction(
        parse_date("2025-12-15".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Credit,
        dec!(150.0),
        "Atm".into(),
    )
    .unwrap();

    // #6 Debit (2025-11-12) : 15.70 => OP_ID = 7
    cb.register_transaction(
        parse_date("2025-11-12".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(15.70),
        "Bakery".into(),
    )
    .unwrap();

    // #4 Debit (2025-10-21) : 11.00 => OP_ID = 8
    cb.register_transaction(
        parse_date("2025-10-21".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Debit,
        dec!(11.00),
        "Fruits".into(),
    )
    .unwrap();

    // #9 Credit (2025-12-10) : 10.00 => OP_ID = 9
    cb.register_transaction(
        parse_date("2025-12-10".into()).unwrap(),
        OperationKind::Regular(RegularKind::Transaction),
        OperationFlow::Credit,
        dec!(10.0),
        "Refund".into(),
    )
    .unwrap();

    // #7 Debit (2025-11-20) : 23.60 => OP_ID = 10
    cb.register_transaction(
        parse_date("2025-11-20".into()).unwrap(),
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
    let balance_result = Balance::balance(&balance_items);

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
        .from(Some(parse_date("2025-10-01".into()).unwrap()))
        .to(Some(parse_date("2025-10-31".into()).unwrap()))
        .build()
        .unwrap();
    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::balance(&balance_items);

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
        .from(Some(parse_date("2025-11-01".into()).unwrap()))
        .to(Some(parse_date("2025-11-30".into()).unwrap()))
        .build()
        .unwrap();
    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::balance(&balance_items);

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
        .from(Some(parse_date("2025-12-01".into()).unwrap()))
        .to(Some(parse_date("2025-12-31".into()).unwrap()))
        .build()
        .unwrap();
    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::balance(&balance_items);

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
    let balance_result = Balance::balance(&balance_items);

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
        .from(Some(parse_date("2025-12-04".into()).unwrap()))
        .to(Some(parse_date("2025-12-06".into()).unwrap()))
        .build()
        .unwrap();

    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::balance(&balance_items);

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
        .from(Some(parse_date("2025-12-06".into()).unwrap()))
        .to(Some(parse_date("2025-12-06".into()).unwrap()))
        .build()
        .unwrap();

    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::balance(&balance_items);

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
        .from(Some(parse_date("2025-11-01".into()).unwrap()))
        .to(Some(parse_date("2025-11-30".into()).unwrap()))
        .build()
        .unwrap();

    let balance_items = search(&account, &params).unwrap();
    let balance_result = Balance::balance(&balance_items);

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
            parse_date("2025-10-22".into()).unwrap(),
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
        .from(Some(parse_date("2025-10-01".into()).unwrap()))
        .to(Some(parse_date("2025-10-31".into()).unwrap()))
        .build()
        .unwrap();

    // operation void outside the criteria
    let items = search(&account, &params).unwrap();

    // Stat does not take into account init and close amount
    let stats_no_net = StatsEntry::stats_entry(&items, false).unwrap();
    let stats_net = StatsEntry::stats_entry(&items, true).unwrap();

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
            parse_date("2025-10-22".into()).unwrap(),
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
        .from(Some(parse_date("2025-09-01".into()).unwrap()))
        .to(Some(parse_date("4000-12-31".into()).unwrap()))
        .build()
        .unwrap();

    // operation void and voided in criteria
    let items = search(&account, &params).unwrap();

    // balance: 510.00 - 134.80 = 375.20  = 375.20 - init(+200) = 175.20
    // credit: 200 + 50.00 + 100.00 + 160 (+10 Void)= 510.00 - init(+200) = 310.00
    // debit: 0 + 70.00 + 39.30 + 25.50 (-10 Voided)= 134.80

    let stats_no_net = StatsEntry::stats_entry(&items, false).unwrap();
    let stats_net = StatsEntry::stats_entry(&items, true).unwrap();

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
            parse_date("2026-01-30".into()).unwrap(),
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

    let res = account.initialize(parse_date("2026-01-15".into()).unwrap(), dec!(100.0));

    assert!(res.is_err());
}

#[test]
fn init_operation_ok() {
    let mut account = setup_empty_account();

    let res = account.initialize(parse_date("2026-01-15".into()).unwrap(), dec!(100.0));

    assert!(res.is_ok());
}

#[test]
fn void_operation() {
    let mut account = setup_account_with_data();

    // original op id 11
    let op_id_voided = account
        .register_transaction(
            parse_date("2026-01-15".into()).unwrap(),
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
            parse_date("2026-01-15".into()).unwrap(),
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
            parse_date("2026-01-15".into()).unwrap(),
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
