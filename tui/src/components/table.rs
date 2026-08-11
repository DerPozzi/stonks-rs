use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Style,
    text::Span,
    widgets::{Block, BorderType, Borders, Row, Table},
};

use crate::app::App;

pub fn render_table<'a, I>(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    label: &str,
    header: Row<'a>,
    rows: I,
    focused: bool,
) where
    I: IntoIterator<Item = Row<'a>>,
{
    let mut transactions = app.transactions.clone();
    transactions.reverse();

    let widths = [
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ];

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
