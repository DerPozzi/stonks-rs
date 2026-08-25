use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::theme::Theme;

pub fn render_select(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value: String,
    focused: bool,
    input_text: bool,
    theme: &Theme,
) {
    let border_color = if focused && input_text {
        theme.primary
    } else if focused {
        theme.secondary
    } else {
        theme.border
    };

    let title_style = if focused {
        Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.text)
    };

    let line = Line::from(vec![
        Span::styled(value, Style::default().fg(theme.text)),
        Span::styled(" ▼", Style::default().fg(theme.muted)),
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
