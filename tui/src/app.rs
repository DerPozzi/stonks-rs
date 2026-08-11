use std::path::PathBuf;

use ratatui::crossterm::event::MouseEvent;

use anyhow::Result;
use config::{Config, File};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use stonks_rs::{
    service::{helpers::get_all_transactions, service::init_db},
    types::{Connection, Currency, Transaction, TransactionType},
};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    pages::{
        self,
        add_transaction::CreateTransaction,
        transactions::{TransactionPage, TransactionUiAreas},
    },
    theme::{Theme, load_theme},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
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

pub trait FocusField: Copy + PartialEq {}

#[derive(Debug, Default, PartialEq)]
pub enum CurrentFocus {
    #[default]
    None,
    TransactionPage(pages::transactions::InputFocus),
    AddTransaction(pages::add_transaction::InputField),
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
    pub focused_field: CurrentFocus,

    pub create_transaction: CreateTransaction,
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
            create_transaction: CreateTransaction::default(),
            focused_field: CurrentFocus::None,
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
            trade_date: tx.trade_date,
            quantity: Decimal::try_from(tx.quantity.parse::<f32>()?)?,
            price: Decimal::try_from(tx.price.parse::<f32>()?)?,
            currency: tx.currency,
            fees: Decimal::try_from(tx.fees.parse::<f32>()?)?,
        };

        stonks_rs::service::helpers::add_transaction_to_list(
            &self.db_connection,
            &mut self.transactions,
            new_tx,
        )?;

        self.current_page = Page::Transactions;

        self.create_transaction = CreateTransaction::default();
        Ok(())
    }

    pub fn focus_page(&mut self) {
        self.current_page_focused = true;
    }

    pub fn unfocus_page(&mut self) {
        self.current_page_focused = false;
    }

    pub fn handle_layout_focus(&mut self, number: usize) {
        if self.current_page_focused {
            match &self.current_page {
                Page::Dashboard => todo!(),
                Page::Overview => todo!(),
                Page::Transactions => crate::pages::transactions::handle_focus(self, number),
                Page::Dividends => todo!(),
                Page::Settings(page) => todo!(),
                Page::AddTransaction => pages::add_transaction::handle_focus(self, number),
            }
        }
    }

    pub fn input_char(&mut self, c: char) {
        match self.create_transaction.focused_field {
            pages::add_transaction::InputField::Ticker => {
                self.create_transaction.ticker.push(c);
            }

            pages::add_transaction::InputField::TradeDate => {
                self.create_transaction.trade_date_input.push(c);
            }

            pages::add_transaction::InputField::Quantity => {
                self.create_transaction.quantity.push(c);
            }

            pages::add_transaction::InputField::Price => {
                self.create_transaction.price.push(c);
            }

            pages::add_transaction::InputField::Fees => {
                self.create_transaction.fees.push(c);
            }

            pages::add_transaction::InputField::Taxes => {
                self.create_transaction.taxes.push(c);
            }

            pages::add_transaction::InputField::TransactionType
            | pages::add_transaction::InputField::Currency
            | pages::add_transaction::InputField::None => {}
        }
    }
    pub fn input_backspace(&mut self) {
        match self.create_transaction.focused_field {
            pages::add_transaction::InputField::Ticker => {
                self.create_transaction.ticker.pop();
            }

            pages::add_transaction::InputField::TradeDate => {
                self.create_transaction.trade_date_input.pop();
            }

            pages::add_transaction::InputField::Quantity => {
                self.create_transaction.quantity.pop();
            }

            pages::add_transaction::InputField::Price => {
                self.create_transaction.price.pop();
            }

            pages::add_transaction::InputField::Fees => {
                self.create_transaction.fees.pop();
            }

            pages::add_transaction::InputField::Taxes => {
                self.create_transaction.taxes.pop();
            }

            pages::add_transaction::InputField::TransactionType
            | pages::add_transaction::InputField::Currency
            | pages::add_transaction::InputField::None => {}
        }
    }
    pub fn cycle_transaction_type(&mut self) {
        let trans_type: Vec<TransactionType> = TransactionType::iter().collect();

        let current = self.create_transaction.transaction_type;

        if let Some(index) = trans_type.iter().position(|c| *c == current) {
            let next = (index + 1) % trans_type.len();
            self.create_transaction.transaction_type = trans_type[next];
        }
    }

    pub fn cycle_currency(&mut self) {
        let currencies: Vec<Currency> = Currency::iter().collect();

        let current = self.create_transaction.currency;

        if let Some(index) = currencies.iter().position(|c| *c == current) {
            let next = (index + 1) % currencies.len();
            self.create_transaction.currency = currencies[next];
        }
    }
}
