use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use strum::{EnumIter, FromRepr};

use crate::{
    app::App,
    components::inputs::{CurrentFocus, input::*, select::render_select},
};

use stonks_rs::types::{Currency, CycleEnum, TransactionType};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter)]
pub enum InputFocus {
    #[default]
    Ticker,
    TransactionType,
    TradeDate,
    Quantity,
    Price,
    Currency,
    Fees,
    Taxes,
}

impl CycleEnum for InputFocus {}

#[derive(Debug)]
pub struct CreateTransaction {
    pub ticker: String,

    pub transaction_type: TransactionType,

    pub trade_date_input: String,

    pub quantity: String,
    pub price: String,

    pub fees: String,
    pub taxes: String,

    pub currency: Currency,
}
impl Default for CreateTransaction {
    fn default() -> Self {
        let today = chrono::Local::now().date_naive();

        Self {
            ticker: String::new(),

            transaction_type: TransactionType::Buy,

            trade_date_input: today.format("%Y-%m-%d").to_string(),

            quantity: String::new(),
            price: String::new(),

            fees: "0".to_string(),
            taxes: "0".to_string(),

            currency: Currency::EUR,
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

fn is_currently_focused(current: Option<&InputFocus>, check: InputFocus) -> bool {
    if let Some(focus) = current {
        return *focus == check;
    }
    false
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let currently_focused = match &app.focused_field {
        CurrentFocus::AddTransaction(input_focus) => Some(input_focus),
        _ => None,
    };

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
        frame,
        row_1[0],
        "[1] Ticker",
        &app.create_transaction.ticker,
        is_currently_focused(currently_focused, InputFocus::Ticker),
        app.input_mode,
        &app.theme,
    );

    render_select(
        frame,
        row_1[1],
        "[2] Type",
        app.create_transaction.transaction_type.to_string(),
        is_currently_focused(currently_focused, InputFocus::TransactionType),
        app.input_mode,
        &app.theme,
    );

    render_input(
        frame,
        row_2[0],
        "[3] Trade Date",
        &app.create_transaction.trade_date_input.to_string(),
        is_currently_focused(currently_focused, InputFocus::TradeDate),
        app.input_mode,
        &app.theme,
    );

    render_input(
        frame,
        row_2[1],
        "[4] Quantity",
        &app.create_transaction.quantity,
        is_currently_focused(currently_focused, InputFocus::Quantity),
        app.input_mode,
        &app.theme,
    );

    render_input(
        frame,
        row_3[0],
        "[5] Price",
        &app.create_transaction.price,
        is_currently_focused(currently_focused, InputFocus::Price),
        app.input_mode,
        &app.theme,
    );

    render_select(
        frame,
        row_3[1],
        "[6] Currency",
        app.create_transaction.currency.to_string(),
        is_currently_focused(currently_focused, InputFocus::Currency),
        app.input_mode,
        &app.theme,
    );

    render_input(
        frame,
        row_4[0],
        "[7] Fees",
        &app.create_transaction.fees,
        is_currently_focused(currently_focused, InputFocus::Fees),
        app.input_mode,
        &app.theme,
    );

    render_input(
        frame,
        row_4[1],
        "[8] Taxes",
        &app.create_transaction.taxes,
        is_currently_focused(currently_focused, InputFocus::Taxes),
        app.input_mode,
        &app.theme,
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

    let _debug = Paragraph::new(format!("Create Transaction: {:#?}", app.create_transaction));

    // frame.render_widget(debug, rows[4]);

    render_button(app, frame, save_area, "[s] Save", true, false);

    let _create_transaction_areas = CreateTransactionAreas {
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

fn render_button(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    label: &str,
    primary: bool,
    disabled: bool,
) {
    let (foreground, background) = if disabled {
        (app.theme.background, app.theme.muted)
    } else if primary {
        (app.theme.text, app.theme.primary)
    } else {
        (app.theme.text, app.theme.secondary)
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
                .border_style(Style::default().fg(foreground).bg(background)),
        );

    frame.render_widget(button, area);
}

pub fn handle_input_char(app: &mut App, field: InputFocus, c: char) {
    match field {
        InputFocus::Ticker => {
            app.create_transaction.ticker.push(c);
        }

        InputFocus::TradeDate => {
            app.create_transaction.trade_date_input.push(c);
        }

        InputFocus::Quantity => {
            app.create_transaction.quantity.push(c);
        }

        InputFocus::Price => {
            app.create_transaction.price.push(c);
        }

        InputFocus::Fees => {
            app.create_transaction.fees.push(c);
        }

        InputFocus::Taxes => {
            app.create_transaction.taxes.push(c);
        }

        InputFocus::TransactionType | InputFocus::Currency => {}
    }
}

pub fn handle_input_backspace(app: &mut App, field: InputFocus) {
    match field {
        InputFocus::Ticker => {
            app.create_transaction.ticker.pop();
        }

        InputFocus::TradeDate => {
            app.create_transaction.trade_date_input.pop();
        }

        InputFocus::Quantity => {
            app.create_transaction.quantity.pop();
        }

        InputFocus::Price => {
            app.create_transaction.price.pop();
        }

        InputFocus::Fees => {
            app.create_transaction.fees.pop();
        }

        InputFocus::Taxes => {
            app.create_transaction.taxes.pop();
        }

        InputFocus::TransactionType | InputFocus::Currency => {}
    }
}

pub fn handle_selector_tab(app: &mut App, field: InputFocus) {
    match field {
        InputFocus::TransactionType => {
            &mut app.create_transaction.transaction_type.next();
        }

        InputFocus::Currency => {
            &mut app.create_transaction.currency.next();
        }

        _ => {}
    }
}
