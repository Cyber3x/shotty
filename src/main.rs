mod app;
mod app_state;
mod screen;
mod screens;
mod shortcuts;
mod style;
mod utils;

use std::{
    error::Error,
    io,
};

use ratatui::{
    crossterm::{
        execute,
        terminal::{
            disable_raw_mode,
            enable_raw_mode,
            EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    },
    prelude::CrosstermBackend,
    Terminal,
};

use crate::{
    app::App,
    app_state::AppState,
    shortcuts::Shortcuts,
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let shortcuts = match Shortcuts::load_from_file("shortcuts.json".into()) {
        Ok(shortcuts) => shortcuts,
        Err(_) => Shortcuts::new(),
    };
    let app_state = AppState::new(shortcuts);
    let mut app = App::new(app_state);
    let _ = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
