use crate::{
    AppState, Command, CommandResult,
    utils::{self, take_input_number},
};

pub struct List;

impl Command for List {
    fn execute(&self, app_state: &mut AppState) -> crate::CommandResult {
        if app_state.shortcuts.get_all_shortcuts().is_empty() {
            println!("No shortcuts are stored! Use the `add` command to add new shortcuts.");
            return CommandResult::Continue;
        };

        let shortcuts = app_state.shortcuts.get_all_shortcuts();

        let headers = ["Number", "Shortcut", "Description", "Lookup count"]
            .map(String::from)
            .to_vec();

        let sorted_indexes = app_state.shortcuts.get_sorted_indexes();

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

        let Some(target_number) =
            take_input_number("\nWhich of the following shortcuts you came for: ")
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
        app_state.shortcuts.increment_lookup_count(real_idx, 5);

        app_state.shortcuts.save().unwrap();
        println!("Shortcut updated!");

        CommandResult::Continue
    }
}
