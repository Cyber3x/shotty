use crate::utils::{self, take_input_number};
use crate::{AppState, Command, CommandResult};

pub struct Remove;

impl Command for Remove {
    fn execute(&self, app_state: &mut AppState) -> CommandResult {
        if app_state.shortcuts.get_all_shortcuts().is_empty() {
            println!("No shortcuts are stored! Nothing to delete.");
            return CommandResult::Continue;
        };

        let shortcuts = app_state.shortcuts.get_all_shortcuts();

        let sorted_indexes = app_state.shortcuts.get_sorted_indexes();

        let headers = ["Number", "Shortcut", "Description", "Lookup count"]
            .map(String::from)
            .to_vec();

        utils::print_table(
            headers,
            sorted_indexes
                .iter()
                .enumerate()
                .map(|(display_idx, &shortcut_index)| {
                    let s = &shortcuts[shortcut_index];
                    vec![
                        (display_idx + 1).to_string(),
                        s.get_key_combo().to_string(),
                        s.get_description().to_string(),
                        s.get_lookup_count().to_string(),
                    ]
                })
                .collect(),
        );

        let Some(target_number) = take_input_number("\nWhich shortcut do you want to remove: ")
        else {
            println!("Invalid number entered!");
            return CommandResult::Continue;
        };

        if target_number < 1 || target_number > shortcuts.len().try_into().unwrap() {
            println!("Target is out of range.");
            return CommandResult::Continue;
        }

        let target_index = target_number as usize - 1;

        let real_idx = sorted_indexes[target_index];

        app_state.shortcuts.remove_at(real_idx).unwrap();
        app_state.shortcuts.save().unwrap();

        println!("Shortcut removed!");

        CommandResult::Continue
    }
}
