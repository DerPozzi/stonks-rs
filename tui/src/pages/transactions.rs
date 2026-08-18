use std::cmp;

use chrono::Datelike;

use chrono::NaiveDate;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row},
};
use stonks_rs::types::{CycleEnum, TimeFrame, Transaction, TransactionType};
use strum::{EnumIter, FromRepr};

use crate::{
    app::App,
    components::{
        inputs::{
            input::render_input,
            select::{cycle_enum, render_select},
        },
        table::render_table,
    },
};

/*
┌─────────────────────────────────────────────────────────────────────────────┐
│ Transactions                                                                │
│                                                                             │
│ ┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐             │
│ │ TRANSACTIONS     │ │ BUYS             │ │ SELLS            │             │
│ │       128        │ │        76        │ │        52        │             │
│ └──────────────────┘ └──────────────────┘ └──────────────────┘             │
│                                                                             │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ Period: [ All Time ▼ ]    Type: [ All ▼ ]    Asset: [ All ▼ ]           │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
│ ┌────────────┬──────────┬──────────────┬───────────┬──────────┬───────────┐ │
│ │ DATE       │ TYPE     │ ASSET        │ QUANTITY  │ PRICE    │ TOTAL     │ │
│ ├────────────┼──────────┼──────────────┼───────────┼──────────┼───────────┤ │
│ │ 11.08.2026 │ BUY      │ AAPL         │ 10        │ 201.50 € │ 2,015 €   │ │
│ │ 08.08.2026 │ SELL     │ MSFT         │ 5         │ 512.20 € │ 2,561 €   │ │
│ │ 02.08.2026 │ BUY      │ VWCE         │ 15        │ 142.80 € │ 2,142 €   │ │
│ │ 28.07.2026 │ BUY      │ NVDA         │ 8         │ 171.30 € │ 1,370 €   │ │
│ │ ...        │ ...      │ ...          │ ...       │ ...      │ ...       │ │
│ └────────────┴──────────┴──────────────┴───────────┴──────────┴───────────┘ │
│                                                                             │
│                         ← 1  2  3  4  5 →                                   │
└─────────────────────────────────────────────────────────────────────────────┘
*/
#[derive(Debug, Default)]
pub struct TransactionFilters {
    pub period: Period,
    pub transaction_type: TransactionTypeFilter,
    pub ticker: String,
}

#[derive(Debug, Default, PartialEq, Copy, Clone, EnumIter)]
pub enum TransactionTypeFilter {
    #[default]
    All,
    Buy,
    Sell,
}

impl CycleEnum for TransactionTypeFilter {}

impl ToString for TransactionTypeFilter {
    fn to_string(&self) -> String {
        match self {
            TransactionTypeFilter::All => "All".to_string(),
            TransactionTypeFilter::Buy => "Buy".to_string(),
            TransactionTypeFilter::Sell => "Sell".to_string(),
        }
    }
}

type Period = stonks_rs::types::TimeFrame;

#[derive(Debug, Default)]
pub struct TransactionPage {
    pub filters: TransactionFilters,
    pub _ui_areas: TransactionUiAreas,
}

#[derive(Debug)]
pub struct FilterAreas {
    pub _period: Rect,
    pub _transaction_type: Rect,
    pub _asset: Rect,
}

#[derive(Debug, Default)]
pub struct TransactionUiAreas {
    pub filters: Option<FilterAreas>,
}

