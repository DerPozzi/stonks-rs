use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render_input(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value: &str,
    focused: bool,
) {
    let border_color = if app.input_text && focused {
        app.theme.primary
    } else if focused {
        app.theme.secondary
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

    let widget = Paragraph::new(value)
        .block(
            Block::default()
                .title(Span::styled(format!(" {title} "), title_style))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(app.theme.text));

    frame.render_widget(widget, area);
}
