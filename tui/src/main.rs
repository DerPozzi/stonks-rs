use anyhow::Result;
use chrono::NaiveDate;
use rust_decimal_macros::dec;
use serde::Deserialize;

use config::{Config, File};
use stonks_rs::service::service::init_db;

#[derive(Debug, Deserialize, Default)]
struct Settings {
    app: AppConfig,
}
#[derive(Debug, Deserialize, Default)]
struct AppConfig {
    theme: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let home_path = dirs::home_dir().expect("Could not find home directory of current user.");
    let config_path = home_path
        .join(".config")
        .join("stonks-rs")
        .join("config.toml");
    // create_if_no_cfg(config_path.clone())?;
    // let builder = Config::builder()
    //     .add_source(File::with_name(config_path.to_str().unwrap()))
    //     .build()?;
    //
    // let settings = builder.try_deserialize::<Settings>();
    // println!("{:#?}", settings);
    let conn = init_db(home_path)?;
    let test_transactions = vec![
        stonks_rs::types::Transaction {
            id: None,
            ticker: String::from("LMT"),
            transaction_type: stonks_rs::types::TransactionType::Buy,
            trade_date: NaiveDate::from_ymd_opt(2026, 06, 27).unwrap(),
            quantity: dec!(0.12044169),
            price: dec!(519.17),
            currency: stonks_rs::types::Currency::EUR,
            fees: dec!(0.08),
        },
        // stonks_rs::types::Transaction {
        //     id: None,
        //     ticker: String::from("PLTR"),
        //     transaction_type: stonks_rs::types::TransactionType::Sell,
        //     trade_date: NaiveDate::from_ymd_opt(2026, 08, 10).unwrap(),
        //     quantity: dec!(0.30699589),
        //     price: dec!(169.74),
        //     currency: stonks_rs::types::Currency::EUR,
        //     fees: dec!(0.07),
        // },
    ];
    let current_value = stonks_rs::service::service::get_market_value(
        &test_transactions,
        "LMT",
        stonks_rs::types::Currency::EUR,
    )
    .await?;
    println!("Executed transactions: {:#?} ", test_transactions);
    println!("Current value in EUR: {}", current_value);
    // println!("Current value in EUR: {}", eur_value);

    Ok(())
}
