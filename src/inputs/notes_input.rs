use std::io::{self, Write}; // Import Write trait

pub fn notes_input() -> String {
    let mut input = String::new();
    let mut stdout = io::stdout();

    print!("\nPlease enter notes name: ");
    stdout.flush().expect("Failed to flush output");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
