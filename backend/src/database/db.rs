use anyhow::Result;
use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> Result<()> {
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

    quantity            TEXT NOT NULL,
    price               TEXT NOT NULL,

    fees                TEXT NOT NULL DEFAULT 0,
    taxes               TEXT NOT NULL DEFAULT 0,

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

    amount              TEXT NOT NULL,

    taxes               TEXT NOT NULL DEFAULT 0,

    currency            TEXT NOT NULL


);

    ";
    conn.execute_batch(create_tables_string)?;
    Ok(())
}
