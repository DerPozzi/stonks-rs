use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    time::{Duration, Instant},
};

use chrono::NaiveDate;
use ratatui::crossterm::event::MouseEvent;

use anyhow::Result;
use config::{Config, File};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use stonks_rs::{
    service::{helpers::get_all_transactions, stonks::init_db},
    types::{Connection, Currency, CycleEnum, TickerData, Transaction},
};
use tokio::sync::mpsc;

use crate::{
    components::inputs::CurrentFocus,
    pages::{
        self, Page, PageState,
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

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    None,
    _Edit(u64),
    _Delete(u64),
    Hotkeys,
}

#[derive(Debug, Default)]
pub struct UiAreas {
    pub transaction_page: Option<TransactionUiAreas>,
}

#[derive(Debug, Default)]
pub struct Portfolio {
    pub value: Option<Decimal>,
    pub ticker_info: HashMap<String, TickerData>,
}

#[derive(Debug)]
pub struct App {
    pub transactions: Vec<Transaction>,
    pub db_connection: Connection,
    pub settings: Settings,
    pub should_quit: bool,
    pub current_page: Page,
    pub page_state: PageState,
    pub current_action: Action,
    pub theme: Theme,

    pub update_tx: Option<mpsc::UnboundedSender<UpdateRequest>>,
    pub update_rx: Option<mpsc::UnboundedReceiver<UpdateMessage>>,

    pub ui_areas: UiAreas,

    pub error: String,

    pub transaction_page: TransactionPage,

    pub current_page_focused: bool,
    pub input_mode: bool,
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
            page_state: PageState::None,
            current_action: Action::default(),
            theme,
            ui_areas: UiAreas::default(),
            transaction_page: TransactionPage::default(),
            current_page_focused: false,
            input_mode: false,
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
            Page::Dashboard => {
                self.page_state = PageState::Overview(pages::overview::OverviewState::default());
                Page::Overview
            }
            Page::Overview => {
                self.page_state =
                    PageState::Transaction(pages::transactions::TransactionState::default());
                Page::Transactions
            }
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
                self.page_state = PageState::Overview(pages::overview::OverviewState::default());
                Page::Overview
            }
            Page::Dividends => {
                self.page_state =
                    PageState::Transaction(pages::transactions::TransactionState::default());
                Page::Transactions
            }
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
        self.focused_field =
            CurrentFocus::AddTransaction(pages::add_transaction::InputFocus::default());
        self.current_page_focused = true;
    }

    pub fn save_new_transaction(&mut self) -> Result<()> {
        let trans = &self.create_transaction;

        let new_tx = Transaction {
            id: None,
            ticker: trans.ticker.clone(),
            transaction_type: trans.transaction_type,
            trade_date: NaiveDate::parse_from_str(&trans.trade_date_input, "%Y-%m-%d").unwrap(),
            quantity: Decimal::try_from(trans.quantity.parse::<f32>()?)?,
            price: Decimal::try_from(trans.price.parse::<f32>()?)?,
            currency: trans.currency,
            fees: Decimal::try_from(trans.fees.parse::<f32>()?)?,
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
        self.focused_field = match &self.current_page {
            Page::Dashboard => todo!(),
            Page::Overview => todo!(),
            Page::Transactions => {
                CurrentFocus::TransactionPage(pages::transactions::InputFocus::default())
            }
            Page::Dividends => todo!(),
            Page::Settings(_page) => todo!(),
            Page::AddTransaction => {
                CurrentFocus::AddTransaction(pages::add_transaction::InputFocus::default())
            }
        };
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
    pub fn handle_tab(&mut self) {
        if self.input_mode {
            match self.focused_field {
                CurrentFocus::TransactionPage(input_focus) => match input_focus {
                    pages::transactions::InputFocus::TransactionType => {
                        self.transaction_page.filters.transaction_type =
                            self.transaction_page.filters.transaction_type.next();
                    }

                    pages::transactions::InputFocus::Period => {
                        self.transaction_page.filters.period =
                            self.transaction_page.filters.period.next();
                    }
                    _ => {}
                },
                CurrentFocus::AddTransaction(input_focus) => match input_focus {
                    pages::add_transaction::InputFocus::TransactionType => {
                        self.create_transaction.transaction_type =
                            self.create_transaction.transaction_type.next();
                    }

                    pages::add_transaction::InputFocus::Currency => {
                        self.create_transaction.currency = self.create_transaction.currency.next();
                    }

                    _ => {}
                },
                _ => {}
            }
            return;
        }
        self.focused_field = self.focused_field.next()
        // cycle if it's a selector
    }

    pub fn handle_shift_tab(&mut self) {
        if self.input_mode {
            match self.focused_field {
                CurrentFocus::TransactionPage(input_focus) => match input_focus {
                    pages::transactions::InputFocus::TransactionType => {
                        self.transaction_page.filters.transaction_type =
                            self.transaction_page.filters.transaction_type.previous();
                    }

                    pages::transactions::InputFocus::Period => {
                        self.transaction_page.filters.period =
                            self.transaction_page.filters.period.previous();
                    }
                    _ => {}
                },
                CurrentFocus::AddTransaction(input_focus) => match input_focus {
                    pages::add_transaction::InputFocus::TransactionType => {
                        self.create_transaction.transaction_type =
                            self.create_transaction.transaction_type.previous();
                    }

                    pages::add_transaction::InputFocus::Currency => {
                        self.create_transaction.currency =
                            self.create_transaction.currency.previous();
                    }

                    _ => {}
                },
                _ => {}
            }
            return;
        }
        self.focused_field = self.focused_field.previous()
    }

    pub fn request_updates(&mut self) {
        self.last_update = Some(Instant::now());
        let tickers: HashSet<String> = self.transactions.iter().map(|t| t.ticker.clone()).collect();

        if let Some(tx) = &self.update_tx {
            let _ = tx.send(UpdateRequest::PortfolioValue(
                self.transactions.clone(),
                self.settings.default.currency,
            ));

            for t in tickers.iter() {
                let _ = tx.send(UpdateRequest::TickerData(
                    t.clone(),
                    self.transactions.clone(),
                    self.settings.default.currency,
                ));
            }
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
                    self.portfolio.value = Some(value);
                }

                UpdateMessage::Error(error) => {
                    self.error = error.clone();
                    tracing::error!("Background update returned an error: {error}");
                }

                UpdateMessage::Ticker { ticker, data } => {
                    if data.total_shares == dec!(0) {
                        let _ = self.portfolio.ticker_info.remove_entry(&ticker);
                    } else if let Some(existing) = self.portfolio.ticker_info.get_mut(&ticker) {
                        existing.update_from(data);
                    } else {
                        self.portfolio.ticker_info.insert(ticker, data);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.process_updates();

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
    }
}
