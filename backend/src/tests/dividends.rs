use chrono::NaiveDate;
use rusqlite::Connection;

use crate::{
    database::{
        database::create_tables,
        dividends::{add_dividend, delete_dividend, edit_dividend, load_all_dividends_from_db},
    },
    types::{Currency, Dividend},
};
use rust_decimal_macros::dec;

fn test_db() -> anyhow::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    create_tables(&conn)?;
    Ok(conn)
}

fn sample_dividend() -> Dividend {
    Dividend {
        id: Some(0),
        ticker: "VWCE".into(),
        isin: "test".into(),
        payment_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        amount: dec!(10.0),
        currency: Currency::EUR,
        taxes: dec!(0.02),
    }
}

#[test]
fn insert_dividend() -> anyhow::Result<()> {
    let conn = test_db()?;

    let tx = sample_dividend();

    add_dividend(&conn, tx)?;

    let dividends = load_all_dividends_from_db(&conn)?;

    assert_eq!(dividends.len(), 1);
    assert_eq!(dividends[0].ticker, "VWCE");

    Ok(())
}

#[test]
fn update_dividend() -> anyhow::Result<()> {
    let conn = test_db()?;

    let mut div = add_dividend(&conn, sample_dividend())?;

    div.amount = dec!(250.0);
    div.taxes = dec!(20.0);

    edit_dividend(&conn, div)?;

    let dividends = load_all_dividends_from_db(&conn)?;

    assert_eq!(dividends.len(), 1);
    assert_eq!(dividends[0].amount, dec!(250.0));
    assert_eq!(dividends[0].taxes, dec!(20.0));

    Ok(())
}

#[test]
fn delete_dividend_test() -> anyhow::Result<()> {
    let conn = test_db()?;

    let tx = add_dividend(&conn, sample_dividend())?;

    delete_dividend(&conn, tx.id.unwrap())?;

    let dividends = load_all_dividends_from_db(&conn)?;

    assert!(dividends.is_empty());

    Ok(())
}
