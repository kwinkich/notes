use std::fs::File;
use std::io::{Read, Result};

pub fn notes_read(file_name: &str) -> Result<()> {
    let mut file = File::open(format!("{}.txt", file_name)).expect("Error opening file!");
    let mut file_data = String::new();
    file.read_to_string(&mut file_data)
        .expect("Error opening file!");
    print!("\nFile data\n{}\n", file_data);
    Ok(())
}
