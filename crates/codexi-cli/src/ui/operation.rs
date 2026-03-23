// src/ui/operations.rs

use rust_decimal::Decimal;
use thousands::Separable;

use codexi::{
    core::{format_id_short, format_optional_id_short},
    logic::codexi::OperationDetailItem,
};

use crate::ui::{CREDIT_STYLE, DEBIT_STYLE, LABEL_STYLE, NOTE_STYLE, TITLE_STYLE, VALUE_STYLE};

/// Display a detailed view of a single operation
pub fn view_operation(item: &OperationDetailItem) {
    // ── Operation ─────────────────────────────────────────────
    println!();
    println!("{}", TITLE_STYLE.apply_to("Operation"));
    println!("{}", "─".repeat(45));

    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Id"),
        VALUE_STYLE.apply_to(&item.id)
    );
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Short"),
        VALUE_STYLE.apply_to(format_id_short(&item.id))
    );
    println!("  {:<14} {}", LABEL_STYLE.apply_to("Date"), &item.date);
    println!("  {:<14} {}", LABEL_STYLE.apply_to("Type"), &item.kind);

    let flow_display = if item.flow == "Debit" {
        DEBIT_STYLE.apply_to(&item.flow)
    } else {
        CREDIT_STYLE.apply_to(&item.flow)
    };
    // add indicateur can_be_void on the same line of the flow
    let can_void_indicator = if item.can_be_void {
        format!("  {}", NOTE_STYLE.apply_to("[can void]"))
    } else {
        String::new()
    };

    println!(
        "  {:<14} {}{}",
        LABEL_STYLE.apply_to("Flow"),
        flow_display,
        can_void_indicator
    );

    let amount_str = format!("{:.2}", item.amount).separate_with_commas();
    let amount_display = if item.flow == "Debit" {
        DEBIT_STYLE.apply_to(&amount_str)
    } else {
        CREDIT_STYLE.apply_to(&amount_str)
    };
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Amount"),
        amount_display
    );

    let balance_str = format!("{:.2}", item.balance).separate_with_commas();
    let balance_display = if item.balance < Decimal::ZERO {
        DEBIT_STYLE.apply_to(&balance_str)
    } else {
        CREDIT_STYLE.apply_to(&balance_str)
    };
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Balance"),
        balance_display
    );
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Description"),
        &item.description
    );

    // ── Links ─────────────────────────────────────────────────
    println!();
    println!("{}", TITLE_STYLE.apply_to("Links"));
    println!("{}", "─".repeat(45));

    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Void of"),
        format_optional_id_short(item.void_of.as_deref()),
    );
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Void by"),
        format_optional_id_short(item.void_by.as_deref()),
    );

    // Transfer — show both twin op and twin account if present
    let transfer_display = match (&item.transfer_id, &item.transfer_account_id) {
        (Some(tid), Some(aid)) => format!(
            "{} (account {})",
            format_id_short(tid),
            format_id_short(aid)
        ),
        (Some(tid), None) => format_id_short(tid),
        _ => "—".into(),
    };
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Transfer"),
        transfer_display
    );

    // ── Context ───────────────────────────────────────────────
    println!();
    println!("{}", TITLE_STYLE.apply_to("Context"));
    println!("{}", "─".repeat(45));

    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Currency"),
        &item.currency
    );
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Rate"),
        format!("{:.2}", item.exchange_rate).separate_with_commas()
    );
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Category"),
        &item.category
    );
    println!("  {:<14} {}", LABEL_STYLE.apply_to("Payee"), &item.payee);
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Reconciled"),
        &item.reconciled
    );

    // ── Meta ──────────────────────────────────────────────────
    println!();
    println!("{}", TITLE_STYLE.apply_to("Meta"));
    println!("{}", "─".repeat(45));

    println!("  {:<14} {}", LABEL_STYLE.apply_to("Tags"), &item.tags);
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Note"),
        NOTE_STYLE.apply_to(&item.note)
    );
    println!(
        "  {:<14} {}",
        LABEL_STYLE.apply_to("Attachment"),
        &item.attachment
    );

    println!();
}

/// Display raw debug output — for diagnostic purposes
pub fn view_operation_raw(item: &OperationDetailItem) {
    println!("{:#?}", item);
}
