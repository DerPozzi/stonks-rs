use crate::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget},
};
use std::fmt::Display;
use stonks_rs::types::CycleEnum;

pub struct SelectDropdown<'a, I> {
    pub title: &'a str,
    pub _values: I,
    pub focused: bool,
    pub theme: &'a Theme,
    pub input_mode: bool,
}

#[derive(Debug)]
pub struct DropdownState<T: CycleEnum> {
    pub selected: T,
    pub _expanded: bool,
}

impl<T: CycleEnum> DropdownState<T> {
    pub fn next(&mut self) {
        self.selected = self.selected.next();
    }

    pub fn previous(&mut self) {
        self.selected = self.selected.previous();
    }
}

impl<T: CycleEnum + Default> Default for DropdownState<T> {
    fn default() -> Self {
        Self {
            selected: T::default(),
            _expanded: false,
        }
    }
}

impl<'a, I> StatefulWidget for SelectDropdown<'a, I>
where
    I: Iterator + Clone,
    I::Item: Copy + PartialEq + Display + Default + CycleEnum,
{
    type State = DropdownState<I::Item>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let border_color = if self.focused && self.input_mode {
            self.theme.primary
        } else if self.focused {
            self.theme.secondary
        } else {
            self.theme.border
        };

        let title_style = if self.focused {
            Style::default()
                .fg(self.theme.primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.theme.text)
        };

        let icon = if self.focused && self.input_mode {
            Span::styled(" ▼", Style::default().fg(self.theme.muted))
        } else {
            Span::styled(" ◀", Style::default().fg(self.theme.muted))
        };

        let line = Line::from(vec![
            Span::styled(
                state.selected.to_string(),
                Style::default().fg(self.theme.text),
            ),
            icon,
        ]);

        let footer = Line::from(vec![Span::styled(
            " <TAB> | <SHIFT + TAB> ",
            Style::default().fg(self.theme.primary),
        )])
        .right_aligned();

        let select = Paragraph::new(line)
            .block(
                Block::default()
                    .title(Span::styled(format!(" {} ", self.title), title_style))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(border_color))
                    .title_bottom(if self.focused && self.input_mode {
                        footer
                    } else {
                        Span::raw("").into()
                    }),
            )
            .alignment(Alignment::Left);

        select.render(area, buf);
    }
}
