use chrono::NaiveDate;
use stonks_rs::{
    calculate::calc_cost_price,
    types::{Currency, Transaction, TransactionType},
};

use rust_decimal_macros::dec;

#[test]
fn cost_price() -> anyhow::Result<()> {
    let transactions = vec![
        Transaction {
            id: None,
            ticker: "AMD".into(),
            transaction_type: TransactionType::Buy,
            trade_date: NaiveDate::from_ymd_opt(2026, 6, 27).unwrap(),
            quantity: dec!(0.09937081),
            price: dec!(45.97),
            currency: Currency::EUR,
            fees: dec!(0.07),
        },
        Transaction {
            id: None,
            ticker: "AMD".into(),
            transaction_type: TransactionType::Buy,
            trade_date: NaiveDate::from_ymd_opt(2026, 07, 1).unwrap(),
            quantity: dec!(0.02882124),
            price: dec!(14.53),
            currency: Currency::EUR,
            fees: dec!(0.02),
        },
    ];

    let cost_price = calc_cost_price(&transactions, "AMD");

    assert_eq!(cost_price.round_dp(2), dec!(39.60));

    Ok(())
}
