use std::path::PathBuf;

use anyhow::Result;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use stonks_rs::{
    service::{helpers::get_all_transactions, service::init_db},
    types::{Connection, Transaction},
};
use strum::EnumIter;

use crate::theme::{Theme, load_theme};

#[derive(Debug, Serialize, Deserialize, Default)]
struct Settings {
    app: AppConfig,
}
#[derive(Debug, Serialize, Deserialize, Default)]
struct AppConfig {
    theme: Option<String>,
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

fn init() -> Result<(Connection, Vec<Transaction>, Settings, Theme)> {
    let home_path = dirs::home_dir().expect("Could not find home directory of current user.");
    let config_path = home_path.join(".config").join("stonks-rs");

    let settings = load_config(config_path.join("config.toml"))?;
    let conn = init_db(home_path)?;

    let theme = load_theme(&config_path, settings.app.theme.as_deref())?;

    let transactions = get_all_transactions(&conn)?;

    Ok((conn, transactions, settings, theme))
}

#[derive(Debug, PartialEq, EnumIter, Default)]
pub enum Page {
    #[default]
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

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    None,
    Add,
    Edit(u64),
    Delete(u64),
}

#[derive(Debug)]
pub struct App {
    pub transactions: Vec<Transaction>,
    pub db_connection: Connection,
    pub settings: Settings,
    pub should_quit: bool,
    pub current_page: Page,
    pub current_action: Action,
    pub theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        let (db_connection, transactions, settings, theme) = init().expect("Failed to init app");

        Self {
            transactions,
            db_connection,
            settings,
            should_quit: false,
            current_page: Page::default(),
            current_action: Action::default(),
            theme,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_page(&mut self) {
        self.current_action = Action::default();
        self.current_page = match self.current_page {
            Page::Dashboard => Page::Overview,
            Page::Overview => Page::Transactions,
            Page::Transactions => Page::Dividends,
            Page::Dividends => Page::Dashboard,
        }
    }
    pub fn previous_page(&mut self) {
        self.current_action = Action::default();
        self.current_page = match self.current_page {
            Page::Dashboard => Page::Dividends,
            Page::Overview => Page::Dashboard,
            Page::Transactions => Page::Overview,
            Page::Dividends => Page::Transactions,
        }
    }
}
