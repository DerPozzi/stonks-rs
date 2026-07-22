use crate::{
    backend::yahoo::{TimeFrame, get_asset_data},
    init::{init_db, load_from_db},
};
use anyhow::Result;

mod backend;
mod init;

struct Config {
    app_files: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_files: format!(
                "{}/.stonks-rs/",
                dirs::home_dir()
                    .expect("Could not find home directory of current user.")
                    .display()
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let conf = Config::default();
    let conn = init_db(&conf.app_files)?;
    let transactions = load_from_db(&conn)?;
    println!("{:#?}", transactions);
    let _ = get_asset_data("PLTR".to_string(), TimeFrame::OneDay).await?;
    Ok(())
}
