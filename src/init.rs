use anyhow::{Error, Result};
use chrono::NaiveDate;
use rusqlite::{Connection, types::FromSql};

#[derive(Debug)]
pub enum Currency {
    USD,
    EUR,
}

#[derive(Debug)]
pub enum TransactionType {
    Buy,
    Sell,
}

impl FromSql for TransactionType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            "buy" => Ok(TransactionType::Buy),
            "sell" => Ok(TransactionType::Sell),

            _ => Err(rusqlite::types::FromSqlError::Other(
                "Unknown transaction type".into(),
            )),
        }
    }
}

impl FromSql for Currency {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            "usd" => Ok(Currency::USD),
            "eur" => Ok(Currency::EUR),

            _ => Err(rusqlite::types::FromSqlError::Other(
                "Unknown currency".into(),
            )),
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    id: i64,
    ticker: String,
    transaction_type: TransactionType,
    trade_date: NaiveDate,
    quantity: f32,
    price: f32,
    currency: Currency,
}

pub fn load_from_db(db_conn: &Connection) -> Result<Vec<Transaction>> {
    let mut stmt = db_conn.prepare(
        "
    SELECT 
        id,
        ticker,
        transaction_type,
        trade_date,
        quantity,
        price,
        currency
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
                quantity: row.get(4)?,
                price: row.get(5)?,
                currency: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(transactions)
}

pub fn init_db(home_path: &str) -> Result<Connection> {
    let db_path = format!("{}stonks-rs.db", home_path);
    let conn = Connection::open(db_path)?;

    let create_tables_string = r"PRAGMA foreign_keys = ON;

------------------------------------------------------------
-- Watchlist
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS watchlist (
    ticker      TEXT PRIMARY KEY,
    isin        TEXT,
    name        TEXT,
    asset_type  TEXT NOT NULL CHECK(asset_type IN (
        'stock',
        'etf',
        'fund',
        'bond',
        'crypto',
        'cash'
    )),
    created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

------------------------------------------------------------
-- Transactions
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS transactions (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,

    ticker              TEXT NOT NULL,
    isin                TEXT,

    transaction_type    TEXT NOT NULL CHECK(transaction_type IN (
        'buy',
        'sell'
    )),

    trade_date          TEXT NOT NULL,

    quantity            DECIMAL(20,8) NOT NULL,
    price               DECIMAL(20,8) NOT NULL,

    fees                DECIMAL(20,8) NOT NULL DEFAULT 0,
    taxes               DECIMAL(20,8) NOT NULL DEFAULT 0,

    currency            TEXT NOT NULL


);

------------------------------------------------------------
-- Dividends
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS dividends (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,

    ticker              TEXT NOT NULL,
    isin                TEXT,

    payment_date        TEXT NOT NULL,

    amount              DECIMAL(20,8) NOT NULL,

    taxes               DECIMAL(20,8) NOT NULL DEFAULT 0,

    currency            TEXT NOT NULL


);

    ";
    conn.execute_batch(create_tables_string)?;
    Ok(conn)
}
