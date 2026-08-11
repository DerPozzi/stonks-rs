use anyhow::Result;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::App,
    event::{Event, EventHandler},
    tui::Tui,
};

mod app;
mod event;
mod pages;
mod theme;
mod tui;
mod ui;
mod update;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new();

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
            Event::Tick => {}
            Event::Key(key_event) => update::keyboard_update(&mut app, key_event),
            Event::Mouse(mouse_event) => update::mouse_update(&mut app, mouse_event),
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
