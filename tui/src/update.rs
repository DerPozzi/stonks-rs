use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        KeyCode::Char('l') if key_event.modifiers == KeyModifiers::CONTROL => app.next_page(),
        KeyCode::Right => app.next_page(),
        KeyCode::Char('h') if key_event.modifiers == KeyModifiers::CONTROL => app.previous_page(),
        KeyCode::Left => app.previous_page(),
        _ => {}
    };
}
