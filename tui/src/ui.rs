use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};
use strum::IntoEnumIterator;

use crate::{
    app::{App, Page},
    pages::{self, transactions::TransactionUiAreas},
};

#[derive(Debug, Default)]
pub struct UiAreas {
    pub transaction_ui_areas: Option<TransactionUiAreas>,
}

fn bottom_title(app: &App) -> Line<'static> {
    Page::iter()
        .enumerate()
        .flat_map(|(index, page)| {
            let style = if page == app.current_page {
                Style::default()
                    .fg(Color::Black)
                    .bg(if app.current_page_focused {
                        app.theme.primary
                    } else {
                        app.theme.secondary
                    })
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };

            let separator = if index > 0 {
                Span::raw("  ")
            } else {
                Span::raw("")
            };

            [separator, Span::styled(format!(" {page} "), style)]
        })
        .collect()
}

fn render_page(app: &mut App, frame: &mut Frame, area: Rect) {
    match &app.current_page {
        Page::Dashboard => pages::dashboard::render(app, frame, area),
        Page::Transactions => pages::transactions::render(app, frame, area),
        Page::Overview => pages::overview::render(app, frame, area),
        Page::Dividends => pages::dividends::render(app, frame, area),
        Page::Settings(_page) => todo!(),
        Page::AddTransaction => pages::add_transaction::render(app, frame, area),
    }
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let block = Block::default()
        .title(format!(" Stonks-rs | {} - {}", app.current_page, app.error))
        .title_alignment(Alignment::Center)
        .title_bottom(bottom_title(app))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.current_page_focused {
            app.theme.primary
        } else {
            app.theme.secondary
        }))
        .border_type(BorderType::Rounded);

    let inner_area = block.inner(frame.area());

    frame.render_widget(block, frame.area());

    render_page(app, frame, inner_area);
}
