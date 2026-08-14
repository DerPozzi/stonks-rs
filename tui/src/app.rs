use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use chrono::NaiveDate;
use ratatui::crossterm::event::MouseEvent;

use anyhow::Result;
use config::{Config, File};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use stonks_rs::{
    service::{helpers::get_all_transactions, service::init_db},
    types::{Connection, Currency, TickerData, Transaction},
};
use strum::EnumIter;
use tokio::sync::mpsc;

use crate::{
    pages::{
        self,
        add_transaction::CreateTransaction,
        transactions::{TransactionPage, TransactionUiAreas},
    },
    theme::{Theme, load_theme},
    update::{UpdateMessage, UpdateRequest},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    app: AppConfig,
    pub default: DefaultConfig,
}
#[derive(Debug, Serialize, Deserialize, Default)]
struct AppConfig {
    theme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DefaultConfig {
    pub currency: Currency,
    refresh_rate: Option<u64>,
}

fn load_config(path: PathBuf) -> Result<Settings> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        let defaults = Settings::default();

        let content =
            toml::to_string_pretty(&defaults).expect("Failed to serialize default config");

        println!("{}", content);

        std::fs::write(&path, content)
            .unwrap_or_else(|_| panic!("Failed to write config to {}", path.display()));
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
    #[strum(disabled)]
    AddTransaction,
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Dashboard => write!(f, "Dashboard"),
            Page::Overview => write!(f, "Overview"),
            Page::Transactions => write!(f, "Transactions"),
            Page::Dividends => write!(f, "Dividends"),
            Page::Settings(_) => write!(f, "Settings"),
            Page::AddTransaction => write!(f, "Add a Transaction"),
        }
    }
}

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    None,
    Edit(u64),
    Delete(u64),
    Hotkeys,
}

#[derive(Debug, Default)]
pub struct UiAreas {
    pub transaction_page: Option<TransactionUiAreas>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CurrentFocus {
    #[default]
    None,
    TransactionPage(pages::transactions::InputFocus),
    AddTransaction(pages::add_transaction::InputFocus),
}

#[derive(Debug, Default)]
pub struct Portfolio {
    pub value: Decimal,
    pub ticker_info: Vec<TickerData>,
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

    pub update_tx: Option<mpsc::UnboundedSender<UpdateRequest>>,
    pub update_rx: Option<mpsc::UnboundedReceiver<UpdateMessage>>,

    pub ui_areas: UiAreas,

    pub error: String,

    pub transaction_page: TransactionPage,

    pub current_page_focused: bool,
    pub input_text: bool,
    pub focused_field: CurrentFocus,

    pub create_transaction: CreateTransaction,

    pub portfolio: Portfolio,

    pub last_update: Option<Instant>,
}

impl Default for App {
    fn default() -> Self {
        let (db_connection, transactions, settings, theme) = init().expect("Failed to init app");

        tracing::info!("App defaults have been loaded");

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
            create_transaction: CreateTransaction::default(),
            focused_field: CurrentFocus::None,
            portfolio: Portfolio::default(),
            last_update: None,
            update_tx: None,
            update_rx: None,
            error: String::new(),
        }
    }
}

impl App {
    pub fn new(
        update_tx: mpsc::UnboundedSender<UpdateRequest>,
        update_rx: mpsc::UnboundedReceiver<UpdateMessage>,
    ) -> Self {
        Self {
            update_tx: Some(update_tx),
            update_rx: Some(update_rx),
            ..Default::default()
        }
    }
    /// Handles the tick event of the terminal.
    pub fn _tick(&self) {}

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

    pub fn _mouse_press(&mut self, event: MouseEvent) {
        let _x = event.column;
        let _y = event.row;

        // ...
    }

    pub fn add_transaction(&mut self) {
        self.current_page = Page::AddTransaction;
        self.current_page_focused = true;
    }

