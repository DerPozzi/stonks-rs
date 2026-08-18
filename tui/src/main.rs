use std::fs::OpenOptions;

use anyhow::Result;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::App,
    event::{Event, EventHandler},
    tui::Tui,
};

mod app;
mod components;
mod event;
mod pages;
mod theme;
mod tui;
mod ui;
mod update;

fn init_logger() -> Result<()> {
    let home_path = dirs::home_dir().expect("Couldn't get users home dir");

    if let Some(parent) = home_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!(
            "{}stonks.log",
            home_path.join(".stonks-rs/").display()
        ))?;

    tracing_subscriber::fmt().with_writer(log_file).init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    let (update_tx, update_rx) = update::start_update_task();

    let mut app = App::new(update_tx, update_rx);

    tracing::info!("Initialising TUI");

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.update(),
            Event::Key(key_event) => update::keyboard_update(&mut app, key_event),
            Event::Mouse(mouse_event) => update::mouse_update(&mut app, mouse_event),
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
