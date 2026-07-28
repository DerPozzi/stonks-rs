use crate::types::{Transaction, TransactionType};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Calculates the number of bought and sold shares and returns them as (bought, sold)
pub fn calc_shares(transactions: &[Transaction], ticker: &str) -> (Decimal, Decimal) {
    let mut buy = dec!(0);
    let mut sell = dec!(0);
    for tx in transactions.iter().filter(|t| t.ticker == ticker) {
        match tx.transaction_type {
            TransactionType::Buy => buy = buy + tx.quantity,
            TransactionType::Sell => sell = sell + tx.quantity,
        }
    }
    (buy, sell)
}
