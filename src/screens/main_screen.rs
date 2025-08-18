use ratatui::{
    crossterm::event::KeyCode,
    prelude::*,
    style::{Color, Style},
    text::Text,
    widgets::Paragraph,
};

use crate::{
    app_state::AppState,
    screen::{Screen, ScreenCommand},
    screens::AddShortcutScreen,
};

pub struct MainScreen;

impl MainScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Screen for MainScreen {
    fn draw(&self, frame: &mut ratatui::Frame, area: Rect, state: &AppState) {
        let paragraph = Paragraph::new(Text::styled(
            state.temp_text.clone(),
            Style::default().fg(Color::Yellow),
        ));

        frame.render_widget(paragraph, area);
    }

    fn handle_event(
        &mut self,
        code: KeyCode,
        state: &mut AppState,
    ) -> crate::screen::ScreenCommand {
        match code {
            KeyCode::Char('q') => ScreenCommand::Quit(true),
            KeyCode::Char('n') => ScreenCommand::Push(Box::new(AddShortcutScreen)),
            _ => ScreenCommand::None,
        }
    }
}
