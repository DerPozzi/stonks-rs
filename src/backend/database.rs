use crate::types::Transaction;
use anyhow::Result;
use rusqlite::Connection;

pub fn add_transaction(db_conn: &Connection, transaction: Transaction) -> Result<Transaction> {
    let new_transaction = db_conn.query_row(
        "
        INSERT INTO transactions (
            ticker,
            transaction_type,
            trade_date,
            quantity,
            price,
            currency
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING id, ticker, transaction_type, trade_date, quantity, price, currency
        ",
        (
            transaction.ticker,
            transaction.transaction_type,
            transaction.trade_date,
            transaction.quantity,
            transaction.price,
            transaction.currency,
        ),
        |row| {
            Ok(Transaction {
                id: Some(row.get(0)?),
                ticker: row.get(1)?,
                transaction_type: row.get(2)?,
                trade_date: row.get(3)?,
                quantity: row.get(4)?,
                price: row.get(5)?,
                currency: row.get(6)?,
            })
        },
    )?;
    Ok(new_transaction)
}
