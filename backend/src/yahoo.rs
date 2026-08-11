use anyhow::Result;
use anyhow::anyhow;
use rust_decimal::Decimal;
use yahoo_finance_api::{self as yahoo, YResponse};

use crate::types::Currency;

pub enum TimeFrame {
    OneMinute,
    OneHour,
    OneDay,
    OneWeek,
    OneMonth,
    ThreeMonth,
    SixMonth,
    YearToDate,
    OneYear,
    FiveYear,
    Max,
}

impl ToString for TimeFrame {
    fn to_string(&self) -> String {
        match self {
            TimeFrame::OneMinute => String::from("1m"),
            TimeFrame::OneHour => String::from("1h"),
            TimeFrame::OneDay => String::from("1d"),
            TimeFrame::OneWeek => String::from("1w"),
            TimeFrame::OneMonth => String::from("1m"),
            TimeFrame::ThreeMonth => String::from("3m"),
            TimeFrame::SixMonth => String::from("6m"),
            TimeFrame::YearToDate => String::from("ytd"),
            TimeFrame::OneYear => String::from("1y"),
            TimeFrame::FiveYear => String::from("5y"),
            TimeFrame::Max => String::from("max"),
        }
    }
}

pub async fn get_current_asset_price(asset_ticker: &str) -> Result<Decimal> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider
        .get_quote_range(
            asset_ticker,
            &TimeFrame::OneMinute.to_string(),
            &TimeFrame::OneDay.to_string(),
        )
        .await?;
    let quote = response.last_quote().unwrap();

    let close = Decimal::try_from(quote.close).unwrap();

    Ok(close)
}

pub async fn get_asset_data(asset_ticker: String, _time_frame: TimeFrame) -> Result<YResponse> {
    let provider = yahoo::YahooConnector::new()?;

    let response = provider
        .get_latest_quotes(asset_ticker.as_str(), "1d")
        .await?;

    println!("{:#?}", response);

    Ok(response)
}

pub async fn get_exchange_rate(current: Currency, target: Currency) -> Result<Decimal> {
    let pair = format!("{}{}=X", current.to_string(), target.to_string());
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.get_latest_quotes(&pair, "1d").await?;

    let close = response.last_quote()?.close;
    Ok(Decimal::try_from(close)?)
}

pub async fn get_asset_currency(ticker: &str) -> Result<Currency> {
    let provider = yahoo::YahooConnector::new()?;

    // Should be this but seems to be broken
    // let response = provider.get_ticker_info(ticker).await?;

    let response = provider.get_latest_quotes(ticker, "1d").await?;

    let currency = &response.chart.result.as_ref().unwrap()[0]
        .meta
        .currency
        .as_ref()
        .unwrap()
        .clone();

    match currency.to_uppercase().as_str() {
        "USD" => Ok(Currency::USD),
        "EUR" => Ok(Currency::EUR),
        _ => Err(anyhow!("Unknown currency")),
    }
}
