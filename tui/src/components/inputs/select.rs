use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render_select(
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
