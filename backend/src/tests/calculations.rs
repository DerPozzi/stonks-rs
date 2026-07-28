use crate::{
    calculate::{
        calc_avg_cost, calc_gross_dividends, calc_market_value, calc_net_dividends,
        calc_realized_gains, calc_sale_value, calc_unrealized_gain,
    },
    types::{Currency, Dividend, Transaction, TransactionType},
};
use chrono::NaiveDate;
use rust_decimal::Decimal;

use rust_decimal_macros::dec;

const CURRENT_PRICE: Decimal = dec!(500);

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

    let cost_price = calc_avg_cost(&transactions, "AMD");

    assert_eq!(cost_price.round_dp(2), dec!(39.60));

    Ok(())
}

#[test]
fn unrealized_gain() -> anyhow::Result<()> {
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

    let unrealized_gain = calc_unrealized_gain(&transactions, "AMD", CURRENT_PRICE)?;

    assert_eq!(unrealized_gain.round_dp(2), dec!(24.49));
    Ok(())
}

#[test]
fn realized_gain() -> anyhow::Result<()> {
    let transactions = vec![
        Transaction {
            id: None,
            ticker: "AMD".into(),
            transaction_type: TransactionType::Buy,
            trade_date: NaiveDate::from_ymd_opt(2026, 6, 27).unwrap(),
            quantity: dec!(10),
            price: dec!(100),
            currency: Currency::EUR,
            fees: dec!(1),
        },
        Transaction {
            id: None,
            ticker: "AMD".into(),
            transaction_type: TransactionType::Sell,
            trade_date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            quantity: dec!(5),
            price: dec!(120),
            currency: Currency::EUR,
            fees: dec!(2),
        },
    ];

    let realized_gain = calc_realized_gains(&transactions, "AMD")?;

    assert_eq!(realized_gain.round_dp(2), dec!(97.50));

    Ok(())
}

#[test]
fn dividends() -> anyhow::Result<()> {
    let dividends = vec![
        Dividend {
            id: None,
            ticker: "AMD".into(),
            payment_date: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            amount: dec!(10),
            taxes: dec!(2),
            currency: Currency::EUR,
        },
        Dividend {
            id: None,
            ticker: "AMD".into(),
            payment_date: NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
            amount: dec!(15),
            taxes: dec!(3),
            currency: Currency::EUR,
        },
    ];

    let gross = calc_gross_dividends(&dividends, "AMD");
    let net = calc_net_dividends(&dividends, "AMD");

    assert_eq!(gross, dec!(25));
    assert_eq!(net, dec!(20));

    Ok(())
}

#[test]
fn sale_value() -> anyhow::Result<()> {
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

    let sale_value = calc_sale_value(&transactions, "AMD")?;
    assert_eq!(sale_value, dec!(0));
    Ok(())
}

#[test]
fn market_value() -> anyhow::Result<()> {
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
    let result = calc_market_value(&transactions, "AMD", CURRENT_PRICE)?;

    assert_eq!(result.round_dp(2), dec!(64.1));

    Ok(())
}
