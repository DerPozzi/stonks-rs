use chrono::NaiveDate;
use rusqlite::{
    ToSql,
    types::{FromSql, ToSqlOutput},
};
use rust_decimal::Decimal;

pub type Connection = rusqlite::Connection;

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
/// Represents a completed trade of an asset.
///
/// A transaction stores the price and monetary values in the currency
/// specified by [`Transaction::currency`]. This currency does not
/// necessarily have to be the asset's native/quote currency.
///
/// For example, an asset may normally be quoted in USD while a broker
/// executes and settles the transaction in EUR. In that case, `price`
/// represents the price per share in EUR and `currency` is `EUR`.
pub struct Transaction {
    pub id: Option<i64>,

    /// Ticker symbol identifying the traded asset.
    ///
    /// The ticker must be interpreted together with the corresponding
    /// exchange to uniquely identify the asset.
    pub ticker: String,

    /// Type of transaction, e.g. buy or sell.
    pub transaction_type: TransactionType,

    /// Date on which the trade was executed.
    pub trade_date: NaiveDate,

    /// Number of shares traded.
    pub quantity: Decimal,

    /// Price paid or received per share.
    ///
    /// The price is denominated in [`Transaction::currency`].
    /// Consequently, `quantity * price` represents the transaction
    /// value in this currency.
    ///
    /// The currency of the price does not necessarily have to match
    /// the asset's native or quote currency. For example, a USD-denominated
    /// asset can have a transaction price in EUR if the broker settles
    /// the trade in EUR.
    pub price: Decimal,

    /// Currency in which the transaction price and transaction value
    /// are denominated.
    ///
    /// This is the currency used for the monetary values stored in this
    /// transaction and does not necessarily represent the asset's
    /// native/quote currency.
    pub currency: Currency,

    /// Fees charged for the transaction.
    ///
    /// Fees are assumed to be denominated in [`Transaction::currency`].
    pub fees: Decimal,
}

#[derive(Debug, Clone)]
/// Represents a dividend payment received for an asset.
///
/// The dividend amount and associated taxes are denominated in the
/// currency specified by [`Dividend::currency`].
pub struct Dividend {
    pub id: Option<i64>,

    /// Ticker symbol identifying the asset that paid the dividend.
    ///
    /// The ticker must be interpreted together with the corresponding
    /// exchange to uniquely identify the asset.
    pub ticker: String,

    /// Date on which the dividend was paid.
    pub payment_date: NaiveDate,

    /// Gross dividend amount paid by the asset.
    ///
    /// The amount is denominated in [`Dividend::currency`].
    pub amount: Decimal,

    /// Taxes withheld from the dividend.
    ///
    /// The taxes are denominated in [`Dividend::currency`].
    pub taxes: Decimal,

    /// Currency in which the dividend amount and withheld taxes
    /// are denominated.
    ///
    /// This is the currency of the actual dividend payment and does
    /// not necessarily have to match the asset's native/quote currency.
    pub currency: Currency,
}
