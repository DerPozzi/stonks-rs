use anyhow::Result;
use anyhow::anyhow;
use rust_decimal::Decimal;
use yahoo_finance_api::Quote;
use yahoo_finance_api::{self as yahoo, YResponse};

use crate::types::Currency;
use crate::types::TimeFrame;

pub async fn get_current_asset_price(asset_ticker: &str) -> Result<Decimal> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider
        .get_quote_range(
            asset_ticker,
            &TimeFrame::OneMinute.to_string(),
            &TimeFrame::OneDay.to_string(),
        )
        .await?;

    let quote: Quote = match response.last_quote() {
        Ok(q) => q,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Problem looking at quotes for ticker {}: {}",
                asset_ticker,
                e
            ));
        }
    };

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
    let pair = format!("{}{}=X", current, target);
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

pub async fn get_ticker_name(t: &str) -> Result<String> {
    let provider = yahoo_finance_api::YahooConnector::new()?;

    let response = provider.get_latest_quotes(t, "1d").await?;

    let long_name = response.chart.result.as_ref().unwrap()[0]
        .meta
        .long_name
        .as_ref()
        .unwrap_or(&"".to_string())
        .clone();

    Ok(long_name)
}

pub async fn get_todays_change(t: &str) -> Result<Decimal> {
    let provider = yahoo_finance_api::YahooConnector::new()?;

    let response = provider.get_quote_range(t, "1d", "1mo").await?;

    let quotes = response.quotes()?;

    let previous_day = quotes.get(quotes.len() - 2);
    let current = quotes.last();

    if let (Some(previous), Some(current)) = (previous_day, current) {
        let change = current.close - previous.close;
        let change_percent = Decimal::from_f64_retain(change / previous.close).unwrap()
            * rust_decimal_macros::dec!(100);
        return Ok(change_percent);
    }

    Ok(rust_decimal_macros::dec!(0))
}
