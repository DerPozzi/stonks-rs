use std::collections::HashSet;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use anyhow::Result;

use crate::{
    service::service::calc_shares,
    types::{Currency, Dividend, Transaction, TransactionType},
    yahoo::{get_asset_currency, get_current_asset_price, get_exchange_rate},
};

/*
 *
 *  POSITIONS
 *
 */

pub fn calc_avg_cost(transactions: &[Transaction], ticker: &str) -> Decimal {
    let (buy, _) = calc_shares(transactions, ticker);
    let mut total_cost = dec!(0.0);

    // Einstandspreis = Menge(Stückzahl x Kurspreis + Gebühren) / Menge(Stückzahl)

    for tx in transactions.iter().filter(|t| t.ticker == ticker) {
        if let TransactionType::Buy = tx.transaction_type {
            total_cost += tx.quantity * tx.price + tx.fees;
        }
    }

    if buy == dec!(0.0) {
        return dec!(0.0);
    }

    total_cost / buy
}

pub fn calc_cost_basis(transactions: &[Transaction], ticker: &str) -> Result<Decimal> {
    let avg_cost = calc_avg_cost(transactions, ticker);
    let (num_shares, _) = calc_shares(transactions, ticker);

    Ok(avg_cost * num_shares)
}

pub fn calc_unrealized_gain(
    transactions: &[Transaction],
    ticker: &str,
    current_price: Decimal,
) -> Result<Decimal> {
    let market_value = calc_market_value(transactions, ticker, current_price)?;
    let cost_basis = calc_avg_cost(transactions, ticker);

    Ok(market_value - cost_basis)
}

pub fn calc_unrealized_gain_prec(
    transactions: &[Transaction],
    ticker: &str,
    current_price: Decimal,
) -> Result<Decimal> {
    let unrealized_gain = calc_unrealized_gain(transactions, ticker, current_price)?;
    let cost_basis = calc_avg_cost(transactions, ticker);

    Ok(unrealized_gain / cost_basis * dec!(100))
}

pub fn calc_market_value(
    transactions: &[Transaction],
    ticker: &str,
    current_price: Decimal,
) -> Result<Decimal> {
    let (buy, sell) = calc_shares(transactions, ticker);
    let market_value = current_price * (buy - sell);

    Ok(Decimal::try_from(market_value)?)
}

/*
 *
 * SELLS
 *
 */

pub fn calc_sale_value(transactions: &[Transaction], ticker: &str) -> Result<Decimal> {
    Ok(transactions
        .iter()
        .filter(|t| t.ticker == ticker)
        .map(|t| match t.transaction_type {
            TransactionType::Sell => t.quantity * t.price - t.fees,
            _ => dec!(0),
        })
        .sum())
}

pub fn calc_realized_gains(transactions: &[Transaction], ticker: &str) -> Result<Decimal> {
    let (_, sold_quantity) = calc_shares(transactions, ticker);
    let sale_value = calc_sale_value(transactions, ticker)?;
    let avg_cost = calc_avg_cost(transactions, ticker);

    Ok(sale_value - avg_cost * sold_quantity)
}

/*
 *
 *  PORTFOLIO
 *
 */

pub async fn calc_portfolio_value(
    transactions: &[Transaction],
    target_currency: Option<Currency>,
) -> Result<Decimal> {
    let tickers: HashSet<String> = transactions.iter().map(|t| t.ticker.clone()).collect();

    let mut total_portfolio_value = dec!(0);

    for ticker in tickers {
        let current_price = get_current_asset_price(&ticker).await?;
        let asset_currency = get_asset_currency(&ticker).await?;

        let mut market_value = calc_market_value(transactions, &ticker, current_price)?;

        if let Some(target_currency) = target_currency
            && target_currency != asset_currency
        {
            let exchange_rate = get_exchange_rate(asset_currency, target_currency).await?;
            market_value = market_value * exchange_rate;
        }

        total_portfolio_value += market_value;
    }

    Ok(total_portfolio_value)
}

/// Portfolio weight of given asset
///
/// "How many percent of my total portfolio is asset x?"
pub async fn calc_portfolio_weight(transactions: &[Transaction], ticker: &str) -> Result<Decimal> {
    let current_asset_price = get_current_asset_price(ticker).await?;
    let market_value = calc_market_value(transactions, ticker, current_asset_price)?;
    let portfolio_value = calc_portfolio_value(transactions, Some(Currency::EUR)).await?;

    let portfolio_weight = market_value / portfolio_value * dec!(100);
    Ok(portfolio_weight)
}

/*
 *
 *  DIVIDENDS
 *
 */

pub fn calc_gross_dividends(dividends: &[Dividend], ticker: &str) -> Decimal {
    dividends
        .iter()
        .filter(|d| d.ticker == ticker)
        .map(|d| d.amount)
        .sum()
}

pub fn calc_net_dividends(dividends: &[Dividend], ticker: &str) -> Decimal {
    dividends
        .iter()
        .filter(|d| d.ticker == ticker)
        .map(|d| d.amount - d.taxes)
        .sum()
}

/*
 *
 *  TOTAL
 *
 */

pub async fn calc_total_return(
    transactions: &[Transaction],
    dividends: &[Dividend],
    ticker: &str,
) -> Result<Decimal> {
    let current_price = get_current_asset_price(ticker).await?;
    let unrealized_gains = calc_unrealized_gain(transactions, ticker, current_price)?;
    let realized_gains = calc_realized_gains(transactions, ticker)?;
    let net_dividends = calc_net_dividends(dividends, ticker);

    let total_return = unrealized_gains + realized_gains + net_dividends;
    Ok(total_return)
}
