use ratatui::{Frame, crossterm::event::KeyCode, layout::Rect};

use crate::app_state::AppState;

pub trait Screen {
    fn draw(&self, frame: &mut Frame, area: Rect, state: &AppState);
    fn handle_event(&mut self, code: KeyCode, state: &mut AppState) -> ScreenCommand;
}

pub enum ScreenCommand {
    /// we handled a key and dont want anything to happen
    None,

    // The current screen needs to close
    Close,

    // We want to open a new screen on top of this one
    Push(Box<dyn Screen>),

    // We want to swap the current screen with a new one
    Swap(Box<dyn Screen>),

    // We want to completly exit the application
    Quit(bool),
}
