use std::path::PathBuf;

use anyhow::Result;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use stonks_rs::{
    service::{helpers::get_all_transactions, service::init_db},
    types::{Connection, Transaction},
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Serialize, Deserialize, Default)]
struct Settings {
    app: AppConfig,
}
#[derive(Debug, Serialize, Deserialize, Default)]
struct AppConfig {
    theme: String,
}

fn load_config(path: PathBuf) -> Result<Settings> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        let defaults = Settings::default();

        println!("{:?}", defaults);

        let content =
            toml::to_string_pretty(&defaults).expect("Failed to serialize default config");

        println!("{}", content);

        std::fs::write(&path, content)
            .expect(format!("Failed to write config to {}", path.display()).as_str());
    }

    let config = Config::builder().add_source(File::from(path)).build()?;

    let settings = config.try_deserialize()?;
    Ok(settings)
}

fn init() -> Result<(Connection, Vec<Transaction>, Settings)> {
    let home_path = dirs::home_dir().expect("Could not find home directory of current user.");
    let config_path = home_path
        .join(".config")
        .join("stonks-rs")
        .join("config.toml");
    let settings = load_config(config_path)?;
    let conn = init_db(home_path)?;

    let transactions = get_all_transactions(&conn)?;

    Ok((conn, transactions, settings))
}

#[derive(Debug, PartialEq, EnumIter)]
pub enum Page {
    Dashboard,
    Overview,
    Transactions,
    Dividends,
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Dashboard => write!(f, "Dashboard"),
            Page::Overview => write!(f, "Overview"),
            Page::Transactions => write!(f, "Transactions"),
            Page::Dividends => write!(f, "Dividends"),
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::Dashboard
    }
}

#[derive(Debug)]
pub enum CurrentAction {
    None,
    Add,
    Edit(u64),
    Delete(u64),
}

impl Default for CurrentAction {
    fn default() -> Self {
        CurrentAction::None
    }
}

#[derive(Debug)]
pub struct App {
    pub transactions: Vec<Transaction>,
    pub db_connection: Connection,
    pub settings: Settings,
    pub exit: bool,
    pub current_page: Page,
    pub current_action: CurrentAction,
}

impl Default for App {
    fn default() -> Self {
        let (db_connection, transactions, settings) = init().expect("Failed to init app");

        Self {
            transactions,
            db_connection,
            settings,
            exit: false,
            current_page: Page::default(),
            current_action: CurrentAction::default(),
        }
    }
}

impl App {
    fn new() -> Self {
        Self::default()
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.exit = true;
    }

    pub fn next_page(&mut self) {
        self.current_page = match self.current_page {
            Page::Dashboard => Page::Overview,
            Page::Overview => Page::Transactions,
            Page::Transactions => Page::Dividends,
            Page::Dividends => Page::Dashboard,
        }
    }
    pub fn previous_page(&mut self) {
        self.current_page = match self.current_page {
            Page::Dashboard => Page::Dividends,
            Page::Overview => Page::Dashboard,
            Page::Transactions => Page::Overview,
            Page::Dividends => Page::Transactions,
        }
    }
}
