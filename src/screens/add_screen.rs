use ratatui::{
    crossterm::event::{
        KeyCode,
        KeyEvent,
    },
    prelude::*,
    style::{
        palette::tailwind::{
            self,
            Palette,
        },
        Color,
        Style,
    },
    text::Text,
    widgets::{
        Block,
        Borders,
        Padding,
        Paragraph,
    },
};

use crate::{
    app_state::AppState,
    screen::{
        Screen,
        ScreenCommand,
    },
    utils,
};

enum ActiveInput {
    Shortcut,
    Description,
}

pub struct AddShortcutScreen {
    shortcut_input_text: String,
    description_input_text: String,
    active_input: ActiveInput,
}

impl AddShortcutScreen {
    pub fn new() -> Self {
        // create the inital state
        Self {
            shortcut_input_text: String::new(),
            description_input_text: String::new(),
            active_input: ActiveInput::Shortcut,
        }
    }

    fn render_shortcut_input(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        // Example: draw a label + input box
        let bg_color = match self.active_input {
            ActiveInput::Shortcut => tailwind::ORANGE.c200,
            _ => Color::Red,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Shortcut")
            .bg(bg_color);
        block.render(area, buf);
    }

    fn render_description_input(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        let block = Block::default().borders(Borders::ALL).title("Description");
        block.render(area, buf);
    }
}

impl Screen for AddShortcutScreen {
    fn draw(&mut self, frame: &mut ratatui::Frame, state: &AppState) {
        let area = utils::centered_rect(60, 25, frame.area());
        let buf = frame.buffer_mut();

        let title = Span::styled(
            "Add new shortcut",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(tailwind::ORANGE.c400),
        );

        let block = Block::default()
            .borders(Borders::ALL)
            .bg(tailwind::GRAY.c700)
            .title(title);
        let inner = block.inner(area);
        block.render(area, buf);

        let [shortcut, desc] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(inner);

        self.render_shortcut_input(shortcut, buf, state);
        self.render_description_input(desc, buf, state);
    }

    fn handle_event(
        &mut self,
        key_event: KeyEvent,
        state: &mut AppState,
    ) -> crate::screen::ScreenCommand {
        match key_event.code {
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
