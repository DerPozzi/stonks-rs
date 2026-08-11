use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use stonks_rs::types::Transaction;

use crate::app::App;

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
pub struct TransactionFiltersValues {
    pub period_filter: String,
    pub transaction_type_filter: String,
    pub ticker_filter: String,
}

#[derive(Debug, Default)]
pub enum TransactionFilter {
    #[default]
    None,
    Period,
    TransactionType,
}

#[derive(Debug, Default)]
pub struct TransactionPage {
    pub filters: TransactionFiltersValues,
    pub open_filters: TransactionFilter,
    pub ui_areas: TransactionUiAreas,

    input_focus: InputFocus,
}

#[derive(Debug)]
pub struct FilterAreas {
    pub period: Rect,
    pub transaction_type: Rect,
    pub asset: Rect,
}

#[derive(Debug, Default)]
pub struct TransactionUiAreas {
    pub filters: Option<FilterAreas>,
}

fn render_transaction_count(app: &App, frame: &mut Frame, area: Rect) {
    let total_count = app.transactions.len();
    let (buy_count, sell_count) = get_transaction_type_count(&app.transactions);

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
                    "Total Transactions",
                    Style::default().fg(app.theme.text),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.primary)),
        )
        .alignment(Alignment::Center);

    let buy = Paragraph::new(buy_count.to_string())
        .style(Style::default().fg(app.theme.success))
        .block(
            Block::default()
                .title(Span::styled("Buys", Style::default().fg(app.theme.text)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.primary)),
        )
        .alignment(Alignment::Center);

    let sell = Paragraph::new(sell_count.to_string())
        .style(Style::default().fg(app.theme.error))
        .block(
            Block::default()
                .title(Span::styled("Sells", Style::default().fg(app.theme.text)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.primary)),
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

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let areas = Layout::vertical([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(area);

    render_transaction_count(app, frame, areas[0]);
    let transaction_filter_areas = render_filter_bar(app, frame, areas[1]);
    let transaction_ui_areas = TransactionUiAreas {
        filters: Some(transaction_filter_areas),
    };
    app.ui_areas.transaction_page = Some(transaction_ui_areas);
}

pub fn render_filter_bar(app: &App, frame: &mut Frame, area: Rect) -> FilterAreas {
    let block = Block::default()
        .title(" Filters ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.secondary));

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

    render_filter(
        app,
        frame,
        chunks[0],
        "Period",
        app.transaction_page.filters.period_filter.to_string(),
    );

    render_filter(
        app,
        frame,
        chunks[1],
        "Type",
        app.transaction_page
            .filters
            .transaction_type_filter
            .to_string(),
    );

    render_search(
        app,
        frame,
        chunks[2],
        "Ticker",
        app.transaction_page.filters.ticker_filter.to_string(),
    );

    FilterAreas {
        period: chunks[0],
        transaction_type: chunks[1],
        asset: chunks[2],
    }
}

#[derive(Debug, Default, PartialEq)]
enum InputFocus {
    #[default]
    None,
    Ticker,
    Period,
    TransactionType,
    Table,
}

fn render_search(app: &App, frame: &mut Frame, area: Rect, _label: &str, _value: String) {
    let border_color = if app.transaction_page.input_focus == InputFocus::Ticker {
        app.theme.primary
    } else {
        app.theme.secondary
    };

    let input = Paragraph::new(app.transaction_page.filters.ticker_filter.as_str())
        .block(
            Block::default()
                .title(" Ticker ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(app.theme.text));

    frame.render_widget(input, area);
}

fn render_filter(app: &App, frame: &mut Frame, area: Rect, label: &str, value: String) {
    let line = Line::from(vec![
        Span::styled(format!("{label}: "), Style::default().fg(app.theme.text)),
        Span::styled(
            value,
            Style::default()
                .fg(app.theme.primary)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ▼", Style::default().fg(app.theme.muted)),
    ]);

    frame.render_widget(Paragraph::new(line), area);
}
