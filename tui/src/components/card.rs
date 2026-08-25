use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    label: &str,
    value: String,
    style: Option<Style>,
) {
    let border_style = app.theme.secondary;

    let w = Paragraph::new(value)
        .style(style.unwrap_or(Style::default().fg(app.theme.primary)))
        .block(
            Block::default()
                .title(Span::styled(label, Style::default().fg(app.theme.text)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_style)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(w, area);
}
