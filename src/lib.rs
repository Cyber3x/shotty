use std::{
    collections::HashMap,
    error::Error,
    io::{self, Write},
};

mod commands {
    pub mod add;
    pub mod exit;
    pub mod help;
    pub mod list;
    pub mod remove;
}
mod utils;

pub mod shortcuts;
use shortcuts::Shortcuts;

// pub struct Config {}

// impl Config {
//     pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
//         Ok(Config {})
//     }
// }
enum CommandResult {
    Continue,
    Exit,
}

struct AppState {
    shortcuts: Shortcuts,
}

trait Command {
    fn execute(&self, app_state: &mut AppState) -> CommandResult;
}

struct CommandRegistry {
    commands: HashMap<&'static str, Box<dyn Command>>,
}

impl CommandRegistry {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    fn add_command(&mut self, cmd_name: &'static str, cmd: Box<dyn Command>) {
        self.commands.insert(cmd_name, cmd);
    }

    fn get_command_names(&self) -> Vec<&str> {
        let mut command_names: Vec<&str> = self.commands.keys().cloned().collect();
        command_names.sort();
        command_names
    }

    fn get_command(&self, cmd_name: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(cmd_name)
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("No path provided, looking for shortcuts.json in the current directory.");

    let shortcuts = Shortcuts::load_from_file(None)?;

    let mut command_registry = CommandRegistry::new();
    let mut app_state = AppState { shortcuts };

    command_registry.add_command("help", Box::new(commands::help::Help));
    command_registry.add_command("add", Box::new(commands::add::Add));
    command_registry.add_command("list", Box::new(commands::list::List));
    command_registry.add_command("remove", Box::new(commands::remove::Remove));
    command_registry.add_command("exit", Box::new(commands::exit::Exit));

    println!("Shortcuts CLI");
    println!(
        "Commands: {}",
        command_registry.get_command_names().join(", ")
    );

    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input)?;
        let cmd_name = input.trim().to_lowercase();

        let cmd = command_registry.get_command(&cmd_name);

        match cmd {
            None => {
                println!("Command '{}' not found!", cmd_name);
                continue;
            }
            Some(cmd) => {
                match cmd.execute(&mut app_state) {
                    CommandResult::Exit => break,
                    CommandResult::Continue => continue,
                };
            }
        }
    }
    Ok(())
}
