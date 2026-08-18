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
    target_currency: Option<Currency>,
) -> Result<Decimal> {
    let current_price = get_current_asset_price(ticker).await?;
    let asset_currency = get_asset_currency(ticker).await?;
    let market_value = calc_market_value(transactions, ticker, current_price)?;

    let exchange_rate = if let Some(target_currency) = target_currency {
        get_exchange_rate(asset_currency, target_currency).await?
    } else {
        dec!(1)
    };

    Ok(market_value * exchange_rate)
}

pub async fn get_portfolio_value(
    transactions: &[Transaction],
    target_currency: Option<Currency>,
) -> Result<Decimal> {
    let portfolio_value = calc_portfolio_value(transactions, target_currency).await?;
    Ok(portfolio_value)
}

pub fn db_add_transaction(conn: &Connection, tx: Transaction) -> Result<Transaction> {
    transactions::add_transaction(conn, tx)
}

pub async fn get_ticker_info(
    t: String,
    tx: &[Transaction],
    curr: Option<Currency>,
) -> Result<TickerData> {
    let t = t.as_str();
    let (buy, sell) = calc_shares(tx, t);

    if buy - sell == dec!(0) {
        return Ok(TickerData {
            total_shares: buy - sell,
            ..Default::default()
        });
    }
    let current_price = get_current_asset_price(t).await?;
    let asset_currency = get_asset_currency(t).await?;

    let exchange_rate = if let Some(t) = curr {
        get_exchange_rate(asset_currency, t).await?
    } else {
        dec!(1)
    };

    let name = get_ticker_name(t).await?;
    let market_value = calc_market_value(tx, t, current_price)? * exchange_rate;
    let avg_cost = calc_avg_cost(tx, t) * exchange_rate;
    let cost_basis = calc_cost_basis(tx, t) * exchange_rate;
    let unrealized_gain = calc_unrealized_gain(tx, t, current_price)? * exchange_rate;
    let unrealized_gain_perc = calc_unrealized_gain_perc(tx, t, current_price)?;
    let realized_gain = calc_realized_gains(tx, t)? * exchange_rate;

    Ok(TickerData {
        name,
        ticker: t.to_string(),
        cost_basis,
        current_price: current_price * exchange_rate,
        market_value,
        total_shares: buy - sell,
        avg_cost,
        unrealized_gain,
        unrealized_gain_perc,
        realized_gain,
    })
}
