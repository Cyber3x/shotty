use std::io::{self, Write};
use crate::utils::take_input_line;
use crate::{AppState, Command, CommandResult, Shortcut};

pub struct Add;

impl Command for Add {
    fn execute(&self, app_state: &mut AppState) -> CommandResult {
        println!("You are adding a command!\n");

        let new_shortcut = Shortcut::from(
            take_input_line("Shortcut (e.g. CTRL+S): "),
            take_input_line("Description (e.g. open new tab): "),
        );

        println!("New shortcut added! {}", new_shortcut);
        app_state.shortcuts.add_shortcut(new_shortcut);
        app_state.shortcuts.save(&app_state.save_path).unwrap();

        CommandResult::Continue
    }
}

