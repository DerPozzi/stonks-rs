use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{Frame, layout::Rect};
use rust_decimal::Decimal;
use stonks_rs::types::TickerData;

use crate::app::App;
use crate::components::*;

/*
┌────────────────────────────────────────────────────────────────────────────────────┐
│ Portfolio Overview                                                                 │
│                                                                                    │
│ ┌──────────────────────┐ ┌──────────────────────┐ ┌──────────────────────┐         │
│ │ PORTFOLIO VALUE      │ │ INVESTED            │ │ UNREALIZED GAIN      │         │
│ │                      │ │                      │ │                      │         │
│ │     24,850.32 €      │ │     21,430.00 €      │ │     +3,420.32 €      │         │
│ │                      │ │                      │ │        +15.96 %      │         │
│ └──────────────────────┘ └──────────────────────┘ └──────────────────────┘         │
│                                                                                    │
│ ┌──────────────────────┐ ┌──────────────────────┐ ┌──────────────────────┐         │
│ │ REALIZED GAIN        │ │ NET DIVIDENDS        │ │ TOTAL RETURN         │         │
│ │                      │ │                      │ │                      │         │
│ │       +820.50 €      │ │       +245.80 €      │ │     +4,486.62 €      │         │
│ │                      │ │                      │ │        +20.93 %      │         │
│ └──────────────────────┘ └──────────────────────┘ └──────────────────────┘         │
│                                                                                    │
│ ┌───────────────────────────────────────┐ ┌────────────────────────────────────┐   │
│ │ TOP PERFORMERS TODAY                 │ │ WORST PERFORMERS TODAY             │   │
│ │                                       │ │                                    │   │
│ │ AMD        +4.82 %    +182.30 €       │ │ NVDA       -3.21 %    -145.20 €    │   │
│ │ AAPL       +2.41 %     +94.10 €       │ │ MSFT       -2.17 %     -82.40 €    │   │
│ │ VWCE       +1.35 %     +41.20 €       │ │ TSLA       -1.86 %     -64.30 €    │   │
│ │                                       │ │                                    │   │
│ └───────────────────────────────────────┘ └────────────────────────────────────┘   │
│                                                                                    │
│ ┌───────────────────────────────────────┐ ┌────────────────────────────────────┐   │
│ │ RECENT TRANSACTIONS                  │ │ DIVIDENDS                          │   │
│ │                                       │ │                                    │   │
│ │ 11.08  BUY   AAPL   10 × 201.50 €    │ │ AMD       +12.40 €   08.08.2026   │   │
│ │ 08.08  SELL  MSFT    5 × 512.20 €    │ │ VWCE      +18.20 €   01.08.2026   │   │
│ │ 02.08  BUY   VWCE   15 × 142.80 €    │ │ AAPL       +6.40 €   15.07.2026   │   │
│ │ 28.07  BUY   NVDA    8 × 171.30 €    │ │                                    │   │
│ │                                       │ │ Total: +245.80 €                    │   │
│ └───────────────────────────────────────┘ └────────────────────────────────────┘   │
│                                                                                    │
└────────────────────────────────────────────────────────────────────────────────────┘
*/

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    // Dashboard rendern

    let mut assets: Vec<_> = app.portfolio.ticker_info.values().collect();

    assets.sort_by(|a, b| a.todays_change.cmp(&b.unrealized_gain));

    // Areas anlegen

    let areas = Layout::vertical([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(area);

    let portfolio_value = format!(
        "{} {}",
        if let Some(v) = app.portfolio.value {
            v.round_dp(2).to_string()
        } else {
            "Loading".to_string()
        },
        if app.portfolio.value.is_some() {
            app.settings.default.currency.to_string()
        } else {
            String::new()
        }
    );
    card::render(
        frame,
        areas[0],
        app,
        " Portfolio Value ",
        portfolio_value,
        None,
    );

    render_top_three(
        frame,
        areas[1],
        app,
        " Worst Performers ",
        assets
            .iter()
            .filter(|td| td.todays_change <= Decimal::ZERO).copied()
            .take(3)
            .collect(),
    );
    assets.reverse();
    render_top_three(
        frame,
        areas[2],
        app,
        " Top Performers ",
        assets
            .iter()
            .filter(|td| td.todays_change > Decimal::ZERO).copied()
            .take(3)
            .collect(),
    );
}

fn render_top_three(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    label: &str,
    assets: Vec<&TickerData>,
) {
    let border_style = app.theme.secondary;

    let lines: Vec<Line> = assets
        .iter()
        .map(|ticker| {
            let change_style = if ticker.todays_change > Decimal::ZERO {
                Style::default().fg(app.theme.success)
            } else if ticker.todays_change < Decimal::ZERO {
                Style::default().fg(app.theme.error)
            } else {
                Style::default().fg(app.theme.text)
            };

            Line::from(vec![
                Span::styled(&ticker.name, Style::default().fg(app.theme.text)),
                Span::raw(" "),
                Span::styled(
                    format!("{}%", ticker.todays_change.round_dp(2)),
                    change_style,
                ),
            ])
        })
        .collect();

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .title(Span::styled(label, Style::default().fg(app.theme.text)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_style)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(widget, area);
}
