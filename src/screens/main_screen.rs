use std::{cmp::max, vec};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    prelude::*,
    widgets::{Cell, Row, Table, TableState},
};
use style::palette::tailwind;

use crate::{
    app_state::AppState,
    screen::{Screen, ScreenCommand},
    screens::AddShortcutScreen,
    shortcuts::Shortcut,
};

const INFO_TEXT: [&str; 2] = ["(Esc\\q) quit | (j) move down | (k) move up", ""];

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

pub struct MainScreen {
    colors: TableColors,
    table_state: TableState,
}

impl Screen for MainScreen {
    fn draw(&mut self, frame: &mut ratatui::Frame, state: &AppState) {
        self.render_table(frame, state);
    }

    fn handle_event(
        &mut self,
        key_event: KeyEvent,
        _state: &mut AppState,
    ) -> crate::screen::ScreenCommand {
        match key_event {
            // q or esc
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }
            | KeyEvent {
                code: KeyCode::Esc, ..
            } => ScreenCommand::Quit(true),
            KeyEvent {
                code: KeyCode::Char('n'),
                ..
            } => ScreenCommand::Push(Box::new(AddShortcutScreen::new())),
            _ => ScreenCommand::None,
        }
    }
}

impl MainScreen {
    pub fn new() -> Self {
        Self {
            colors: TableColors::new(&PALETTES[0]),
            table_state: TableState::default().with_selected(0),
        }
    }
    fn render_table(&mut self, frame: &mut Frame, state: &AppState) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header_text = ["Lookup count", "Shortcut", "Description"];
        let header = header_text
            .clone()
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let longest_item_lens =
            calc_longest_lens(state.shortcuts.get_all_shortcuts(), &header_text);

        let rows = state
            .shortcuts
            .get_all_shortcuts()
            .iter()
            .enumerate()
            .map(|(i, shortcut)| {
                let color = match i % 2 {
                    0 => self.colors.normal_row_color,
                    _ => self.colors.alt_row_color,
                };
                Row::new(vec![
                    Cell::new(shortcut.get_lookup_count().to_string()),
                    Cell::new(shortcut.get_key_combo()),
                    Cell::new(shortcut.get_description()),
                ])
                .style(Style::new().fg(self.colors.row_fg).bg(color))
            });
        let table = Table::new(
            rows,
            [
                Constraint::Length(longest_item_lens[0] as u16),
                Constraint::Min(longest_item_lens[1] as u16),
                Constraint::Min(longest_item_lens[2] as u16),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style);
        frame.render_stateful_widget(table, frame.area(), &mut self.table_state);
    }
}

fn calc_longest_lens(shortcuts: &[Shortcut], headers: &[&str]) -> Vec<usize> {
    vec![
        max(
            shortcuts
                .iter()
                .map(Shortcut::get_lookup_count)
                .map(|a| a.to_string().len())
                .max()
                .unwrap_or(0),
            headers[0].len(),
        ),
        max(
            shortcuts
                .iter()
                .map(Shortcut::get_key_combo)
                .map(|a| a.to_string().len())
                .max()
                .unwrap_or(0),
            headers[1].len(),
        ),
        max(
            shortcuts
                .iter()
                .map(Shortcut::get_description)
                .map(|a| a.to_string().len())
                .max()
                .unwrap_or(0),
            headers[2].len(),
        ),
    ]
}
