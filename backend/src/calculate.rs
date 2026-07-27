use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::types::{Transaction, TransactionType};

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
