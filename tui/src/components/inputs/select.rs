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
        .alignment(Alignment::Left)
        .style(Style::default().fg(theme.text));

    frame.render_widget(widget, area);
}

#[cfg(test)]
mod tests {
    use ratatui::{Terminal, backend::TestBackend, layout::Rect};

    use super::render_select;
    use crate::theme::Theme;

    fn buffer_to_string(terminal: &Terminal<TestBackend>) -> String {
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>()
    }

    #[test]
    fn rerendering_select_shows_latest_value() {
        let backend = TestBackend::new(30, 5);
        let mut terminal = Terminal::new(backend).expect("terminal should initialize");
        let theme = Theme::default();
        let area = Rect::new(0, 0, 30, 3);

        terminal
            .draw(|frame| {
                render_select(frame, area, "Type", "Sell", true, true, &theme);
            })
            .expect("first draw should render");

        terminal
            .draw(|frame| {
                render_select(frame, area, "Type", "Buy", true, true, &theme);
            })
            .expect("second draw should render");

        let rendered = buffer_to_string(&terminal);
        assert!(rendered.contains("Buy"));
        assert!(!rendered.contains("Sell"));
    }
}
