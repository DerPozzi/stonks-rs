use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

use crate::{app::App, ui::UiAreas};

pub fn keyboard_update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => {
            if !app.input_text {
                app.quit()
            }
        }
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        KeyCode::Char('l') => {
            if !app.input_text {
                app.current_page_focused = false;
                app.next_page();
            }
        }

        KeyCode::Right => {
            app.current_page_focused = false;
            app.next_page();
        }
        KeyCode::Char('h') => {
            if !app.input_text {
                app.current_page_focused = false;
                app.previous_page();
            }
        }
        KeyCode::Left => {
            app.current_page_focused = false;
            app.previous_page();
        }

        KeyCode::Char('?') => app.toggle_hotkeys(),
        KeyCode::Char(',') if key_event.modifiers == KeyModifiers::CONTROL => app.open_settings(),

        KeyCode::Enter => {
            if !app.input_text {
                app.focus_page()
            }
        }

        KeyCode::Esc => {
            if !app.input_text {
                app.unfocus_page()
            }
        }
        _ => {}
    };
}

pub fn mouse_update(app: &mut App, mouse_event: MouseEvent) {
    let ui_areas = &app.ui_areas;

    if let MouseEventKind::Down(MouseButton::Left) = mouse_event.kind {
        let position = (mouse_event.column, mouse_event.row);

        if let Some(transaction_ui_areas) = &ui_areas.transaction_page {
            if let Some(filters) = &transaction_ui_areas.filters {
                // if filters.period.contains(position.into()) {
                //     app.open_period_filter();
                // } else if filters.transaction_type.contains(position.into()) {
                //     app.open_transaction_type_filter();
                // } else if filters.asset.contains(position.into()) {
                //     app.open_asset_filter();
                // }
            }
        }
    }
}
