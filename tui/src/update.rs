use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use rust_decimal::Decimal;
use stonks_rs::types::{Currency, TickerData, Transaction};
use tokio::sync::mpsc;

use crate::app::{App, CurrentFocus, Page};

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
            KeyCode::Tab => {
                app.handle_selector_tab();
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
            app.handle_layout_focus(Some(index));
        }

        // Add Transaction
        KeyCode::Char('a') => {
            app.add_transaction();
        }

        // Transaction speichern
        KeyCode::Char('s') if app.current_page == Page::AddTransaction => {
            let _ = app.save_new_transaction();
        }

        KeyCode::Enter => {
            if app.input_text {
                return;
            }

            if !app.current_page_focused {
                app.focus_page();
                return;
            }

            match &app.focused_field {
                CurrentFocus::None => {
                    // Page ist fokussiert, aber noch kein Input-Feld
                }

                CurrentFocus::TransactionPage(_) => {
                    app.input_text = true;
                }

                CurrentFocus::AddTransaction(_) => {
                    app.input_text = true;
                }
            }
        }
        // Esc
        KeyCode::Esc => {
            if app.input_text {
                app.input_text = false;
            } else if app.focused_field != CurrentFocus::None {
                app.focused_field = CurrentFocus::None;
            } else if app.current_page_focused {
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

/// Nachrichten vom App/UI-Thread zum Background-Task.
#[derive(Debug)]
pub enum UpdateRequest {
    PortfolioValue(Vec<Transaction>, Currency),
    TickerData(String, Vec<Transaction>, Currency),
}

/// Nachrichten vom Background-Task zurück zur App.
#[derive(Debug)]
pub enum UpdateMessage {
    PortfolioValue(Decimal),

    Ticker { ticker: String, data: TickerData },

    Error(String),
}

pub fn start_update_task() -> (
    mpsc::UnboundedSender<UpdateRequest>,
    mpsc::UnboundedReceiver<UpdateMessage>,
) {
    let (request_tx, mut request_rx) = mpsc::unbounded_channel::<UpdateRequest>();

    let (message_tx, message_rx) = mpsc::unbounded_channel::<UpdateMessage>();

    tracing::info!("Starting update task");

    tokio::spawn(async move {
        while let Some(request) = request_rx.recv().await {
            match request {
                UpdateRequest::PortfolioValue(tx, curr) => {
                    match stonks_rs::service::service::get_portfolio_value(&tx, Some(curr)).await {
                        Ok(portfolio) => {
                            let _ = message_tx.send(UpdateMessage::PortfolioValue(portfolio));
                        }

                        Err(error) => {
                            tracing::error!("Failed to get portfolio value: {error}");

                            let _ = message_tx.send(UpdateMessage::Error(error.to_string()));
                        }
                    }
                }

                UpdateRequest::TickerData(t, tx, curr) => {
                    match stonks_rs::service::service::get_ticker_info(t, &tx, Some(curr)).await {
                        Ok(ticker_data) => {
                            let _ = message_tx
                                .send(UpdateMessage::Ticker {
                                    ticker: ticker_data.ticker.clone(),
                                    data: ticker_data,
                                })
                                .unwrap();
                        }

                        Err(error) => {
                            tracing::error!("Failed to get info for a ticker: {error}");

                            let _ = message_tx.send(UpdateMessage::Error(error.to_string()));
                        }
                    }
                }

                request => {
                    tracing::warn!("Unhandled update request: {:?}", request);
                }
            }
        }

        tracing::info!("Update task stopped");
    });

    (request_tx, message_rx)
}
