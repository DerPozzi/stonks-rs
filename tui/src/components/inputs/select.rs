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
    input_mode: bool,
    theme: &Theme,
) {
    let border_color = if focused && input_mode {
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

    let icon = if focused && input_mode {
        Span::styled(" ▼", Style::default().fg(theme.muted))
    } else {
        Span::styled(" ◀", Style::default().fg(theme.muted))
    };

    let line = Line::from(vec![
        Span::styled(value, Style::default().fg(theme.text)),
        icon,
    ]);

    let footer = Line::from(vec![Span::styled(
        " <TAB> | <SHIFT + TAB> ",
        Style::default().fg(theme.primary),
    )])
    .right_aligned();

    let widget = Paragraph::new(line)
        .block(
            Block::default()
                .title(Span::styled(format!(" {title} "), title_style))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color))
                .title_bottom(if focused && input_mode {
                    footer
                } else {
                    Span::raw("").into()
                }),
        )
        .alignment(Alignment::Left);

    frame.render_widget(widget, area);
}
