use crate::{
    database::{
        dividends::{add_dividend, delete_dividend, edit_dividend},
        transactions::{add_transaction, delete_transaction, edit_transaction},
    },
    types::{Dividend, Transaction, TransactionType},
};
use anyhow::{Result, anyhow};
use rusqlite::Connection;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub fn add_transaction_to_list(
    conn: &Connection,
    trans_list: &mut Vec<Transaction>,
    trans: Transaction,
) -> Result<()> {
    let new_transaction = add_transaction(conn, trans)?;
    trans_list.push(new_transaction);
    Ok(())
}

pub fn delete_transaction_from_list(
    conn: &Connection,
    trans_list: &mut Vec<Transaction>,
    index: i64,
) -> Result<()> {
    let _ = delete_transaction(conn, index)?;
    let index = trans_list
        .iter()
        .position(|t| t.id.unwrap() == index)
        .ok_or_else(|| {
            anyhow!(
                "Couldn't find transaction with id: {} in transaction list",
                index
            )
        })?;
    let _ = trans_list.remove(index);
    Ok(())
}

pub fn edit_transaction_in_list(
    conn: &Connection,
    trans_list: &mut Vec<Transaction>,
    trans: Transaction,
) -> Result<()> {
    let index = trans_list
        .iter()
        .position(|t| t.id == trans.id)
        .ok_or_else(|| {
            anyhow!(
                "Couldn't find transaction with id: {} in transaction list",
                trans.id.unwrap()
            )
        })?;
    let _ = trans_list.remove(index);

    let new_trans = edit_transaction(conn, trans)?;
    trans_list.push(new_trans);
    Ok(())
}

pub fn add_dividend_to_list(
    conn: &Connection,
    div_list: &mut Vec<Dividend>,
    div: Dividend,
) -> Result<()> {
    let new_dividend = add_dividend(conn, div)?;
    div_list.push(new_dividend);
    Ok(())
}

pub fn delete_dividend_from_list(
    conn: &Connection,
    div_list: &mut Vec<Dividend>,
    index: i64,
) -> Result<()> {
    let _ = delete_dividend(conn, index)?;
    let index = div_list
        .iter()
        .position(|t| t.id.unwrap() == index)
        .ok_or_else(|| {
            anyhow!(
                "Couldn't find dividend with id: {} in transaction list",
                index
            )
        })?;
    let _ = div_list.remove(index);
    Ok(())
}

pub fn edit_dividend_in_list(
    conn: &Connection,
    div_list: &mut Vec<Dividend>,
    div: Dividend,
) -> Result<()> {
    let index = div_list
        .iter()
        .position(|t| t.id == div.id)
        .ok_or_else(|| {
            anyhow!(
                "Couldn't find dividend with id: {} in transaction list",
                div.id.unwrap()
            )
        })?;
    let _ = div_list.remove(index);

    let new_div = edit_dividend(conn, div)?;
    div_list.push(new_div);
    Ok(())
}

/// Calculates the number of bought and sold shares and returns them as (bought, sold)
pub fn calc_shares(transactions: &[Transaction], ticker: &str) -> (Decimal, Decimal) {
    let mut buy = dec!(0);
    let mut sell = dec!(0);
    for tx in transactions.iter().filter(|t| t.ticker == ticker) {
        match tx.transaction_type {
            TransactionType::Buy => buy = buy + tx.quantity,
            TransactionType::Sell => sell = sell + tx.quantity,
        }
    }
    (buy, sell)
}
