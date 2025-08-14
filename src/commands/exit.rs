use crate::{AppState, Command, CommandResult};

pub struct Exit;

impl Command for Exit {
    fn execute(&self, _app_state: &mut AppState) -> CommandResult {
        println!("Goodbye!");
        CommandResult::Exit
    }
}
