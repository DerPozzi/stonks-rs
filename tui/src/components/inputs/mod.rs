use stonks_rs::types::CycleEnum;
use strum::EnumIter;

use crate::pages;

pub mod input;
pub mod select;

#[derive(Debug, Default, Clone, Copy, PartialEq, EnumIter)]
pub enum CurrentFocus {
    #[default]
    None,
    TransactionPage(pages::transactions::InputFocus),
    AddTransaction(pages::add_transaction::InputFocus),
    OverviewPage(pages::overview::InputFocus),
}

impl CycleEnum for CurrentFocus {
    fn previous(&self) -> Self {
        match self {
            CurrentFocus::TransactionPage(input_focus) => {
                CurrentFocus::TransactionPage(input_focus.previous())
            }

            CurrentFocus::AddTransaction(input_focus) => {
                CurrentFocus::AddTransaction(input_focus.previous())
            }
            _ => CurrentFocus::None,
        }
    }

    fn next(&self) -> Self {
        match self {
            CurrentFocus::TransactionPage(input_focus) => {
                CurrentFocus::TransactionPage(input_focus.next())
            }

            CurrentFocus::AddTransaction(input_focus) => {
                CurrentFocus::AddTransaction(input_focus.next())
            }
            _ => CurrentFocus::None,
        }
    }
}
