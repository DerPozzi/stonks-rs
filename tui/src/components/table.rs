use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Row, StatefulWidget, Table, TableState},
};

use crate::theme::Theme;

pub struct TransactionDividendTable<'a, I> {
    pub header: Vec<Cell<'a>>,
    pub tool_tip: &'a str,
    pub label: &'a str,
    pub rows: I,
    pub focused: bool,
    pub theme: Theme,
}

impl<'a, I> StatefulWidget for TransactionDividendTable<'a, I>
where
    I: IntoIterator<Item = Row<'a>>,
{
    type State = TableState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let widths = vec![Constraint::Fill(1); self.header.len()];
        let header = Row::new(self.header)
            .style(
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1);

        let footer = Line::from(vec![Span::styled(
            self.tool_tip,
            Style::default().fg(self.theme.primary),
        )])
        .right_aligned();

        let table = Table::new(self.rows, widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(if self.focused {
                        self.theme.primary
                    } else {
                        self.theme.border
                    }))
                    .title(Span::styled(
                        format!(" {} ", self.label),
                        Style::default().fg(if !self.focused {
                            self.theme.text
                        } else {
                            self.theme.primary
                        }),
                    ))
                    .title_bottom(if self.focused {
                        footer
                    } else {
                        Span::raw("").into()
                    }),
            )
            .column_spacing(1)
            .row_highlight_style(Style::default().bg(self.theme._warning));

        table.render(area, buf, state);
    }
}

pub struct TickerDataTable<'a, I> {
    pub header: Vec<Cell<'a>>,
    pub tool_tip: &'a str,
    pub label: &'a str,
    pub rows: I,
    pub focused: bool,
    pub theme: Theme,
}

impl<'a, I> StatefulWidget for TickerDataTable<'a, I>
where
    I: IntoIterator<Item = Row<'a>>,
{
    type State = TableState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let widths = vec![Constraint::Fill(1); self.header.len()];
        let header = Row::new(self.header)
            .style(
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1);

        let footer = Line::from(vec![Span::styled(
            self.tool_tip,
            Style::default().fg(self.theme.primary),
        )])
        .right_aligned();

        let table = Table::new(self.rows, widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(if self.focused {
                        self.theme.primary
                    } else {
                        self.theme.border
                    }))
                    .title(Span::styled(
                        format!(" {} ", self.label),
                        Style::default().fg(if !self.focused {
                            self.theme.text
                        } else {
                            self.theme.primary
                        }),
                    ))
                    .title_bottom(if self.focused {
                        footer
                    } else {
                        Span::raw("").into()
                    }),
            )
            .column_spacing(1)
            .row_highlight_style(Style::default().bg(self.theme._warning));

        table.render(area, buf, state);
    }
}
