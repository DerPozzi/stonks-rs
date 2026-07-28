use anyhow::Result;
use rusqlite::Connection;

use crate::types::Transaction;

pub fn delete_transaction(db_conn: &Connection, id: i64) -> Result<()> {
    let _ = db_conn.execute(
        "DELETE FROM transactions
        WHERE id = ?1
        ",
        (id,),
    )?;
    Ok(())
}

pub fn edit_transaction(db_conn: &Connection, transaction: Transaction) -> Result<Transaction> {
    let new_transaction = db_conn.query_row(
        "UPDATE transactions
        SET 
            ticker = ?1,
            transaction_type = ?2,
            trade_date = ?3,
            quantity = ?4,
            price = ?5,
            currency = ?6,
            fees = ?7
        WHERE id = ?8
        RETURNING id, ticker, transaction_type, trade_date, quantity, price, currency, fees",
        (
            transaction.ticker,
            transaction.transaction_type,
            transaction.trade_date,
            transaction.quantity.to_string(),
            transaction.price.to_string(),
            transaction.currency,
            transaction.fees.to_string(),
            transaction.id,
        ),
        |row| {
            Ok(Transaction {
                id: row.get(0)?,
                ticker: row.get(1)?,
                transaction_type: row.get(2)?,
                trade_date: row.get(3)?,
                quantity: row.get::<_, String>(4)?.parse().unwrap(),
                price: row.get::<_, String>(5)?.parse().unwrap(),
                currency: row.get(6)?,
                fees: row.get::<_, String>(7)?.parse().unwrap(),
            })
        },
    )?;
    Ok(new_transaction)
}

pub fn load_all_transactions_from_db(db_conn: &Connection) -> Result<Vec<Transaction>> {
    let mut stmt = db_conn.prepare(
        "
    SELECT 
        id,
        ticker,
        transaction_type,
        trade_date,
        quantity,
        price,
        currency,
        fees
    FROM transactions
    ORDER BY trade_date DESC
    ",
    )?;

    let transactions = stmt
        .query_map([], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                ticker: row.get(1)?,
                transaction_type: row.get(2)?,
                trade_date: row.get(3)?,
                quantity: row.get::<_, String>(4)?.parse().unwrap(),
                price: row.get::<_, String>(5)?.parse().unwrap(),
                currency: row.get(6)?,
                fees: row.get::<_, String>(7)?.parse().unwrap(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(transactions)
}

pub fn add_transaction(db_conn: &Connection, transaction: Transaction) -> Result<Transaction> {
    let new_transaction = db_conn.query_row(
        "
        INSERT INTO transactions (
            ticker,
            transaction_type,
            trade_date,
            quantity,
            price,
            currency,
            fees
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        RETURNING id, ticker, transaction_type, trade_date, quantity, price, currency, fees
        ",
        (
            transaction.ticker,
            transaction.transaction_type,
            transaction.trade_date,
            transaction.quantity.to_string(),
            transaction.price.to_string(),
            transaction.currency,
            transaction.fees.to_string(),
        ),
        |row| {
            Ok(Transaction {
                id: row.get(0)?,
                ticker: row.get(1)?,
                transaction_type: row.get(2)?,
                trade_date: row.get(3)?,
                quantity: row.get::<_, String>(4)?.parse().unwrap(),
                price: row.get::<_, String>(5)?.parse().unwrap(),
                currency: row.get(6)?,
                fees: row.get::<_, String>(7)?.parse().unwrap(),
            })
        },
    )?;
    Ok(new_transaction)
}
