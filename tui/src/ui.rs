use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use strum::IntoEnumIterator;

use crate::app::{App, Page};

fn bottom_title(app: &App) -> Line<'static> {
    Page::iter()
        .enumerate()
        .flat_map(|(index, page)| {
            let style = if page == app.current_page {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
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

pub fn render(app: &mut App, frame: &mut Frame) {
    let block = Block::default()
        .title(format!(" Stonks-rs | {} ", app.current_page))
        .title_alignment(Alignment::Center)
        .title_bottom(bottom_title(app))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        No. of transactions stored: {}
      ",
            app.transactions.len()
        ))
        .block(block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center),
        frame.area(),
    )
}
