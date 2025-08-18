use ratatui::{
    crossterm::{event::KeyCode, terminal::SetTitle},
    prelude::*,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app_state::AppState,
    screen::{Screen, ScreenCommand},
    utils,
};

pub struct AddShortcutScreen;

impl AddShortcutScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Screen for AddShortcutScreen {
    fn draw(&self, frame: &mut ratatui::Frame, area: Rect, state: &AppState) {
        let area = utils::centered_rect(60, 25, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let paragraph = Paragraph::new(Text::styled(
            state.temp_text.clone(),
            Style::default().fg(Color::Yellow),
        ))
        .block(block);

        frame.render_widget(paragraph, area);
    }

    fn handle_event(
        &mut self,
        code: KeyCode,
        state: &mut AppState,
    ) -> crate::screen::ScreenCommand {
        match code {
            KeyCode::Char('q') => ScreenCommand::Close,
            KeyCode::Char(value) => {
                state.temp_text.push(value);
                ScreenCommand::None
            }
            KeyCode::Backspace => {
                state.temp_text.pop();
                ScreenCommand::None
            }
            _ => ScreenCommand::None,
        }
    }
}
