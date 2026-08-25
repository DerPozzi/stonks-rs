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

#[allow(dead_code)]
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

    if quotes.len() < 2 {
        return Ok(rust_decimal_macros::dec!(0));
    }

    let previous = &quotes[quotes.len() - 2];
    let current = quotes.last().expect("quotes.len() checked");
    if previous.close == 0.0 {
        return Ok(rust_decimal_macros::dec!(0));
    }

    let change_percent_f = (current.close - previous.close) / previous.close;
    let Some(change_percent) = Decimal::from_f64_retain(change_percent_f) else {
        return Ok(rust_decimal_macros::dec!(0));
    };
    Ok(change_percent * rust_decimal_macros::dec!(100))
}