    pub fn save_new_transaction(&mut self) -> Result<()> {
        let tx = &self.create_transaction;

        let new_tx = Transaction {
            id: None,
            ticker: tx.ticker.clone(),
            transaction_type: tx.transaction_type,
            trade_date: NaiveDate::parse_from_str(&tx.trade_date_input, "%Y-%m-%d").unwrap(),
            quantity: Decimal::try_from(tx.quantity.parse::<f32>()?)?,
            price: Decimal::try_from(tx.price.parse::<f32>()?)?,
            currency: tx.currency,
            fees: Decimal::try_from(tx.fees.parse::<f32>()?)?,
        };

        match stonks_rs::service::helpers::add_transaction_to_list(
            &self.db_connection,
            &mut self.transactions,
            new_tx,
        ) {
            Ok(t) => tracing::info!("Added new transaction with id: {}", t.id.unwrap()),
            Err(e) => tracing::error!("An error occured, when creating a new transaction: {e}"),
        };

        self.current_page = Page::Transactions;
        self.focused_field = CurrentFocus::None;

        self.create_transaction = CreateTransaction::default();
        Ok(())
    }

    pub fn focus_page(&mut self) {
        self.current_page_focused = true;
    }

    pub fn unfocus_page(&mut self) {
        self.current_page_focused = false;
    }

    pub fn handle_layout_focus(&mut self, number: Option<usize>) {
        if !self.current_page_focused {
            return;
        }

        let Some(number) = number else {
            return;
        };

        self.focused_field = match self.current_page {
            Page::Transactions => {
                let Some(field) = pages::transactions::InputFocus::from_repr(number - 1) else {
                    return;
                };

                CurrentFocus::TransactionPage(field)
            }

            Page::AddTransaction => {
                let Some(field) = pages::add_transaction::InputFocus::from_repr(number - 1) else {
                    return;
                };

                CurrentFocus::AddTransaction(field)
            }

            _ => return,
        };
    }

    pub fn input_char(&mut self, c: char) {
        match self.focused_field {
            CurrentFocus::AddTransaction(field) => {
                pages::add_transaction::handle_input_char(self, field, c)
            }

            CurrentFocus::TransactionPage(field) => {
                pages::transactions::handle_input_char(self, field, c)
            }

            _ => {}
        }
    }
    pub fn input_backspace(&mut self) {
        match self.focused_field {
            CurrentFocus::AddTransaction(field) => {
                pages::add_transaction::handle_input_backspace(self, field)
            }

            CurrentFocus::TransactionPage(field) => {
                pages::transactions::handle_input_backspace(self, field)
            }
            _ => {}
        }
    }
    pub fn handle_selector_tab(&mut self) {
        match self.focused_field {
            CurrentFocus::TransactionPage(field) => {
                pages::transactions::handle_selector_tab(self, field)
            }
            CurrentFocus::AddTransaction(field) => {
                pages::add_transaction::handle_selector_tab(self, field)
            }

            _ => {}
        }
    }

    pub fn request_updates(&mut self) {
        self.last_update = Some(Instant::now());

        if let Some(tx) = &self.update_tx {
            let _ = tx.send(UpdateRequest::PortfolioValue(
                self.transactions.clone(),
                self.settings.default.currency,
            ));

            let _ = tx.send(UpdateRequest::AllTickers(
                self.transactions.clone(),
                self.settings.default.currency,
            ));
        }
    }

    pub fn process_updates(&mut self) {
        let mut messages = Vec::new();

        if let Some(rx) = self.update_rx.as_mut() {
            while let Ok(message) = rx.try_recv() {
                messages.push(message);
            }
        }

        for message in messages {
            match message {
                UpdateMessage::PortfolioValue(value) => {
                    self.portfolio.value = value;
                }

                UpdateMessage::Error(error) => {
                    tracing::error!("Background update returned an error: {error}");
                }

                UpdateMessage::AllTickers(ti) => self.portfolio.ticker_info = ti,

                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        if let Some(last_update) = self.last_update
            && last_update.elapsed()
                < Duration::from_secs(self.settings.default.refresh_rate.unwrap_or(30))
        {
            return;
        }

        match stonks_rs::service::helpers::get_all_transactions(&self.db_connection) {
            Ok(tx) => self.transactions = tx,
            Err(e) => tracing::error!("Failed to update transactions: {e}"),
        }
        self.request_updates();
        self.process_updates();
    }
}
