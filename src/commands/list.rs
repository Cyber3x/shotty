use crate::{Command, CommandResult};

pub struct List;

impl Command for List {
    fn execute(&self, app_state: &mut crate::AppState) -> crate::CommandResult {
        let shortcuts = app_state.shortcuts.get_all_shortcuts();

        if shortcuts.len() == 0 {
            println!("No shortcuts are stored! Use the `add` command to add new shortcuts.");
        } else {
            for shortcut in app_state.shortcuts.get_all_shortcuts() {
                println!("{shortcut:?}");
            }
        }

        CommandResult::Continue
    }
}
