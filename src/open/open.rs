use std::fs::File;
use std::io::{Read, Result};
use std::str;

pub fn notes_read(file_name: &str) -> Result<()> {
    let mut file = File::open(format!("{}.txt", file_name)).expect("Error opening file!");

    let mut file_data = String::new();

    file.read_to_string(&mut file_data)
        .expect("Error reading file!");

    if file_data.is_empty() {
        println!("File data is empty");
    } else {
        println!("File data:{}", file_data);
    }

    Ok(())
}
