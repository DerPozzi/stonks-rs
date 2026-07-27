use crate::{
    database::{add_transaction, delete_transaction, edit_transaction},
    types::Transaction,
};
use anyhow::{Result, anyhow};
use rusqlite::Connection;

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
