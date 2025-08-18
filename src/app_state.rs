use crate::shortcuts::Shortcuts;

pub struct AppState {
    pub shortcuts: Shortcuts,
    pub temp_text: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            shortcuts: Shortcuts::new(),
            temp_text: String::from("Hello world"),
        }
    }
}
