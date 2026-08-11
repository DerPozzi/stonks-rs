use chrono::NaiveDate;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use strum::FromRepr;

use crate::app::App;

use stonks_rs::types::{Currency, TransactionType};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum InputField {
    #[default]
    None,
    Ticker,
    TransactionType,
    TradeDate,
    Quantity,
    Price,
    Currency,
    Fees,
    Taxes,
}

#[derive(Debug)]
pub struct CreateTransaction {
    pub ticker: String,

    pub transaction_type: TransactionType,

    pub trade_date: NaiveDate,
    pub trade_date_input: String,

    pub quantity: String,
    pub price: String,

    pub fees: String,
    pub taxes: String,

    pub currency: Currency,

    pub focused_field: InputField,
}
impl Default for CreateTransaction {
    fn default() -> Self {
        let today = chrono::Local::now().date_naive();

        Self {
            ticker: String::new(),

            transaction_type: TransactionType::Buy,

            trade_date: today,
            trade_date_input: today.format("%Y-%m-%d").to_string(),

            quantity: String::new(),
            price: String::new(),

            fees: "0".to_string(),
            taxes: "0".to_string(),

            currency: Currency::EUR,

            focused_field: InputField::Ticker,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateTransactionAreas {
    pub ticker: Rect,
    pub transaction_type: Rect,
    pub trade_date: Rect,
    pub quantity: Rect,
    pub price: Rect,
    pub fees: Rect,
    pub taxes: Rect,
    pub currency: Rect,

    pub cancel: Rect,
    pub save: Rect,
}

pub fn handle_focus(app: &mut App, number: usize) {
    app.create_transaction.focused_field = InputField::from_repr(number).unwrap_or_default();
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(Line::from(vec![
            Span::raw(" "),
            Span::styled(
                "New Transaction",
                Style::default()
                    .fg(app.theme.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));

    let inner = block.inner(area);

    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Ticker / Type
            Constraint::Length(3), // Date / Quantity
            Constraint::Length(3), // Price / Currency
            Constraint::Length(3), // Fees / Taxes
            Constraint::Min(1),    // Spacer
            Constraint::Length(3), // Buttons
        ])
        .spacing(1)
        .split(inner);

    let row_1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(2)
        .split(rows[0]);

    let row_2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(2)
        .split(rows[1]);

    let row_3 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(2)
        .split(rows[2]);

    let row_4 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(2)
        .split(rows[3]);

    render_input(
        app,
        frame,
        row_1[0],
        "[1] Ticker",
        &app.create_transaction.ticker,
        app.create_transaction.focused_field == InputField::Ticker,
    );

    render_select(
        app,
        frame,
        row_1[1],
        "[2] Type",
        app.create_transaction.transaction_type.to_string(),
        app.create_transaction.focused_field == InputField::TransactionType,
    );

    render_input(
        app,
        frame,
        row_2[0],
        "[3] Trade Date",
        &app.create_transaction.trade_date_input.to_string(),
        app.create_transaction.focused_field == InputField::TradeDate,
    );

    render_input(
        app,
        frame,
        row_2[1],
        "[4] Quantity",
        &app.create_transaction.quantity,
        app.create_transaction.focused_field == InputField::Quantity,
    );

    render_input(
        app,
        frame,
        row_3[0],
        "[5] Price",
        &app.create_transaction.price,
        app.create_transaction.focused_field == InputField::Price,
    );

    render_select(
        app,
        frame,
        row_3[1],
        "[6] Currency",
        app.create_transaction.currency.to_string(),
        app.create_transaction.focused_field == InputField::Currency,
    );

    render_input(
        app,
        frame,
        row_4[0],
        "[7] Fees",
        &app.create_transaction.fees,
        app.create_transaction.focused_field == InputField::Fees,
    );

    render_input(
        app,
        frame,
        row_4[1],
        "[8] Taxes",
        &app.create_transaction.taxes,
        app.create_transaction.focused_field == InputField::Taxes,
    );

    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(12),
            Constraint::Length(20),
        ])
        .spacing(2)
        .split(rows[5]);

    let cancel_area = buttons[1];
    let save_area = buttons[2];

    let filled_out = true;

    render_button(app, frame, save_area, "[s] Save", filled_out);

    let create_transaction_areas = CreateTransactionAreas {
        ticker: row_1[0],
        transaction_type: row_1[1],
        trade_date: row_2[0],
        quantity: row_2[1],
        price: row_3[0],
        currency: row_3[1],
        fees: row_4[0],
        taxes: row_4[1],
        cancel: cancel_area,
        save: save_area,
    };
}

fn render_input(app: &App, frame: &mut Frame, area: Rect, title: &str, value: &str, focused: bool) {
    let border_color = if app.input_text && focused {
        app.theme.primary
    } else if focused {
        app.theme.secondary
    } else {
        app.theme.border
    };

    let title_style = if focused {
        Style::default()
            .fg(app.theme.primary)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.text)
    };

    let widget = Paragraph::new(value)
        .block(
            Block::default()
                .title(Span::styled(format!(" {title} "), title_style))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(app.theme.text));

    frame.render_widget(widget, area);
}

fn render_select(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value: String,
    focused: bool,
) {
    let border_color = if focused {
        app.theme.primary
    } else {
        app.theme.border
    };

    let title_style = if focused {
        Style::default()
            .fg(app.theme.primary)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.text)
    };

    let line = Line::from(vec![
        Span::styled(value, Style::default().fg(app.theme.text)),
        Span::styled(" ▼", Style::default().fg(app.theme.muted)),
    ]);

    let widget = Paragraph::new(line)
        .block(
            Block::default()
                .title(Span::styled(format!(" {title} "), title_style))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color)),
        )
        .alignment(Alignment::Left);

    frame.render_widget(widget, area);
}

fn render_button(app: &App, frame: &mut Frame, area: Rect, label: &str, filled_out: bool) {
    let (foreground, mut background) = if filled_out {
        (app.theme.background, app.theme.primary)
    } else {
        (app.theme.text, app.theme.border)
    };

    let border_color = if filled_out {
        app.theme.primary
    } else {
        app.theme.border
    };

    let button = Paragraph::new(label)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(foreground)
                .bg(background)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color)),
        );

    frame.render_widget(button, area);
}
