use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    collections::HashMap,
    env,
    error::Error,
    fmt::{self},
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

mod commands {
    pub mod add;
    pub mod exit;
    pub mod help;
    pub mod list;
    pub mod remove;
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
            "{} {} ({})",
            self.key_combo, self.description, self.lookup_count
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

    #[serde(skip_serializing)]
    save_path: Option<PathBuf>,
}

impl Shortcuts {
    fn new() -> Self {
        Self {
            shortcuts: Vec::new(),
            save_path: None,
        }
    }

    fn set_save_path(&mut self, new_save_path: PathBuf) {
        self.save_path = Some(new_save_path);
    }

    fn get_all_shortcuts(&self) -> &[Shortcut] {
        self.shortcuts.as_ref()
    }

    fn add_shortcut(&mut self, shortcut: Shortcut) {
        self.shortcuts.push(shortcut);
    }

    fn increment_lookup_count(&mut self, index: usize, amount: u32) {
        if let Some(shortcut) = self.shortcuts.get_mut(index) {
            shortcut.lookup_count += amount;
        }
    }

    /// Returns the indices of all shortcuts, sorted in descending order of `lookup_count`.
    ///
    /// The returned vector contains the zero-based indices into `self.shortcuts`,
    /// ordered so that the shortcut with the highest `lookup_count` appears first.
    /// This can be used to display shortcuts ranked by their popularity or usage frequency.
    ///
    /// # Examples
    /// ```
    /// let sorted_indices = shortcuts.get_sorted_indexes();
    /// // `sorted_indices[0]` is the index of the most frequently looked-up shortcut.
    /// ```
    fn get_sorted_indexes(&self) -> Vec<usize> {
        // get all indexes
        let mut indexes: Vec<usize> = (0..self.shortcuts.len()).collect();
        // sort them by lookup count desc
        indexes.sort_by_key(|&i| Reverse(self.shortcuts[i].lookup_count));
        indexes
    }

    fn save(&self) -> Result<(), std::io::Error> {
        // TODO: remove this unwrap
        let file = File::create(self.save_path.clone().unwrap())?;
        serde_json::to_writer(file, &self)?;

        Ok(())
    }

    /// Loads shortcuts from a JSON file.
    ///
    /// If `path_maybe` is `Some`, that path is used. Otherwise, it defaults to
    /// `shortcuts.json` in the current working directory.
    /// If the file exists, it is read and deserialized into a `Shortcuts` instance.
    /// If it does not exist, a new `Shortcuts` instance is created.
    /// In both cases, the `save_path` is set to the chosen path.
    fn load_from_file(path_maybe: Option<PathBuf>) -> Result<Shortcuts, std::io::Error> {
        let default_path = env::current_dir()?.join("shortcuts.json");

        let path = path_maybe.unwrap_or(default_path);

        let mut shortcuts = if path.exists() {
            let contents = std::fs::read_to_string(&path)?;
            serde_json::from_str(&contents)?
        } else {
            Shortcuts::new()
        };

        shortcuts.set_save_path(path);
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
