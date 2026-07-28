use chrono::NaiveDate;
use rusqlite::Connection;

use crate::{
    database::{
        database::create_tables,
        transactions::{
            add_transaction, delete_transaction, edit_transaction, load_all_transactions_from_db,
        },
    },
    types::{Currency, Transaction, TransactionType},
};
use rust_decimal_macros::dec;

fn test_db() -> anyhow::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    create_tables(&conn)?;
    Ok(conn)
}

fn sample_transaction() -> Transaction {
    Transaction {
        id: Some(0),
        ticker: "VWCE".into(),
        transaction_type: TransactionType::Buy,
        trade_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        quantity: dec!(10.0),
        price: dec!(100.0),
        currency: Currency::EUR,
        fees: dec!(0.02),
    }
}

#[test]
fn insert_transaction() -> anyhow::Result<()> {
    let conn = test_db()?;

    let tx = sample_transaction();

    add_transaction(&conn, tx)?;

    let transactions = load_all_transactions_from_db(&conn)?;

    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions[0].ticker, "VWCE");

    Ok(())
}

#[test]
fn update_transaction() -> anyhow::Result<()> {
    let conn = test_db()?;

    let mut tx = add_transaction(&conn, sample_transaction())?;

    tx.price = dec!(250.0);
    tx.quantity = dec!(20.0);

    edit_transaction(&conn, tx)?;

    let transactions = load_all_transactions_from_db(&conn)?;

    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions[0].price, dec!(250.0));
    assert_eq!(transactions[0].quantity, dec!(20.0));

    Ok(())
}

#[test]
fn delete_transaction_test() -> anyhow::Result<()> {
    let conn = test_db()?;

    let tx = add_transaction(&conn, sample_transaction())?;

    delete_transaction(&conn, tx.id.unwrap())?;

    let transactions = load_all_transactions_from_db(&conn)?;

    assert!(transactions.is_empty());

    Ok(())
}
