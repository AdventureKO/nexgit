use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::events::TuiEvent;

#[derive(Debug)]
pub struct TuiApp {
    pub should_quit: bool,
    pub title: String,
    pub status: String,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            title: "nexgit".to_string(),
            status: "Ready. Press q to quit.".to_string(),
        }
    }

    pub fn handle_event(&mut self, event: TuiEvent) {
        match event {
            TuiEvent::Key(key) => self.handle_key(key),
            TuiEvent::Tick => {}
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => self.quit(),
            _ => {
                self.status = format!("Unhandled key: {:?}", key.code);
            }
        }
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}
