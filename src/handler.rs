use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind != crossterm::event::KeyEventKind::Release{
        return Ok(());
    }
    
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Char(' ') => {
            app.pet_cat();
        }

        KeyCode::Char('m') => {
            app.buy_multiplier();
        }

        KeyCode::Char('f') => {
            app.buy_factory();
        }
        // Other handlers you could add here.
        _ => {}
    }

    Ok(())
}
