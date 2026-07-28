use chrono::NaiveDate;
use rusqlite::{
    ToSql,
    types::{FromSql, ToSqlOutput},
};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
pub enum Currency {
    USD,
    EUR,
}

#[derive(Debug, Clone, Copy)]
pub enum TransactionType {
    Buy,
    Sell,
}

impl ToSql for TransactionType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let value = match self {
            TransactionType::Buy => "buy",
            TransactionType::Sell => "sell",
        };
        Ok(ToSqlOutput::from(value))
    }
}

impl ToSql for Currency {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let value = match self {
            Currency::USD => "usd",
            Currency::EUR => "eur",
        };
        Ok(ToSqlOutput::from(value))
    }
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

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Option<i64>,
    pub ticker: String,
    pub transaction_type: TransactionType,
    pub trade_date: NaiveDate,
    pub quantity: Decimal,
    pub price: Decimal,
    pub currency: Currency,
    pub fees: Decimal,
}

#[derive(Debug, Clone)]
pub struct Dividend {
    pub id: Option<i64>,
    pub ticker: String,
    pub payment_date: NaiveDate,
    pub amount: Decimal,
    pub taxes: Decimal,
    pub currency: Currency,
}
