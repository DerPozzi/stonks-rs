use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState},
};

use crate::theme::Theme;

#[allow(clippy::too_many_arguments)]
pub fn render<'a, I>(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    header: Vec<Cell<'_>>,
    rows: I,
    focused: bool,
    table_state: &mut TableState,
    theme: Theme,
) where
    I: IntoIterator<Item = Row<'a>>,
{
    let widths = vec![Constraint::Fill(1); header.len()];
    let header = Row::new(header)
        .style(
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let footer = Line::from(vec![
        Span::styled(" ↑↓", Style::default().fg(theme.primary)),
        Span::raw(" Navigate "),
    ])
    .right_aligned();

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if focused {
                    theme.primary
                } else {
                    theme.border
                }))
                .title(Span::styled(
                    format!(" {label} "),
                    Style::default().fg(if !focused { theme.text } else { theme.primary }),
                ))
                .title_bottom(if focused {
                    footer
                } else {
                    Span::raw("").into()
                }),
        )
        .column_spacing(1)
        .row_highlight_style(Style::default().bg(theme.background));

    frame.render_stateful_widget(table, area, table_state);
}
