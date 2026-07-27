use anyhow::Result;
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

pub async fn get_asset_data(asset_ticker: String, _time_frame: TimeFrame) -> Result<YResponse> {
    let provider = yahoo::YahooConnector::new()?;

    let response = provider
        .get_latest_quotes(asset_ticker.as_str(), "1d")
        .await?;

    println!("{:#?}", response);

    Ok(response)
}
