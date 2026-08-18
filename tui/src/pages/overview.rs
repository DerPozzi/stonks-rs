use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Cell, Paragraph, Row},
};
use rust_decimal_macros::dec;

use crate::{app::App, components::*};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let _test = Paragraph::new(format!("{:#?}", app.portfolio.ticker_info));

    let table_header = vec![
        Cell::from("Name"),
        Cell::from("Ticker"),
        Cell::from("Invested"),
        Cell::from("Market Value"),
        Cell::from("Unrealized Gain"),
        Cell::from("Amount @ price"),
        Cell::from("Current price"),
    ];

    let table_rows = app.portfolio.ticker_info.iter().map(|(t, tx)| {
        let gain_style = Style::default().fg(if tx.unrealized_gain_perc > dec!(0) {
            app.theme.success
        } else if tx.unrealized_gain_perc == dec!(0) {
            app.theme.text
        } else {
            app.theme.error
        });
        Row::new(vec![
            Cell::from(tx.name.clone()),
            Cell::from(t.to_uppercase()),
            Cell::from(format!(
                "{} {}",
                tx.cost_basis.round_dp(2).to_string(),
                app.settings.default.currency
            )),
            Cell::from(format!(
                "{} {}",
                tx.market_value.round_dp(2).to_string(),
                app.settings.default.currency
            )),
            Cell::from(Span::styled(
                format!(
                    "{} {} | {} %",
                    tx.unrealized_gain.round_dp(2).to_string(),
                    app.settings.default.currency,
                    tx.unrealized_gain_perc.round_dp(2).to_string()
                ),
                gain_style,
            )),
            Cell::from(format!(
                "{} @ {} {}",
                tx.total_shares.round_dp(2).to_string(),
                tx.avg_cost.round_dp(2).to_string(),
                app.settings.default.currency
            )),
            Cell::from(format!(
                "{} {}",
                tx.current_price.round_dp(2).to_string(),
                app.settings.default.currency
            )),
        ])
    });

    table::render(
        app,
        frame,
        area,
        "Ticker Info",
        table_header,
        table_rows,
        // is_currently_focused(currently_focused, InputFocus::Table),
        true,
    );
}
