use crate::{backend::database::edit_transaction, types::Transaction};
use anyhow::{Result, anyhow};
use rusqlite::Connection;

pub fn edit_transaction_in_list(
    trans_list: Vec<Transaction>,
    trans: Transaction,
    conn: &Connection,
) -> Result<Vec<Transaction>> {
    let index = trans_list
        .iter()
        .position(|t| t.id == trans.id)
        .ok_or_else(|| {
            anyhow!(
                "Couldn't find transaction with id: {} in transaction list",
                trans.id.unwrap()
            )
        })?;
    let mut transactions: Vec<Transaction> = trans_list.clone();
    let _ = transactions.remove(index);

    let new_trans = edit_transaction(conn, trans)?;
    transactions.push(new_trans);
    Ok(transactions)
}
