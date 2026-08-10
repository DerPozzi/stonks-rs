use chrono::NaiveDate;
use rusqlite::{
    ToSql,
    types::{FromSql, ToSqlOutput},
};
use rust_decimal::Decimal;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Currency {
    USD,
    EUR,
}

impl ToString for Currency {
    fn to_string(&self) -> String {
        match self {
            Currency::USD => String::from("USD"),
            Currency::EUR => String::from("EUR"),
        }
    }
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
/// Transaction type holding all informtaion on past transactions
pub struct Transaction {
    pub id: Option<i64>,
    /// Ticker of the given asset, make sure to use the right ticker from the right exchange
    pub ticker: String,
    /// Can either be buy or sell
    pub transaction_type: TransactionType,
    pub trade_date: NaiveDate,
    /// Amount of shares traded in transaction
    pub quantity: Decimal,
    /// Price per share
    pub price: Decimal,
    /// Currency the transaction was made in, conversion will be done automatically if the assets
    /// currency differs
    pub currency: Currency,
    pub fees: Decimal,
}

#[derive(Debug, Clone)]
/// Dividend type holding all informtaion on paid out dividends
pub struct Dividend {
    pub id: Option<i64>,
    /// Ticker of the given asset, make sure to use the right ticker from the right exchange
    pub ticker: String,
    pub payment_date: NaiveDate,
    /// Amount of paid out
    pub amount: Decimal,
    pub taxes: Decimal,
    /// Currency of the pay out, will be converted automatically
    pub currency: Currency,
}
