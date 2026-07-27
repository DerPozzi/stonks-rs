use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use anyhow::Result;

use crate::{
    helpers::calc_shares,
    types::{Transaction, TransactionType},
    yahoo::get_current_asset_price,
};

pub fn calc_cost_price(transactions: &Vec<Transaction>, ticker: &str) -> Decimal {
    let mut total_quantity = dec!(0.0);
    let mut total_cost = dec!(0.0);

    // Einstandspreis = Menge(Stückzahl x Kurspreis + Gebühren) / Menge(Stückzahl)

    for tx in transactions.iter().filter(|t| t.ticker == ticker) {
        match tx.transaction_type {
            TransactionType::Buy => {
                total_quantity += tx.quantity;
                total_cost += tx.quantity * tx.price + tx.fees;
            }

            TransactionType::Sell => {
                total_quantity -= tx.quantity;
            }
        }
    }

    if total_quantity == dec!(0.0) {
        return dec!(0.0);
    }

    total_cost / total_quantity
}

pub async fn calc_market_value(transactions: &Vec<Transaction>, ticker: &str) -> Result<Decimal> {
    let current_price = get_current_asset_price(ticker).await?;
    // let cost_price = calc_cost_price(transactions, ticker);
    let num_shares = calc_shares(transactions, ticker);
    // let investment = cost_price * num_shares;

    // let gain = (current_price - cost_price) * num_shares;

    let market_value = current_price * num_shares;

    Ok(Decimal::try_from(market_value)?)
}
