use crate::shortcuts::Shortcuts;

pub struct AppState {
    pub shortcuts: Shortcuts,
    pub temp_text: String,
}

impl AppState {
    pub fn new(shortcuts: Shortcuts) -> Self {
        Self {
            shortcuts,
            temp_text: String::from("Hello world"),
        }
    }
}
