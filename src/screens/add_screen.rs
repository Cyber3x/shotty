use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::*,
    style::{
        palette::tailwind::{self},
        Color, Style,
    },
    widgets::{Block, Borders},
};

use crate::{
    app_state::AppState,
    screen::{Screen, ScreenCommand},
    utils,
    widgets::{input::Input, Widget},
};

#[derive(Debug, PartialEq, Default, Hash, Eq)]
enum FocusedWidget {
    #[default]
    ShortcutInput,
    DescriptionInput,
}

pub struct AddShortcutScreen {
    shortcut_input: Input,
    description_input: Input,
    focused_widget: Option<FocusedWidget>,
}

const BG_COLOR: Color = tailwind::GRAY.c700;

impl AddShortcutScreen {
    pub fn new() -> Self {
        Self {
            shortcut_input: Input::new("Shortcut", "placeholder text"),
            description_input: Input::new("Description", "placeholder text"),
            focused_widget: Some(FocusedWidget::ShortcutInput),
        }
    }

    fn render_shortcut_input(&mut self, area: Rect, frame: &mut Frame, _state: &AppState) {
        self.shortcut_input
            .set_focused(self.focused_widget == Some(FocusedWidget::ShortcutInput));
        self.shortcut_input.render(area, frame);
    }

    fn render_description_input(&mut self, area: Rect, frame: &mut Frame, _state: &AppState) {
        self.description_input
            .set_focused(self.focused_widget == Some(FocusedWidget::DescriptionInput));
        self.description_input.render(area, frame);
    }

    fn handle_normal_mode(&mut self, key_event: KeyEvent) -> ScreenCommand {
        match key_event.code {
            KeyCode::Char('q') if self.focused_widget.is_none() => {
                return ScreenCommand::Close;
            }
            KeyCode::Tab => {
                self.focused_widget = match self.focused_widget {
                    Some(FocusedWidget::ShortcutInput) => Some(FocusedWidget::DescriptionInput),
                    Some(FocusedWidget::DescriptionInput) => Some(FocusedWidget::ShortcutInput),
                    None => Some(FocusedWidget::ShortcutInput),
                };
            }
            KeyCode::Esc => {
                self.focused_widget = None;
            }
            _ => {
                // Forward input to focused widget
                if let Some(focused) = &self.focused_widget {
                    match focused {
                        FocusedWidget::ShortcutInput => {
                            self.shortcut_input.handle_event(key_event);
                        }
                        FocusedWidget::DescriptionInput => {
                            self.description_input.handle_event(key_event); // Fixed!
                        }
                    }
                }
            }
        }
        ScreenCommand::None
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

        let [shortcut, desc, _buttons] = Layout::vertical([
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
        _state: &mut AppState,
    ) -> crate::screen::ScreenCommand {
        self.handle_normal_mode(key_event)
    }
}
