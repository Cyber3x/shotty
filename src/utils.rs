use std::{
    fmt::Display,
    io::{self, Write},
};

pub fn take_input_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{prompt}");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn take_input_number(prompt: &str) -> Option<i32> {
    let input: i32 = take_input_line(prompt).parse().ok()?;

    Some(input)
}

pub fn print_table<T: Display>(headers: Vec<String>, rows: Vec<Vec<T>>) {
    let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for row in &rows {
        assert_eq!(
            row.len(),
            headers.len(),
            "Row lenght must match headers length"
        );
        for (i, cell) in row.iter().enumerate() {
            let cell_len = cell.to_string().len();
            col_widths[i] = col_widths[i].max(cell_len);
        }
    }

    let print_border = || {
        print!("+");
        for &w in &col_widths {
            print!("-{}-", "-".repeat(w));
            print!("+");
        }
        println!();
    };

    print_border();
    print!("|");
    for (h, &w) in headers.iter().zip(&col_widths) {
        print!(" {:<width$} |", h, width = w);
    }
    println!();
    print_border();

    for row in &rows {
        print!("|");
        for (cell, &w) in row.iter().zip(&col_widths) {
            print!(" {:<width$} |", cell, width = w);
        }
        println!();
    }

    print_border();
}
