use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyEventKind};

#[derive(Debug, Clone)]
pub enum TuiEvent {
    Key(KeyEvent),
    Tick,
}

pub fn next(timeout: Duration) -> Result<Option<TuiEvent>> {
    if event::poll(timeout)? {
        match event::read()? {
            CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => {
                Ok(Some(TuiEvent::Key(key)))
            }
            _ => Ok(None),
        }
    } else {
        Ok(Some(TuiEvent::Tick))
    }
}
