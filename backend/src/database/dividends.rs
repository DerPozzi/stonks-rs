use anyhow::Result;
use rusqlite::Connection;

use crate::types::Dividend;

pub fn delete_dividend(db_conn: &Connection, id: i64) -> Result<()> {
    let _ = db_conn.execute(
        "DELETE FROM dividends
        WHERE id = ?1
        ",
        (id,),
    )?;
    Ok(())
}

pub fn edit_dividend(db_conn: &Connection, dividend: Dividend) -> Result<Dividend> {
    println!("{:?}", dividend);
    let new_dividend = db_conn.query_row(
        "UPDATE dividends
        SET 
            ticker = ?1,
            payment_date = ?2,
            amount = ?3,
            currency = ?4,
            taxes = ?5
        WHERE id = ?6
        RETURNING id, ticker, payment_date, amount,  currency, taxes",
        (
            dividend.ticker,
            dividend.payment_date,
            dividend.amount.to_string(),
            dividend.currency,
            dividend.taxes.to_string(),
            dividend.id,
        ),
        |row| {
            Ok(Dividend {
                id: row.get(0)?,
                ticker: row.get(1)?,
                payment_date: row.get(2)?,
                amount: row.get::<_, String>(3)?.parse().unwrap(),
                currency: row.get(4)?,
                taxes: row.get::<_, String>(5)?.parse().unwrap(),
            })
        },
    )?;
    Ok(new_dividend)
}

pub fn load_all_dividends_from_db(db_conn: &Connection) -> Result<Vec<Dividend>> {
    let mut stmt = db_conn.prepare(
        "
    SELECT 
        id,
        ticker,
        payment_date,
        amount,
        currency,
        taxes
    FROM dividends
    ORDER BY payment_date DESC
    ",
    )?;

    let dividends = stmt
        .query_map([], |row| {
            Ok(Dividend {
                id: row.get(0)?,
                ticker: row.get(1)?,
                payment_date: row.get(2)?,
                amount: row.get::<_, String>(3)?.parse().unwrap(),
                currency: row.get(4)?,
                taxes: row.get::<_, String>(5)?.parse().unwrap(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(dividends)
}

pub fn add_dividend(db_conn: &Connection, dividend: Dividend) -> Result<Dividend> {
    let new_dividend = db_conn.query_row(
        "
        INSERT INTO dividends (
            ticker,
            payment_date,
            amount,
            currency,
            taxes
        )
        VALUES (?1, ?2, ?3, ?4, ?5)
        RETURNING id, ticker, payment_date, amount, currency, taxes
        ",
        (
            dividend.ticker,
            dividend.payment_date,
            dividend.amount.to_string(),
            dividend.currency,
            dividend.taxes.to_string(),
        ),
        |row| {
            Ok(Dividend {
                id: row.get(0)?,
                ticker: row.get(1)?,
                payment_date: row.get(2)?,
                amount: row.get::<_, String>(3)?.parse().unwrap(),
                currency: row.get(4)?,
                taxes: row.get::<_, String>(5)?.parse().unwrap(),
            })
        },
    )?;
    Ok(new_dividend)
}
