use crate::types::{Transaction, TransactionType};
use anyhow::Result;

pub fn calc_avg_price(transactions: &Vec<Transaction>, ticker: &str) -> f32 {
    let mut total_quantity = 0.0;
    let mut total_cost = 0.0;

    for tx in transactions.iter().filter(|t| t.ticker == ticker) {
        match tx.transaction_type {
            TransactionType::Buy => {
                total_quantity += tx.quantity;
                total_cost += tx.quantity * tx.price;
            }

            TransactionType::Sell => {
                total_quantity -= tx.quantity;
            }
        }
    }

    if total_quantity == 0.0 {
        return 0.0;
    }

    total_cost / total_quantity
}
