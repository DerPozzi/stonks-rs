use std::path::PathBuf;

use anyhow::Result;
use rusqlite::Connection;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::calculate::*;
use crate::database::*;
use crate::init::*;
use crate::types::*;
use crate::yahoo::*;

/// Init db
///
/// Initializes the database at the given path, creates all neccesary tables and returns the db
/// connection
pub fn init_db(path: PathBuf) -> Result<Connection> {
    let conn = open_database(path)?;
    database::create_tables(&conn)?;
    Ok(conn)
}

/// Calculate amount of shares
///
/// Calculates the number of bought and sold shares and returns them as (bought, sold)
pub fn calc_shares(transactions: &[Transaction], ticker: &str) -> (Decimal, Decimal) {
    let mut buy = dec!(0);
    let mut sell = dec!(0);
    for tx in transactions.iter().filter(|t| t.ticker == ticker) {
        match tx.transaction_type {
            TransactionType::Buy => buy += tx.quantity,
            TransactionType::Sell => sell += tx.quantity,
        }
    }
    (buy, sell)
}

pub async fn get_market_value(
    transactions: &[Transaction],
    ticker: &str,
    target_currency: Currency,
) -> Result<Decimal> {
    let current_price = get_current_asset_price(ticker).await?;
    let asset_currency = get_asset_currency(ticker).await?;
    let market_value = calc_market_value(transactions, ticker, current_price)?;

    if target_currency != asset_currency {
        let exchange_rate = get_exchange_rate(asset_currency, target_currency).await?;
        return Ok(market_value * exchange_rate);
    }

    Ok(market_value)
}

pub fn db_add_transaction(conn: &Connection, tx: Transaction) -> Result<Transaction> {
    transactions::add_transaction(conn, tx)
}
