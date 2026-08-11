use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

use crate::{
    app::{App, CurrentFocus, Page},
    pages::{self, add_transaction::InputField},
};

pub fn keyboard_update(app: &mut App, key_event: KeyEvent) {
    // ============================================================
    // INPUT MODE
    // ============================================================

    if app.input_text {
        match key_event.code {
            KeyCode::Char(c) => {
                app.input_char(c);
            }

            KeyCode::Backspace => {
                app.input_backspace();
            }

            KeyCode::Esc => {
                app.input_text = false;
            }

            _ => {}
        }

        return;
    }

    // ============================================================
    // NORMAL MODE
    // ============================================================

    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
        }

        KeyCode::Char('l') | KeyCode::Right => {
            if !app.current_page_focused {
                app.next_page();
            }
        }

        KeyCode::Char('h') | KeyCode::Left => {
            if !app.current_page_focused {
                app.previous_page();
            }
        }

        KeyCode::Char('?') => {
            app.toggle_hotkeys();
        }

        KeyCode::Char(',') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.open_settings();
        }

        // Feld auswählen
        KeyCode::Char(c @ '0'..='9') => {
            let index = c.to_digit(10).unwrap_or(0) as usize;
            app.handle_layout_focus(index);
        }

        // Add Transaction
        KeyCode::Char('a') => {
            app.add_transaction();
        }

        // Transaction speichern
        KeyCode::Char('s') if app.current_page == Page::AddTransaction => {
            let _ = app.save_new_transaction();
        }

        // Enter
        KeyCode::Enter => match app.create_transaction.focused_field {
            InputField::None => {
                app.focus_page();
            }

            InputField::Ticker
            | InputField::Quantity
            | InputField::Price
            | InputField::Fees
            | InputField::Taxes
            | InputField::TradeDate => {
                app.input_text = true;
            }

            InputField::TransactionType => {
                app.cycle_transaction_type();
            }

            InputField::Currency => {
                app.cycle_currency();
            }
        },

        // Esc
        KeyCode::Esc => {
            if app.focused_field != CurrentFocus::None {
                app.focused_field = CurrentFocus::None;
            } else {
                app.unfocus_page();
            }
        }

        _ => {}
    }
}

pub fn mouse_update(app: &mut App, mouse_event: MouseEvent) {
    let ui_areas = &app.ui_areas;

    if let MouseEventKind::Down(MouseButton::Left) = mouse_event.kind {
        let _position = (mouse_event.column, mouse_event.row);

        if let Some(transaction_ui_areas) = &ui_areas.transaction_page
            && let Some(_filters) = &transaction_ui_areas.filters
        {
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
