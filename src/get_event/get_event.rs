use std::io::{self, Write};

pub fn get_event() -> char {
    let mut event = String::new();
    let mut stdout = io::stdout();

    print!("\nSelect event: ");
    stdout.flush().expect("Failed to flush output");
    io::stdin()
        .read_line(&mut event)
        .expect("Failed to read line");

    event.chars().next().unwrap_or('\0')
}
