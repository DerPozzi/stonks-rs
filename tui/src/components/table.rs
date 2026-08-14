use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

use crate::app::App;

pub fn render_table<'a, I>(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    label: &str,
    header: Vec<Cell<'_>>,
    rows: I,
    focused: bool,
) where
    I: IntoIterator<Item = Row<'a>>,
{
    let widths = vec![Constraint::Fill(1); header.len()];
    let header = Row::new(header)
        .style(
            Style::default()
                .fg(app.theme.primary)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if focused {
                    app.theme.primary
                } else {
                    app.theme.border
                }))
                .title(Span::styled(
                    format!(" {label} "),
                    Style::default().fg(if !focused {
                        app.theme.text
                    } else {
                        app.theme.primary
                    }),
                )),
        )
        .column_spacing(5);

    frame.render_widget(table, area);
}
