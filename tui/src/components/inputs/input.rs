use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::theme::Theme;

pub fn render_input(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value: &str,
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

    let widget = Paragraph::new(value)
        .block(
            Block::default()
                .title(Span::styled(format!(" {title} "), title_style))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(theme.text));

    frame.render_widget(widget, area);
}
