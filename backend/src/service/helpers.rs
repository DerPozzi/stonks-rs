use anyhow::Result;
use anyhow::anyhow;
use rusqlite::Connection;
use rust_decimal::Decimal;

use crate::database::*;
use crate::types::*;

pub fn add_transaction_to_list(
    conn: &Connection,
    trans_list: &mut Vec<Transaction>,
    trans: Transaction,
) -> Result<()> {
    let new_transaction = transactions::add_transaction(conn, trans)?;
    trans_list.push(new_transaction);
    Ok(())
}

pub fn delete_transaction_from_list(
    conn: &Connection,
    trans_list: &mut Vec<Transaction>,
    index: i64,
) -> Result<()> {
    transactions::delete_transaction(conn, index)?;
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

pub async fn get_exchange_rate(current: Currency, target: Currency) -> Result<Decimal> {
    crate::yahoo::get_exchange_rate(current, target).await
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

    let new_trans = transactions::edit_transaction(conn, trans)?;
    trans_list.push(new_trans);
    Ok(())
}

pub fn add_dividend_to_list(
    conn: &Connection,
    div_list: &mut Vec<Dividend>,
    div: Dividend,
) -> Result<()> {
    let new_dividend = dividends::add_dividend(conn, div)?;
    div_list.push(new_dividend);
    Ok(())
}
