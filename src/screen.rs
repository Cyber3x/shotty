#![allow(dead_code)]
use ratatui::{crossterm::event::KeyEvent, Frame};

use crate::app_state::AppState;

pub trait Screen {
    fn draw(&mut self, frame: &mut Frame, state: &AppState);
    fn handle_event(&mut self, key_event: KeyEvent, state: &mut AppState) -> ScreenCommand;
}

pub enum ScreenCommand {
    /// we handled a key and don't want anything to happen
    None,

    // The current screen needs to close
    Close,

    // We want to open a new screen on top of this one
    Push(Box<dyn Screen>),

    // We want to swap the current screen with a new one
    Swap(Box<dyn Screen>),

    // We want to completely exit the application
    Quit(bool),
}
