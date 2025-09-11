use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

pub mod input;

pub trait Widget {
    fn render(&self, area: Rect, frame: &mut Frame);
    fn handle_event(&mut self, key_event: KeyEvent) -> bool;

    fn set_focused(&mut self, focused: bool);

    fn is_focused(&self) -> bool;
}
