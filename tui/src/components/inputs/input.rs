use ratatui::{
    Frame,
    layout::{Position, Rect},
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

    let widget = Paragraph::new(value)
        .block(
            Block::default()
                .title(Span::styled(format!(" {title} "), title_style))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(theme.text));

    let character_index = value.to_string().len();

    frame.render_widget(widget, area);
    if input_mode && focused {
        frame.set_cursor_position(Position::new(
            area.x + character_index as u16 + 1,
            area.y + 1,
        ));
    }
}
