use pet_the_cat_tui::app::{AppResult, App};
use pet_the_cat_tui::event::{Event, EventHandler};
use pet_the_cat_tui::handler::handle_key_events;
use pet_the_cat_tui::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

fn main() -> AppResult<()> {
    // Set the current localization to the system's locale.
    pet_the_cat_tui::localization::set_to_system();

    // Create an application.
    let mut app = App::new();

    // Load the saved game state.
    app.load();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
