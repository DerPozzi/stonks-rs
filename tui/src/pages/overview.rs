use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, StatefulWidget, TableState, Wrap},
};
use rust_decimal_macros::dec;
use stonks_rs::types::CycleEnum;
use strum::{EnumIter, FromRepr};

use crate::{
    app::App,
    components::{inputs::CurrentFocus, table::TickerDataTable},
    pages::PageState,
};

#[derive(Debug, Default)]
pub struct OverviewState {
    pub overview_table: TableState,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, FromRepr, EnumIter)]
pub enum InputFocus {
    #[default]
    Table,
}

impl CycleEnum for InputFocus {}

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let mut entries: Vec<_> = app.portfolio.ticker_info.iter().collect();

    if entries.is_empty() {
        let paragraph =
            Paragraph::new("No data to display right now.\nIt might still be loading...")
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(app.theme.text));

        frame.render_widget(paragraph, area);
        return;
    }

    entries.sort_by_key(|(_, a)| a.unrealized_gain);
    entries.reverse();

    let table_header = vec![
        Cell::from("Name"),
        // Cell::from("Ticker"),
        Cell::from("Invested"),
        Cell::from("Market Value"),
        Cell::from("Unrealized Gain"),
        Cell::from("Amount @ Price"),
        Cell::from("Current Price"),
    ];

    let table_rows = entries.iter().map(|(_t, tx)| {
        let gain_style = Style::default().fg(if tx.unrealized_gain_perc > dec!(0) {
            app.theme.success
        } else if tx.unrealized_gain_perc == dec!(0) {
            app.theme.text
        } else {
            app.theme.error
        });
        Row::new(vec![
            Cell::from(if let Some(long_name) = tx.meta.long_name.as_ref() {
                long_name
            } else {
                "Loading ..."
            }),
            // Cell::from(t.to_uppercase()),
            Cell::from(format!(
                "{} {}",
                tx.cost_basis.round_dp(2),
                app.settings.default.currency
            )),
            Cell::from(format!(
                "{} {}",
                tx.market_value.round_dp(2),
                app.settings.default.currency
            )),
            Cell::from(Span::styled(
                format!(
                    "{} {} | {} %",
                    tx.unrealized_gain.round_dp(2),
                    app.settings.default.currency,
                    tx.unrealized_gain_perc.round_dp(2)
                ),
                gain_style,
            )),
            Cell::from(format!(
                "{} @ {} {}",
                tx.total_shares.round_dp(2),
                tx.avg_cost.round_dp(2),
                app.settings.default.currency
            )),
            Cell::from(format!(
                "{} {}",
                tx.financial.current_price.round_dp(2),
                app.settings.default.currency
            )),
        ])
    });

    let PageState::Overview(state) = &mut app.page_state else {
        todo!()
    };

    let CurrentFocus::OverviewPage(currently_focused) = app.focused_field else {
        return;
    };

    let table = TickerDataTable {
        header: table_header,
        tool_tip: " ↑↓ Navigate - <i> Information ",
        label: "Ticker Info",
        rows: table_rows,
        focused: currently_focused == InputFocus::Table,
        theme: app.theme.clone(),
    };

    table.render(area, frame.buffer_mut(), &mut state.overview_table);
}
