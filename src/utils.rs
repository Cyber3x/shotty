use std::io::{self, Write};

pub fn take_input_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{prompt}");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()    
}