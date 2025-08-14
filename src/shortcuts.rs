use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    env,
    fmt::{self},
    fs::File,
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shortcut {
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
    pub fn from(key_combo: String, description: String) -> Shortcut {
        Shortcut {
            lookup_count: 0,
            key_combo,
            description,
        }
    }

    pub fn get_lookup_count(&self) -> u32 {
        self.lookup_count
    }

    pub fn get_key_combo(&self) -> &str {
        &self.key_combo
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

#[derive(Deserialize, Serialize)]
pub struct Shortcuts {
    shortcuts: Vec<Shortcut>,

    #[serde(skip_serializing)]
    save_path: Option<PathBuf>,
}
impl Default for Shortcuts {
    fn default() -> Self {
        Shortcuts::new()
    }
}

impl Shortcuts {
    pub fn new() -> Self {
        Self {
            shortcuts: Vec::new(),
            save_path: None,
        }
    }

    pub fn set_save_path(&mut self, new_save_path: PathBuf) {
        self.save_path = Some(new_save_path);
    }

    pub fn get_all_shortcuts(&self) -> &[Shortcut] {
        self.shortcuts.as_ref()
    }

    pub fn add_shortcut(&mut self, shortcut: Shortcut) {
        self.shortcuts.push(shortcut);
    }

    pub fn increment_lookup_count(&mut self, index: usize, amount: u32) {
        if let Some(shortcut) = self.shortcuts.get_mut(index) {
            shortcut.lookup_count += amount;
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<Shortcut> {
        if index < self.shortcuts.len() {
            Some(self.shortcuts.remove(index))
        } else {
            None
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
    pub fn get_sorted_indexes(&self) -> Vec<usize> {
        // get all indexes
        let mut indexes: Vec<usize> = (0..self.shortcuts.len()).collect();
        // sort them by lookup count desc
        indexes.sort_by_key(|&i| Reverse(self.shortcuts[i].lookup_count));
        indexes
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
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
    pub fn load_from_file(path_maybe: Option<PathBuf>) -> Result<Shortcuts, std::io::Error> {
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
