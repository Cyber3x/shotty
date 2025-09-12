use ratatui::{
    layout::Rect,
    style::{palette::tailwind, Style},
    widgets::{Block, Paragraph},
};

use crate::{
    style::{compose_style, PatchStyle},
    widgets::Widget,
};

#[derive(Default)]
pub struct Input {
    label: String,
    text: String,

    focused: bool,
}

impl Input {
    pub fn new(label: &str, text: &str) -> Self {
        Self {
            label: label.to_owned(),
            text: text.to_owned(),
            ..Default::default()
        }
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused
    }

    pub fn get_value(&self) -> &str {
        &self.label
    }
}

impl Widget for Input {
    fn render(&self, area: Rect, frame: &mut ratatui::Frame) {
        let style = Style::default();

        let style = compose_style(
            style,
            vec![PatchStyle::new(
                self.focused,
                style.fg(tailwind::GREEN.c500),
            )],
        );

        let paragraph = Paragraph::new(self.text.as_str()).block(
            Block::bordered()
                .border_style(style)
                .title(self.label.as_str()),
        );

        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, _key_event: ratatui::crossterm::event::KeyEvent) -> bool {
        todo!()
    }
}
