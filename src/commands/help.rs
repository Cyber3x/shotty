use crate::{AppState, Command, CommandResult};

pub struct Help;

impl Command for Help {
    fn execute(&self, _app_state: &mut AppState) -> CommandResult {
        println!("This is a help commadn");

        CommandResult::Continue
    }
}
