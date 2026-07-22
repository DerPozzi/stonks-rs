use crate::{
    backend::{
        database::add_transaction,
        yahoo::{TimeFrame, get_asset_data},
    },
    init::{create_if_no_cfg, init_db, load_from_db},
    types::Transaction,
};
use anyhow::Result;
use chrono::NaiveDate;
use serde::Deserialize;

mod backend;
mod init;
mod types;

use config::{Config, File};

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
    create_if_no_cfg(config_path.clone())?;
    let builder = Config::builder()
        .add_source(File::with_name(config_path.to_str().unwrap()))
        .build()?;

    let settings = builder.try_deserialize::<Settings>();
    println!("{:#?}", settings);
    let conn = init_db(home_path)?;
    let test_transaction = Transaction {
        id: None,
        ticker: String::from("PLTR"),
        transaction_type: types::TransactionType::Buy,
        trade_date: NaiveDate::from_ymd_opt(2026, 06, 06).unwrap(),
        quantity: 1.32932221,
        price: 128.70,
        currency: types::Currency::USD,
    };
    let _ = add_transaction(&conn, test_transaction)?;
    let transactions = load_from_db(&conn)?;
    println!("{:#?}", transactions);
    let _ = get_asset_data("PLTR".to_string(), TimeFrame::OneDay).await?;
    Ok(())
}
