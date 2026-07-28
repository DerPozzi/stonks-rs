use anyhow::Result;
use rust_decimal::Decimal;
use yahoo_finance_api::{self as yahoo, YResponse};

pub enum TimeFrame {
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

pub async fn get_current_asset_price(asset_ticker: &str) -> Result<Decimal> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.get_latest_quotes(asset_ticker, "1d").await?;
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
