use std::io;

use ratatui::{
    crossterm::event::{
        self,
        Event,
    },
    prelude::Backend,
    Terminal,
};

use crate::{
    app_state::AppState,
    screen::{
        Screen,
        ScreenCommand,
    },
    screens::MainScreen,
};

pub struct App {
    pub state: AppState,
    pub screens: Vec<Box<dyn Screen>>,
}

impl App {
    pub fn new(state: AppState) -> Self {
        Self {
            state,
            screens: vec![Box::new(MainScreen::new())],
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            let _ = terminal.draw(|frame| {
                for screen in &mut self.screens {
                    screen.draw(frame, &self.state);
                }
            });

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                let cmd = self
                    .screens
                    .last_mut()
                    .unwrap()
                    .handle_event(key, &mut self.state);

                match cmd {
                    ScreenCommand::None => {}
                    ScreenCommand::Close => {
                        self.screens.pop();
                        if self.screens.is_empty() {
                            return Ok(());
                        }
                    }
                    ScreenCommand::Push(screen) => {
                        self.screens.push(screen);
                    }

                    ScreenCommand::Quit(_value) => return Ok(()),

                    ScreenCommand::Swap(screen) => {
                        self.screens.pop();
                        self.screens.push(screen);
                    }
                }
            }
        }
    }
}
