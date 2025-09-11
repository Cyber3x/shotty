use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::*,
    style::{
        palette::tailwind::{self},
        Color, Style,
    },
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app_state::AppState,
    screen::{Screen, ScreenCommand},
    style::{compose_style, PatchStyle},
    utils,
    widgets::{input::Input, Widget},
};

#[derive(Debug, PartialEq, Default)]
enum ActiveInput {
    #[default]
    Shortcut,
    Description,
}

#[derive(Debug, PartialEq, Default)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct AddShortcutScreen {
    shortcut_input: Input,
    /// Currently selected box
    active_input: ActiveInput,
    /// Current input mode
    input_mode: InputMode,
}

const BG_COLOR: Color = tailwind::GRAY.c700;

impl AddShortcutScreen {
    pub fn new() -> Self {
        // create the inital state
        Self {
            shortcut_input: Input::new(
                "Shortcut",
                "placeholder text",
            ),
            ..Default::default()
        }
    }

    fn render_shortcut_input(&self, area: Rect, frame: &mut Frame, _state: &AppState) {
        self.shortcut_input
            .render(
                area, frame,
            );
    }

    fn render_description_input(&self, area: Rect, frame: &mut Frame, _state: &AppState) {
        let border_style = compose_style(
            Style::default(),
            vec![
                PatchStyle::new(
                    self.active_input == ActiveInput::Description,
                    Style::default().fg(tailwind::GREEN.c500),
                ),
                PatchStyle::new(
                    self.active_input == ActiveInput::Description
                        && self.input_mode == InputMode::Editing,
                    Style::default().fg(tailwind::YELLOW.c500),
                ),
            ],
        );

        let block = Block::bordered()
            .title("Description")
            .border_style(border_style);

        frame.render_widget(
            block, area,
        );
    }

    fn handle_normal_mode(&mut self, key_event: KeyEvent) -> ScreenCommand {
        match key_event.code {
            KeyCode::Char('q') => {
                return ScreenCommand::Close;
            }
            KeyCode::Tab => match self.active_input {
                ActiveInput::Shortcut => self.active_input = ActiveInput::Description,
                ActiveInput::Description => self.active_input = ActiveInput::Shortcut,
            },
            KeyCode::Enter => self.input_mode = InputMode::Editing,
            _ => {}
        };

        ScreenCommand::None
    }
}

impl Screen for AddShortcutScreen {
    fn draw(&mut self, frame: &mut Frame, state: &AppState) {
        let area = utils::centered_rect(
            60,
            25,
            frame.area(),
        );

        let title = Span::styled(
            "Add new shortcut",
            Style::default().add_modifier(Modifier::BOLD),
        );

        let block = Block::default()
            .borders(Borders::ALL)
            .bg(BG_COLOR)
            .title(title);

        let inner = block.inner(area);

        frame.render_widget(
            block, area,
        );

        let [shortcut, desc, _buttons] = Layout::vertical(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ],
        )
        .areas(inner);

        self.render_shortcut_input(
            shortcut, frame, state,
        );
        self.render_description_input(
            desc, frame, state,
        );
        // TODO: render buttons
    }

    fn handle_event(
        &mut self,
        key_event: KeyEvent,
        _state: &mut AppState,
    ) -> crate::screen::ScreenCommand {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(key_event),
            _ => ScreenCommand::Close,
        }
    }
}
