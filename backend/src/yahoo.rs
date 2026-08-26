use anyhow::Result;
use rust_decimal::Decimal;
use yahoo_finance_api::Quote;
use yahoo_finance_api::{self as yahoo};

use crate::types::TickerFinancialData;
use crate::types::TimeFrame;
use crate::types::{Currency, TickerMetaData};

pub async fn get_ticker_financial(asset_ticker: &str) -> Result<TickerFinancialData> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider
        .get_latest_quotes(asset_ticker, &TimeFrame::OneDay.to_string())
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

    let close = Decimal::try_from(quote.close)?;

    let meta = response.metadata()?;
    let currency = Currency::try_from(meta.currency.unwrap_or_default())?;

    Ok(TickerFinancialData {
        current_price: close,
        currency,
        ..Default::default()
    })
}

pub async fn get_exchange_rate(current: Currency, target: Currency) -> Result<Decimal> {
    let pair = format!("{}{}=X", current, target);
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.get_latest_quotes(&pair, "1d").await?;

    let close = response.last_quote()?.close;
    Ok(Decimal::try_from(close)?)
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

pub async fn get_ticker_meta(t: &str) -> Result<TickerMetaData> {
    let provider = yahoo_finance_api::YahooConnector::new()?;

    let response = provider.get_latest_quotes(t, "1d").await?;
    let meta = response.chart.result.as_ref().unwrap()[0].meta.clone();

    let long_name = meta.long_name;

    Ok(TickerMetaData {
        ticker: t.into(),
        long_name,
        ..Default::default()
    })
}
