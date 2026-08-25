use strum::EnumIter;

use crate::pages::overview::OverviewState;

pub mod add_transaction;
pub mod dashboard;
pub mod dividends;
pub mod overview;
pub mod transactions;

#[derive(Debug)]
pub enum PageState {
    None,
    Overview(overview::OverviewState),
    Transaction(transactions::TransactionState),
}

impl PageState {
    pub fn overview_mut(&mut self) -> Option<&mut OverviewState> {
        match self {
            PageState::Overview(state) => Some(state),
            _ => None,
        }
    }
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