fn render_transaction_count(app: &App, frame: &mut Frame, area: Rect) {
    let total_count = app.transactions.len();
    let (buy_count, sell_count) = get_transaction_type_count(&app.transactions);

    let border_style = app.theme.secondary;

    let areas = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(area);

    let total = Paragraph::new(total_count.to_string())
        .style(Style::default().fg(app.theme.primary))
        .block(
            Block::default()
                .title(Span::styled(
                    " Total Transactions ",
                    Style::default().fg(app.theme.text),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_style)),
        )
        .alignment(Alignment::Center);

    let buy = Paragraph::new(buy_count.to_string())
        .style(Style::default().fg(app.theme.success))
        .block(
            Block::default()
                .title(Span::styled(" Buys ", Style::default().fg(app.theme.text)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_style)),
        )
        .alignment(Alignment::Center);

    let sell = Paragraph::new(sell_count.to_string())
        .style(Style::default().fg(app.theme.error))
        .block(
            Block::default()
                .title(Span::styled(" Sells ", Style::default().fg(app.theme.text)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_style)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(total, areas[0]);
    frame.render_widget(buy, areas[1]);
    frame.render_widget(sell, areas[2]);
}

fn get_transaction_type_count(tx: &[Transaction]) -> (u64, u64) {
    let mut buy_transactions = 0;
    let mut sell_transactions = 0;

    for t in tx.iter() {
        match t.transaction_type {
            stonks_rs::types::TransactionType::Buy => buy_transactions += 1,
            stonks_rs::types::TransactionType::Sell => sell_transactions += 1,
        }
    }

    (buy_transactions, sell_transactions)
}

fn is_currently_focused(current: Option<&InputFocus>, check: InputFocus) -> bool {
    if let Some(focus) = current {
        return *focus == check;
    }
    false
}

fn today() -> NaiveDate {
    chrono::Local::now().date_naive()
}

fn filter_transactions(app: &App, transactions: &mut Vec<Transaction>) {
    let ticker = app.transaction_page.filters.ticker.to_lowercase();

    transactions.retain(|tx| {
        let matches_period = match app.transaction_page.filters.period {
            TimeFrame::OneDay => tx.trade_date >= today() - chrono::Duration::days(1),
            TimeFrame::OneWeek => tx.trade_date >= today() - chrono::Duration::days(7),
            TimeFrame::OneMonth => tx.trade_date >= today() - chrono::Duration::days(30),
            TimeFrame::ThreeMonth => tx.trade_date >= today() - chrono::Duration::days(90),
            TimeFrame::SixMonth => tx.trade_date >= today() - chrono::Duration::days(180),
            TimeFrame::YearToDate => tx.trade_date.year() == today().year(),
            TimeFrame::OneYear => tx.trade_date >= today() - chrono::Duration::days(365),
            TimeFrame::FiveYear => tx.trade_date >= today() - chrono::Duration::days(365 * 5),
            TimeFrame::Max => true,

            // Für die Transaktionstabelle vermutlich irrelevant:
            TimeFrame::OneMinute | TimeFrame::OneHour => true,
        };

        let matches_type = match app.transaction_page.filters.transaction_type {
            TransactionTypeFilter::All => true,
            TransactionTypeFilter::Sell => tx.transaction_type == TransactionType::Sell,
            TransactionTypeFilter::Buy => tx.transaction_type == TransactionType::Buy,
        };
        let matches_ticker = ticker.is_empty() || tx.ticker.to_lowercase().contains(&ticker);

        matches_period && matches_type && matches_ticker
    });
}

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let currently_focused = match &app.focused_field {
        crate::app::CurrentFocus::TransactionPage(input_focus) => Some(input_focus),
        _ => None,
    };
    let areas = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Fill(1),
    ])
    .split(area);

    render_transaction_count(app, frame, areas[0]);

    let transaction_filter_areas = render_filter_bar(app, frame, areas[1]);
    let table_header = vec![
        Cell::from("ID"),
        Cell::from("Date"),
        Cell::from("Ticker"),
        Cell::from("Type"),
        Cell::from("Quantity"),
        Cell::from("Price"),
        Cell::from("Currency"),
        Cell::from("Fees"),
    ];
    let mut transactions = app.transactions.clone();

    transactions.sort_by(|a, b| {
        if a.trade_date == b.trade_date {
            cmp::Ordering::Equal
        } else if a.trade_date < b.trade_date {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    });

    transactions.reverse();

    filter_transactions(app, &mut transactions);

    let table_rows = transactions.iter().map(|tx| {
        Row::new(vec![
            Cell::from(tx.id.unwrap().to_string()),
            Cell::from(tx.trade_date.to_string()),
            Cell::from(tx.ticker.clone()),
            Cell::from(Span::styled(
                tx.transaction_type.to_string(),
                Style::default().fg(if tx.transaction_type == TransactionType::Buy {
                    app.theme.success
                } else {
                    app.theme.error
                }),
            )),
            Cell::from(tx.quantity.to_string()),
            Cell::from(tx.price.round_dp(2).to_string()),
            Cell::from(tx.currency.to_string()),
            Cell::from(tx.fees.to_string()),
        ])
    });

    render_table(
        app,
        frame,
        areas[2],
        "[4] Transactions",
        table_header,
        table_rows,
        is_currently_focused(currently_focused, InputFocus::Table),
    );
    let transaction_ui_areas = TransactionUiAreas {
        filters: Some(transaction_filter_areas),
    };
    app.ui_areas.transaction_page = Some(transaction_ui_areas);
}

pub fn render_filter_bar(app: &App, frame: &mut Frame, area: Rect) -> FilterAreas {
    let currently_focused = match &app.focused_field {
        crate::app::CurrentFocus::TransactionPage(input_focus) => Some(input_focus),
        _ => None,
    };

    let block = Block::default()
        .title(Span::styled(
            " Filters ",
            Style::default().fg(app.theme.text),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.border));

    let inner = block.inner(area);

    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Fill(1),
        ])
        .spacing(2)
        .split(inner);

    render_select(
        app,
        frame,
        chunks[0],
        "[1] Period",
        app.transaction_page.filters.period.to_string(),
        is_currently_focused(currently_focused, InputFocus::Period),
    );

    render_select(
        app,
        frame,
        chunks[1],
        "[2] Type",
        app.transaction_page.filters.transaction_type.to_string(),
        is_currently_focused(currently_focused, InputFocus::TransactionType),
    );

    render_input(
        app,
        frame,
        chunks[2],
        "[3] Ticker",
        &app.transaction_page.filters.ticker.to_string(),
        is_currently_focused(currently_focused, InputFocus::Ticker),
    );

    FilterAreas {
        _period: chunks[0],
        _transaction_type: chunks[1],
        _asset: chunks[2],
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, FromRepr)]
pub enum InputFocus {
    #[default]
    Period,
    TransactionType,
    Ticker,
    Table,
}

pub fn handle_input_char(app: &mut App, field: InputFocus, c: char) {
    match field {
        InputFocus::Ticker => {
            app.transaction_page.filters.ticker.push(c);
        }

        InputFocus::TransactionType | InputFocus::Period | InputFocus::Table => {}
    }
}

pub fn handle_input_backspace(app: &mut App, field: InputFocus) {
    match field {
        InputFocus::Ticker => {
            app.transaction_page.filters.ticker.pop();
        }

        InputFocus::TransactionType | InputFocus::Period | InputFocus::Table => {}
    }
}

pub fn handle_selector_tab(app: &mut App, field: InputFocus) {
    match field {
        InputFocus::TransactionType => {
            cycle_enum(&mut app.transaction_page.filters.transaction_type);
        }

        InputFocus::Period => {
            cycle_enum(&mut app.transaction_page.filters.period);
        }

        _ => {}
    }
}
