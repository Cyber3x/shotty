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
        Styled,
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

#[derive(Debug, PartialEq)]
enum ActiveInput {
    Shortcut,
    Description,
}

pub struct AddShortcutScreen {
    shortcut_input_text: String,
    description_input_text: String,
    active_input: ActiveInput,
}

const BG_COLOR: Color = tailwind::GRAY.c700;

impl AddShortcutScreen {
    pub fn new() -> Self {
        // create the inital state
        Self {
            shortcut_input_text: String::new(),
            description_input_text: String::new(),
            active_input: ActiveInput::Shortcut,
        }
    }

    fn render_shortcut_input(&self, area: Rect, frame: &mut Frame, state: &AppState) {
        let border_style = compose_style(
            Style::default(),
            vec![PatchStyle::new(
                self.active_input == ActiveInput::Shortcut,
                Style::default().fg(tailwind::GREEN.c500),
            )],
        );

        let input = Paragraph::new("Hello".to_owned()).block(
            Block::bordered()
                .title("Shortcut")
                .border_style(border_style),
        );

        frame.render_widget(input, area);
    }

    fn render_description_input(&self, area: Rect, frame: &mut Frame, state: &AppState) {
        let border_style = compose_style(
            Style::default(),
            vec![PatchStyle::new(
                self.active_input == ActiveInput::Description,
                Style::default().fg(tailwind::GREEN.c500),
            )],
        );

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Description")
            .border_style(border_style);

        frame.render_widget(block, area);
    }
}

impl Screen for AddShortcutScreen {
    fn draw(&mut self, frame: &mut Frame, state: &AppState) {
        let area = utils::centered_rect(60, 25, frame.area());

        let title = Span::styled(
            "Add new shortcut",
            Style::default().add_modifier(Modifier::BOLD),
        );

        let block = Block::default()
            .borders(Borders::ALL)
            .bg(BG_COLOR)
            .title(title);

        let inner = block.inner(area);

        frame.render_widget(block, area);

        let [shortcut, desc, buttons] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .areas(inner);

        self.render_shortcut_input(shortcut, frame, state);
        self.render_description_input(desc, frame, state);
        // TODO: render buttons
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
            KeyCode::Tab => {
                match self.active_input {
                    ActiveInput::Shortcut => self.active_input = ActiveInput::Description,
                    ActiveInput::Description => self.active_input = ActiveInput::Shortcut,
                }
                ScreenCommand::None
            }
            _ => ScreenCommand::None,
        }
    }
}
