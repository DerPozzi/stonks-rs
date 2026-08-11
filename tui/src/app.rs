use std::path::PathBuf;

use ratatui::crossterm::event::MouseEvent;

use anyhow::Result;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use stonks_rs::{
    service::{helpers::get_all_transactions, service::init_db},
    types::{Connection, Transaction},
};
use strum::EnumIter;

use crate::{
    pages::transactions::{TransactionPage, TransactionUiAreas},
    theme::{Theme, load_theme},
};

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

#[derive(Debug, PartialEq, EnumIter, Default, Clone)]
pub enum Page {
    #[default]
    Dashboard,
    Overview,
    Transactions,
    Dividends,
    #[strum(disabled)]
    Settings(Box<Page>),
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Dashboard => write!(f, "Dashboard"),
            Page::Overview => write!(f, "Overview"),
            Page::Transactions => write!(f, "Transactions"),
            Page::Dividends => write!(f, "Dividends"),
            Page::Settings(_) => write!(f, "Settings"),
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
    Settings,
    Hotkeys,
}

#[derive(Debug, Default)]
pub struct UiAreas {
    pub transaction_page: Option<TransactionUiAreas>,
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

    pub ui_areas: UiAreas,

    pub transaction_page: TransactionPage,

    pub current_page_focused: bool,
    pub input_text: bool,
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
            ui_areas: UiAreas::default(),
            transaction_page: TransactionPage::default(),
            current_page_focused: false,
            input_text: false,
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
            Page::Transactions => {
                self.transaction_page = TransactionPage::default();
                Page::Dividends
            }
            Page::Dividends => Page::Dashboard,
            _ => Page::Dashboard,
        }
    }
    pub fn previous_page(&mut self) {
        self.current_action = Action::default();
        self.current_page = match self.current_page {
            Page::Dashboard => Page::Dividends,
            Page::Overview => Page::Dashboard,
            Page::Transactions => {
                self.transaction_page = TransactionPage::default();
                Page::Overview
            }
            Page::Dividends => Page::Transactions,
            _ => Page::Dashboard,
        }
    }

    pub fn open_settings(&mut self) {
        let current = self.current_page.clone();
        self.current_page = Page::Settings(Box::new(current));
    }
    pub fn toggle_hotkeys(&mut self) {
        self.current_action = Action::Hotkeys;
    }

    pub fn mouse_press(&mut self, event: MouseEvent) {
        let _x = event.column;
        let _y = event.row;

        // ...
    }

    pub fn focus_page(&mut self) {
        self.current_page_focused = true;
    }

    pub fn unfocus_page(&mut self) {
        self.current_page_focused = false;
    }
}
