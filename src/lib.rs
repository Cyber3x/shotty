use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    error::Error,
    fmt,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

mod commands {
    pub mod add;
    pub mod exit;
    pub mod help;
    pub mod list;
}
mod utils;

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

#[derive(Debug, Serialize, Deserialize)]
struct Shortcut {
    lookup_count: u32,
    key_combo: String,
    description: String,
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.lookup_count, self.key_combo, self.description
        )
    }
}

impl Shortcut {
    fn from(key_combo: String, description: String) -> Shortcut {
        Shortcut {
            lookup_count: 0,
            key_combo,
            description,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Shortcuts {
    shortcuts: Vec<Shortcut>,
}

impl Shortcuts {
    fn new() -> Self {
        Self {
            shortcuts: Vec::new(),
        }
    }

    fn get_all_shortcuts(&self) -> &Vec<Shortcut> {
        self.shortcuts.as_ref()
    }

    fn add_shortcut(&mut self, shortcut: Shortcut) {
        self.shortcuts.push(shortcut);
    }

    fn save(&self, save_path: &PathBuf) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_vec(&self.shortcuts)?;

        println!("{save_path:?}");
        let mut file = File::open(save_path).expect("a valid file should be here");

        file.write_all(&serialized[..])?;

        Ok(())
    }

    fn from_file(path: &PathBuf) -> Result<Shortcuts, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let shortcuts = serde_json::from_str(&contents)?;

        Ok(shortcuts)
    }
}

impl Default for Shortcuts {
    fn default() -> Self {
        Shortcuts::new()
    }
}

struct AppState {
    shortcuts: Shortcuts,
    save_path: PathBuf,
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

    let default_path = env::current_dir()?.join("shortcuts.json");

    if !default_path.exists() {
        File::create(&default_path)?;
    }

    let shortcuts = Shortcuts::from_file(&default_path).unwrap_or(Shortcuts::new());

    let mut command_registry = CommandRegistry::new();
    let mut app_state = AppState {
        shortcuts,
        save_path: default_path,
    };

    println!("{}", app_state.save_path.to_str().unwrap());

    command_registry.add_command("help", Box::new(commands::help::Help));
    command_registry.add_command("add", Box::new(commands::add::Add));
    command_registry.add_command("list", Box::new(commands::list::List));
    // command_registry.add_command("remove", Box::new(Add));
    command_registry.add_command("exit", Box::new(commands::exit::Exit));
    // command_registry.add_command("add", Box::new(Add));
    // command_registry.add_command("add", Box::new(Add));

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
        let input = input.trim();

        match command_registry.get_command(input) {
            None => {
                println!("Command '{}' not found!", input);
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
